use gumdrop::Options;
use std::io::{stderr, stdout, Write};
use yansi::Paint;

use whatfeatures::{crates, error::*, output::*};

mod args;
use args::Args;

fn list_all_versions<W: Write>(
    args: &Args,
    output: &mut Output<'_, W>,
) -> std::result::Result<(), UserError> {
    let name = &args.name;
    let versions = crates::lookup_versions(&name).map_err(|err| UserError::CannotLookup {
        name: name.to_string(),
        version: args.version.clone(),
        error: err.to_string(),
    })?;

    if versions.is_empty() {
        return Err(UserError::NoVersions {
            name: name.to_string(),
        });
    }

    macro_rules! create_and_output {
        ($ty:ident => $list:ident) => {{
            let mut _vec = Vec::with_capacity(versions.len());
            for version in &versions {
                if !args.show_yanked && version.yanked {
                    continue;
                }
                _vec.push($ty::from_version(&version));
            }

            let model = $list::new(_vec);
            if args.json {
                output.write_json(&model);
            } else {
                output.write_text(&model);
            }
            Ok(())
        }};
    }

    if args.short {
        create_and_output!(SimpleModel => SimpleListModel)
    } else {
        create_and_output!(FeaturesModel => FeaturesListModel)
    }
}

fn display_specific<W: Write>(
    args: &Args,
    version: &str,
    output: &mut Output<'_, W>,
) -> Result<(), UserError> {
    let version =
        crates::lookup_version(&args.name, &version).map_err(|_| UserError::InvalidVersion {
            name: args.name.clone(),
            version: version.to_string(),
        })?;

    macro_rules! create_and_output {
        ($ty:ident) => {{
            let model = $ty::from_version(&version);
            if args.json {
                output.write_json(&model);
            } else {
                output.write_text(&model);
            }
            Ok(())
        }};
    }

    if args.short {
        create_and_output!(SimpleModel)
    } else {
        create_and_output!(FeaturesModel)
    }
}

fn display_deps<W: Write>(args: &Args, output: &mut Output<'_, W>) -> Result<(), UserError> {
    fn get_deps(name: &str, version: &str) -> Result<Vec<crates::Dependency>, UserError> {
        crates::lookup_deps(name, version).map_err(|err| UserError::CannotLookup {
            name: name.to_string(),
            version: Some(version.to_string()),
            error: err.to_string(),
        })
    }

    let (deps, version) = if let Some(ver) = &args.version {
        let version = crates::lookup_version(&args.name, &ver) //
            .map_err(|_| UserError::InvalidVersion {
                name: args.name.clone(),
                version: ver.to_string(),
            })?;
        (get_deps(&args.name, &ver)?, version)
    } else {
        let version = crates::lookup_versions(&args.name)
            .ok()
            .and_then(|vers| {
                vers.into_iter()
                    .skip_while(|k| !args.show_yanked && k.yanked)
                    .next()
            })
            .ok_or_else(|| UserError::NoVersions {
                name: args.name.clone(),
            })?;
        (get_deps(&version.crate_, &version.num)?, version)
    };

    macro_rules! create_and_output {
        ($ty:ident) => {{
            let model = $ty::from_vers_deps(&version, deps);
            if args.json {
                output.write_json(&model);
            } else {
                output.write_text(&model);
            };
            Ok(())
        }};
    }

    if *args.features {
        create_and_output!(CompositeModel)
    } else {
        create_and_output!(DependencyModel)
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
struct WrapError {
    error: UserError,
}

fn ensure_sane_args(args: &Args) {
    let disable_colors = std::env::var("NO_COLOR").is_ok();
    if disable_colors || !*args.color || cfg!(windows) && !Paint::enable_windows_ascii() {
        Paint::disable();
    }

    if args.version.is_some() && args.list {
        let error = UserError::InvalidArgs(&["version", "list"]);
        maybe_abort(args, Err(error));
    }

    if !*args.features && !args.deps {
        let error = UserError::InvalidArgs(&["!features", "!deps"]);
        maybe_abort(args, Err(error));
    }
}

fn maybe_abort(args: &Args, res: Result<(), UserError>) {
    if let Err(error) = res {
        if args.json {
            render_as_json(&WrapError { error }, &mut stderr())
        } else {
            render_as_text(&error, &mut stderr())
        }
        .expect("write error");
        std::process::exit(1);
    }
}

fn main() {
    let args = Args::parse_args_default_or_exit();
    ensure_sane_args(&args);

    if args.name.is_empty() {
        maybe_abort(&args, Err(UserError::NoNameProvided));
    }

    let mut stdout = stdout();
    let mut output = Output::new(&mut stdout);
    if *args.features {
        if args.list {
            maybe_abort(&args, list_all_versions(&args, &mut output));
            return;
        }

        if let Some(version) = &args.version {
            maybe_abort(&args, display_specific(&args, &version, &mut output));
            if !args.deps {
                return;
            }
        }
    }
    if args.deps {
        maybe_abort(&args, display_deps(&args, &mut output));
    }
}
