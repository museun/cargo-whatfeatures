use gumdrop::Options;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

enum Error {
    NoNameProvided,
    CannotLookup {
        name: String,
        version: Option<String>,
        error: String,
    },
    NoVersions(String),
    InvalidVersion(String, String),
}

trait Output: Write {
    fn output(&mut self, vers: &[CrateVersion]) -> std::io::Result<()>;
    fn error(&mut self, error: Error) -> std::io::Result<()>;
}

struct Text<W>(pub W);
impl<W: Write> Write for Text<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

impl<W: Write> Output for Text<W> {
    fn output(&mut self, vers: &[CrateVersion]) -> std::io::Result<()> {
        for ver in vers {
            if ver.yanked {
                writeln!(self, "{}/{} has been yanked", ver.crate_, ver.num)?
            } else {
                writeln!(self, "{}/{}", ver.crate_, ver.num)?
            }
            if let Some(default) = ver.features.get("default") {
                write!(self, "    default")?;
                if !default.is_empty() {
                    write!(self, ": {}", default.join(", "))?;
                }
                writeln!(self)?;
            } else {
                writeln!(self, "  no default features")?;
            }
            for (k, v) in &ver.features {
                if k == "default" {
                    continue;
                }
                write!(self, "    {}", k)?;
                if !v.is_empty() {
                    write!(self, ": {}", v.join(", "))?;
                }
                writeln!(self)?;
            }
        }
        Ok(())
    }

    fn error(&mut self, error: Error) -> std::io::Result<()> {
        match error {
            Error::NoNameProvided => {
                eprintln!("no name was provided");
            }
            Error::CannotLookup {
                name,
                version: Some(version),
                error,
            } => {
                eprintln!("cannot lookup '{}/{}': {}", name, version, error);
            }
            Error::CannotLookup { name, error, .. } => {
                eprintln!("cannot lookup '{}': {}", name, error);
            }
            Error::NoVersions(name) => {
                eprintln!("no versions published for '{}", name);
            }
            Error::InvalidVersion(name, version) => {
                eprintln!("invalid version '{}' for '{}'", version, name);
            }
        }
        Ok(())
    }
}

struct Json<W>(pub W);
impl<W: Write> Write for Json<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

impl<W: Write> Output for Json<W> {
    fn output(&mut self, vers: &[CrateVersion]) -> std::io::Result<()> {
        self.write_all(&[b'['])?;
        let len = vers.len();
        for (i, ver) in vers.iter().enumerate() {
            if i > 0 && i < len {
                self.write_all(&[b','])?;
            }
            let data = serde_json::to_vec(&serde_json::json!({
                "yanked": ver.yanked,
                "name": ver.crate_,
                "version": ver.num,
                "features": ver.features
            }))
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
            self.write_all(&data)?;
        }
        self.write_all(&[b']'])
    }

    fn error(&mut self, error: Error) -> std::io::Result<()> {
        let val = match error {
            Error::NoNameProvided => serde_json::json!({
                "error": "no name provided"
            }),
            Error::CannotLookup {
                name,
                version,
                error,
            } => serde_json::json!({
                "error": "cannot lookup crate",
                "name": name,
                "version": version,
                "inner": error,
            }),
            Error::NoVersions(name) => serde_json::json!({
                "error": "no versions published",
                "name": name
            }),
            Error::InvalidVersion(name, version) => serde_json::json!({
                "error": "invalid version",
                "name": name,
                "version": version
            }),
        };

        let data = serde_json::to_vec(&val)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
        self.write_all(&data).and_then(|_| self.flush())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CrateVersion {
    id: u64,
    #[serde(rename = "crate")]
    crate_: String,
    num: String,
    features: HashMap<String, Vec<String>>,
    yanked: bool,
}

impl CrateVersion {
    fn lookup(crate_name: &str) -> Result<Vec<Self>, String> {
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

#[derive(Debug, Clone, Options)]
struct Args {
    #[options(help = "display this message")]
    help: bool,

    #[options(help = "specific version to lookup")]
    version: Option<String>,

    #[options(help = "list all version")]
    list: bool,

    #[options(free)]
    name: String,

    #[options(help = "prints results as json")]
    json: bool,
}

fn main() {
    let w = std::io::stdout();
    let w = w.lock();

    let args = Args::parse_args_default_or_exit();
    let mut writer: Box<dyn Output> = if args.json {
        Box::new(Json(w))
    } else {
        Box::new(Text(w))
    };

    macro_rules! abort {
        ($code:expr=> $err:expr) => {{
            writer.error($err).expect("write error");
            std::process::exit($code);
        }};
    }

    if args.name.is_empty() {
        abort!(1=> Error::NoNameProvided);
    }

    let mut versions = CrateVersion::lookup(&args.name).unwrap_or_else(|err| {
        let args = args.clone();
        let err = Error::CannotLookup {
            name: args.name,
            version: args.version,
            error: err,
        };
        abort!(1=> err)
    });

    if versions.is_empty() {
        abort!(1=> Error::NoVersions(args.name));
    }

    if let Some(ver) = &args.version {
        if let Some(pos) = versions.iter().position(|k| k.num == ver.as_str()) {
            writer.output(&[versions.remove(pos)]).expect("write");
            return;
        }
        abort!(1=> Error::InvalidVersion(args.name.clone(), ver.clone()));
    }

    if args.list {
        writer.output(&versions).expect("write");
        return;
    }

    writer.output(&[versions.remove(0)]).expect("write")
}

