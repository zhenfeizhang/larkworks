//! Ring APIs
//!

use std::{
    iter::Product,
    ops::{Mul, MulAssign},
};

use rand::RngCore;

use crate::{field::NTTField, polynomial::PolynomialOps};

/// A ring element is a polynomial that also allows for multiplication.
// Although in theory a ring can work on non-NTT friendly field,
// we restrict it to NTTField for convenience.
pub trait PolynomialRing<F: NTTField>:
    PolynomialOps<F>
    + Mul<Output = Self>
    + Product
    + MulAssign
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> Product<&'a Self>
    + for<'a> MulAssign<&'a Self>
{
    /// Degree of ring polynomial
    // FIXME: (alex) should we call this trait PolynomialRing instead?
    const DEGREE: usize;

    /// Parameters that define the ring.
    /// For example `Z_p[x]/(x^N+1)`
    type RingParam;

    /// Normalize self into a polynomial within `[-MODULUS_OVER_2, MODULUS_OVER_2)`
    fn lift(&self) -> Self {
        Self::from_coefficients_vec_unchecked(self.coefficients().map(|x| x.lift()).collect())
    }

    /// Normalize self into a polynomial within `[0, MODULUS)`
    fn normalize(&self) -> Self {
        Self::from_coefficients_vec_unchecked(self.coefficients().map(|x| x.normalize()).collect())
    }

    /// sample a uniformly random ring element
    fn random(rng: impl RngCore) -> Self;
}
