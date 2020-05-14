use crate::{
    client::Version,
    features::{Dependency, Features},
};

use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
};
use yansi::Paint;

mod deps;
use deps::{GroupedDeps, SortedDeps};

mod tree;
use tree::{Node, Printer as _};

/// When to show yanked crates
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum YankStatus {
    /// Only show yanked crates
    Only,
    /// Exclude all yanked crates
    Exclude,
    /// Include yanked crates
    Include,
}

impl Default for YankStatus {
    fn default() -> Self {
        Self::Exclude
    }
}

// TODO build a tree so we can convert it to a DAG or json or just print it here
// currently, we do an adhoc-walk over the features building a bespoke tree
// but this could be totally be generalized to handle walking/visiting the tree
// in other contexts
//
/// Output for the program
pub struct Printer<'a, W: ?Sized> {
    writer: &'a mut W,
}

impl<'a, W: Write + ?Sized> Printer<'a, W> {
    /// Create a new printer with this writer
    pub fn new(writer: &'a mut W) -> Self {
        Self { writer }
    }

    /// Write out all of the versions, filtered by the `YankStatus`
    pub fn write_versions(
        &mut self,
        name: &str,
        versions: &[Version],
        yank: YankStatus,
    ) -> std::io::Result<()> {
        for ver in versions {
            let Version {
                yanked, version, ..
            } = ver;

            match yank {
                YankStatus::Exclude if *yanked => continue,
                YankStatus::Exclude => {}

                YankStatus::Only if !*yanked => continue,
                YankStatus::Only => {
                    write!(self.writer, "{}: ", Paint::red("yanked"))?;
                }

                YankStatus::Include if *yanked => {
                    write!(self.writer, "{}: ", Paint::red("yanked"))?
                }
                YankStatus::Include => {}
            }

            writeln!(
                self.writer,
                "{}/{}",
                Paint::white(name),
                Paint::yellow(&version)
            )?
        }
        Ok(())
    }

    /// WRute the latest crate name and version
    pub fn write_latest(&mut self, name: &str, version: &str) -> std::io::Result<()> {
        writeln!(
            self.writer,
            "{}/{}",
            Paint::white(&name),
            Paint::yellow(&version)
        )
    }

    /// Write the crate name and version
    pub fn write_header(&mut self, features: &Features) -> std::io::Result<()> {
        self.write_latest(&features.name, &features.version)
    }

    /// Write all of the features for the crate
    pub fn write_features(&mut self, features: &Features) -> std::io::Result<()> {
        let mut sorted: BTreeMap<&String, BTreeSet<&String>> = features
            .features
            .iter()
            .map(|(k, v)| (&*k, v.iter().collect()))
            .collect();

        if sorted.is_empty() {
            let node = Node::empty(Paint::green("no features"));
            return node.print(self.writer);
        }

        let default_node = match sorted.remove(&"default".to_string()) {
            Some(default) if !default.is_empty() => Node::new(
                Paint::magenta("default"),
                default.into_iter().map(Paint::green),
            ),
            _ => Node::empty(Paint::yellow("no default features")),
        };

        let iter = sorted.iter().map(|(k, v)| {
            let k = Paint::magenta(k);
            if v.is_empty() {
                Node::empty(k)
            } else {
                Node::new(k, v.iter().map(Paint::white))
            }
        });

        let node = Node::new(
            Paint::green("features"),
            std::iter::once(default_node).chain(iter),
        );
        node.print(self.writer)
    }

    /// Write all of the optional dependencies for the crate
    pub fn write_opt_deps(&mut self, features: &Features) -> std::io::Result<()> {
        let sorted = SortedDeps::from_kind_map(features.optional_deps.clone());
        if !sorted.normal.has_deps() {
            let node = Node::empty(Paint::yellow("no optional dependencies"));
            return node.print(self.writer);
        }

        let node = build_features_tree(Paint::yellow("optional dependencies"), sorted.normal);
        node.print(self.writer)
    }

    /// Write all of the other dependencies for the crate
    pub fn write_deps(&mut self, features: &Features) -> std::io::Result<()> {
        let sorted = SortedDeps::from_kind_map(features.required_deps.clone());
        if !sorted.normal.has_deps() && !sorted.development.has_deps() && !sorted.build.has_deps() {
            let node = Node::empty(Paint::cyan("no required dependencies"));
            return node.print(self.writer);
        }

        let mut nodes = vec![];
        if sorted.normal.has_deps() {
            nodes.push(build_features_tree(Paint::blue("normal"), sorted.normal));
        } else {
            // this should should always be visible
            nodes.push(Node::empty(Paint::cyan("no normal dependencies")));
        }

        if sorted.development.has_deps() {
            nodes.push(build_features_tree(
                Paint::blue("development"),
                sorted.development,
            ));
        } else {
            // TODO make this only visible via a verbosity flag
            nodes.push(Node::empty(Paint::cyan("no development dependencies")));
        }

        if sorted.build.has_deps() {
            nodes.push(build_features_tree(Paint::blue("build"), sorted.build));
        } else {
            // TODO make this only visible via a verbosity flag
            nodes.push(Node::empty(Paint::cyan("no build dependencies")));
        }

        let node = Node::new(Paint::cyan("required dependencies"), nodes);
        node.print(self.writer)
    }
}

fn build_features_tree(text: impl ToString, deps: GroupedDeps) -> Node {
    fn build_features(deps: Vec<Dependency>) -> impl Iterator<Item = Node> {
        let mut tree = vec![];
        for dep in deps {
            let name = format_dep(&dep);
            if !dep.features.is_empty() {
                tree.push(Node::new(
                    format!("{}{}", name, Paint::blue("(has enabled features)")),
                    dep.features,
                ));
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
            Node::new(format!("for {}", Paint::red(target)), build_features(deps))
        })
        .chain(build_features(deps.without_targets));

    Node::new(text, iter)
}

fn format_dep(dep: &Dependency) -> String {
    format!(
        "{} = {} {}",
        dep.name,
        Paint::yellow(&dep.req),
        Paint::white(
            dep.rename
                .as_deref()
                .map(|r| format!("(renamed to {}) ", Paint::blue(r)))
                .unwrap_or_else(|| "".into()),
        )
    )
}
