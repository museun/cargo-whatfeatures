use cargo_whatfeatures::*;

fn main() -> anyhow::Result<()> {
    let mut args = Args::parse()?;

    let options = cargo_whatfeatures::Options {
        print_features: !args.no_features,
        show_deps: args.show_deps,
        verbose: args.verbose,
        show_private: args.show_private,
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

        return VersionPrinter::new(&mut std::io::stdout(), options)
            .write_versions(&versions, args.show_yanked.unwrap_or_default())
            .map_err(Into::into);
    }

    let workspace = match cargo_whatfeatures::lookup(&args.pkgid, &client)? {
        Lookup::Partial { name, version } => {
            if args.name_only {
                return VersionPrinter::new(&mut std::io::stdout(), options)
                    .write_latest(&name, &version)
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
        Lookup::Workspace(workspace) => {
            if args.name_only {
                let mut packages = workspace
                    .map
                    .values()
                    .map(|pkg| (&pkg.name, &pkg.version, pkg.published))
                    .collect::<Vec<_>>();
                packages.sort_by(|(l, ..), (r, ..)| l.cmp(r));

                return VersionPrinter::new(&mut std::io::stdout(), options)
                    .write_many_versions(packages)
                    .map_err(Into::into);
            }
            workspace
        }
    };

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
