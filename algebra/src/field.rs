//! Field APIs.

/// Trait definitions
mod definitions;

/// Instances
mod instances;

/// generic implementation of ZZ mod p
mod zz_p;

pub use definitions::{ConfigZZp, Field};
pub use instances::{
    ConfigZZp12289, ConfigZZp202753, ConfigZZp3168257, ConfigZZp3329, ConfigZZp8380417,
    ConfigZZpGoldilocks, Goldilocks, F12289, F202753, F3168257, F3329, F8380417,
};
pub use zz_p::ZZp;
