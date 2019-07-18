use gumdrop::Options;
use std::io::Write;
use yansi::Paint;

mod args;
mod crates;
mod error;
mod json;
mod text;

use args::Args;
use crates::{Dependency, DependencyKind};
use error::{InternalError, UserError};

use json::{AsJson, Json};
use text::{AsText, DepState};

pub struct NameVer<'a>(pub &'a str, pub &'a str);
pub struct YankedNameVer<'a>(pub &'a str, pub &'a str);

fn main() {
    let args = Args::parse_args_default_or_exit();

    let disable_colors = std::env::var("NO_COLOR").is_ok();
    if disable_colors || args.no_color || cfg!(windows) && !Paint::enable_windows_ascii() {
        Paint::disable();
    }

    let use_json = args.json;
    let (mut stdout, mut stderr) = make_stdio(use_json);

    macro_rules! write_format {
        ($item:expr) => {
            write_format!($item, &mut stdout, &Default::default())
        };
        ($item:expr, $output:expr, $pad:expr) => {
            if use_json {
                $item.write_as_json($output)
            } else {
                $item.write_as_text($output, $pad)
            }
            .expect("write output")
        };
        ($item:expr, state=> $state:expr) => {
            write_format!($item, &mut stdout, $state)
        };
    }

    macro_rules! report_error {
        ($error:expr) => {{
            if use_json {
                $error.write_as_json(&mut stdout)
            } else {
                $error.write_as_text(&mut stderr, &Default::default())
            }
            .expect("write error");
            std::process::exit(1);
        }};
    }

    let name = &args.name;
    if name.is_empty() {
        report_error!(UserError::NoNameProvided);
    }

    if !*args.features && !args.deps {
        report_error!(UserError::MustOutputSomething)
    }

    if *args.features {
        let versions = crates::lookup_versions(&name).unwrap_or_else(|err| {
            report_error!(UserError::CannotLookup {
                name: name.clone(),
                version: args.version.clone(),
                error: err,
            });
        });

        if versions.is_empty() {
            report_error!(UserError::NoVersions(name.clone()))
        }

        if let Some(ver) = &args.version {
            if let Some(ver) = versions.iter().find(|k| &k.num == ver.as_str()) {
                write_format!(&ver);
            } else {
                report_error!(UserError::InvalidVersion(name.clone(), ver.clone()))
            }
        }

        if args.list {
            for version in versions {
                if args.only_version {
                    if version.yanked {
                        let nv = YankedNameVer(&version.crate_, &version.num);
                        write_format!(&nv);
                    } else {
                        let nv = NameVer(&version.crate_, &version.num);
                        write_format!(&nv);
                    }
                } else {
                    write_format!(&version);
                }
            }
            return;
        }

        if args.version.is_none() {
            for ver in versions.into_iter() {
                if !ver.yanked {
                    if args.only_version {
                        let nv = NameVer(&ver.crate_, &ver.num);
                        write_format!(&nv);
                    } else {
                        write_format!(&ver);
                    }
                    break;
                }

                if args.show_yanked {
                    if args.only_version {
                        let nv = YankedNameVer(&ver.crate_, &ver.num);
                        write_format!(&nv);
                    } else {
                        write_format!(&ver);
                    }
                }
            }
        }
    }

    if !args.deps {
        return;
    }

    let ver = match args.version {
        Some(ver) => ver.clone(),
        None => {
            let versions = crates::lookup_versions(&name).unwrap_or_else(|err| {
                report_error!(UserError::CannotLookup {
                    name: name.clone(),
                    version: args.version.clone(),
                    error: err,
                });
            });

            if versions.is_empty() {
                report_error!(UserError::NoVersions(name.clone()))
            }
            match versions.into_iter().skip_while(|k| k.yanked).next() {
                Some(ver) => ver.num,
                None => report_error!(UserError::NoVersions(name.clone())),
            }
        }
    };

    let deps = crates::lookup_deps(&name, &ver).unwrap_or_else(|err| {
        report_error!(UserError::CannotLookup {
            name: name.clone(),
            version: Some(ver.clone()),
            error: err,
        });
    });

    if deps.is_empty() {
        report_error!(UserError::InvalidVersion(name.clone(), ver.clone()))
    }

    use std::collections::HashMap;
    let mut deps = deps.into_iter().fold(
        HashMap::<DependencyKind, Vec<Dependency>>::new(),
        |mut map, dep| {
            map.entry(dep.kind.clone()).or_default().push(dep);
            map
        },
    );

    let kinds = &[
        DependencyKind::Normal,
        DependencyKind::Dev,
        DependencyKind::Build,
    ];

    let mut state = DepState::default();
    for kind in kinds {
        if let Some(dep) = deps.get(&kind) {
            let (left, right) = dep
                .iter()
                .fold((state.left, state.right), |(left, right), d| {
                    (width(left, &d.crate_id), width(right, &d.req))
                });

            state.left = left;
            state.right = right;
        }
    }

    write_format!(NameVer(&name, &ver));
    for kind in kinds {
        if let Some(ref mut deps) = deps.get_mut(&kind) {
            state.pad = 2;
            write_format!(&kind, state=>&state);
            state.pad = 4;
            deps.sort_unstable_by(|l, r| l.crate_id.cmp(&r.crate_id));
            for dep in deps.iter() {
                write_format!(&dep, state=>&state);
            }
        }
    }
}

fn make_stdio(use_json: bool) -> (Box<dyn Write>, Box<dyn Write>) {
    let (stdout, stderr) = (std::io::stdout(), std::io::stderr());
    if use_json {
        (Box::new(Json::new(stdout)), Box::new(Json::new(stderr)))
    } else {
        (Box::new(stdout), Box::new(stderr))
    }
}

#[inline]
fn width(old: usize, s: &str) -> usize {
    std::cmp::max(old, s.chars().map(|c| c.len_utf8()).sum())
}
