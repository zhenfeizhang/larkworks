use core::iter::{Product, Sum};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    slice::Iter,
};

use rand::RngCore;

use crate::{Field, ZZpConfig};

/// Trait definition of vector configurations
pub trait ZZVecConfig: Copy + Debug + Default + Eq + 'static {
    /// Config for the base field
    type BaseConfig: ZZpConfig;
    /// Number of coefficients in a poly
    const DIM: usize;
}

/// larkwork's vector trait
pub trait Vector<C: ZZVecConfig>:
    Sized
    + Eq
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
    /// Base field of the vector
    type BaseField: Field;


    /// sample a uniformly random vector over modulus
    /// if modulus is None, over the modulus of F
    fn random(rng: impl RngCore, modulus: Option<Self::BaseField>) -> Self;

    /// Sample a random binary vector
    fn random_binary(rng: impl RngCore) -> Self {
        // This is likely inefficient.
        // Implementor should overload it with an optimized implementation.
        Self::random(rng, Some(Self::BaseField::from(2u64)))
    }

    /// If the vector's coefficients are binary
    fn is_binary(&self) -> bool {
        self.coefficients()
            .all(|&x| x == Self::BaseField::zero() || x == Self::BaseField::one())

    }


    /// If the vector's coefficients are ternary
    fn is_ternary(&self) -> bool {
        self.coefficients().all(|&x| {
            x == Self::BaseField::zero()
                || x == Self::BaseField::one()
                || x == -Self::BaseField::one()
        })
    }

    /// Return the dimension of the vector
    fn dimension(&self) -> usize;

    /// A 32 bytes digest of the Vector
    fn digest(&self) -> [u8; 32];

    /// Infinity norm of the vector
    fn infinity_norm(&self) -> u32;

    /// L2 norm of the vector
    fn l2_norm(&self) -> u32;

    /// Expose coefficients as a iter
    fn coefficients(&self) -> Iter<'_, Self::BaseField>;


    /// From coefficients; without checking the range
    fn from_coefficients_unchecked(coeffs: &[Self::BaseField]) -> Self {
        Self::from_coefficients_vec_unchecked(coeffs.to_vec())
    }

    /// From coefficients; without checking the range
    fn from_coefficients_vec_unchecked(coeffs: Vec<Self::BaseField>) -> Self;

}
