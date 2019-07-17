use super::*;
use std::io::Write;
use yansi::{Color, Paint};

pub struct Text<W>(W);

impl<W> Text<W> {
    pub fn new(w: W) -> Self {
        Text(w)
    }
}

impl<W: Write> Write for Text<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

impl<W: Write> Output for Text<W> {
    fn output(&mut self, vers: &[Version]) -> std::io::Result<()> {
        fn print_args(
            list: &[String],
            main: Color,
            sep: Color,
            w: &mut impl Write,
        ) -> std::io::Result<()> {
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

        for ver in vers {
            if ver.yanked {
                writeln!(
                    self,
                    "{}: {}/{}",
                    Paint::new("yanked").fg(Color::Red),
                    ver.crate_,
                    Paint::new(&ver.num).fg(Color::Yellow),
                )?;
                continue;
            } else {
                writeln!(
                    self,
                    "{}/{}",
                    &ver.crate_,
                    Paint::new(&ver.num).fg(Color::Yellow)
                )?
            }
            if let Some(default) = ver.features.get("default") {
                write!(self, "{}", Paint::new("    default").fg(Color::Cyan))?;
                if !default.is_empty() {
                    write!(self, "{}", Paint::new(": ").fg(Color::White))?;
                    print_args(&default, Color::Green, Color::White, self)?;
                }
                writeln!(self)?;
            } else {
                writeln!(
                    self,
                    "{}",
                    Paint::new("  no default features").fg(Color::Red)
                )?;
            }
            for (k, v) in &ver.features {
                if k == "default" {
                    continue;
                }
                write!(self, "    {}", Paint::new(&k).fg(Color::Cyan))?;
                if !v.is_empty() {
                    write!(self, "{}", Paint::new(": ").fg(Color::White))?;
                    print_args(&v, Color::Green, Color::White, self)?;
                }
                writeln!(self)?;
            }
        }
        Ok(())
    }

    fn error(&mut self, error: UserError) -> std::io::Result<()> {
        let err_msg = Paint::new("error").fg(Color::Red);
        match error {
            UserError::NoNameProvided => {
                eprintln!("{}: no name was provided", err_msg);
            }
            UserError::CannotLookup {
                name,
                version: Some(version),
                ..
            } => {
                eprintln!("{}: cannot lookup '{}/{}'", err_msg, name, version);
            }
            UserError::CannotLookup { name, .. } => {
                eprintln!(
                    "{}: cannot lookup '{}'. no version found",
                    err_msg,
                    Paint::new(&name).fg(Color::Green)
                );
            }
            UserError::NoVersions(name) => {
                eprintln!(
                    "{}: no versions published for '{}",
                    err_msg,
                    Paint::new(&name).fg(Color::Green),
                );
            }
            UserError::InvalidVersion(name, version) => {
                eprintln!(
                    "{}: invalid version '{}' for '{}'",
                    err_msg,
                    Paint::new(&version).fg(Color::Yellow),
                    Paint::new(&name).fg(Color::Green),
                );
            }
        }
        Ok(())
    }
}
