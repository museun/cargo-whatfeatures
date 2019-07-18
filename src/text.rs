use std::io::{Result, Write};
use yansi::{Color, Paint};

use super::crates::{Dependency, DependencyKind, Version};
use super::error::UserError;
use super::{NameVer, YankedNameVer};

pub trait AsText<W> {
    type State: Default;
    fn write_as_text(&self, writer: &mut W, state: &Self::State) -> Result<()>;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct DepState {
    pub pad: usize,
    pub left: usize,
    pub right: usize,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct NoTextState;

impl<'a, W: Write> AsText<W> for NameVer<'a> {
    type State = NoTextState;
    fn write_as_text(&self, writer: &mut W, _state: &Self::State) -> Result<()> {
        let NameVer(name, ver) = self;
        writeln!(writer, "{}/{}", name, Paint::new(&ver).fg(Color::Yellow))
    }
}

impl<'a, W: Write> AsText<W> for YankedNameVer<'a> {
    type State = NoTextState;
    fn write_as_text(&self, writer: &mut W, _state: &Self::State) -> Result<()> {
        let YankedNameVer(name, ver) = self;
        writeln!(
            writer,
            "{}: {}/{}",
            Paint::new("yanked").fg(Color::Red),
            name,
            Paint::new(&ver).fg(Color::Yellow)
        )
    }
}

impl<W: Write> AsText<W> for Dependency {
    type State = DepState;
    fn write_as_text(&self, writer: &mut W, state: &Self::State) -> Result<()> {
        if let Some(target) = &self.target {
            writeln!(
                writer,
                "{}{: <l$} = {: <r$} if {}",
                " ".repeat(state.pad),
                Paint::new(&self.crate_id).fg(Color::Green),
                Paint::new(&self.req).fg(Color::Yellow),
                Paint::new(&target).fg(Color::Cyan),
                l = state.left,
                r = state.right,
            )
        } else {
            writeln!(
                writer,
                "{}{: <l$} = {}",
                " ".repeat(state.pad),
                Paint::new(&self.crate_id).fg(Color::Green),
                Paint::new(&self.req).fg(Color::Yellow),
                l = state.left,
            )
        }
    }
}

impl<W: Write> AsText<W> for DependencyKind {
    type State = DepState;
    fn write_as_text(&self, writer: &mut W, state: &Self::State) -> Result<()> {
        match self {
            DependencyKind::Normal => writeln!(
                writer,
                "{}{}",
                " ".repeat(state.pad),
                Paint::new("normal").fg(Color::Magenta)
            ),
            DependencyKind::Dev => writeln!(
                writer,
                "{}{}",
                " ".repeat(state.pad),
                Paint::new(&"dev").fg(Color::Blue)
            ),
            DependencyKind::Build => writeln!(
                writer,
                "{}{}",
                " ".repeat(state.pad),
                Paint::new(&"build").fg(Color::Red)
            ),
        }
    }
}

impl<W: Write> AsText<W> for Version {
    type State = NoTextState;
    fn write_as_text(&self, writer: &mut W, _state: &Self::State) -> Result<()> {
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

impl<W: Write> AsText<W> for UserError {
    type State = NoTextState;

    fn write_as_text(&self, writer: &mut W, _state: &Self::State) -> Result<()> {
        use UserError::*;

        let err_msg = Paint::new("error").fg(Color::Red);
        match self {
            MustOutputSomething => writeln!(
                writer,
                "{}: `-f false` cannot be used without --deps",
                err_msg
            ),
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
