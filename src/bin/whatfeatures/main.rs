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

    let name = args.pkgid.name();

    if args.list {
        let versions = client
            .as_ref()
            .ok_or_else(|| Offline::List.to_error())?
            .list_versions(name)
            .map_err(|_| anyhow::anyhow!("cannot find a crate matching '{}'", &args.pkgid))?;

        if versions.is_empty() {
            anyhow::bail!("no versions published for '{}'", &args.pkgid)
        }

        if versions.len() == 1 && versions[0].yanked {
            args.show_yanked.replace(YankStatus::Include);
        }

        printer.write_versions(&versions, args.show_yanked.unwrap_or_default())?;
        return Ok(());
    }

    let (name, version, mut features) = match &args.pkgid {
        PkgId::Remote {
            name,
            semver: Some(semver),
        } => (name.clone(), semver.clone(), None),

        PkgId::Remote { name, .. } => {
            let c = client
                .as_ref()
                .ok_or_else(|| Offline::Latest.to_error())?
                .get_latest(name)
                .map_err(|_| {
                    anyhow::anyhow!(
                        "cannot find a crate matching '{}'. maybe the latest version was yanked?",
                        &args.pkgid
                    )
                })?;

            (c.name, c.version, None)
        }

        PkgId::Local(path) => {
            let features = Crate::from_path(path)?;
            (
                features.name.to_string(),
                features.version.to_string(),
                Some(features),
            )
        }
    };

    if args.name_only {
        printer.write_latest(&name, &version)?;
        return Ok(());
    }

    if !args.pkgid.is_local() {
        let crate_ = match Registry::from_local()?.get(&name, &version) {
            Some(crate_) => crate_.clone(),
            None => match client
                .as_ref()
                .ok_or_else(|| Offline::CacheMiss.to_error())?
                .cache_crate(&name, &version)
            {
                Ok(res) => res,
                Err(_err) => {
                    let mut out = format!("cannot lookup crate '{}'.", &args.pkgid);
                    if let PkgId::Remote {
                        semver: Some(semver),
                        ..
                    } = args.pkgid
                    {
                        out.push_str(&format!(
                            " perhaps this is an invalid semver: '{}'?",
                            semver
                        ));
                    }
                    anyhow::bail!(out);
                }
            },
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

        features.replace(crate_.get_features()?);
    }

    let features = match features {
        Some(features) => features,
        None => anyhow::bail!("cannot lookup features for '{}'", &args.pkgid),
    };

    printer.write_header(&features)?;

    // this double negative is weird
    let print_features = !args.no_features;

    if print_features {
        printer.write_features(&features, args.verbose)?;
        if args.verbose {
            printer.write_opt_deps(&features, args.verbose)?;
        }
    }

    if args.show_deps {
        printer.write_deps(&features, args.verbose)?;
        if !print_features {
            printer.write_opt_deps(&features, args.verbose)?;
        }
    }

    Ok(())
}
