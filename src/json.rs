use super::*;
use std::io::Write;

pub struct Json<W>(pub W);

impl<W: Write> Write for Json<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

impl<W: Write> Output for Json<W> {
    fn output(&mut self, vers: &[CrateVersion]) -> std::io::Result<()> {
        self.write_all(&[b'['])?;
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
        }
        self.write_all(&[b']'])
    }

    fn error(&mut self, error: Error) -> std::io::Result<()> {
        let val = match error {
            Error::NoNameProvided => serde_json::json!({
                "error": "no name provided"
            }),
            Error::CannotLookup {
                name,
                version,
                error,
            } => serde_json::json!({
                "error": "cannot lookup crate",
                "name": name,
                "version": version,
                "inner": error,
            }),
            Error::NoVersions(name) => serde_json::json!({
                "error": "no versions published",
                "name": name
            }),
            Error::InvalidVersion(name, version) => serde_json::json!({
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
