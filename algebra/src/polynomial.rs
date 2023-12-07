//! Polynomial APIs

/// Polynomial trait definitions
mod definition;
/// Instances
mod instances;
/// generic implementation of ZZ[X] mod p
mod zz_px;

pub use definition::{ConfigZZpX, Polynomial};
pub use instances::{ConfigZZpX12289_512, ConfigZZpX3329_256, Poly12289_512, Poly3329_256};
pub use zz_px::ZZpX;
