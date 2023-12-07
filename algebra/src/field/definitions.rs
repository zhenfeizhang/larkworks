//! Field definitions.

use core::iter::{Product, Sum};
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

use num::cast::AsPrimitive;
use num::{FromPrimitive, ToPrimitive};
use rand::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

/// Larkwork's field.
///
/// Definition adopted from ff::Field, with modifications
pub trait Field:
    Display
    + From<u64>
    + Into<u64>
    + Sized
    + Eq
    + Copy
    + Clone
    + Default
    + Send
    + Sync
    + Debug
    + 'static
    + ConditionallySelectable
    + ConstantTimeEq
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Sum
    + Product
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> Sum<&'a Self>
    + for<'a> Product<&'a Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> MulAssign<&'a Self>
{
    /// The zero element of the field, the additive identity.
    fn zero() -> Self;

    /// The one element of the field, the multiplicative identity.
    fn one() -> Self;

    /// Returns an element chosen uniformly at random using a user-provided RNG.
    fn random(rng: impl RngCore) -> Self;

    /// Returns true iff this element is zero.
    fn is_zero(&self) -> Choice {
        self.ct_eq(&Self::zero())
    }

    /// Returns true iff this element is zero.
    ///
    /// # Security
    ///
    /// This method provides **no** constant-time guarantees. Implementors of the
    /// `Field` trait **may** optimise this method using non-constant-time logic.
    fn is_zero_vartime(&self) -> bool {
        self.is_zero().into()
    }

    /// Squares this element.
    #[must_use]
    fn square(&self) -> Self;

    /// Cubes this element.
    #[must_use]
    fn cube(&self) -> Self {
        self.square() * self
    }

    /// Doubles this element.
    #[must_use]
    fn double(&self) -> Self;

    /// Computes the multiplicative inverse of this element,
    /// failing if the element is zero.
    fn invert(&self) -> CtOption<Self>;

    /// Computes:
    ///
    /// - $(\textsf{true}, \sqrt{\textsf{num}/\textsf{div}})$, if $\textsf{num}$ and
    ///   $\textsf{div}$ are nonzero and $\textsf{num}/\textsf{div}$ is a square in the
    ///   field;
    /// - $(\textsf{true}, 0)$, if $\textsf{num}$ is zero;
    /// - $(\textsf{false}, 0)$, if $\textsf{num}$ is nonzero and $\textsf{div}$ is zero;
    /// - $(\textsf{false}, \sqrt{G_S \cdot \textsf{num}/\textsf{div}})$, if
    ///   $\textsf{num}$ and $\textsf{div}$ are nonzero and $\textsf{num}/\textsf{div}$ is
    ///   a nonsquare in the field;
    ///
    /// where $G_S$ is a non-square.
    ///
    /// # Warnings
    ///
    /// - The choice of root from `sqrt` is unspecified.
    /// - The value of $G_S$ is unspecified, and cannot be assumed to have any specific
    ///   value in a generic context.
    fn sqrt_ratio(num: &Self, div: &Self) -> (Choice, Self);

    /// Equivalent to `Self::sqrt_ratio(self, one())`.
    ///
    /// The provided method is implemented in terms of [`Self::sqrt_ratio`].
    fn sqrt_alt(&self) -> (Choice, Self) {
        Self::sqrt_ratio(self, &Self::one())
    }

    /// Returns the square root of the field element, if it is
    /// quadratic residue.
    ///
    /// The provided method is implemented in terms of [`Self::sqrt_ratio`].
    fn sqrt(&self) -> CtOption<Self> {
        let (is_square, res) = Self::sqrt_ratio(self, &Self::one());
        CtOption::new(res, is_square)
    }

    /// Exponentiates `self` by `exp`, where `exp` is a little-endian order integer
    /// exponent.
    ///
    /// # Guarantees
    ///
    /// This operation is constant time with respect to `self`, for all exponents with the
    /// same number of digits (`exp.as_ref().len()`). It is variable time with respect to
    /// the number of digits in the exponent.
    fn pow<S: AsRef<[u64]>>(&self, exp: S) -> Self {
        let mut res = Self::one();
        for e in exp.as_ref().iter().rev() {
            for i in (0..64).rev() {
                res = res.square();
                let mut tmp = res;
                tmp *= self;
                res.conditional_assign(&tmp, (((*e >> i) & 1) as u8).into());
            }
        }
        res
    }

    /// Exponentiates `self` by `exp`, where `exp` is a little-endian order integer
    /// exponent.
    ///
    /// # Guarantees
    ///
    /// **This operation is variable time with respect to `self`, for all exponent.** If
    /// the exponent is fixed, this operation is effectively constant time. However, for
    /// stronger constant-time guarantees, [`Field::pow`] should be used.
    fn pow_vartime<S: AsRef<[u64]>>(&self, exp: S) -> Self {
        let mut res = Self::one();
        for e in exp.as_ref().iter().rev() {
            for i in (0..64).rev() {
                res = res.square();

                if ((*e >> i) & 1) == 1 {
                    res.mul_assign(self);
                }
            }
        }

        res
    }
}

// /// larkwork's prime field.
// pub trait PrimeField: ff::PrimeField + Field {
//     /// Normalize self into `[-MODULUS_OVER_2, MODULUS_OVER_2)`
//     fn lift(&self) -> Self;

//     /// Normalize self into `[0, MODULUS)`
//     fn normalize(&self) -> Self;
// }

// /// larkwork's NTT friendly field.
// pub trait NTTField: PrimeField {
//     /// The generator of the multiplicative group of the field
//     const GENERATOR: Self;

//     /// Returns the root of unity of order n, if one exists.
//     fn get_root_of_unity(n: u64) -> Option<Self>;
// }

/// Trait definition of configurations
pub trait ZZpConfig: Copy + Debug + Default + Eq + 'static {
    /// Primitive type that used to store the field element
    type PrimitiveType: Display
        + Copy
        + Clone
        + Debug
        + PartialOrd
        + Add<Output = Self::PrimitiveType>
        + AddAssign
        + Sub<Output = Self::PrimitiveType>
        + SubAssign
        + Mul<Output = Self::PrimitiveType>
        + MulAssign
        + Rem<Output = Self::PrimitiveType>
        + Into<u64>
        + AsPrimitive<u64>
        + ConstantTimeEq
        + ConditionallySelectable
        + Sync
        + Send
        + Default
        + Eq
        + ToPrimitive
        + AsPrimitive<Self::ProductType>
        + FromPrimitive;

    /// A larger type that is large enough to store the product of two field elements
    type ProductType: From<Self::PrimitiveType>
        + AsPrimitive<Self::PrimitiveType>
        + Mul<Output = Self::ProductType>
        + Rem<Output = Self::ProductType>
        + AsPrimitive<Self::PrimitiveType>
        + ToPrimitive
        + AsPrimitive<Self::PrimitiveType>;

    /// Modulus
    const MODULUS: Self::PrimitiveType;
}
