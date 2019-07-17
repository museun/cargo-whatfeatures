use crate::InternalError;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub id: u64,
    #[serde(rename = "crate")]
    pub crate_: String,
    pub num: String,
    pub features: HashMap<String, Vec<String>>,
    pub yanked: bool,
}

impl Version {
    pub fn lookup(crate_name: &str) -> Result<Vec<Self>, InternalError> {
        let ep = format!("https://crates.io/api/v1/crates/{}", crate_name);

        #[derive(Deserialize)]
        struct Wrap {
            versions: Vec<Version>,
        }
        fetch(ep).map(|wrap: Wrap| wrap.versions)
    }
}

fn fetch<T>(ep: impl AsRef<str>) -> Result<T, InternalError>
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
