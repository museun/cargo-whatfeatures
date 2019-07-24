use std::io::{Error, ErrorKind, Result, Write};

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Format {
    Pretty,
    Compact,
}

pub(crate) trait RenderAsJson {
    fn render<W: Write>(&self, output: &mut W, format: Format) -> Result<()>;
}

impl<'a, T> RenderAsJson for T
where
    T: serde::Serialize,
{
    fn render<W: Write>(&self, output: &mut W, format: Format) -> Result<()> {
        match format {
            Format::Pretty => (serde_json::to_writer_pretty(output, &self)),
            Format::Compact => serde_json::to_writer(output, &self),
        }
        .map_err(|err| Error::new(ErrorKind::InvalidData, err))
    }
}
