//! Field APIs.

/// Trait definitions
mod definitions;

/// Instances
mod instances;

/// generic implementation of ZZ mod p
mod zz_p;

pub use definitions::{ConfigZZp, Field};
pub use instances::{
    ConfigGoldilocks, ConfigZZp12289, ConfigZZp3329, ConfigZZp8380417, Goldilocks, F12289, F3329,
    F8380417,
};
pub use zz_p::ZZp;
