use core::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num::traits::AsPrimitive;
use num::FromPrimitive;
use num::ToPrimitive;
use rand::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::ConfigZZp;
use crate::Field;

/// Integers modulo P
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZZp<C: ConfigZZp>(pub(crate) C::PrimitiveType);

impl<C: ConfigZZp> std::fmt::Display for ZZp<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ========================
// subtractions
// ========================
impl<C: ConfigZZp> Neg for ZZp<C> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self == Self::zero() {
            self
        } else {
            Self(C::MODULUS - self.0)
        }
    }
}

impl<C: ConfigZZp> Sub for ZZp<C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 >= rhs.0 {
            Self(self.0 - rhs.0)
        } else {
            Self(self.0 + C::MODULUS - rhs.0)
        }
    }
}

impl<'b, C: ConfigZZp> Sub<&'b ZZp<C>> for ZZp<C> {
    type Output = ZZp<C>;

    #[inline]
    fn sub(self, rhs: &'b ZZp<C>) -> ZZp<C> {
        self.sub(*rhs)
    }
}

impl<C: ConfigZZp> SubAssign for ZZp<C> {
    #[inline]
    fn sub_assign(&mut self, rhs: ZZp<C>) {
        *self = (*self).sub(rhs)
    }
}

impl<'b, C: ConfigZZp> SubAssign<&'b ZZp<C>> for ZZp<C> {
    #[inline]
    fn sub_assign(&mut self, rhs: &'b ZZp<C>) {
        *self = (*self).sub(rhs)
    }
}

// ========================
// additions
// ========================
impl<C: ConfigZZp> Add for ZZp<C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut tmp = self.0 + rhs.0;
        if tmp >= C::MODULUS {
            tmp -= C::MODULUS
        }
        Self(tmp)
    }
}

impl<'b, C: ConfigZZp> Add<&'b ZZp<C>> for ZZp<C> {
    type Output = ZZp<C>;

    #[inline]
    fn add(self, rhs: &'b ZZp<C>) -> ZZp<C> {
        self.add(*rhs)
    }
}

impl<C: ConfigZZp> AddAssign for ZZp<C> {
    #[inline]
    fn add_assign(&mut self, rhs: ZZp<C>) {
        *self = (*self).add(rhs)
    }
}

impl<'b, C: ConfigZZp> AddAssign<&'b ZZp<C>> for ZZp<C> {
    #[inline]
    fn add_assign(&mut self, rhs: &'b ZZp<C>) {
        *self = (*self).add(rhs)
    }
}

impl<T, C: ConfigZZp> Sum<T> for ZZp<C>
where
    T: core::borrow::Borrow<Self>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Self::zero(), |acc, item| acc + item.borrow())
    }
}

// ========================
// multiplications
// ========================
impl<C: ConfigZZp> Mul for ZZp<C> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(
            (C::ProductType::from(self.0) * C::ProductType::from(rhs.0)
                % C::ProductType::from(C::MODULUS))
            .as_(),
        )
    }
}

impl<'b, C: ConfigZZp> Mul<&'b ZZp<C>> for ZZp<C> {
    type Output = ZZp<C>;

    #[inline]
    fn mul(self, rhs: &'b ZZp<C>) -> ZZp<C> {
        self.mul(*rhs)
    }
}

impl<C: ConfigZZp> MulAssign for ZZp<C> {
    #[inline]
    fn mul_assign(&mut self, rhs: ZZp<C>) {
        *self = (*self).mul(rhs)
    }
}

impl<'b, C: ConfigZZp> MulAssign<&'b ZZp<C>> for ZZp<C> {
    #[inline]
    fn mul_assign(&mut self, rhs: &'b ZZp<C>) {
        *self = (*self).mul(rhs)
    }
}

impl<T, C: ConfigZZp> Product<T> for ZZp<C>
where
    T: core::borrow::Borrow<Self>,
{
    fn product<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, item| acc * item.borrow())
    }
}

// ========================
// conversions
// ========================
impl<C: ConfigZZp> From<u64> for ZZp<C> {
    fn from(value: u64) -> Self {
        assert!(value < C::MODULUS.into());
        Self(C::PrimitiveType::from_u64(value).unwrap())
    }
}

impl<C: ConfigZZp> From<ZZp<C>> for u64 {
    fn from(value: ZZp<C>) -> Self {
        value.0.into()
    }
}

// ========================
// misc
// ========================
impl<C: ConfigZZp> ConstantTimeEq for ZZp<C> {
    fn ct_eq(&self, other: &Self) -> subtle::Choice {
        self.0.ct_eq(&other.0)
    }
}

impl<C: ConfigZZp> ConditionallySelectable for ZZp<C> {
    fn conditional_select(a: &Self, b: &Self, choice: subtle::Choice) -> Self {
        Self(C::PrimitiveType::conditional_select(&a.0, &b.0, choice))
    }
}

// ========================
// ff::Field
// ========================
impl<C: ConfigZZp> Field for ZZp<C> {
    /// Primitive type used to store the element
    type PrimitiveType = C::PrimitiveType;

    /// The zero element of the field, the additive identity.
    fn zero() -> Self {
        0u64.into()
    }

    /// The one element of the field, the multiplicative identity.
    fn one() -> Self {
        1u64.into()
    }

    /// Build a new instance from primitive type
    fn new(p: &Self::PrimitiveType) -> Self {
        Self(*p)
    }

    /// Returns an element chosen uniformly at random using a user-provided RNG.
    fn random(mut rng: impl RngCore) -> Self {
        (rng.next_u64() % C::MODULUS.into()).into()
    }

    /// Squares this element.
    #[must_use]
    fn square(&self) -> Self {
        *self * *self
    }

    /// Doubles this element.
    #[must_use]
    fn double(&self) -> Self {
        *self + *self
    }

    /// Computes the multiplicative inverse of this element,
    /// failing if the element is zero.
    fn invert(&self) -> CtOption<Self> {
        let tmp = self.pow_vartime([C::MODULUS.to_u64().unwrap() - 2u64]);
        CtOption::new(tmp, !self.ct_eq(&Self::zero()))
    }

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
    fn sqrt_ratio(_num: &Self, _div: &Self) -> (Choice, Self) {
        todo!()
    }
}
