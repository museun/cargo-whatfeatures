use crate::printer::YankStatus;
use pico_args::Arguments;

/// Input for the program
#[derive(Debug)]
pub struct Args {
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
    /// The specified version, if any
    pub semver: Option<String>,
    /// The specified crate name
    pub crate_name: String,
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
            .filter(|&s| s == env!("CARGO_PKG_NAME"))
            .is_none()
        {
            print_help(Help::Cargo)
        }

        if args.contains("-h") {
            print_help(Help::Short)
        }

        if args.contains("--help") {
            print_help(Help::Long)
        }

        if args.contains(["-V", "--version"]) {
            print_version()
        }

        Ok(())
    }

    fn try_parse_cache(args: &mut Arguments) -> anyhow::Result<()> {
        if args.contains(["-p", "--print-cache-dir"]) {
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
            semver,
            ..
        } = &this;

        let map = [
            ("no_features", ["-n", "--no-features"]),
            ("show_deps", ["-d", "--deps"]),
            ("name_only", ["-s", "--short"]),
            ("semver", ["-v", "--vers"]),
            ("list", ["-l", "--list"]),
        ]
        .iter()
        .copied()
        .collect::<std::collections::HashMap<_, _>>();

        let mut bad_flags = vec![];

        macro_rules! lookup {
            ($flag:expr => $expr:expr) => {
                if $expr {
                    bad_flags.push({
                        let [s, l] = map.get(stringify!($flag)).unwrap();
                        format!("`{}, {}`", s, l)
                    });
                }
            };
        }

        macro_rules! maybe_error {
            ($flag:expr) => {{
                if !bad_flags.is_empty() {
                    let [s, l] = map.get($flag).unwrap();
                    anyhow::bail!(
                        "the flag `{}, {}` cannot be used with {}",
                        s,
                        l,
                        bad_flags.join(" OR ")
                    );
                }
            }};
        }

        if *list {
            lookup!(no_features => *no_features);
            lookup!(show_deps => *show_deps);
            lookup!(name_only => *name_only);
            lookup!(semver => semver.is_some());
            maybe_error!("list");
        }

        if *name_only {
            lookup!(show_deps => *show_deps);
            lookup!(list => *list);
            maybe_error!("name_only");
        }

        if *no_features && (!*show_deps && !*name_only) {
            let f = |a| {
                let [s, l] = map.get(a).unwrap();
                format!("`{}, {}`", s, l)
            };

            let flag = f("no_features");
            let deps = f("show_deps");
            let name = f("name_only");

            anyhow::bail!("{} must also be used with {} OR {}", flag, deps, name);
        }

        Ok(this)
    }

    /// Parse the arguments
    pub fn parse() -> anyhow::Result<Self> {
        let mut args = pico_args::Arguments::from_env();

        Self::try_parse_help(&mut args)?;
        Self::try_parse_cache(&mut args)?;
        Self::try_parse_color(&mut args)?;

        let list = args.contains(["-l", "--list"]);
        let name_only = args.contains(["-s", "--short"]);
        let no_features = args.contains(["-n", "--no-features"]);
        let show_deps = args.contains(["-d", "--deps"]);
        let offline = args.contains(["-o", "--offline"]);

        let semver: Option<String> = args.opt_value_from_str(["-v", "--vers"])?;

        let show_yanked = args.opt_value_from_fn(["-y", "--show-yanked"], |s| match s {
            "exclude" => Ok(YankStatus::Exclude),
            "include" => Ok(YankStatus::Include),
            "only" => Ok(YankStatus::Only),
            s => Err(format!(
                "unknown option '{}'. only one of ['exclude', 'include', 'only'] is allowed.",
                s
            )),
        })?;

        let mut crate_names = args.free()?;
        let crate_name = match crate_names.len() {
            0 => anyhow::bail!("a crate name must be provided"),
            1 => crate_names.remove(0),
            n => anyhow::bail!(
                "too many crate names ({}) were provided. only 1 is allowed",
                n
            ),
        };

        Self::verify_flags(Self {
            list,
            no_features,
            show_deps,
            show_yanked,
            name_only,
            semver,
            crate_name,
            offline,
        })
    }
}

#[derive(PartialEq)]
enum Color {
    Always,
    Auto,
    Never,
}

impl std::str::FromStr for Color {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match &input.to_lowercase()[..] {
            "always" => Ok(Self::Always),
            "auto" => Ok(Self::Auto),
            "never" => Ok(Self::Never),
            _ => Err("only [always, auto, never] are supported"),
        }
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
    -o, --offline               Don't connect to the internet, limits the availities of this.
    -p, --print-cache-dir       Prints out the path to the cache directory
    --purge                     Purges the local cache

OPTIONS:
    -c, --color <WHEN>          Attempts to use colors when printing as text [default: auto]
    -v, --vers <semver>         A specific version to lookup. e.g. 0.7.1
    -y, --show-yanked <yanked>  Shows any yanked versions when using `--list`. [default: exclude].

ARGS:
    <crate>                     The name of the crate to retrieve information for. This can be a local path.
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

        -o, --offline
            Don't connect to the internet, limits the availities of this.
            If the crate is in either cargo's local registry, or whatfeatures' cache
            then this will work normally, otherwise it'll give you a nice error.

        -p, --print-cache-dir   
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

        -v, --vers <semver>
            A specific version to lookup. e.g. 0.7.1
            If this is not provided, then the latest crate is used.

        -y, --show-yanked <exclude, include, only>
            Shows any yanked versions when using `--list`. [default: exclude].
            When 'exclude' is provided, only active releases versions will be listed
            When 'include' is provided, the listing will include yanked versions along with active releases.
            When 'only' is provided, only yanked versions will be listed

    ARGS:
        <crate>  The name of the crate to retrieve information for. 
        This can be a local path, to whichever directly contains a 'Cargo.toml'
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
