use crate::error::InternalError;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, InternalError>;

/// A crate, including its version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub id: u64,
    #[serde(rename = "crate")]
    pub crate_: String,
    pub num: String,
    pub features: HashMap<String, Vec<String>>,
    pub yanked: bool,
}

/// A Dependency of a crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub id: usize,
    pub version_id: usize,
    pub crate_id: String,
    pub req: String,
    pub optional: bool,
    pub default_features: bool,
    pub features: Vec<String>,
    pub target: Option<String>,
    pub kind: DependencyKind,
}

/// What kind of dependency it is
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DependencyKind {
    Normal,
    Dev,
    Build,
}

/// Look up all of the deps. for `crate_name` and `version`
pub fn lookup_deps(crate_name: &str, version: &str) -> Result<Vec<Dependency>> {
    #[derive(Deserialize)]
    struct Wrap {
        dependencies: Vec<Dependency>,
    }

    fetch::<Wrap>(&format!(
        "https://crates.io/api/v1/crates/{}/{}/dependencies",
        crate_name, version,
    ))
    .map(|item| item.dependencies)
    .map_err(|err| {
        log::warn!("cannot lookup deps for {}/{}. {}", crate_name, version, err);
        err
    })
}

/// Look up a specific `version` of `crate_name`
pub fn lookup_version(crate_name: &str, version: &str) -> Result<Version> {
    #[derive(Deserialize)]
    struct Wrap {
        version: Version,
    }

    fetch::<Wrap>(&format!(
        "https://crates.io/api/v1/crates/{}/{}", //
        crate_name, version
    ))
    .map(|item| item.version)
    .map_err(|err| {
        log::warn!(
            "cannot lookup version {} for {}. {}",
            version,
            crate_name,
            err
        );
        err
    })
}

/// Lookup all versions for `crate_name`
pub fn lookup_versions(crate_name: &str) -> Result<Vec<Version>> {
    #[derive(Deserialize)]
    struct Wrap {
        versions: Vec<Version>,
    }

    fetch::<Wrap>(&format!(
        "https://crates.io/api/v1/crates/{}", //
        crate_name
    ))
    .map(|item| item.versions)
    .map_err(|err| {
        log::warn!("cannot lookup versions for {}. {}", crate_name, err);
        err
    })
}

fn fetch<T>(ep: &str) -> Result<T>
where
    for<'a> T: serde::Deserialize<'a>,
{
    log::trace!("fetching {}", ep);

    let resp = attohttpc::get(ep)
        .header("User-Agent", env!("WHATFEATURES_USER_AGENT"))
        .send()
        .map_err(InternalError::Http)?;

    let body = resp.text().map_err(InternalError::Http)?;
    serde_json::from_str(&body).map_err(InternalError::Json)
}
