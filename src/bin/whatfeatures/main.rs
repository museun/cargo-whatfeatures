use std::collections::HashSet;

use cargo_whatfeatures::*;

fn real_main(mut args: Args) -> anyhow::Result<()> {
    let options = cargo_whatfeatures::Options {
        print_features: !args.no_features,
        show_deps: args.show_deps,
        verbose: args.verbose,
        show_private: args.show_private,
        theme: args.theme,
    };

    let client = if args.offline {
        None
    } else {
        Some(Client::new("https://crates.io"))
    };

    let name = args.pkgid.name();

    if args.list {
        let versions = client
            .as_ref()
            .ok_or_else(|| OfflineError::List.to_error())?
            .list_versions(name)
            .map_err(|_| anyhow::anyhow!("cannot find a crate matching '{}'", &args.pkgid))?;

        if versions.is_empty() {
            anyhow::bail!("no versions published for '{}'", &args.pkgid)
        }

        if versions.len() == 1 && versions[0].yanked {
            args.show_yanked.replace(YankStatus::Include);
        }

        if args.list && args.json {
            let unique = versions.iter().map(|c| &c.name).collect::<HashSet<_>>();
            anyhow::ensure!(!unique.is_empty(), "no crates were found");
            anyhow::ensure!(
                unique.len() == 1,
                "program is in an invalid state. expected 1 crate, found {}",
                unique.len()
            );

            let name = unique.into_iter().next().unwrap().clone();
            let json = cargo_whatfeatures::json::create_crates_from_versions(&name, versions);
            println!("{json}");
            std::process::exit(0)
        }

        return VersionPrinter::new(&mut std::io::stdout(), options)
            .write_versions(
                &versions,
                args.show_yanked.unwrap_or_default(),
                args.verbose,
            )
            .map_err(Into::into);
    }

    let mut out = std::io::stdout();

    let workspace = match cargo_whatfeatures::lookup(&args.pkgid, &client, args.local_only)? {
        Lookup::Partial(vers) => {
            let Version { name, version, .. } = &vers;

            if args.name_only {
                if args.json {
                    let json = cargo_whatfeatures::json::create_crates_from_versions(
                        name,
                        Some(vers.clone()),
                    );
                    println!("{json}");
                    std::process::exit(0)
                }

                return VersionPrinter::new(&mut std::io::stdout(), options)
                    .write_latest_version(&vers, args.verbose)
                    .map_err(Into::into);
            }

            let crate_ = match Registry::from_local()?.get(&name, &version) {
                Some(crate_) => crate_.clone(),
                None => match client
                    .as_ref()
                    .ok_or_else(|| OfflineError::CacheMiss.to_error())?
                    .cache_crate(&name, &version)
                {
                    Ok(res) => res,
                    Err(_err) => return cannot_lookup(&args.pkgid),
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

            crate_.get_features()?
        }

        pkg @ Lookup::LocalCache(..) | pkg @ Lookup::Workspace(..) => {
            let local = matches!(pkg, Lookup::LocalCache { .. });
            let pkg = match pkg {
                Lookup::LocalCache(pkg) | Lookup::Workspace(pkg) => pkg,
                _ => unreachable!(),
            };

            if local {
                use std::io::Write as _;
                let msg = args.theme.warning.paint(format!(
                    "WARNING: {}",
                    cargo_whatfeatures::labels::POSSIBLY_OLD_CRATE
                ));
                writeln!(out, "{}", msg)?;
            }

            if args.name_only {
                let mut packages = pkg
                    .map
                    .values()
                    .map(|pkg| (&pkg.name, &pkg.version, pkg.published))
                    .collect::<Vec<_>>();
                packages.sort_by(|(l, ..), (r, ..)| l.cmp(r));

                if args.json {
                    let json =
                        cargo_whatfeatures::json::create_crates_from_workspace(&pkg.hint, packages);
                    println!("{json}");
                    std::process::exit(0)
                }

                VersionPrinter::new(&mut out, options).write_many_versions(packages)?;
                return Ok(());
            }
            pkg
        }
    };

    if args.json {
        let json = cargo_whatfeatures::json::workspace(workspace);
        println!("{json}");
        std::process::exit(0)
    }

    WorkspacePrinter::new(&mut std::io::stdout(), workspace, options).print()?;
    Ok(())
}

fn cannot_lookup(pkgid: &PkgId) -> anyhow::Result<()> {
    let mut out = format!("cannot lookup crate '{}'.", &pkgid);
    if let PkgId::Remote {
        semver: Some(semver),
        ..
    } = pkgid
    {
        out.push_str(&format!(
            " perhaps this is an invalid semver: '{}'?",
            semver
        ));
    }

    anyhow::bail!(out)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse()?;
    let theme = args.theme;

    if let Err(err) = real_main(args) {
        eprintln!("{}: {}", theme.error.paint("ERROR"), err);
        std::process::exit(1)
    }

    Ok(())
}
