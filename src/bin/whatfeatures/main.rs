use cargo_whatfeatures::*;

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

    // if we are just listing the versions
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
        // then we can bail early
        return Ok(());
    }

    let version = match args.semver {
        Some(ver) => ver,
        // lookup the latest if none was provided
        None => {
            client
                .as_ref()
                .ok_or_else(|| Offline::Latest.to_error())?
                .get_latest(&args.crate_name)
                .map_err(|_| {
                    anyhow::anyhow!(
                        "cannot find a crate matching '{}'. maybe the latest version was yanked?",
                        args.crate_name
                    )
                })?
                .version
        }
    };

    // if we're only going to print out the name/version
    if args.name_only {
        printer.write_latest(&args.crate_name, &version)?;
        // then we can bail early
        return Ok(());
    }

    let registry = Registry::from_local()?;

    let crate_ = match registry.get(&args.crate_name, &version) {
        Some(crate_) => crate_.clone(),
        None => client
            .as_ref()
            .ok_or_else(|| Offline::CacheMiss.to_error())?
            .cache_crate(&args.crate_name, &version)?,
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

    let features = crate_.get_features()?;
    printer.write_header(&features)?;

    // this double negative is weird
    let print_features = !args.no_features;

    if print_features {
        printer.write_features(&features)?;
        printer.write_opt_deps(&features)?;
    }

    if args.show_deps {
        printer.write_deps(&features)?;
    }

    Ok(())
}
