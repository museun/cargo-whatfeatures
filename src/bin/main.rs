use std::io::{stderr, stdout, Write};

use structopt::StructOpt;
use yansi::Paint;

use cargo_whatfeatures::{crates, error::*, output::*};
mod args;
use args::{Args, Command};

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

fn display_features<W: Write>(args: &Args, output: &mut Output<'_, W>) -> Result<(), UserError> {
    let version = get_version(&args)?;

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
    let version = get_version(&args)?;
    let deps = crates::lookup_deps(&version.crate_, &version.num).map_err(|err| {
        UserError::CannotLookup {
            name: version.crate_.to_string(),
            version: Some(version.num.to_string()),
            error: err.to_string(),
        }
    })?;

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

    if args.no_features {
        create_and_output!(DependencyModel)
    } else {
        create_and_output!(CompositeModel)
    }
}

fn get_version(args: &Args) -> Result<crates::Version, UserError> {
    if let Some(ver) = &args.version {
        crates::lookup_version(&args.name, &ver) //
            .map_err(|_| UserError::InvalidVersion {
                name: args.name.clone(),
                version: ver.to_string(),
            })
    } else {
        crates::lookup_versions(&args.name)
            .ok()
            .and_then(|vers| {
                vers.into_iter()
                    .skip_while(|k| !args.show_yanked && k.yanked)
                    .next()
            })
            .ok_or_else(|| UserError::NoVersions {
                name: args.name.clone(),
            })
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
struct WrapError {
    error: UserError,
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
    let args = Command::from_args();
    let Command::Whatfeatures(args) = args;

    let disable_colors = std::env::var("NO_COLOR").is_ok();
    if disable_colors || !*args.color || cfg!(windows) && !Paint::enable_windows_ascii() {
        Paint::disable();
    }

    let mut stdout = stdout();
    let mut output = Output::new(&mut stdout);
    if !args.no_features {
        if args.list {
            maybe_abort(&args, list_all_versions(&args, &mut output));
            return;
        }
        if !args.deps {
            maybe_abort(&args, display_features(&args, &mut output));
            return;
        }
    }
    if args.deps {
        maybe_abort(&args, display_deps(&args, &mut output));
    }
}
