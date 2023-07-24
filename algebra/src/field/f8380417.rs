use core::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use ff::Field as FfField;
use rand::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::{field_common, Field};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct F8380417(u32);

pub const MODULUS: u32 = 8380417;

// // T * 2^s = 1
// const T: u16 = 3;
// const S: u16 = 12;
// //
// const Q_OVER_TWO_MINUS_ONE: u16 = 6143;

impl Add for F8380417 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut tmp = self.0 + rhs.0;
        if tmp >= MODULUS {
            tmp -= MODULUS
        }
        Self(tmp)
    }
}

impl Sub for F8380417 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 >= rhs.0 {
            Self(self.0 - rhs.0)
        } else {
            Self(self.0 + MODULUS - rhs.0)
        }
    }
}

impl Mul for F8380417 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 as u64 * rhs.0 as u64 % MODULUS as u64) as u32)
    }
}

impl Neg for F8380417 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self == Self::ZERO {
            self
        } else {
            Self(MODULUS - self.0)
        }
    }
}

field_common!(F8380417, MODULUS, u32);

impl Field for F8380417 {
    type PrimitiveType = u32;
}

#[cfg(test)]
mod tests {
    use super::F8380417;
    use crate::tests::field::random_field_tests;

    #[test]
    fn test_integer() {
        random_field_tests::<F8380417>("F8380417".to_string());
    }
}
