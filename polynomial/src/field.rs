use ff::Field;
use rand::RngCore;
use std::ops::{Add, AddAssign};
use subtle::{Choice, ConstantTimeEq, CtOption};

mod arithmetic;
// use crate::impl_field;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct F<const MODULUS: u32> {
    pub(crate) elem: i32,
}

impl<const MODULUS: u32> Field for F<MODULUS> {
    const ZERO: Self = Self { elem: 0 };
    const ONE: Self = Self { elem: 1 };

    /// Returns an element chosen uniformly at random using a user-provided RNG.
    fn random(mut rng: impl RngCore) -> Self {
        Self {
            elem: (rng.next_u32() % MODULUS) as i32,
        }
    }

    /// Returns true iff this element is zero.
    fn is_zero(&self) -> Choice {
        self.ct_eq(&Self::ZERO)
    }

    fn square(&self) -> Self {
        self * self
    }

    fn double(&self) -> Self {
        self + self
    }

    /// Computes the multiplicative inverse of this element,
    /// failing if the element is zero.
    fn invert(&self) -> CtOption<Self> {
        todo!()
    }

    fn sqrt_ratio(num: &Self, div: &Self) -> (Choice, Self) {
        todo!()
    }
}
