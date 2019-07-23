pub struct NameVer<'a>(pub &'a str, pub &'a str);
pub struct YankedNameVer<'a>(pub &'a str, pub &'a str);
pub struct NoDeps;

pub mod crates;

mod args;
mod error;
mod text;

pub use args::Args;
pub use crates::{Dependency, DependencyKind};
pub use error::{InternalError, UserError};
pub use text::{AsText, DepState};
