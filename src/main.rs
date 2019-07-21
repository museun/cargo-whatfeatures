use gumdrop::Options;
use std::io::Write;
use yansi::Paint;

mod args;
mod crates;
mod error;
mod text;

use args::Args;
use crates::{Dependency, DependencyKind};
use error::{InternalError, UserError};

use text::{AsText, DepState};

pub struct NameVer<'a>(pub &'a str, pub &'a str);
pub struct YankedNameVer<'a>(pub &'a str, pub &'a str);
pub struct NoDeps;

fn main() {
    let args = Args::parse_args_default_or_exit();

    let disable_colors = std::env::var("NO_COLOR").is_ok();
    if disable_colors || args.no_color || cfg!(windows) && !Paint::enable_windows_ascii() {
        Paint::disable();
    }

    let (mut stdout, mut stderr) = (std::io::stdout(), std::io::stderr());

    macro_rules! report_error {
        ($error:expr) => {{
            $error
                .write_as_text(&mut stderr, &Default::default())
                .expect("write error");
            std::process::exit(1);
        }};

        (try $res:expr) => {{
            if let Err(err) = $res {
                report_error!(err)
            }
        }};
    }

    let name = args.name;
    if name.is_empty() {
        report_error!(UserError::NoNameProvided);
    }

    match (*args.features, args.deps) {
        (true, true) => {
            report_error!(try print_features(
                name.clone(),
                args.version.clone(),
                args.list,
                args.show_yanked,
                args.short,
                &mut stdout
            ));

            if !args.list {
                report_error!(try print_deps(name, args.version, false, &mut stdout));
            }
        }
        (false, true) => report_error!(try print_deps(name, args.version, true, &mut stdout)),
        (true, false) => report_error!(try print_features(
            name,
            args.version,
            args.list,
            args.show_yanked,
            args.short,
            &mut stdout
        )),
        (false, false) => report_error!(UserError::MustOutputSomething),
    }
}

fn print_features<W>(
    name: String,
    version: Option<String>,
    list: bool,
    show_yanked: bool,
    short: bool,
    mut writer: W,
) -> Result<(), UserError>
where
    W: Write,
{
    if !list && !show_yanked {
        // if we have a specific version
        if let Some(version) = version {
            match crates::lookup_version(&name, &version) {
                Ok(ref version) if short && version.yanked => {
                    YankedNameVer(&version.crate_, &version.num)
                        .write_as_text(&mut writer, &Default::default())
                        .expect("must be able to write");
                }
                Ok(ref version) if short && !version.yanked => {
                    NameVer(&version.crate_, &version.num)
                        .write_as_text(&mut writer, &Default::default())
                        .expect("must be able to write");
                }
                Ok(version) => {
                    version
                        .write_as_text(&mut writer, &Default::default())
                        .expect("must be able to write");
                }
                Err(..) => return Err(UserError::InvalidVersion(name, version)),
            }
            return Ok(());
        }
    }

    let versions = match crates::lookup_versions(&name) {
        Ok(versions) => {
            if versions.is_empty() {
                return Err(UserError::NoVersions(name));
            }
            versions
        }
        Err(error) => {
            return Err(UserError::CannotLookup {
                name,
                version,
                error,
            })
        }
    };

    for version in versions {
        match (short, version.yanked, show_yanked) {
            (true, true, true) => {
                YankedNameVer(&version.crate_, &version.num)
                    .write_as_text(&mut writer, &Default::default())
                    .expect("must be able to write");
                continue; // to list all pre-release yanked version
            }
            (true, _, _) => {
                NameVer(&version.crate_, &version.num)
                    .write_as_text(&mut writer, &Default::default())
                    .expect("must be able to write");
            }
            (false, true, true) | (false, false, ..) => {
                version
                    .write_as_text(&mut writer, &Default::default())
                    .expect("must be able to write");
                if show_yanked && version.yanked {
                    continue;
                }
            }
            (.., true, false) => continue,
        }
        if !list {
            break;
        }
    }
    Ok(())
}

fn print_deps<W>(
    name: String,
    version: Option<String>,
    show_name: bool,
    mut writer: W,
) -> Result<(), UserError>
where
    W: Write,
{
    let ver = match version {
        Some(ver) => ver,
        None => match crates::lookup_versions(&name) {
            Ok(versions) => match versions.into_iter().skip_while(|k| k.yanked).next() {
                Some(ver) => ver.num,
                None => return Err(UserError::NoVersions(name)),
            },
            Err(error) => {
                return Err(UserError::CannotLookup {
                    name,
                    version,
                    error,
                })
            }
        },
    };

    if show_name {
        NameVer(&name, &ver)
            .write_as_text(&mut writer, &Default::default())
            .expect("must be able to write");
    }

    let deps = match crates::lookup_deps(&name, &ver) {
        Ok(deps) => deps,
        Err(error) => {
            return Err(UserError::CannotLookup {
                name,
                version: Some(ver),
                error,
            })
        }
    };

    if deps.is_empty() {
        NoDeps
            .write_as_text(&mut writer, &Default::default())
            .expect("must be able to write");
        return Ok(());
    }

    let mut deps = deps.into_iter().fold(
        std::collections::HashMap::<DependencyKind, Vec<Dependency>>::new(),
        |mut map, dep| {
            map.entry(dep.kind).or_default().push(dep);
            map
        },
    );

    const KINDS: [DependencyKind; 3] = [
        DependencyKind::Normal,
        DependencyKind::Dev,
        DependencyKind::Build,
    ];

    let mut state = DepState::default();
    for kind in &KINDS {
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

    for kind in &KINDS {
        if let Some(ref mut deps) = deps.get_mut(&kind) {
            state.pad = 2;
            kind.write_as_text(&mut writer, &state)
                .expect("must be able to write");
            state.pad = 4;
            deps.sort_unstable_by(|l, r| l.crate_id.cmp(&r.crate_id));
            for dep in deps.iter() {
                dep.write_as_text(&mut writer, &state)
                    .expect("must be able to write");
            }
        }
    }

    Ok(())
}

#[inline]
fn width(old: usize, s: &str) -> usize {
    std::cmp::max(old, s.chars().map(|c| c.len_utf8()).sum())
}
