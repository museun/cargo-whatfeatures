use super::*;
use std::io::Write;

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
        self.flush().expect("must be able to flush json object");
    }
}

impl<W: Write> Write for Json<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf.contains(&b'\n') {
            self.lines.push(std::mem::replace(&mut self.buf, vec![]));
        } else {
            self.buf.extend_from_slice(buf);
        }
        Ok(buf.len())
    }

    // TODO implement a custom serializer now that we're buffering this
    fn flush(&mut self) -> std::io::Result<()> {
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

impl<W: Write> Output for Json<W> {
    fn output(&mut self, vers: &[Version]) -> std::io::Result<()> {
        let len = vers.len();
        for (i, ver) in vers.iter().enumerate() {
            if i > 0 && i < len {
                self.write_all(&[b','])?;
            }
            let data = serde_json::to_vec(&serde_json::json!({
                "yanked": ver.yanked,
                "name": ver.crate_,
                "version": ver.num,
                "features": ver.features
            }))
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
            self.write_all(&data)?;
            self.write_all(&[b'\n'])?;
        }
        Ok(())
    }

    fn error(&mut self, error: UserError) -> std::io::Result<()> {
        let val = match error {
            UserError::NoNameProvided => serde_json::json!({
                "error": "no name provided"
            }),
            UserError::CannotLookup {
                name,
                version,
                error,
            } => serde_json::json!({
                "error": "cannot lookup crate",
                "name": name,
                "version": version,
                "inner": error.to_string(),
            }),
            UserError::NoVersions(name) => serde_json::json!({
                "error": "no versions published",
                "name": name
            }),
            UserError::InvalidVersion(name, version) => serde_json::json!({
                "error": "invalid version",
                "name": name,
                "version": version
            }),
        };

        let data = serde_json::to_vec(&val)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
        self.write_all(&data).and_then(|_| self.flush())
    }
}
