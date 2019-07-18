use std::io::{Error, ErrorKind, Result, Write};

use super::crates::{Dependency, DependencyKind, Version};
use super::error::UserError;
use super::{NameVer, YankedNameVer};

pub struct Json<W: Write> {
    w: W,
    buf: Vec<u8>,
    lines: Vec<Vec<u8>>,
}

impl<W: Write> Json<W> {
    pub fn new(w: W) -> Self {
        Self {
            w,
            buf: vec![],
            lines: vec![],
        }
    }
}

impl<W: Write> Drop for Json<W> {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

impl<W: Write> Write for Json<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if buf.contains(&b'\n') {
            self.lines.push(std::mem::replace(&mut self.buf, vec![]));
        } else {
            self.buf.extend_from_slice(buf);
        }
        Ok(buf.len())
    }

    // TODO implement a custom serializer now that we're buffering this
    fn flush(&mut self) -> Result<()> {
        if self.lines.is_empty() {
            return Ok(());
        }
        self.w.write_all(&[b'['])?;
        let lines = std::mem::replace(&mut self.lines, vec![]);
        for (i, line) in lines.iter().enumerate() {
            if i > 0 && i < lines.len() {
                self.w.write(&[b','])?;
            }
            self.w.write(&line)?;
        }
        self.w.write_all(&[b']'])?;
        self.w.flush()
    }
}

pub trait AsJson<W> {
    fn write_as_json(&self, writer: &mut W) -> Result<()>;
}

impl<'a, W: Write> AsJson<W> for YankedNameVer<'a> {
    fn write_as_json(&self, writer: &mut W) -> Result<()> {
        let YankedNameVer(name, ver) = self;
        let data = serde_json::to_vec(&serde_json::json!({
            "name": name,
            "version": ver,
            "yanked": true,
        }))
        .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;

        writer.write_all(&data)?;
        writeln!(writer)
    }
}

impl<'a, W: Write> AsJson<W> for NameVer<'a> {
    fn write_as_json(&self, writer: &mut W) -> Result<()> {
        let NameVer(name, ver) = self;
        let data = serde_json::to_vec(&serde_json::json!({
            "name": name,
            "version": ver,
            "yanked": false,
        }))
        .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;

        writer.write_all(&data)?;
        writeln!(writer)
    }
}

impl<W: Write> AsJson<W> for Dependency {
    fn write_as_json(&self, writer: &mut W) -> Result<()> {
        let data = serde_json::to_vec(self) //
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
        writer.write_all(&data)?;
        writeln!(writer)
    }
}

impl<W: Write> AsJson<W> for DependencyKind {
    fn write_as_json(&self, _writer: &mut W) -> Result<()> {
        Ok(())
    }
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
        writer.write_all(&data)?;
        writeln!(writer)?;
        writer.flush()
    }
}
