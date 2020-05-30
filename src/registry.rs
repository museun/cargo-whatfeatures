use anyhow::Context as _;
use std::{collections::HashSet, path::PathBuf};

use crate::features::Features;

/// Local disk registry (cargo and our own)
pub struct Registry {
    cached: HashSet<Crate>,
    local: HashSet<Crate>,
}

impl Registry {
    /// Create a registry from the local cache (cargos and ours)
    pub fn from_local() -> anyhow::Result<Self> {
        use crate_version_parse::CrateVersion;

        // TODO use jwalk here
        let home = home::cargo_home()?
            .join("registry")
            .join("src")
            .read_dir()
            .with_context(|| "expected to have a local registry")?;

        let (mut set, mut local) = (HashSet::new(), HashSet::new());

        for path in home
            .filter_map(|dir| dir.ok()?.path().read_dir().ok())
            .flat_map(|dir| dir.flatten())
            .map(|s| s.path())
        {
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                let CrateVersion { name, version } = CrateVersion::try_parse(name)?;
                set.insert(Crate {
                    name: name.to_string(),
                    version: version.to_string(),
                    path,
                    yanked: YankState::UnknownLocal, // TODO we can do an http request to figure this out
                });
            }
        }

        // TODO this should probably be a warning at the least
        if let Ok(base) = crate::util::cache_dir() {
            // TODO use jwalk here
            for dir in base
                .read_dir()
                .into_iter()
                .flat_map(|dir| dir.flatten())
                .filter_map(|dir| {
                    let path = dir.path();
                    if !path.is_dir() {
                        return None;
                    }
                    path.into()
                })
            {
                let name = dir.strip_prefix(&base)?.to_str().expect("valid utf-8");
                let CrateVersion { name, version } = CrateVersion::try_parse(name)?;
                let crate_ = Crate {
                    name: name.to_string(),
                    version: version.to_string(),
                    path: dir.clone(),
                    yanked: YankState::UnknownLocal, // TODO we can do a http request to figure this out
                };

                if set.contains(&crate_) {
                    // remove the cache directory (it already exists in the .cargo/registry)
                    std::fs::remove_dir_all(dir)?;
                } else {
                    local.insert(crate_);
                }
            }
        }

        Ok(Self { cached: set, local })
    }

    /// Tries to get the crate/version from the registry
    pub fn get(&self, crate_name: &str, crate_version: &str) -> Option<&Crate> {
        self.cached
            .iter()
            .chain(self.local.iter())
            .find(|Crate { name, version, .. }| name == crate_name && version == crate_version)
    }

    /// Purge the local cache, returning how many crates it removed
    pub fn purge_local_cache(&mut self) -> anyhow::Result<usize> {
        let mut count = 0;
        for crate_ in self.local.drain() {
            std::fs::remove_dir_all(&crate_.path)?;
            count += 1;
        }
        Ok(count)
    }
}

/// Whether this crate was marked as yanked on crates.io
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum YankState {
    /// It was yanked
    Yanked,
    /// The its cached locally, so we can't know unless we do a http req.
    // technically yanking only exists on crates.io, not other registries
    UnknownLocal,
    /// Its not been yanked
    Available,
}

impl From<bool> for YankState {
    fn from(yanked: bool) -> Self {
        if yanked {
            Self::Yanked
        } else {
            Self::Available
        }
    }
}

/// A crate stored on disk
#[derive(Clone, Debug, Eq)]
pub struct Crate {
    /// Crate name
    pub name: String,
    /// Crate version
    pub version: String,
    /// Path to the crate directory
    pub path: PathBuf,
    /// Whether this crate was marked as yanked
    pub yanked: YankState,
}

impl Crate {
    /// Tries to get the features for the crate
    pub fn get_features(&self) -> anyhow::Result<Features> {
        cargo_metadata::MetadataCommand::new()
            .no_deps()
            .manifest_path(self.path.join("./Cargo.toml"))
            .exec()
            .map(Features::parse)
            .map_err(Into::into)
    }

    /// Tries to get the features from a local crate
    pub fn from_path(path: impl Into<PathBuf>) -> anyhow::Result<Features> {
        cargo_metadata::MetadataCommand::new()
            .current_dir(path)
            .no_deps()
            .exec()
            .map(Features::parse)
            .map_err(Into::into)
    }
}

impl PartialEq for Crate {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
    }
}

impl std::hash::Hash for Crate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes());
        state.write(self.version.as_bytes());
    }
}
