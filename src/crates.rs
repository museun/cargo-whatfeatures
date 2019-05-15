use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CrateVersion {
    pub id: u64,
    #[serde(rename = "crate")]
    pub crate_: String,
    pub num: String,
    pub features: HashMap<String, Vec<String>>,
    pub yanked: bool,
}

impl CrateVersion {
    pub fn lookup(crate_name: &str) -> Result<Vec<Self>, String> {
        use serde_json::Value;

        let resp = attohttpc::get(format!("https://crates.io/api/v1/crates/{}", crate_name))
            .header("User-Agent", "whatfeatures/1.0")
            .send()
            .map_err(|err| err.to_string())?;

        let body = resp.text().map_err(|err| err.to_string())?;
        serde_json::from_str::<Value>(&body)
            .map_err(|err| err.to_string())?
            .get_mut("versions")
            .ok_or_else(|| "unknown crate".to_string())
            .map(Value::take)
            .and_then(|mut obj| {
                obj.as_array_mut()
                    .map(|array| {
                        array
                            .iter_mut()
                            .map(Value::take)
                            .map(serde_json::from_value)
                            .flatten()
                            .collect()
                    })
                    .ok_or_else(|| "no versions published".to_string())
            })
    }
}
