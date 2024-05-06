use super::{
    deps::{GroupedDeps, SortedDeps},
    labels,
    tree::{Node, Printer},
};
use crate::{
    features::{Dependency, Features, Workspace},
    Options, Theme,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
};

pub struct WorkspacePrinter<'a, W: ?Sized> {
    writer: &'a mut W,
    theme: Theme,
    workspace: Workspace,
    options: Options,
}

impl<'a, W: ?Sized> WorkspacePrinter<'a, W>
where
    W: Write,
{
    pub fn new(writer: &'a mut W, workspace: Workspace, options: Options) -> Self {
        Self {
            writer,
            theme: options.theme,
            workspace,
            options,
        }
    }

    pub fn print(self) -> std::io::Result<()> {
        let mut list = self
            .workspace
            .map
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<_>>();

        list.sort_by(|l, r| l.name.cmp(&r.name));

        let (options, theme) = (self.options, self.theme);

        let mut nodes = list.iter().filter_map(|f| make_child_node(f, &options));

        match list.len() {
            0 => unreachable!("empty tree"),
            1 => {
                if let Some(node) = nodes.next() {
                    node.print(self.writer, &theme)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "An empty tree was returned by cargo-metdata",
                    ))
                }
            }
            _ => {
                let name = format!(
                    "workspace for {}",
                    theme.workspace.paint(&self.workspace.hint)
                );
                Node::new(name, nodes)
            }
            .print(self.writer, &theme),
        }
    }
}

fn make_child_node(features: &Features, options: &Options) -> Option<Node> {
    let Options {
        print_features, // not -n
        show_deps,      // -d
        verbose,
        show_private,
        theme,
    } = *options;

    if !features.published && !show_private {
        return None;
    }

    let header = if features.published {
        format!(
            "{} = \"{}\"",
            theme.name.paint(&features.name),
            theme.version.paint(&features.version),
        )
    } else {
        format!(
            "{} = \"{}\" {}",
            theme.name.paint(&features.name),
            theme.version.paint(&features.version),
            theme.is_not_published.paint("(restricted)")
        )
    };

    let mut parent = Node::empty(header);

    if print_features {
        let node = make_features_node(features, &theme, verbose);
        parent.add_child(node);
    }

    if verbose || (!print_features && show_deps) {
        let node = make_opt_deps_node(features, &theme, verbose);
        parent.add_child(node);
    }

    if show_deps {
        let node = make_deps_node(features, &theme, verbose);
        parent.add_child(node)
    }

    Some(parent)
}

fn make_opt_deps_node(features: &Features, theme: &Theme, verbose: bool) -> Node {
    let sorted = SortedDeps::from_kind_map(features.optional_deps.clone());
    if !sorted.normal.has_deps() {
        let name = theme
            .no_optional_deps
            .paint(labels::NO_OPTIONAL_DEPENDENCIES);
        return Node::empty(name);
    }

    build_features_tree(
        theme.optional_deps.paint(labels::OPTIONAL_DEPENDENCIES),
        sorted.normal,
        theme,
        verbose,
    )
}

fn make_deps_node(features: &Features, theme: &Theme, verbose: bool) -> Node {
    let sorted = SortedDeps::from_kind_map(features.required_deps.clone());
    if !sorted.normal.has_deps() && !sorted.development.has_deps() && !sorted.build.has_deps() {
        return Node::empty(
            theme
                .no_required_deps
                .paint(labels::NO_REQUIRED_DEPENDENCIES),
        );
    }

    let mut nodes = vec![];
    if sorted.normal.has_deps() {
        nodes.push(build_features_tree(
            theme.normal_deps.paint(labels::NORMAL),
            sorted.normal,
            theme,
            verbose,
        ));
    } else {
        let name = theme.no_required_deps.paint(labels::NO_NORMAL_DEPENDENCIES);
        nodes.push(Node::empty(name));
    }

    if sorted.development.has_deps() {
        nodes.push(build_features_tree(
            theme.dev_deps.paint(labels::DEVELOPMENT),
            sorted.development,
            theme,
            verbose,
        ));
    } else {
        let name = theme.no_dev_deps.paint(labels::NO_DEVELOPMENT_DEPENDENCIES);
        nodes.push(Node::empty(name));
    }

    if sorted.build.has_deps() {
        nodes.push(build_features_tree(
            theme.build_deps.paint(labels::BUILD),
            sorted.build,
            theme,
            verbose,
        ));
    } else {
        let name = theme.no_build_deps.paint(labels::NO_BUILD_DEPENDENCIES);
        nodes.push(Node::empty(name));
    }

    let name = theme.required_deps.paint(labels::REQUIRED_DEPENDENCIES);
    Node::new(name, nodes)
}

