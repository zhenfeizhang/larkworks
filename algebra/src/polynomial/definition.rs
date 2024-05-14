use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
    slice::Iter,
};

use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

use crate::{ConfigZZp, Field};

/// Trait definition of polynomial configurations
pub trait ConfigZZpX: Copy + Debug + Default + Eq + 'static {
    /// Config for the base field
    type BaseConfig: ConfigZZp;
    /// Number of coefficients in a poly
    const DIM: usize;
}

/// larkwork's polynomial trait
///
/// A polynomial has its coefficients over F.
pub trait Polynomial<Config>:
    Sized
    + Eq
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
    /// Base field of the polynomial
    type BaseField: Field;

    /// Zero element (additive identity)
    fn zero() -> Self;

    /// One element (multiplicative identity)
    fn one() -> Self;

    /// sample a uniformly random polynomial over modulus
    /// if modulus is None, over the modulus of F
    fn random(rng: impl RngCore, modulus: Option<Self::BaseField>) -> Self;

    /// Sample a random binary polynomial
    fn random_binary(rng: impl RngCore) -> Self {
        // This is likely inefficient.
        // Implementor should overload it with an optimized implementation.
        Self::random(rng, Some(Self::BaseField::from(2u64)))
    }

    /// If the polynomial's coefficients are binary
    fn is_binary(&self) -> bool {
        self.coefficients()
            .all(|&x| x == Self::BaseField::zero() || x == Self::BaseField::one())
    }

    /// If the polynomial's coefficients are ternary
    fn is_ternary(&self) -> bool {
        self.coefficients().all(|&x| {
            x == Self::BaseField::zero()
                || x == Self::BaseField::one()
                || x == -Self::BaseField::one()
        })
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
    fn coefficients(&self) -> Iter<'_, Self::BaseField>;

    /// From coefficients; without checking the range
    fn from_coefficients_unchecked(coeffs: &[Self::BaseField]) -> Self {
        Self::from_coefficients_vec_unchecked(coeffs.to_vec())
    }
    /// From coefficients; without checking the range
    fn from_coefficients_vec_unchecked(coeffs: Vec<Self::BaseField>) -> Self;

    /// From primitive types; without checking the range
    fn from_primitive_types(coeffs: &[<Self::BaseField as Field>::PrimitiveType]) -> Self;
}

// /// Represents a sparse polynomial
// pub trait SparsePolynomial<F: Field>: Polynomial<F> {
//     /// Error type
//     type Error;

//     /// Convert from a polynomial.
//     /// Returns an error if the original polynomial is not sparse.
//     fn from_poly<P: Polynomial<F>>(_: &P) -> Result<Self, Self::Error>;

//     /// Convert self into a polynomial.
//     fn into_poly<P: Polynomial<F>>(self) -> P;

//     /// Sample a random ternary polynomial with a fixed weight
//     fn random_balanced_ternary(rng: impl RngCore, half_weight: usize) -> Self;
// }
