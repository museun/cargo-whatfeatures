use crate::InternalError;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, InternalError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub id: u64,
    #[serde(rename = "crate")]
    pub crate_: String,
    pub num: String,
    pub features: HashMap<String, Vec<String>>,
    pub yanked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub id: usize,
    pub version_id: usize,
    pub crate_id: String,
    pub req: String,
    pub optional: bool,
    pub default_features: bool,
    pub features: Vec<String>,
    // TODO parse this to match the semantics of the target section in the manifest
    pub target: Option<String>,
    pub kind: DependencyKind,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DependencyKind {
    Normal,
    Dev,
    Build,
}

pub fn lookup_deps(name: &str, ver: &str) -> Result<Vec<Dependency>> {
    #[derive(Deserialize)]
    struct Wrap {
        dependencies: Vec<Dependency>,
    }

    fetch::<Wrap>(&format!(
        "https://crates.io/api/v1/crates/{}/{}/dependencies",
        name, ver,
    ))
    .map(|item| item.dependencies)
}

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
}

fn fetch<T>(ep: &str) -> Result<T>
where
    for<'a> T: serde::Deserialize<'a>,
{
    let resp = attohttpc::get(ep)
        .header("User-Agent", env!("WHATFEATURES_USER_AGENT"))
        .send()
        .map_err(InternalError::Http)?;

    resp.text()
        .map_err(InternalError::Http)
        .and_then(|body| serde_json::from_str(&body).map_err(InternalError::Json))
}
