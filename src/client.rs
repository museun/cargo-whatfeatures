use crate::registry::Crate;
use anyhow::Context as _;

/// An HTTP client for interacting with crates.io
pub struct Client {
    host: String,
}

impl Client {
    /// Create a new HTTP client with the provided host (e.g. `https://crates.io` or `http://localhost`)
    pub fn new(host: impl ToString) -> Self {
        Self {
            host: host.to_string(),
        }
    }

    /// Lookup and cache the latest version for this crate
    pub fn cache_latest(&self, crate_name: &str) -> anyhow::Result<Crate> {
        let Version { version, .. } = self.get_latest(crate_name)?;
        self.cache_crate(crate_name, &version)
    }

    /// Lookup and cache the specified version for this crate
    pub fn cache_crate(&self, crate_name: &str, crate_version: &str) -> anyhow::Result<Crate> {
        let (yanked, data) = self.download_crate(crate_name, crate_version)?;
        crate::util::extract_crate(&data, crate_name, crate_version).map(|path| Crate {
            name: crate_name.to_string(),
            version: crate_version.to_string(),
            path,
            yanked: yanked.into(),
        })
    }

    /// Get the latest version for this crate
    pub fn get_latest(&self, crate_name: &str) -> anyhow::Result<Version> {
        self.list_versions(crate_name)?
            .into_iter()
            .find(|s| !s.yanked)
            .ok_or_else(|| anyhow::anyhow!("no available version for: {}", crate_name))
    }

    /// Get the latest version for this crate
    pub fn get_version(&self, crate_name: &str, semver: &str) -> anyhow::Result<Version> {
        self.list_versions(crate_name)?
            .into_iter()
            .find(|s| s.version == semver)
            .ok_or_else(|| anyhow::anyhow!("no available version for: {} = {}", crate_name, semver))
    }

    /// Get all versions for this crate
    pub fn list_versions(&self, crate_name: &str) -> anyhow::Result<Vec<Version>> {
        #[derive(serde::Deserialize)]
        struct Resp {
            versions: Vec<Version>,
        }

        self.fetch_json(&format!("/api/v1/crates/{}", crate_name))
            .map(|resp: Resp| resp.versions)
            .with_context(|| anyhow::anyhow!("list versions for: {}", crate_name))
    }
}

impl Client {
    fn download_crate(
        &self,
        crate_name: &str,
        crate_version: &str,
    ) -> anyhow::Result<(bool, Vec<u8>)> {
        #[derive(Debug, serde::Deserialize)]
        struct Resp {
            version: Version,
        }

        let version = self
            .fetch_json(&format!("/api/v1/crates/{}/{}", crate_name, crate_version))
            .map(|resp: Resp| resp.version)
            .with_context(|| anyhow::anyhow!("download crate {}/{}", crate_name, crate_version))?;

        anyhow::ensure!(version.name == crate_name, "received the wrong crate");
        anyhow::ensure!(
            version.version == crate_version,
            "received the wrong version"
        );
        anyhow::ensure!(!version.dl_path.is_empty(), "no download path available");

        self.fetch_bytes(&version.dl_path)
            .map(|data| (version.yanked, data))
    }

    fn fetch_json<T>(&self, endpoint: &str) -> anyhow::Result<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let resp = attohttpc::get(format!("{}{}", self.host, endpoint))
            .header("USER-AGENT", Self::get_user_agent())
            .send()?;

        anyhow::ensure!(
            resp.status().is_success(),
            "cannot fetch json for {}",
            endpoint
        );

        resp.json()
            .with_context(move || format!("cannot parse json from {}", endpoint))
            .map_err(Into::into)
    }

    fn fetch_bytes(&self, endpoint: &str) -> anyhow::Result<Vec<u8>> {
        let resp = attohttpc::get(format!("{}{}", self.host, endpoint))
            .header("USER-AGENT", Self::get_user_agent())
            .send()?;

        anyhow::ensure!(
            resp.status().is_success(),
            "cannot fetch bytes for {}",
            endpoint
        );

        let len = resp
            .headers()
            .get("Content-Length")
            .and_then(|s| s.to_str().ok()?.parse::<usize>().ok())
            .with_context(|| "cannot get Content-Length")?;

        let bytes = resp.bytes()?;
        anyhow::ensure!(len == bytes.len(), "fetch size was wrong");

        Ok(bytes)
    }

    const fn get_user_agent() -> &'static str {
        concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
            " (",
            env!("CARGO_PKG_REPOSITORY"),
            ")"
        )
    }
}

/// A crate version
#[derive(serde::Deserialize, Clone, Debug)]
pub struct Version {
    #[serde(rename = "crate")]
    /// The name of the crate
    pub name: String,
    #[serde(rename = "num")]
    /// The semantic version of the crate
    pub version: String,
    /// Whether this version was yanked
    pub yanked: bool,
    /// The primary license of the crate
    pub license: Option<String>,
    /// When the crate was created
    #[serde(deserialize_with = "time02_parse_timestamp")]
    pub created_at: time::OffsetDateTime,

    dl_path: String,
}

impl Version {
    pub fn format_verbose_time(&self) -> String {
        self.created_at.format("%F %0H:%M:%S %z")
    }

    pub fn format_approx_time_span(&self) -> String {
        let d = time::OffsetDateTime::now_utc() - self.created_at;
        macro_rules! try_time {
            ($($expr:tt => $class:expr)*) => {{
                $(
                    match d.$expr() {
                        0 => {}
                        1 => return format!("1 {} ago", $class),
                        d => return format!("{} {}s ago", d, $class),
                    }
                )*
                String::from("just now")
            }};
        }

        try_time! {
            whole_weeks   => "week"
            whole_days    => "day"
            whole_hours   => "hour"
            whole_minutes => "minute"
            whole_seconds => "second"
        }
    }
}

struct RelativeTimeVisitor;
impl<'de> serde::de::Visitor<'de> for RelativeTimeVisitor {
    type Value = time::OffsetDateTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let mut iter = v.splitn(2, '.');
        let head = iter
            .next()
            .ok_or_else(|| E::custom("timestamp is malformed"))?;

        let ts = format!("{}+0000", head);
        time::OffsetDateTime::parse(&ts, "%FT%T%z").map_err(E::custom)
    }

    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
        self.visit_str(v.as_str())
    }
}

fn time02_parse_timestamp<'de, D>(deserializer: D) -> Result<time::OffsetDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_string(RelativeTimeVisitor)
}
