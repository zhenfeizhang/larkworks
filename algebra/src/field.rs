//! Field APIs.

use std::fmt::Display;

/// Kyber's field
pub mod f3329;

/// Falcon's field
pub mod f12289;

/// Dilithium's field
pub mod f8380417;

/// Useful marcos
mod macros;

/// larkwork's field.
pub trait Field: ff::Field + Display + From<u64> + Into<u64> {}

/// larkwork's prime field.
pub trait PrimeField: ff::PrimeField + Field {
    /// Normalize self into `[-MODULUS_OVER_2, MODULUS_OVER_2)`
    fn lift(&self) -> Self;

    /// Normalize self into `[0, MODULUS)`
    fn normalize(&self) -> Self;
}

/// larkwork's NTT friendly field.
pub trait NTTField: PrimeField {}