fn make_features_node(features: &Features, theme: &Theme, verbose: bool) -> Node {
    let mut sorted: BTreeMap<_, BTreeSet<_>> = features
        .features
        .iter()
        .map(|(k, v)| (&*k, v.iter().collect()))
        .collect();

    if sorted.is_empty() {
        return Node::empty(theme.no_features.paint(labels::NO_FEATURES));
    }

    let mut default = None;

    let default_node = match sorted.remove(&"default".to_string()) {
        Some(def) if !def.is_empty() => {
            let node = Node::new(
                theme.default.paint(labels::DEFAULT),
                def.iter().map(|s| theme.feature_implies.paint(s)),
            );
            default.replace(def);
            node
        }
        _ => Node::empty(theme.no_default_features.paint(labels::NO_DEFAULT_FEATURES)),
    };

    let iter = sorted.iter().map(|(k, v)| {
        let (probably, other, features);
        let k: &dyn std::fmt::Display = if k.starts_with('_') {
            probably = theme.probably_internal.paint(k);
            &probably
        } else if default.as_ref().filter(|def| def.contains(k)).is_some() {
            other = format!(
                "{} ({})",
                theme.feature_name.paint(k),
                theme.default.paint("default")
            );
            &other
        } else {
            features = theme.feature_name.paint(k);
            &features
        };

        if v.is_empty() || !verbose {
            return Node::empty(k);
        }

        let children = v.iter().map(|s| {
            let color = if s.starts_with('_') {
                theme.probably_internal
            } else {
                theme.feature_implies
            };
            color.paint(s)
        });
        Node::new(k, children)
    });

    Node::new(
        theme.features.paint(labels::FEATURES),
        std::iter::once(default_node).chain(iter),
    )
}

fn build_features_tree(
    text: impl ToString,
    deps: GroupedDeps,
    theme: &Theme,
    verbose: bool,
) -> Node {
    let format = |(target, deps)| {
        Node::new(
            format!("for {}", theme.target.paint(target)),
            build_features(deps, theme, verbose),
        )
    };

    let iter = deps
        .with_targets
        .into_iter()
        .map(format)
        .chain(build_features(deps.without_targets, theme, verbose));

    Node::new(text, iter)
}

fn build_features<'a>(
    deps: impl IntoIterator<Item = Dependency> + 'a,
    theme: &'a Theme,
    verbose: bool,
) -> impl Iterator<Item = Node> + 'a {
    let map = move |dep| {
        let name = format_dep(&dep, theme);
        if dep.features.is_empty() {
            return Node::empty(name);
        }

        let enabled = theme
            .has_enabled_features
            .paint(labels::HAS_ENABLED_FEATURES);

        let name = format!("{}{}", name, enabled);

        if verbose {
            let features = dep.features.into_iter().map(|s| theme.dep_feature.paint(s));
            Node::new(name, features)
        } else {
            Node::empty(name)
        }
    };

    deps.into_iter().map(map)
}

fn format_dep(dep: &Dependency, theme: &Theme) -> String {
    if let Some(renamed) = dep.rename.as_deref() {
        let renamed = format!("(renamed to {})", theme.renamed_target.paint(renamed));
        return format!(
            "{} = \"{}\" {}",
            theme.name.paint(&dep.name),
            theme.version.paint(&dep.req),
            theme.renamed.paint(renamed).wrap()
        );
    }

    format!(
        "{} = \"{}\" ",
        theme.name.paint(&dep.name),
        theme.version.paint(&dep.req),
    )
}
