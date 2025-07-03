#[cfg(all(feature = "schemars-alpha", not(feature = "schemars-stable")))]
pub use schemars_alpha as schemars;

#[cfg(feature = "schemars-stable")]
pub use schemars_stable as schemars;
