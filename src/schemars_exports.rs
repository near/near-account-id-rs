#[cfg(all(feature = "schemars-v1", not(feature = "schemars-stable")))]
pub use schemars_v1 as schemars;

#[cfg(feature = "schemars-stable")]
pub use schemars_stable as schemars;
