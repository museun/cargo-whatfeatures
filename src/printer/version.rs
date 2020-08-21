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
        verbose: bool,
    ) -> std::io::Result<()> {
        use YankStatus::*;
        let output = versions
            .iter()
            .filter(|ver| match (yank, ver.yanked) {
                (Exclude, true) | (Only, false) => false,
                _ => true,
            })
            .map(|version| match yank {
                Exclude => self.write_latest(version, verbose),
                Only => self.write_yanked(version, verbose),
                Include if version.yanked => self.write_yanked(version, verbose),
                Include => self.write_latest(version, verbose),
            })
            .collect::<Vec<_>>();

        let left_max = output
            .iter()
            .map(|&VersionOutput { left_len, .. }| left_len)
            .max()
            .unwrap_or_default();

        let padding = " ".repeat(left_max);

        output
            .iter()
            .map(|v| {
                writeln!(
                    self.writer,
                    "{}{} # {}",
                    v.left,
                    &padding[v.left_len..],
                    v.right
                )
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
                    "{: <max$} = \"{}\"",
                    theme.name.paint(&name),
                    theme.version.paint(&version),
                    max = max,
                )
            } else if self.options.show_private {
                format!(
                    "{: <max$} = \"{}\" {}",
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

    pub fn write_latest_version(
        &mut self,
        version: &Version,
        verbose: bool,
    ) -> std::io::Result<()> {
        let VersionOutput { left, right, .. } = self.write_latest(version, verbose);
        writeln!(self.writer, "{} # {}", left, right)
    }

    fn write_yanked(&mut self, version: &Version, verbose: bool) -> VersionOutput {
        let left = format!(
            "{} = \"{}\"",
            self.theme.yanked.paint(&version.name),
            self.theme.yanked.paint(&version.version),
        );

        let right = if !verbose {
            format!(
                "{} -- yanked",
                self.theme
                    .created_at
                    .paint(&version.format_approx_time_span()),
            )
        } else {
            format!(
                "{} -- yanked",
                self.theme.created_at.paint(&version.format_verbose_time()),
            )
        };

        VersionOutput {
            left_len: version.name.len() + version.version.len() + 3,
            left,
            right,
        }
    }

    fn write_latest(&mut self, version: &Version, verbose: bool) -> VersionOutput {
        let left = format!(
            "{} = \"{}\"",
            self.theme.name.paint(&version.name),
            self.theme.version.paint(&version.version),
        );

        let right = if !verbose {
            format!(
                "{}",
                self.theme
                    .created_at
                    .paint(&version.format_approx_time_span()),
            )
        } else {
            format!(
                "{}",
                self.theme.created_at.paint(&version.format_verbose_time()),
            )
        };

        VersionOutput {
            left_len: version.name.len() + version.version.len() + 3,
            left,
            right,
        }
    }
}

pub struct VersionOutput {
    left_len: usize,
    left: String,
    right: String,
}
