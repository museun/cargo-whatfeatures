/// Functions and types for retrieving crates
pub mod crates;

/// Errors created by the crate
#[doc(inline)]
pub mod error;

mod text;
/// Types used for displaying data
pub mod output {
    /// A simple Name/Version pair
    pub struct NameVer<'a>(pub &'a str, pub &'a str);

    /// A simple Name/Version pair that has been yanked
    pub struct YankedNameVer<'a>(pub &'a str, pub &'a str);

    /// No dependencies
    pub struct NoDeps;

    #[doc(inline)]
    pub use crate::text::{AsText, DepState};
}
