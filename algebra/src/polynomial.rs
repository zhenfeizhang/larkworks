//! Polynomial APIs

/// Polynomial trait definitions
mod definitions;
/// Instances
mod instances;
/// generic implementation of ZZ[X] mod p
mod zz_px;

pub use definitions::{Polynomial, ZZpXConfig};
pub use instances::{Poly12289_512, Poly3329_256, ZZpXConfig12289_512, ZZpXConfig3329_256};
pub use zz_px::ZZpX;
