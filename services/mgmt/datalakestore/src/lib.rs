#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2016-11")]
pub mod package_2016_11;
#[cfg(all(feature = "package-2016-11", not(feature = "no-default-version")))]
pub use package_2016_11::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2015-10-preview")]
pub mod package_2015_10_preview;
#[cfg(all(feature = "package-2015-10-preview", not(feature = "no-default-version")))]
pub use package_2015_10_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};