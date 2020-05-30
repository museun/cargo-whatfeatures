#![warn(
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
/*! whatfeatures

print out features and dependencies for a specific crate
*/

mod client;

#[doc(inline)]
pub use client::Client;

mod registry;

#[doc(inline)]
pub use registry::{Crate, Registry, YankState};

mod args;

#[doc(inline)]
pub use args::{Args, PkgId};

mod printer;

#[doc(inline)]
pub use printer::{Printer, YankStatus};

mod features;

// pub use features::Features;

mod util;
