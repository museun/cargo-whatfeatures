use crate::printer::YankStatus;
use pico_args::Arguments;
use std::path::{Path, PathBuf};

#[allow(missing_docs)]
#[derive(Debug)]
pub enum Error {
    PkgIdIsLocal,
    NameRequired,

    Exclusive {
        bad: Vec<Vec<String>>,
    },

    ExclusiveWith {
        bad: Vec<Vec<String>>,
        provided_short: String,
        provided_long: String,
    },
    Inclusive {
        bad: Vec<Vec<String>>,
        provided_short: String,
        provided_long: String,
    },

    NoCrateName,
    NoCrateOrPkgId,

    TooManyCrates {
        n: usize,
    },

    UnknownOption {
        option: String,
        allowed: Vec<&'static str>,
    },
}

impl Error {
    fn exclusive<I, A, S>(bad: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: IntoIterator<Item = S>,
        S: ToString,
    {
        Self::Exclusive {
            bad: bad
                .into_iter()
                .map(|s| s.into_iter().map(|s| s.to_string()).collect())
                .collect(),
        }
    }

    fn inclusive_with<I, A, S>(bad: I, short: impl ToString, long: impl ToString) -> Self
    where
        I: IntoIterator<Item = A>,
        A: IntoIterator<Item = S>,
        S: ToString,
    {
        Self::Inclusive {
            bad: bad
                .into_iter()
                .map(|s| s.into_iter().map(|s| s.to_string()).collect())
                .collect(),
            provided_short: short.to_string(),
            provided_long: long.to_string(),
        }
    }

    fn exclusive_with<I, A, S>(bad: I, short: impl ToString, long: impl ToString) -> Self
    where
        I: IntoIterator<Item = A>,
        A: IntoIterator<Item = S>,
        S: ToString,
    {
        Self::ExclusiveWith {
            bad: bad
                .into_iter()
                .map(|s| s.into_iter().map(|s| s.to_string()).collect())
                .collect(),
            provided_short: short.to_string(),
            provided_long: long.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PkgIdIsLocal => {
                write!(f, "pkgid must be a `name:semver` pair, not a local path.")?;
            }

            Self::NameRequired => {
                write!(f, "A package name must be supplied")?;
            }

            Self::Exclusive { bad } => {
                let flags = join_iter(
                    bad.iter().map(|s| s.as_slice()),
                    |n| format!("`{}`", n),
                    |n| format!("`[{}]`", n.join(", ")),
                    " and ",
                );
                write!(f, "the flags {} cannot be used at the same time", flags)?;
            }

            Self::ExclusiveWith {
                bad,
                provided_short,
                provided_long,
            } => {
                let flags = join_iter(
                    bad.iter().map(|s| s.as_slice()),
                    |n| format!("`{}`", n),
                    |n| format!("`[{}]`", n.join(", ")),
                    " or ",
                );
                write!(
                    f,
                    "`[{}, {}]` cannot be used with: {}",
                    provided_short, provided_long, flags
                )?;
            }

            Self::Inclusive {
                bad,
                provided_short,
                provided_long,
            } => {
                let flags = join_iter(
                    bad.iter().map(|s| s.as_slice()),
                    |n| format!("`{}`", n),
                    |n| format!("`[{}]`", n.join(", ")),
                    " or ",
                );
                write!(
                    f,
                    "`[{}, {}]` must be used with one of {}",
                    provided_short, provided_long, flags
                )?;
            }

            Self::NoCrateName => {
                write!(f, "a crate name must be provided")?;
            }

            Self::NoCrateOrPkgId => {
                write!(f, "no crate name, or pkgid spec provided.")?;
            }

            Self::TooManyCrates { n } => {
                write!(
                    f,
                    "too many crate names ({}) were provided. only 1 is allowed",
                    n
                )?;
            }

            Self::UnknownOption { option, allowed } => {
                let options =
                    allowed
                        .iter()
                        .map(|s| format!("'{}'", s))
                        .fold(String::new(), |mut a, c| {
                            if !a.is_empty() {
                                a.push_str(", ");
                            }
                            a.push_str(&c);
                            a
                        });

                write!(
                    f,
                    "unknown option '{}'. only one of [{}] is allowed.",
                    option, options
                )?;
            }
        };

        Ok(())
    }
}

impl std::error::Error for Error {}

fn join_iter<'a, I, S, O, M, W>(iter: I, one: O, many: M, with: W) -> String
where
    S: AsRef<str> + 'a,
    I: Iterator<Item = &'a [S]> + 'a,

