#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-0.2-preview.1")]
pub mod package_0_2_preview_1;
#[cfg(all(feature = "package-0.2-preview.1", not(feature = "no-default-version")))]
pub use package_0_2_preview_1::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-01-01")]
pub mod package_2021_01_01;
#[cfg(all(feature = "package-2021-01-01", not(feature = "no-default-version")))]
pub use package_2021_01_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-0.2-preview.0")]
pub mod package_0_2_preview_0;
#[cfg(all(feature = "package-0.2-preview.0", not(feature = "no-default-version")))]
pub use package_0_2_preview_0::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-01-01-preview")]
pub mod package_2021_01_01_preview;
#[cfg(all(feature = "package-2021-01-01-preview", not(feature = "no-default-version")))]
pub use package_2021_01_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-02-28-preview")]
pub mod package_2019_02_28_preview;
#[cfg(all(feature = "package-2019-02-28-preview", not(feature = "no-default-version")))]
pub use package_2019_02_28_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};