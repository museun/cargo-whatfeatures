use cargo_whatfeatures::*;
use std::path::PathBuf;

enum Offline {
    List,
    Latest,
    CacheMiss,
}

impl Offline {
    fn to_error(&self) -> anyhow::Error {
        let err = match self {
            Self::List => {
                "must be able to connect to https://crates.io to list versions"
            }
            Self::Latest =>{
                "must be able to connect to https://crates.io to get the latest version"
            } ,
            Self::CacheMiss => {
                "crate not found in local registry or cache. must be able to connect to https://crates.io to fetch it"
            },
        };
        anyhow::anyhow!(err)
    }
}

fn main() -> anyhow::Result<()> {
    let mut args = Args::parse()?;

    let mut stdout = std::io::stdout();
    let mut printer = Printer::new(&mut stdout);

    let client = if args.offline {
        None
    } else {
        Some(Client::new("https://crates.io"))
    };

    if args.list {
        let versions = client
            .as_ref()
            .ok_or_else(|| Offline::List.to_error())?
            .list_versions(&args.crate_name)
            .map_err(|_| anyhow::anyhow!("cannot find a crate matching '{}'", args.crate_name))?;

        if versions.is_empty() {
            anyhow::bail!("no versions published for '{}'", args.crate_name)
        }

        if versions.len() == 1 && versions[0].yanked {
            args.show_yanked.replace(YankStatus::Include);
        }

        printer.write_versions(
            &args.crate_name,
            &versions,
            args.show_yanked.unwrap_or_default(),
        )?;
        return Ok(());
    }

    let path: PathBuf = (&args.crate_name).into();
    let local_crate = if path.is_dir() {
        Some(Crate::from_path(path)?)
    } else {
        None
    };

    let (name, version) = match (&local_crate, &args.semver) {
        // we'll use the current version on disk
        (Some(local), ..) => (local.name.clone(), local.version.clone()),
        // try the name and version provided by the user
        (None, Some(ver)) => (args.crate_name, ver.clone()),
        // lookup the latest if none was provided
        (None, None) => {
            let c = client
                .as_ref()
                .ok_or_else(|| Offline::Latest.to_error())?
                .get_latest(&args.crate_name)
                .map_err(|_| {
                    anyhow::anyhow!(
                        "cannot find a crate matching '{}'. maybe the latest version was yanked?",
                        args.crate_name
                    )
                })?;
            (c.name, c.version)
        }
    };

    if args.name_only {
        printer.write_latest(&name, &version)?;
        return Ok(());
    }

    let mut remote_crate = None;
    if local_crate.is_none() {
        let crate_ = match Registry::from_local()?.get(&name, &version) {
            Some(crate_) => crate_.clone(),
            None => client
                .as_ref()
                .ok_or_else(|| Offline::CacheMiss.to_error())?
                .cache_crate(&name, &version)?,
        };

        if let YankState::Yanked = crate_.yanked {
            use yansi::*;
            println!(
                "{}. {}/{} has been yanked on crates.io",
                Paint::yellow("WARNING"),
                crate_.name,
                crate_.version
            );
        }

        remote_crate.replace(crate_);
    }

    let features = match (local_crate, remote_crate) {
        (Some(local), ..) => local,
        (.., Some(remote)) => remote.get_features()?,
        _ => unreachable!(),
    };

    printer.write_header(&features)?;

    // this double negative is weird
    let print_features = !args.no_features;

    if print_features {
        printer.write_features(&features)?;
        printer.write_opt_deps(&features)?;
    }

    if args.show_deps {
        printer.write_deps(&features)?;
        if !print_features {
            printer.write_opt_deps(&features)?;
        }
    }

    Ok(())
}
