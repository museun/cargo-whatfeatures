use super::*;
use std::io::Write;

pub struct Text<W>(pub W);

impl<W: Write> Write for Text<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

impl<W: Write> Output for Text<W> {
    fn output(&mut self, vers: &[CrateVersion]) -> std::io::Result<()> {
        for ver in vers {
            if ver.yanked {
                writeln!(self, "{}/{} has been yanked", ver.crate_, ver.num)?
            } else {
                writeln!(self, "{}/{}", ver.crate_, ver.num)?
            }
            if let Some(default) = ver.features.get("default") {
                write!(self, "    default")?;
                if !default.is_empty() {
                    write!(self, ": {}", default.join(", "))?;
                }
                writeln!(self)?;
            } else {
                writeln!(self, "  no default features")?;
            }
            for (k, v) in &ver.features {
                if k == "default" {
                    continue;
                }
                write!(self, "    {}", k)?;
                if !v.is_empty() {
                    write!(self, ": {}", v.join(", "))?;
                }
                writeln!(self)?;
            }
        }
        Ok(())
    }

    fn error(&mut self, error: Error) -> std::io::Result<()> {
        match error {
            Error::NoNameProvided => {
                eprintln!("no name was provided");
            }
            Error::CannotLookup {
                name,
                version: Some(version),
                error,
            } => {
                eprintln!("cannot lookup '{}/{}': {}", name, version, error);
            }
            Error::CannotLookup { name, error, .. } => {
                eprintln!("cannot lookup '{}': {}", name, error);
            }
            Error::NoVersions(name) => {
                eprintln!("no versions published for '{}", name);
            }
            Error::InvalidVersion(name, version) => {
                eprintln!("invalid version '{}' for '{}'", version, name);
            }
        }
        Ok(())
    }
}
