#[cfg(all(feature = "schemars-version-one", not(feature = "schemars-stable")))]
pub use schemars_version_one as schemars;

#[cfg(feature = "schemars-stable")]
pub use schemars_stable as schemars;
