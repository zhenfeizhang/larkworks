//! Field APIs.

/// larkwork's field.
pub trait Field: ff::Field + From<u64> + Into<u64> {}

/// larkwork's prime field.
pub trait PrimeField: ff::PrimeField + Field {
    /// Normalize self into `[-MODULUS_OVER_2, MODULUS_OVER_2)`
    fn lift(&self) -> Self;

    /// Normalize self into `[0, MODULUS)`
    fn normalize(&self) -> Self;
}

/// larkwork's NTT friendly field.
pub trait NTTField: PrimeField {}
