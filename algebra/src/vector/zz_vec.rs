use core::slice::Iter;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rand::RngCore;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::Field;
use crate::Vector;
use crate::{ConfigZZVec, ZZp};

/// ZZ_vec
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZZVec<C: ConfigZZVec> {
    pub(crate) coeffs: Vec<ZZp<C::BaseConfig>>,
}

impl<C: ConfigZZVec> Default for ZZVec<C> {
    fn default() -> Self {
        Self {
            coeffs: vec![ZZp::<C::BaseConfig>::default(); C::MAX_DIM],
        }
    }
}

impl<C: ConfigZZVec> Display for ZZVec<C> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "vector:")?;
        for chunk in self.coeffs.chunks(32) {
            for e in chunk {
                write!(f, "{:4} ", e)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// ===========================
// multiplications
// ===========================
impl<'a, C: ConfigZZVec> Mul<&'a Self> for ZZVec<C> {
    type Output = Self;

    // Coefficient wise multiplications with mod reduction.
    fn mul(self, other: &'a Self) -> Self {
        let mut res = self;
        res *= other;
        res
    }
}

impl<C: ConfigZZVec> Mul for ZZVec<C> {
    type Output = Self;

    // Coefficient wise multiplications with mod reduction.
    fn mul(self, other: Self) -> Self {
        self * &other
    }
}

impl<C: ConfigZZVec> MulAssign for ZZVec<C> {
    // Coefficient wise multiplications with mod reduction.
    fn mul_assign(&mut self, rhs: Self) {
        *self *= &rhs;
    }
}

impl<'a, C: ConfigZZVec> MulAssign<&'a Self> for ZZVec<C> {
    // Coefficient wise multiplications with mod reduction.
    fn mul_assign(&mut self, rhs: &'a Self) {
        self.coeffs
            .par_iter_mut()
            .zip(rhs.coeffs.par_iter())
            .for_each(|(x, y)| *x *= y)
    }
}

impl<C, T> Product<T> for ZZVec<C>
where
    C: ConfigZZVec,
    T: core::borrow::Borrow<Self>,
{
    fn product<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, item| acc * item.borrow())
    }
}

// ===========================
// additions
// ===========================
impl<'a, C: ConfigZZVec> Add<&'a Self> for ZZVec<C> {
    type Output = Self;

    // Coefficient wise additions without mod reduction.
    fn add(self, other: &'a Self) -> Self {
        let mut res = self;
        res += other;
        res
    }
}

impl<C: ConfigZZVec> Add for ZZVec<C> {
    type Output = Self;

    // Coefficient wise additions without mod reduction.
    fn add(self, other: Self) -> Self {
        self + &other
    }
}

impl<C: ConfigZZVec> AddAssign for ZZVec<C> {
    // Coefficient wise additions without mod reduction.
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl<'a, C: ConfigZZVec> AddAssign<&'a Self> for ZZVec<C> {
    // Coefficient wise additions without mod reduction.
    fn add_assign(&mut self, rhs: &'a Self) {
        self.coeffs
            .par_iter_mut()
            .zip(rhs.coeffs.par_iter())
            .for_each(|(x, y)| *x += y)
    }
}

// ===========================
// subtract
// ===========================
impl<'a, C: ConfigZZVec> Sub<&'a Self> for ZZVec<C> {
    type Output = Self;

    // Coefficient wise subtractions without mod reduction.
    fn sub(self, other: &'a Self) -> Self {
        let mut res = self;
        res -= other;
        res
    }
}
impl<C: ConfigZZVec> Sub for ZZVec<C> {
    type Output = Self;

    // Coefficient wise subtractions with mod reduction.
    fn sub(self, other: Self) -> Self {
        self - &other
    }
}

impl<C: ConfigZZVec> SubAssign for ZZVec<C> {
    // Coefficient wise subtractions without mod reduction.
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}

impl<'a, C: ConfigZZVec> SubAssign<&'a Self> for ZZVec<C> {
    // Coefficient wise subtractions without mod reduction.
    fn sub_assign(&mut self, rhs: &'a Self) {
        self.coeffs
            .par_iter_mut()
            .zip(rhs.coeffs.par_iter())
            .for_each(|(x, y)| *x -= y)
    }
}

// ===========================
// neg
// ===========================
impl<C: ConfigZZVec> Neg for ZZVec<C> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut res = self;
        res.coeffs.par_iter_mut().for_each(|x| *x = -*x);
        res
    }
}

impl<C: ConfigZZVec, T> Sum<T> for ZZVec<C>
where
    T: core::borrow::Borrow<Self>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Self::default(), |acc, item| acc + item.borrow())
    }
}

impl<C: ConfigZZVec> Vector<C> for ZZVec<C> {
    /// Base field of the vector
    type BaseField = ZZp<C::BaseConfig>;

    /// Zero element (additive identity)
    fn zero() -> Self {
        Self {
            coeffs: vec![Self::BaseField::zero(); C::MAX_DIM],
        }
    }

    /// One element (multiplicative identity)
    fn one() -> Self {
        Self {
            coeffs: vec![Self::BaseField::one(); C::MAX_DIM],
        }
    }

    /// sample a uniformly random Vector over modulus
    /// if modulus is None, over the modulus of F
    fn random(mut rng: impl RngCore, modulus: Option<Self::BaseField>) -> Self {
        let coeff: Vec<Self::BaseField> = match modulus {
            Some(modulus) => (0..C::MAX_DIM)
                .map(|_| Self::BaseField::from(rng.next_u64() % modulus.0.into()))
                .collect(),
            None => (0..C::MAX_DIM)
                .map(|_| Self::BaseField::random(&mut rng))
                .collect(),
        };

        Self { coeffs: coeff }
    }

    /// Sample a random binary Vector
    fn random_binary(mut rng: impl RngCore) -> Self {
        let coeff: Vec<Self::BaseField> = (0..C::MAX_DIM)
            .map(|_| (rng.next_u64() % 2).into())
            .collect();
        Self { coeffs: coeff }
    }

    /// A 32 bytes digest of the Vector
    fn digest(&self) -> [u8; 32] {
        todo!()
    }

    /// Infinity norm of the Vector
    fn infinity_norm(&self) -> u32 {
        todo!()
    }

    /// L2 norm of the Vector
    fn l2_norm(&self) -> u32 {
        todo!()
    }

    /// Max supported dimension
    fn max_dimension(&self) -> usize {
        C::MAX_DIM
    }

    /// degree of the Vector
    fn dimension(&self) -> usize {
        self.coefficients().len()
    }

    /// Expose coefficients as a iter, starting from the constant term (x_0,...x_{d-1})
    fn coefficients(&self) -> Iter<'_, Self::BaseField> {
        self.coeffs.iter()
    }
    /// From coefficients; without checking the range
    fn from_coefficients_vec_unchecked(coeffs: Vec<Self::BaseField>) -> Self {
        Self { coeffs }
    }

    /// From primitive types; without checking the range
    fn from_primitive_types(coeffs: &[<Self::BaseField as Field>::PrimitiveType]) -> Self {
        let coeffs = coeffs.iter().map(Self::BaseField::new).collect::<Vec<_>>();
        Self { coeffs }
    }
}
