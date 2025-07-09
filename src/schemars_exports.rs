#[cfg(all(feature = "schemars-alpha", not(feature = "schemars-stable"), not(feature = "schemars-v1"), not(feature = "schemars-v0_8")))]
pub use schemars_v1 as schemars;

#[cfg(all(feature = "schemars-v1", not(feature = "schemars-alpha"), not(feature = "schemars-stable"), not(feature = "schemars-v0_8")))]
pub use schemars_v1 as schemars;

#[cfg(all(feature = "schemars-v0_8", not(feature = "schemars-v1"), not(feature = "schemars-alpha"), not(feature = "schemars-stable")))]
pub use schemars_v0_8 as schemars;

#[cfg(feature = "schemars-stable")]
pub use schemars_v0_8 as schemars;
