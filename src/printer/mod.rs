use crate::{
    client::Version,
    features::{Dependency, Features},
};

use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
};
use yansi::Paint;

mod labels;

mod deps;
use deps::{GroupedDeps, SortedDeps};

mod tree;
use tree::{Node, Printer as _};

mod theme;
use theme::Theme;

mod yank_status;
pub use yank_status::YankStatus;

// TODO build a tree so we can convert it to a DAG or json or just print it here
// currently, we do an adhoc-walk over the features building a bespoke tree
// but this could be totally be generalized to handle walking/visiting the tree
// in other contexts
//
/// Output for the program
pub struct Printer<'a, W: ?Sized> {
    writer: &'a mut W,
    theme: Theme,
}

impl<'a, W: Write + ?Sized> Printer<'a, W> {
    /// Create a new printer with this writer
    pub fn new(writer: &'a mut W) -> Self {
        Self {
            writer,
            theme: Theme::default(),
        }
    }

    /// Write out all of the versions, filtered by the `YankStatus`
    pub fn write_versions(
        &mut self,
        versions: &[Version],
        yank: YankStatus,
    ) -> std::io::Result<()> {
        for ver in versions {
            let Version {
                yanked,
                version,
                name,
                ..
            } = ver;

            match yank {
                YankStatus::Exclude if *yanked => continue,
                YankStatus::Exclude => {}

                YankStatus::Only if !*yanked => continue,
                YankStatus::Only => {
                    write!(self.writer, "{}: ", self.theme.yanked.paint(labels::YANKED))?;
                }

                YankStatus::Include if *yanked => {
                    write!(self.writer, "{}: ", self.theme.yanked.paint(labels::YANKED))?
                }
                YankStatus::Include => {}
            }

            self.write_latest(name, version)?
        }
        Ok(())
    }

    /// WRute the latest crate name and version
    pub fn write_latest(&mut self, name: &str, version: &str) -> std::io::Result<()> {
        writeln!(
            self.writer,
            "{}/{}",
            self.theme.name.paint(&name),
            self.theme.version.paint(&version),
        )
    }

    /// Write the crate name and version
    pub fn write_header(&mut self, features: &Features) -> std::io::Result<()> {
        self.write_latest(&features.name, &features.version)
    }

    /// Write all of the features for the crate
    pub fn write_features(&mut self, features: &Features, verbose: bool) -> std::io::Result<()> {
        let mut sorted: BTreeMap<&String, BTreeSet<&String>> = features
            .features
            .iter()
            .map(|(k, v)| (&*k, v.iter().collect()))
            .collect();

        if sorted.is_empty() {
            return Node::empty(self.theme.no_features.paint(labels::NO_FEATURES))
                .print(self.writer, &self.theme);
        }

        let default_node = match sorted.remove(&"default".to_string()) {
            Some(default) if !default.is_empty() => Node::new(
                self.theme.default.paint(labels::DEFAULT),
                default
                    .into_iter()
                    .map(|s| self.theme.default_features.paint(s)),
            ),
            _ => Node::empty(
                self.theme
                    .no_default_features
                    .paint(labels::NO_DEFAULT_FEATURES),
            ),
        };

        let iter = sorted.iter().map(|(k, v)| {
            let k = if k.starts_with('_') {
                self.theme.probably_internal.paint(k)
            } else {
                self.theme.feature_name.paint(k)
            };

            if v.is_empty() || !verbose {
                Node::empty(k)
            } else {
                Node::new(
                    k,
                    v.iter().map(|s| {
                        if s.starts_with('_') {
                            self.theme.probably_internal.paint(s)
                        } else {
                            self.theme.feature_implies.paint(s)
                        }
                    }),
                )
            }
        });

        Node::new(
            self.theme.features.paint(labels::FEATURES),
            std::iter::once(default_node).chain(iter),
        )
        .print(self.writer, &self.theme)
    }