    O: Fn(&S) -> String,
    M: Fn(&'a [S]) -> String,
    W: std::fmt::Display,
{
    let sep = with.to_string();
    iter.map(|s| match s {
        [n] => one(n),
        n => many(n),
    })
    .fold(String::new(), |mut a, c| {
        if !a.is_empty() {
            a.push_str(&sep);
        }
        a.push_str(&c);
        a
    })
}

#[derive(PartialEq)]
enum Color {
    Always,
    Auto,
    Never,
}

impl std::str::FromStr for Color {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match &input.to_lowercase()[..] {
            "always" => Ok(Self::Always),
            "auto" => Ok(Self::Auto),
            "never" => Ok(Self::Never),
            option => Err(Error::UnknownOption {
                option: option.to_string(),
                allowed: vec!["always", "auto", "never"],
            }),
        }
    }
}

// TODO verify that 'local' contains a Cargo.toml
// TODO verify that 'semver' is a correct semver
/// A 'pkgid' spec, either local or 'remote'
#[derive(Debug)]
pub enum PkgId {
    /// Remote path (e.g. look it up in the registry)
    Remote {
        /// Name of the crate
        name: String,
        /// Specified semver
        semver: Option<String>,
    }, // TODO supports more registries than just crates.io
    /// Local directory or file
    Local(PathBuf),
}

impl PkgId {
    /// Name of the package
    pub fn name(&self) -> &str {
        match &self {
            Self::Remote { name, .. } => name.as_str(),
            Self::Local(s) => s.to_str().unwrap(),
        }
    }

    /// Whether this is a local package
    pub fn is_local(&self) -> bool {
        match self {
            Self::Local(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for PkgId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Remote { name, semver } => {
                write!(f, "{}", name)?;
                if let Some(ver) = &semver {
                    write!(f, ":{}", ver)?;
                }
                Ok(())
            }
            Self::Local(l) => write!(f, "{}", l.display()),
        }
    }
}

impl std::str::FromStr for PkgId {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let path = Path::new(input);
        if path.is_dir() || path.is_file() {
            return Err(Error::PkgIdIsLocal);
        }

        let mut iter = input.splitn(2, ':');
        let name = iter.next().ok_or_else(|| Error::NameRequired)?;
        let semver = iter.next().map(ToString::to_string);

        let path = Path::new(name);
        if path.is_dir() || path.is_file() {
            return Err(Error::PkgIdIsLocal);
        }

        Ok(Self::Remote {
            name: name.to_string(),
            semver,
        })
    }
}

/// Input for the program
#[derive(Debug)]
pub struct Args {
    /// Verbose output (list leaves, etc.)
    pub verbose: bool,

    /// Should we list all versions?
    pub list: bool,

    /// Should no features be printed out?
    pub no_features: bool,
    /// Should we should the dependencies?
    pub show_deps: bool,
    /// SHould we show yanked versions?
    pub show_yanked: Option<YankStatus>,
    /// Should we show only the name and version?
    pub name_only: bool,

    /// The pkgid specified
    pub pkgid: PkgId,

    /// Don't try to connect to the internet
    pub offline: bool,
}

impl Args {
    fn try_parse_help(args: &mut Arguments) -> anyhow::Result<()> {
        if args.contains(["-V", "--version"]) {
            print_version()
        }

        if args
            .subcommand()?
            .as_deref()
            .filter(|&s| format!("cargo-{}", s) == env!("CARGO_PKG_NAME"))
            .is_none()
        {
            print_help(Help::Cargo)
        }

        match (args.contains("-h"), args.contains("--help")) {
            (true, ..) => print_help(Help::Short),
            (.., true) => print_help(Help::Long),
            _ => {}
        }

        Ok(())
    }

    fn try_parse_cache(args: &mut Arguments) -> anyhow::Result<()> {
        if args.contains("--print-cache-dir") {
            println!("{}", crate::util::cache_dir()?.display());
            std::process::exit(0);
        }

        if args.contains("--purge") {
            let total = crate::Registry::from_local()?.purge_local_cache()?;
            println!(
                "purged {} crates from {}",
                total,
                crate::util::cache_dir()?.display()
            );
            std::process::exit(0)
        }

        Ok(())
    }

