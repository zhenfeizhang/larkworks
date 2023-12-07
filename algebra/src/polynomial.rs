//! Polynomial APIs

/// Polynomial trait definitions
mod definitions;
/// Instances
mod instances;
/// generic implementation of ZZ[X] mod p
mod zz_px;

pub use definitions::{Polynomial, ConfigZZpX};
pub use instances::{Poly12289_512, Poly3329_256, ConfigZZpX12289_512, ConfigZZpX3329_256};
pub use zz_px::ZZpX;
