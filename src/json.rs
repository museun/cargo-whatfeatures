use std::io::{Result, Write};

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