    fn try_parse_yank_status(args: &mut Arguments) -> anyhow::Result<Option<YankStatus>> {
        args.opt_value_from_fn(["-y", "--show-yanked"], |s| match s {
            "exclude" => Ok(YankStatus::Exclude),
            "include" => Ok(YankStatus::Include),
            "only" => Ok(YankStatus::Only),
            s => Err(Error::UnknownOption {
                option: s.to_string(),
                allowed: vec!["exclude", "include", "only"],
            }),
        })
        .map_err(Into::into)
    }

    fn try_parse_color(args: &mut Arguments) -> anyhow::Result<()> {
        let color: Option<Color> = args.opt_value_from_str(["-c", "--color"])?;

        let disable_colors = std::env::var("NO_COLOR").is_ok();
        if disable_colors
            || color == Some(Color::Never)
            || cfg!(windows) && !yansi::Paint::enable_windows_ascii()
        {
            yansi::Paint::disable()
        }

        Ok(())
    }

    fn verify_flags(this: Self) -> anyhow::Result<Self> {
        let Self {
            list,
            no_features,
            show_deps,
            name_only,
            pkgid,
            ..
        } = &this;

        /*
        list is exclusive with:
            no_features
            show_deps
            crate_name

        name_only is exclusive with:
            show_deps

        no_features is inclusive with:
            no_features
            show_deps
        */

        if *list {
            let mut bad = vec![];
            if *no_features {
                bad.push(vec!["-n", "--no-features"]);
            }
            if *show_deps {
                bad.push(vec!["-d", "--deps"]);
            }
            if pkgid.is_local() {
                bad.push(vec!["<crate>"]);
            }

            if !bad.is_empty() {
                anyhow::bail!(Error::exclusive_with(bad, "-l", "--list"))
            }
        }

        if *name_only {
            let mut bad = vec![];
            if *show_deps {
                bad.push(vec!["-d", "--deps"]);
            }

            if !bad.is_empty() {
                anyhow::bail!(Error::exclusive_with(bad, "-s", "--short"))
            }
        }

        if *no_features && (!show_deps && !*name_only) {
            anyhow::bail!(Error::inclusive_with(
                vec![vec!["-d", "--deps"], vec!["-s", "--short"]],
                "-n",
                "--no-features"
            ))
        }

        Ok(this)
    }

