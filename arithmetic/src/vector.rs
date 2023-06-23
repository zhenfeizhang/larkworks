use std::{
    fmt::Debug,
    iter::{Product, Sum},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    slice::Iter,
};

use rand::RngCore;

use crate::{field::NTTField, Field, RingElement};

/// larkwork's vector trait
pub trait Vector<F: Field>:
    Sized
    + Eq
    + Copy
    + Clone
    + Default
    + Debug
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    // pair-wise multiplication
    + Mul<Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + Sum
    + Product
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> MulAssign<&'a Self>
    + for<'a> Sum<&'a Self>
    + for<'a> Product<&'a Self>
{
    /// sample a uniformly random vector over modulus
    /// if modulus is None, over the modulus of F
    fn random(rng: impl RngCore, modulus: Option<F>) -> Self;

    /// Sample a random binary vector
    fn random_binary(rng: impl RngCore) -> Self {
        // This is likely inefficient.
        // Implementor should overload it with an optimized implementation.
        Self::random(rng, Some(F::from(2u64)))
    }

    /// If the vector's coefficients are binary
    fn is_binary(&self) -> bool {
        self.coefficients().all(|&x| x == F::ZERO || x == F::ONE)
    }


    /// If the vector's coefficients are ternary
    fn is_ternary(&self) -> bool {
        self.coefficients()
            .all(|&x| x == F::ZERO || x == F::ONE || x == -F::ONE)
    }

    /// Return the dimension of the vector
    fn dimension(&self) -> usize;

    /// Infinity norm of the vector
    fn infinity_norm(&self) -> u32;

    /// L2 norm of the vector
    fn l2_norm(&self) -> u32;

    /// Expose coefficients as a iter
    fn coefficients(&self) -> Iter<'_, F>;

    /// From coefficients; without checking the range
    fn from_coefficients_unchecked(coeff: &[F]) -> Self;

    /// From coefficients; without checking the range
    fn from_coefficients_vec_unchecked(coeff: Vec<F>) -> Self;
}

/// Associating the vector with a lattice
pub trait LatticeVector<F: Field>: Vector<F> {
    type LatticeParam;
}

/// Associating the vector with an NTT domain and a ring
pub trait NTTVector<F: NTTField>: Vector<F> {
    type RingElement: RingElement<F>;

    /// Build an NTT vector from a ring element.
    fn from_ring_element(_: &Self::RingElement) -> Self;

    /// Convert NTT vector to a ring element.
    fn into_ring_element(&self) -> Self::RingElement;
}