    /// Write all of the optional dependencies for the crate
    pub fn write_opt_deps(&mut self, features: &Features, verbose: bool) -> std::io::Result<()> {
        let sorted = SortedDeps::from_kind_map(features.optional_deps.clone());
        if !sorted.normal.has_deps() {
            return Node::empty(
                self.theme
                    .no_optional_deps
                    .paint(labels::NO_OPTIONAL_DEPENDENCIES),
            )
            .print(self.writer, &self.theme);
        }

        build_features_tree(
            self.theme
                .optional_deps
                .paint(labels::OPTIONAL_DEPENDENCIES),
            sorted.normal,
            &self.theme,
            verbose,
        )
        .print(self.writer, &self.theme)
    }

    /// Write all of the other dependencies for the crate
    pub fn write_deps(&mut self, features: &Features, verbose: bool) -> std::io::Result<()> {
        let sorted = SortedDeps::from_kind_map(features.required_deps.clone());
        if !sorted.normal.has_deps() && !sorted.development.has_deps() && !sorted.build.has_deps() {
            return Node::empty(
                self.theme
                    .no_required_deps
                    .paint(labels::NO_REQUIRED_DEPENDENCIES),
            )
            .print(self.writer, &self.theme);
        }

        let mut nodes = vec![];
        if sorted.normal.has_deps() {
            nodes.push(build_features_tree(
                self.theme.normal_deps.paint(labels::NORMAL),
                sorted.normal,
                &self.theme,
                verbose,
            ));
        } else {
            // this should should always be visible
            nodes.push(Node::empty(
                self.theme
                    .no_required_deps
                    .paint(labels::NO_NORMAL_DEPENDENCIES),
            ));
        }

        if sorted.development.has_deps() {
            nodes.push(build_features_tree(
                self.theme.dev_deps.paint(labels::DEVELOPMENT),
                sorted.development,
                &self.theme,
                verbose,
            ));
        } else {
            nodes.push(Node::empty(
                self.theme
                    .no_dev_deps
                    .paint(labels::NO_DEVELOPMENT_DEPENDENCIES),
            ));
        }

        if sorted.build.has_deps() {
            nodes.push(build_features_tree(
                self.theme.build_deps.paint(labels::BUILD),
                sorted.build,
                &self.theme,
                verbose,
            ));
        } else {
            nodes.push(Node::empty(
                self.theme
                    .no_build_deps
                    .paint(labels::NO_BUILD_DEPENDENCIES),
            ));
        }

        Node::new(
            self.theme
                .required_deps
                .paint(labels::REQUIRED_DEPENDENCIES),
            nodes,
        )
        .print(self.writer, &self.theme)
    }
}

fn build_features_tree(
    text: impl ToString,
    deps: GroupedDeps,
    theme: &Theme,
    verbose: bool,
) -> Node {
    fn build_features(
        deps: Vec<Dependency>,
        theme: &Theme,
        verbose: bool,
    ) -> impl Iterator<Item = Node> {
        let mut tree = vec![];
        for dep in deps {
            let name = format_dep(&dep, theme);
            if !dep.features.is_empty() {
                let name = format!(
                    "{}{}",
                    name, // TODO does this have a color?
                    theme
                        .has_enabled_features
                        .paint(labels::HAS_ENABLED_FEATURES)
                );
                if !verbose {
                    tree.push(Node::empty(name));
                } else {
                    tree.push(Node::new(
                        name,
                        dep.features.into_iter().map(|s| theme.dep_feature.paint(s)),
                    ));
                }
                continue;
            }
            tree.push(Node::empty(name));
        }
        tree.into_iter()
    }

    let iter = deps
        .with_targets
        .into_iter()
        .map(|(target, deps)| {
            Node::new(
                format!("for {}", theme.target.paint(target)),
                build_features(deps, theme, verbose),
            )
        })
        .chain(build_features(deps.without_targets, theme, verbose));

    Node::new(text, iter)
}

fn format_dep(dep: &Dependency, theme: &Theme) -> String {
    format!(
        "{} = {} {}",
        theme.name.paint(&dep.name),
        theme.version.paint(&dep.req),
        Paint::white(
            dep.rename
                .as_deref()
                .map(|r| format!("(renamed to {}) ", theme.renamed.paint(r)))
                .unwrap_or_else(|| "".into()),
        )
    )
}
