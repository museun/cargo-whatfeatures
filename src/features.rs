use cargo_metadata::{DependencyKind, Metadata};
use std::collections::HashMap;

/// A feature mapping
#[derive(Debug, Default)]
pub struct Features {
    /// The name of the crate
    pub name: String,
    /// The version of the crate
    pub version: String,
    // TOOO ignore features that begin with _
    // or atleast note that they aren't meant for public usage (see reqwest's `__internal_proxy_sys_no_cache`)
    pub features: HashMap<String, Vec<String>>,
    pub optional_deps: HashMap<Kind, Vec<Dependency>>,
    pub required_deps: HashMap<Kind, Vec<Dependency>>,
}

impl Features {
    pub(crate) fn parse(metadata: Metadata) -> Self {
        let (mut name, mut version) = (None, None);
        let (mut features, mut optional_deps, mut required_deps) =
            (HashMap::new(), HashMap::new(), HashMap::new());

        for pkg in metadata.packages {
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
        }

        Self {
            name: name.unwrap(),
            version: version.unwrap(),
            features,
            optional_deps,
            required_deps,
        }
    }
}

/// A crate dependency
#[derive(Debug, Clone)]
pub struct Dependency {
    /// The name fo the dependency
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
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
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
