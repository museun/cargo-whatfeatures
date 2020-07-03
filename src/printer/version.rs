use crate::{client::Version, Options, Theme, YankStatus};
use std::io::Write;

/// Output for the program
pub struct VersionPrinter<'a, W: ?Sized> {
    writer: &'a mut W,
    theme: Theme,
    options: Options,
}

impl<'a, W: Write + ?Sized> VersionPrinter<'a, W> {
    /// Create a new printer with this writer
    pub fn new(writer: &'a mut W, options: Options) -> Self {
        Self {
            writer,
            theme: Theme::default(),
            options,
        }
    }

    /// Write out all of the versions, filtered by the `YankStatus`
    pub fn write_versions(
        &mut self,
        versions: &[Version],
        yank: YankStatus,
    ) -> std::io::Result<()> {
        fn write_yanked(
            version: &Version,
            writer: &mut impl Write,
            theme: &Theme,
        ) -> std::io::Result<()> {
            writeln!(
                writer,
                "{} = {} # yanked",
                theme.yanked.paint(&version.name),
                theme.yanked.paint(&version.version),
            )
        }

        use YankStatus::*;
        versions
            .iter()
            .filter(|ver| match (yank, ver.yanked) {
                (Exclude, true) | (Only, false) => false,
                _ => true,
            })
            .map(|ver| match yank {
                Exclude => self.write_latest(&ver.name, &ver.version),
                Only => write_yanked(ver, &mut self.writer, &self.theme),
                Include if ver.yanked => write_yanked(ver, &mut self.writer, &self.theme),
                Include => self.write_latest(&ver.name, &ver.version),
            })
            .collect()
    }

    /// Writes many 'name = version' pairs with column alignment
    pub fn write_many_versions(
        &mut self,
        list: Vec<(&String, &String, bool)>,
    ) -> std::io::Result<()> {
        let max = list.iter().map(|(s, ..)| s.len()).max().unwrap();

        let theme = self.theme;
        for (name, version, published) in list {
            let header = if published {
                format!(
                    "{: <max$} = {}",
                    theme.name.paint(&name),
                    theme.version.paint(&version),
                    max = max,
                )
            } else if self.options.show_private {
                format!(
                    "{: <max$} = {} {}",
                    theme.name.paint(&name),
                    theme.version.paint(&version),
                    theme.is_not_published.paint("(restricted)"),
                    max = max,
                )
            } else {
                continue;
            };

            writeln!(self.writer, "{}", header)?;
        }

        Ok(())
    }

    /// Write the latest crate name and version
    pub fn write_latest(&mut self, name: &str, version: &str) -> std::io::Result<()> {
        writeln!(
            self.writer,
            "{} = {}",
            self.theme.name.paint(&name),
            self.theme.version.paint(&version),
        )
    }
}
