//! Ring APIs
//!

mod definition;
mod instances;

pub use definition::PolynomialRing;
pub use instances::{
    ConfigRing12289_512, ConfigRing202753_512, ConfigRingGoldilocks256, Ring12289_512,
    Ring202753_512, RingGoldilock256,
};
