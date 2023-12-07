//! Field APIs.

/// Trait definitions
mod definitions;

/// Instances
mod instances;

/// generic implementation of ZZ mod p
mod zz_p;

pub use definitions::{Field, ZZpConfig};
pub use instances::{ZZpConfig12289, ZZpConfig3329, ZZpConfig8380417, F12289, F3329, F8380417};
pub use zz_p::ZZp;
