use core::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use ff::Field as FfField;
use rand::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::{field_common, Field, PrimeField};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct F12289(u16);

pub const MODULUS: u16 = 12289;

// // T * 2^s = 1
// const T: u16 = 3;
// const S: u16 = 12;
// //
// const Q_OVER_TWO_MINUS_ONE: u16 = 6143;

impl Add for F12289 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut tmp = self.0 + rhs.0;
        if tmp >= MODULUS {
            tmp -= MODULUS
        }
        Self(tmp)
    }
}

impl Sub for F12289 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 >= rhs.0 {
            Self(self.0 - rhs.0)
        } else {
            Self(self.0 + MODULUS - rhs.0)
        }
    }
}

impl Mul for F12289 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 as u32 * rhs.0 as u32 % MODULUS as u32) as u16)
    }
}

impl Neg for F12289 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self == Self::ZERO {
            self
        } else {
            Self(MODULUS - self.0)
        }
    }
}

field_common!(F12289, MODULUS, u16);

impl Field for F12289 {}

impl ff::PrimeField for F12289 {
    const MODULUS: &'static str = "12289";
    const NUM_BITS: u32 = 14;
    const CAPACITY: u32 = 13;
    /// 1/2 = 6145
    const TWO_INV: Self = Self(6145);
    /// R = Zmod(12289)
    /// R.multiplicative_generator()           
    /// 11
    const MULTIPLICATIVE_GENERATOR: Self = Self(11);
    const S: u32 = 12;

    /// 7^(2^12) == 1
    const ROOT_OF_UNITY: Self = Self(7);
    /// 1/7 == 8778
    const ROOT_OF_UNITY_INV: Self = Self(8778);
    /// 11^(2^12) == 6240
    const DELTA: Self = Self(6240);

    type Repr = [u8; 2];

    fn from_repr(repr: Self::Repr) -> CtOption<Self> {
        let tmp = repr[0] as u16 + (repr[1] as u16) << 8;
        CtOption::new(Self(tmp), Choice::from(1))
    }

    fn to_repr(&self) -> Self::Repr {
        [(self.0 & 0xFF) as u8, (self.0 >> 8) as u8]
    }

    fn is_odd(&self) -> Choice {
        Choice::from((self.0 & 1) as u8)
    }
}

impl PrimeField for F12289 {
    /// we wrap a u16 so there is no lifted value
    fn lift(&self) -> Self {
        unimplemented!()
    }

    fn normalize(&self) -> Self {
        Self(self.0 % MODULUS)
    }
}

#[cfg(test)]
mod tests {
    use super::F12289;
    use crate::tests::field::random_field_tests;

    #[test]
    fn test_integer() {
        random_field_tests::<F12289>("F12289".to_string());
    }
}
