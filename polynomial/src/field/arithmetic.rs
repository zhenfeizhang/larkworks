//! This module defines arithmetics and some additional trait bound that are required for `Field` trait.

use std::{
    cmp::Ordering,
    iter::{Product, Sum},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use ff::Field;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

use super::F;

// =================
// additions
// =================

impl<'a, 'b, const MODULUS: u32> Add<&'a F<MODULUS>> for &'b F<MODULUS> {
    type Output = F<MODULUS>;
    #[inline]
    fn add(self, rhs: &'a F<MODULUS>) -> Self::Output {
        F {
            elem: (self.elem + rhs.elem) % MODULUS as i32,
        }
    }
}

impl<const MODULUS: u32> Add<&F<MODULUS>> for F<MODULUS> {
    type Output = F<MODULUS>;
    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        &self + rhs
    }
}

impl<const MODULUS: u32> Add<F<MODULUS>> for &F<MODULUS> {
    type Output = F<MODULUS>;
    #[inline]
    fn add(self, rhs: F<MODULUS>) -> Self::Output {
        self + &rhs
    }
}

impl<const MODULUS: u32> Add<F<MODULUS>> for F<MODULUS> {
    type Output = F<MODULUS>;
    #[inline]
    fn add(self, rhs: F<MODULUS>) -> Self::Output {
        &self + &rhs
    }
}

impl<const MODULUS: u32> AddAssign<F<MODULUS>> for F<MODULUS> {
    #[inline]
    fn add_assign(&mut self, rhs: F<MODULUS>) {
        *self = &*self + &rhs;
    }
}

impl<'b, const MODULUS: u32> AddAssign<&'b F<MODULUS>> for F<MODULUS> {
    #[inline]
    fn add_assign(&mut self, rhs: &'b F<MODULUS>) {
        *self = &*self + rhs;
    }
}

impl<T, const MODULUS: u32> Sum<T> for F<MODULUS>
where
    T: core::borrow::Borrow<Self>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Self::ZERO, |acc, item| acc + item.borrow())
    }
}

// =================
// subtractions
// =================
impl<'a, 'b, const MODULUS: u32> Sub<&'a F<MODULUS>> for &'b F<MODULUS> {
    type Output = F<MODULUS>;
    #[inline]
    fn sub(self, rhs: &'a F<MODULUS>) -> Self::Output {
        F {
            elem: (self.elem - rhs.elem) % MODULUS as i32,
        }
    }
}

impl<const MODULUS: u32> Sub<&F<MODULUS>> for F<MODULUS> {
    type Output = F<MODULUS>;
    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output {
        &self - rhs
    }
}

impl<const MODULUS: u32> Sub<F<MODULUS>> for &F<MODULUS> {
    type Output = F<MODULUS>;
    #[inline]
    fn sub(self, rhs: F<MODULUS>) -> Self::Output {
        self - &rhs
    }
}

impl<const MODULUS: u32> Sub<F<MODULUS>> for F<MODULUS> {
    type Output = F<MODULUS>;
    #[inline]
    fn sub(self, rhs: F<MODULUS>) -> Self::Output {
        &self - &rhs
    }
}

impl<const MODULUS: u32> SubAssign<F<MODULUS>> for F<MODULUS> {
    #[inline]
    fn sub_assign(&mut self, rhs: F<MODULUS>) {
        *self = &*self - &rhs;
    }
}

impl<'b, const MODULUS: u32> SubAssign<&'b F<MODULUS>> for F<MODULUS> {
    #[inline]
    fn sub_assign(&mut self, rhs: &'b F<MODULUS>) {
        *self = &*self - rhs;
    }
}

// =================
// multiplications
// =================
impl<'a, 'b, const MODULUS: u32> Mul<&'b F<MODULUS>> for &'a F<MODULUS> {
    type Output = F<MODULUS>;

    fn mul(self, rhs: &'b F<MODULUS>) -> Self::Output {
        F {
            elem: (self.elem as i64 * (rhs.elem as i64) % MODULUS as i64) as i32,
        }
    }
}

impl<const MODULUS: u32> Mul<&F<MODULUS>> for F<MODULUS> {
    type Output = F<MODULUS>;
    #[inline]
    fn mul(self, rhs: &Self) -> Self::Output {
        &self - rhs
    }
}

impl<const MODULUS: u32> Mul<F<MODULUS>> for &F<MODULUS> {
    type Output = F<MODULUS>;
    #[inline]
    fn mul(self, rhs: F<MODULUS>) -> Self::Output {
        self - &rhs
    }
}

impl<const MODULUS: u32> Mul<F<MODULUS>> for F<MODULUS> {
    type Output = F<MODULUS>;
    #[inline]
    fn mul(self, rhs: F<MODULUS>) -> Self::Output {
        &self - &rhs
    }
}

impl<const MODULUS: u32> MulAssign<F<MODULUS>> for F<MODULUS> {
    #[inline]
    fn mul_assign(&mut self, rhs: F<MODULUS>) {
        *self = &*self - &rhs;
    }
}

impl<'b, const MODULUS: u32> MulAssign<&'b F<MODULUS>> for F<MODULUS> {
    #[inline]
    fn mul_assign(&mut self, rhs: &'b F<MODULUS>) {
        *self = &*self - rhs;
    }
}

impl<const MODULUS: u32, T> Product<T> for F<MODULUS>
where
    T: core::borrow::Borrow<Self>,
{
    fn product<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, item| acc * item.borrow())
    }
}

// =================
// negation
// =================

impl<const MODULUS: u32> Neg for F<MODULUS> {
    type Output = F<MODULUS>;

    fn neg(self) -> <Self as Neg>::Output {
        Self { elem: -self.elem }
    }
}

// =================
// order
// =================

impl<const MODULUS: u32> PartialOrd for F<MODULUS> {
    fn partial_cmp(&self, other: &F<MODULUS>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const MODULUS: u32> Ord for F<MODULUS> {
    fn cmp(&self, other: &F<MODULUS>) -> Ordering {
        self.elem.cmp(&other.elem)
    }
}

// =================
// order
// =================
impl<const MODULUS: u32> ConditionallySelectable for F<MODULUS> {
    fn conditional_select(a: &Self, b: &Self, c: Choice) -> Self {
        // WARNING: not constant time
        if c.unwrap_u8() == 0 {
            *a
        } else {
            *b
        }
    }
}

impl<const MODULUS: u32> ConstantTimeEq for F<MODULUS> {
    fn ct_eq(&self, rhs: &Self) -> Choice {
        self.elem.ct_eq(&rhs.elem)
    }
}
