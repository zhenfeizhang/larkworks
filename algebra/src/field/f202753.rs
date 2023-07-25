use core::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use ff::Field as FfField;
use rand::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::{field_common, Field, NTTField, PrimeField};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct F202753(i32);

pub const MODULUS: i32 = 202753;

impl Add for F202753 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut tmp = self.0 + rhs.0;
        if tmp >= MODULUS {
            tmp -= MODULUS
        }
        Self(tmp)
    }
}

impl Sub for F202753 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 >= rhs.0 {
            Self(self.0 - rhs.0)
        } else {
            Self(self.0 + MODULUS - rhs.0)
        }
    }
}

impl Mul for F202753 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 as i64 * rhs.0 as i64 % MODULUS as i64) as i32)
    }
}

impl Neg for F202753 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self == Self::ZERO {
            self
        } else {
            Self(MODULUS - self.0)
        }
    }
}

field_common!(F202753, MODULUS, i32);

impl Field for F202753 {
    type PrimitiveType = i32;
}

impl ff::PrimeField for F202753 {
    type Repr = [u8; 4];

    const MODULUS: &'static str = "202753";
    const NUM_BITS: u32 = 18;
    const CAPACITY: u32 = 17;
    const TWO_INV: Self = Self(101377);
    const MULTIPLICATIVE_GENERATOR: Self = Self(10);
    const S: u32 = 11;

    // FIXME: this root is not generated from
    // exponentiating `Self::MULTIPLICATIVE_GENERATOR` by `t`
    // (there are multiple number of roots here)
    const ROOT_OF_UNITY: Self = Self(202470);
    // FIXME
    const ROOT_OF_UNITY_INV: Self = Self(85973);
    // FIXME
    const DELTA: Self = Self(0);

    fn from_repr(repr: Self::Repr) -> CtOption<Self> {
        // FIXME: normalize the data, and return error if not in range?
        CtOption::new(Self(i32::from_le_bytes(repr)), Choice::from(1))
    }

    fn to_repr(&self) -> Self::Repr {
        self.0.to_le_bytes()
    }

    fn is_odd(&self) -> Choice {
        // FIXME: do we need to normalize this first?
        Choice::from((self.0 % 2) as u8)
    }
}

impl PrimeField for F202753 {
    const MODULUS: Self::PrimitiveType = MODULUS;

    /// Normalize self into `[-MODULUS_OVER_2, MODULUS_OVER_2)`
    fn lift(&self) -> Self {
        let mut t = self.0 % Self::MODULUS;
        if t * 2 > Self::MODULUS {
            t -= Self::MODULUS
        };
        Self(t)
    }
    /// Normalize self into `[0, MODULUS)`
    fn normalize(&self) -> Self {
        Self((self.0 % Self::MODULUS + Self::MODULUS) % Self::MODULUS)
    }
}

impl NTTField for F202753 {}

#[cfg(test)]
mod tests {
    use super::F202753;
    use crate::tests::field::random_field_tests;

    #[test]
    fn test_integer() {
        random_field_tests::<F202753>("F202753".to_string());
    }
}
