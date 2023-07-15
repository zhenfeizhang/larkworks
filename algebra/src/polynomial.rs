//! Polynomial APIs

use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
    slice::Iter,
};

use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

use crate::Field;

/// core implementation of polynomial traits
mod core;
/// Kyber's polynomial
mod poly3329;

pub use poly3329::Poly3329;

/// A general polynomial in coefficient representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Polynomial<F: Field, const DEGREE: usize> {
    coeffs: [F; DEGREE],
}

/// larkwork's polynomial trait
///
/// A polynomial has its coefficients over F.
pub trait PolynomialOps<F: Field>:
    Sized
    + Eq
    + Copy
    + Clone
    + Default
    + Debug
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + AddAssign
    + SubAssign
    + Sum
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> Sum<&'a Self>
{
    /// Zero element (additive identity)
    const ZERO: Self;
    /// One element (multiplicative identity)
    const ONE: Self;

    /// sample a uniformly random polynomial over modulus
    /// if modulus is None, over the modulus of F
    fn random(rng: impl RngCore, modulus: Option<F>) -> Self;

    /// Sample a random binary polynomial
    fn random_binary(rng: impl RngCore) -> Self {
        // This is likely inefficient.
        // Implementor should overload it with an optimized implementation.
        Self::random(rng, Some(F::from(2u64)))
    }

    /// If the polynomial's coefficients are binary
    fn is_binary(&self) -> bool {
        self.coefficients().all(|&x| x == F::ZERO || x == F::ONE)
    }

    /// If the polynomial's coefficients are ternary
    fn is_ternary(&self) -> bool {
        self.coefficients()
            .all(|&x| x == F::ZERO || x == F::ONE || x == -F::ONE)
    }

    /// If the polynomial is a constant polynomial
    fn is_const(&self) -> bool {
        self.degree() == 0
    }

    /// A 32 bytes digest of the polynomial
    fn digest(&self) -> [u8; 32];

    /// Hash a blob into a message polynomial
    fn from_hash_message(msg: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(msg);
        let digest = hasher.finalize();
        let rng = ChaCha20Rng::from_seed(digest.into());
        Self::random(rng, None)
    }

    /// Infinity norm of the polynomial
    fn infinity_norm(&self) -> u32;

    /// L2 norm of the polynomial
    fn l2_norm(&self) -> u32;

    /// degree of the polynomial
    fn degree(&self) -> usize;

    /// Expose coefficients as a iter, starting from the constant term (x_0,...x_{d-1})
    fn coefficients(&self) -> Iter<'_, F>;

    /// From coefficients; without checking the range
    fn from_coefficients_unchecked(coeff: &[F]) -> Self;

    /// From coefficients; without checking the range
    fn from_coefficients_vec_unchecked(coeff: Vec<F>) -> Self;
}

/// Represents a sparse polynomial
pub trait SparsePolynomial<F: Field>: PolynomialOps<F> {
    /// Error type
    type Error;

    /// Convert from a polynomial.
    /// Returns an error if the original polynomial is not sparse.
    fn from_poly<P: PolynomialOps<F>>(_: &P) -> Result<Self, Self::Error>;

    /// Convert self into a polynomial.
    fn into_poly<P: PolynomialOps<F>>(self) -> P;

    /// Sample a random ternary polynomial with a fixed weight
    fn random_balanced_ternary(rng: impl RngCore, half_weight: usize) -> Self;
}
