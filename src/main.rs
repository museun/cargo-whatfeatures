use gumdrop::Options;
use std::io::Write;
use yansi::Paint;

mod crates;
mod error;
mod json;
mod text;

use crates::Version;
use error::{Error, Result};
use json::Json;
use text::Text;

pub trait Output: Write {
    fn output(&mut self, item: &[Version]) -> std::io::Result<()>;
    fn error(&mut self, error: Error) -> std::io::Result<()>;
}

#[derive(Debug, Clone, Options)]
struct Args {
    #[options(help = "display this message")]
    help: bool,

    #[options(help = "a specific version")]
    version: Option<String>,

    #[options(help = "list all versions")]
    list: bool,

    #[options(free)]
    name: String,

    #[options(help = "prints results as json")]
    json: bool,

    #[options(help = "shows any yanked versions before the latest stable")]
    show_yanked: bool,

    #[options(help = "disables using colors when printing as text")]
    no_color: bool,

    #[options(help = "tries to use colors when printing as text")]
    color: bool,
}

fn main() {
    let args = Args::parse_args_default_or_exit();
    let disable_colors = std::env::var("NO_COLOR").is_ok();

    if disable_colors || args.no_color || cfg!(windows) && !Paint::enable_windows_ascii() {
        Paint::disable();
    }

    let w = std::io::stdout();
    let w = w.lock();
    let mut writer: Box<dyn Output> = if args.json {
        Box::new(Json::new(w))
    } else {
        Box::new(Text::new(w))
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

    let mut versions = Version::lookup(&args.name).unwrap_or_else(|err| {
        let args = args.clone();
        let err = Error::CannotLookup {
            name: args.name,
            version: args.version,
            error: Box::new(err),
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

    for ver in versions.into_iter() {
        if ver.yanked {
            if args.show_yanked {
                writer.output(&[ver]).expect("write");
            }
            continue;
        }

        writer.output(&[ver]).expect("write");
        break;
    }
}