    /// Parse the arguments
    pub fn parse() -> anyhow::Result<Self> {
        let mut args = pico_args::Arguments::from_env();

        Self::try_parse_help(&mut args)?;
        Self::try_parse_cache(&mut args)?;
        Self::try_parse_color(&mut args)?;

        let show_yanked = Self::try_parse_yank_status(&mut args)?;

        let list = args.contains(["-l", "--list"]);
        let name_only = args.contains(["-s", "--short"]);
        let no_features = args.contains(["-n", "--no-features"]);
        let show_deps = args.contains(["-d", "--deps"]);
        let offline = args.contains(["-o", "--offline"]);
        let verbose = args.contains(["-v", "--verbose"]);

        let manifest_path: Option<PathBuf> = args.opt_value_from_str("--manifest-path")?;
        let mut pkgid: Option<PkgId> = args.opt_value_from_str(["-p", "--pkgid"])?;

        match (&pkgid, &manifest_path) {
            (Some(..), Some(..)) => {
                // "both `[-p, --pkgid]` and `--manifest-path` cannot be used at the same time"
                // TODO this could be done with like 3 less allocations
                anyhow::bail!(Error::exclusive(vec![
                    vec!["-p", "--pkgid"],
                    vec!["--manifest-path"],
                ]));
            }
            (None, Some(..)) | (Some(..), None) => {}
            (None, None) => {}
        };

        // TODO redo all of this
        let mut crate_names = args.free()?;
        match crate_names.len() {
            0 if pkgid.is_some() => {}
            0 if manifest_path.is_some() => {
                pkgid.replace(PkgId::Local(manifest_path.unwrap()));
            }
            0 => anyhow::bail!(Error::NoCrateName),
            n if n > 0 && pkgid.is_some() => anyhow::bail!(Error::exclusive(vec![
                vec!["-p", "--pkgid"],
                vec!["<crate>"]
            ])),
            1 => {
                pkgid.replace(PkgId::Remote {
                    name: crate_names.remove(0),
                    semver: None,
                });
            }
            n => anyhow::bail!(Error::TooManyCrates { n }),
        };

        if pkgid.is_none() {
            anyhow::bail!(Error::NoCrateOrPkgId)
        }

        Self::verify_flags(Self {
            verbose,

            list,

            no_features,
            show_deps,
            show_yanked,
            name_only,

            pkgid: pkgid.unwrap(),

            offline,
        })
    }
}

pub enum Help {
    Long,
    Short,
    Cargo,
}

fn print_help(help: Help) -> ! {
    static CARGO_HELP: &str = "USAGE:
    cargo <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help             Prints this message or the help of the given subcommand(s)
    whatfeatures     the `whatfeatures` command";

    static SHORT_HELP: &str = r#"the `whatfeatures` command

USAGE:
    cargo whatfeatures [FLAGS] [OPTIONS] <crate>

FLAGS:
    -h, --help                  Prints help information
    -V, --version               Displays the program name and version
    -d, --deps                  Display dependencies for the crate
    -n, --no-features           Disable listing the features for the crate
    -l, --list                  List all versions for the crate
    -s, --short                 Display only the name and latest version
    -v, --verbose               Print all leaf nodes and optional deps
    -o, --offline               Don't connect to the internet, limits the availities of this.
    --print-cache-dir           Prints out the path to the cache directory
    --purge                     Purges the local cache

OPTIONS:
    -c, --color <WHEN>          Attempts to use colors when printing as text [default: auto]
    -p, --pkgid <SPEC>          A `pkgid` spec. e.g. cargo:1.43.0
    --manifest-path <PATH>      A path to the Cargo.toml you want to read, locally.
    -y, --show-yanked <yanked>  Shows any yanked versions when using `--list`. [default: exclude].

ARGS:
    <crate>                     The name of a remote crate to retrieve information for.
                                Using this means you want a 'remote' crate.
                                This is exclusive with -p, --pkgid and with --manifest-path.
"#;

    static LONG_HELP: &str = r#"the `whatfeatures` command

    USAGE:
        cargo whatfeatures [FLAGS] [OPTIONS] <crate>

    FLAGS:
        -h, --help
            Prints help information

        -V, --version
            Displays the program name and version

        -d, --deps
            Display dependencies for the crate
            This will list the required dependencies

        -n, --no-features
            Disable listing the features for the crate

        -l, --list
            List all versions for the crate.
            When using the `-y` option, yanked crates can be filtered.

        -s, --short
            Display only the name and latest version, such as foo/0.1.2

        -v, --verbose
            When this is enabled, all 'implied' features will be listed.
            Also, optional dependencies will be listed. Optional deps are technically features.

        -o, --offline
            Don't connect to the internet, limits the availities of this.
            If the crate is in either cargo's local registry, or whatfeatures' cache
            then this will work normally, otherwise it'll give you a nice error.

        --print-cache-dir
            Prints out the path to the cache directory

        --purge
            Purges the local cache. The command will automatically clean up after
            itself if it sees the crate in the cargo local registry. If its not
            in the cargo registry, it'll download the crate from crates.io and place
            it in its cache. This flag causes that cache to become invalidated.

            The cache is located at these locations:
            * Linux: $XDG_CACHE_HOME/museun/whatfeatures
            * Windows: %LOCALAPPDATA/museun/whatfeatures
            * macOS: $HOME/Library/Caches/museun/whatfeatures

    OPTIONS:
        -c, --color [always, auto, never]
            Attempts to use colors when printing as text [default: auto]
            *NOTE* When NO_COLOR is set to any value, all colors will be disabled

        -p, --pkgid <semver>
            A specific version to lookup. e.g. 0.7.1
            If this is not provided, then the latest crate is used.

        --manifest-path <PATH>
            A path to the Cargo.toml you want to read, locally.
            Use this to read from a local crate, rather than a remote one.

        -y, --show-yanked <exclude, include, only>
            Shows any yanked versions when using `--list`. [default: exclude].
            When 'exclude' is provided, only active releases versions will be listed
            When 'include' is provided, the listing will include yanked versions along with active releases.
            When 'only' is provided, only yanked versions will be listed

    ARGS:
        <crate>  The name of the remote crate to retrieve information for.
                 Using this means you want a 'remote' crate.
                 This is exclusive with -p, --pkgid and with --manifest-path.
"#;

    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    match help {
        Help::Long => println!("{}", LONG_HELP),
        Help::Short => println!("{}", SHORT_HELP),
        Help::Cargo => println!("{}", CARGO_HELP),
    }

    std::process::exit(0)
}

fn print_version() -> ! {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    std::process::exit(0)
}
