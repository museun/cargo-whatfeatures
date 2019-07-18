#![allow(dead_code)]

use gumdrop::Options;
use std::io::{Error, ErrorKind, Result, Write};
use yansi::Paint;

mod args;
mod crates;
mod error;
mod json;

use args::Args;
use crates::{Dependency, Version};
use error::{InternalError, UserError};
use json::Json;

pub trait AsJson<W> {
    fn write_as_json(&self, writer: &mut W) -> Result<()>;
}

pub trait AsText<W> {
    fn write_as_text(&self, writer: &mut W) -> Result<()>;
}

impl<W: Write> AsJson<W> for Version {
    fn write_as_json(&self, writer: &mut W) -> Result<()> {
        let data = serde_json::to_vec(&serde_json::json!({
            "yanked": self.yanked,
            "name": self.crate_,
            "version": self.num,
            "features": self.features
        }))
        .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;

        writer.write_all(&data)?;
        writeln!(writer)
    }
}

impl<W: Write> AsText<W> for Version {
    fn write_as_text(&self, writer: &mut W) -> Result<()> {
        use yansi::Color;

        fn print_args(list: &[String], main: Color, sep: Color, w: &mut impl Write) -> Result<()> {
            let len = list.len();
            let sep = Paint::new(",").fg(sep);
            for (i, e) in list.iter().enumerate() {
                if i > 0 && i < len {
                    write!(w, "{} ", sep)?;
                }
                write!(w, "{}", Paint::new(&e).fg(main))?;
            }
            Ok(())
        }

        if self.yanked {
            writeln!(
                writer,
                "{}: {}/{}",
                Paint::new("yanked").fg(Color::Red),
                self.crate_,
                Paint::new(&self.num).fg(Color::Yellow),
            )?;
            return Ok(());
        } else {
            writeln!(
                writer,
                "{}/{}",
                &self.crate_,
                Paint::new(&self.num).fg(Color::Yellow)
            )?
        }

        if let Some(default) = self.features.get("default") {
            write!(writer, "{}", Paint::new("    default").fg(Color::Cyan))?;
            if !default.is_empty() {
                write!(writer, "{}", Paint::new(": ").fg(Color::White))?;
                print_args(&default, Color::Green, Color::White, writer)?;
            }
            writeln!(writer)?;
        } else {
            writeln!(
                writer,
                "{}",
                Paint::new("  no default features").fg(Color::Red)
            )?;
        }

        use std::collections::BTreeMap;
        for (&k, v) in &self.features.iter().collect::<BTreeMap<_, _>>() {
            if k == "default" {
                continue;
            }
            write!(writer, "    {}", Paint::new(&k).fg(Color::Cyan))?;
            if !v.is_empty() {
                write!(writer, "{}", Paint::new(": ").fg(Color::White))?;
                print_args(&v, Color::Green, Color::White, writer)?;
            }
            writeln!(writer)?;
        }
        Ok(())
    }
}

impl<W: Write> AsJson<W> for UserError {
    fn write_as_json(&self, writer: &mut W) -> Result<()> {
        use UserError::*;

        let val = match self {
            NoNameProvided => serde_json::json!({
                "error": "no name provided"
            }),
            CannotLookup {
                name,
                version,
                error,
            } => serde_json::json!({
                "error": "cannot lookup crate",
                "name": name,
                "version": version,
                "inner": error.to_string(),
            }),
            NoVersions(name) => serde_json::json!({
                "error": "no versions published",
                "name": name
            }),
            InvalidVersion(name, version) => serde_json::json!({
                "error": "invalid version",
                "name": name,
                "version": version
            }),
        };

        let data = serde_json::to_vec(&val) //
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;

        writer.write_all(&data)
    }
}

impl<W: Write> AsText<W> for UserError {
    fn write_as_text(&self, writer: &mut W) -> Result<()> {
        use yansi::Color;
        use UserError::*;

        let err_msg = Paint::new("error").fg(Color::Red);
        match self {
            NoNameProvided => writeln!(writer, "{}: no name was provided", err_msg),
            CannotLookup {
                name,
                version: Some(version),
                ..
            } => writeln!(writer, "{}: cannot lookup '{}/{}'", err_msg, name, version),
            CannotLookup { name, .. } => writeln!(
                writer,
                "{}: cannot lookup '{}'. no version found",
                err_msg,
                Paint::new(&name).fg(Color::Green)
            ),
            NoVersions(name) => writeln!(
                writer,
                "{}: no versions published for '{}",
                err_msg,
                Paint::new(&name).fg(Color::Green),
            ),
            InvalidVersion(name, version) => writeln!(
                writer,
                "{}: invalid version '{}' for '{}'",
                err_msg,
                Paint::new(&version).fg(Color::Yellow),
                Paint::new(&name).fg(Color::Green),
            ),
        }
    }
}

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
            write_format!($item, &mut stdout)
        };
        ($item:expr, $output:expr) => {
            if use_json {
                $item.write_as_json($output)
            } else {
                $item.write_as_text($output)
            }
            .expect("write output")
        };
    }

    macro_rules! report_error {
        ($error:expr) => {{
            if use_json {
                // TODO should we write errors as json to stderr instead?
                $error.write_as_json(&mut stdout)
            } else {
                $error.write_as_text(&mut stderr)
            }
            .expect("write error");
            std::process::exit(1);
        }};
    }

    let name = &args.name;
    if name.is_empty() {
        report_error!(UserError::NoNameProvided);
    }

    if !args.deps {
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

        if let Some(ver) = args.version {
            if let Some(ver) = versions.iter().find(|k| &k.num == ver.as_str()) {
                write_format!(&ver);
                return;
            }
            report_error!(UserError::InvalidVersion(name.clone(), ver.clone()))
        }

        if args.list {
            for version in versions {
                write_format!(&version);
            }
            return;
        }

        for ver in versions.into_iter() {
            if !ver.yanked {
                write_format!(&ver);
                break;
            }

            if args.show_yanked {
                write_format!(&ver);
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
