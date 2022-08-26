use cargo_metadata::{DependencyKind, Metadata, Package, PackageId};
use std::collections::HashMap;

#[derive(Debug, Default, serde::Serialize)]
pub struct Workspace {
    pub hint: String, // TODO: what an awesome name
    pub map: HashMap<PackageId, Features>,
}

impl Workspace {
    pub(crate) fn parse(metadata: Metadata, crate_name: impl ToString) -> Self {
        let map = metadata.workspace_members.iter().fold(
            HashMap::with_capacity(metadata.workspace_members.len()),
            |mut map, id| {
                map.insert(id.clone(), Features::parse(metadata[id].clone()));
                map
            },
        );

        Self {
            hint: crate_name.to_string(),
            map,
        }
    }

    pub fn is_workspace(&self) -> bool {
        self.map.len() > 1
    }
}

/// A feature mapping
#[derive(Debug, Default, serde::Serialize)]
pub struct Features {
    /// The name of the crate
    pub name: String,
    /// The version of the crate
    pub version: String,
    /// Whether this crate was published
    // TODO list /which/ registry it was published to
    pub published: bool,
    /// Feature map
    pub features: HashMap<String, Vec<String>>,
    /// Optional deps. map
    pub optional_deps: HashMap<Kind, Vec<Dependency>>,
    /// Required deps. map
    pub required_deps: HashMap<Kind, Vec<Dependency>>,
}

impl Features {
    // TODO this should just take a Package and parse it
    pub(crate) fn parse(pkg: Package) -> Self {
        let (mut name, mut version) = (None, None);
        let (mut features, mut optional_deps, mut required_deps) =
            (HashMap::new(), HashMap::new(), HashMap::new());

        name.get_or_insert_with(|| pkg.name.clone()); // why
        version.get_or_insert_with(|| pkg.version.to_string());
        features.extend(pkg.features);

        for dep in pkg.dependencies {
            let key = dep.kind.into();
            let value = Dependency {
                name: dep.name,
                req: dep.req.to_string(),
                target: dep.target.map(|s| s.to_string()),
                rename: dep.rename,
                features: dep.features,
            };

            let map: &mut HashMap<Kind, Vec<Dependency>> = if dep.optional {
                &mut optional_deps
            } else {
                &mut required_deps
            };
            map.entry(key).or_default().push(value)
        }

        let published = match pkg.publish {
            Some(d) if d.is_empty() => false,
            _ => true,
        };

        Self {
            name: name.unwrap(),
            version: version.unwrap(),
            published,
            features,
            optional_deps,
            required_deps,
        }
    }
}

/// A crate dependency
#[derive(Debug, Clone, serde::Serialize)]
pub struct Dependency {
    /// The name of the dependency
    pub name: String,
    /// The required version of the dependency
    pub req: String,
    /// Which target, if any, that this dependency is required for
    pub target: Option<String>,
    /// What this dependency was renamed to, if it was renamed
    pub rename: Option<String>,
    /// Features available for this dependency
    pub features: Vec<String>,
}

/// The kind of dependency
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// A normal dependency
    Normal,
    /// A development dependency
    Development,
    /// A build dependency
    Build,
}

impl From<DependencyKind> for Kind {
    fn from(kind: DependencyKind) -> Self {
        match kind {
            DependencyKind::Normal => Self::Normal,
            DependencyKind::Development => Self::Development,
            DependencyKind::Build => Self::Build,
            _ => unreachable!(),
        }
    }
}
