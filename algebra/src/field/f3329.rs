use core::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use ff::Field as FfField;
use rand::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::{field_common, Field};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct F3329(u16);

pub const MODULUS: u16 = 3329;

impl Add for F3329 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut tmp = self.0 + rhs.0;
        if tmp >= MODULUS {
            tmp -= MODULUS
        }
        Self(tmp)
    }
}

impl Sub for F3329 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 >= rhs.0 {
            Self(self.0 - rhs.0)
        } else {
            Self(self.0 + MODULUS - rhs.0)
        }
    }
}

impl Mul for F3329 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 as u32 * rhs.0 as u32 % MODULUS as u32) as u16)
    }
}

impl Neg for F3329 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self == Self::ZERO {
            self
        } else {
            Self(MODULUS - self.0)
        }
    }
}

field_common!(F3329, MODULUS, u16);

impl Field for F3329 {
    type PrimitiveType = u32;
}

#[cfg(test)]
mod tests {
    use super::F3329;
    use crate::tests::field::random_field_tests;

    #[test]
    fn test_integer() {
        random_field_tests::<F3329>("F3329".to_string());
    }
}
