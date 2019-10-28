//! whatfeatures -- displays features and deps for crates
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

/// Functions and types for retrieving crates
pub mod crates;

/// Errors created by the crate
#[doc(inline)]
pub mod error;

/// Types used for displaying data
pub mod output;
