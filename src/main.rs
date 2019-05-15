use gumdrop::Options;
use std::io::Write;

mod crates;
mod error;
mod json;
mod text;

use crates::CrateVersion;
use error::Error;
use json::Json;
use text::Text;

pub trait Output: Write {
    fn output(&mut self, vers: &[CrateVersion]) -> std::io::Result<()>;
    fn error(&mut self, error: Error) -> std::io::Result<()>;
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

