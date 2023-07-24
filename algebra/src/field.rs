//! Field APIs.

use std::fmt::Display;

/// Falcon's field
pub mod f12289;
/// Chipmunk's field
pub mod f202753;
/// Kyber's field
pub mod f3329;
/// Dilithium's field
pub mod f8380417;

/// Useful marcos
mod macros;

/// larkwork's field.
pub trait Field: ff::Field + Display + From<u64> + Into<u64> {
    /// the primitive type that holds this field element.
    type PrimitiveType;
}

/// larkwork's prime field.
// FIXME:
// The ff::PrimeField trait is indeed PrimeField and NTTField.
// we may consider to split them.
pub trait PrimeField: ff::PrimeField + Field {
    /// Modulus for the prime field
    const MODULUS: Self::PrimitiveType;

    /// Normalize self into `[-MODULUS_OVER_2, MODULUS_OVER_2)`
    fn lift(&self) -> Self;

    /// Normalize self into `[0, MODULUS)`
    fn normalize(&self) -> Self;
}

/// larkwork's NTT friendly field.
pub trait NTTField: PrimeField {
    // Those are already defined in PrimeField.

    // /// The generator of the multiplicative group of the field
    // const GENERATOR: Self;

    // /// Returns the root of unity of order n, if one exists.
    // fn get_root_of_unity(n: u64) -> Option<Self>;
}
