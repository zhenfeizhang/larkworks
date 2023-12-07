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

use crate::Vector;
use crate::{ZZVecConfig, ZZp};

/// ZZ_vec
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZZVec<C: ZZVecConfig> {
    pub(crate) coeffs: Vec<ZZp<C::BaseConfig>>,
}

impl<C: ZZVecConfig> Default for ZZVec<C> {
    fn default() -> Self {
        Self {
            coeffs: vec![ZZp::<C::BaseConfig>::default(); C::DIM],
        }
    }
}

impl<C: ZZVecConfig> Display for ZZVec<C> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "vector:")?;
        for chunk in self.coeffs.chunks(8) {
            for e in chunk {
                write!(f, "{}", e)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

// ===========================
// multiplications
// ===========================
impl<'a, C: ZZVecConfig> Mul<&'a Self> for ZZVec<C> {
    type Output = Self;

    // Coefficient wise multiplications with mod reduction.
    fn mul(self, other: &'a Self) -> Self {
        let mut res = self;
        res += other;
        res
    }
}

impl<C: ZZVecConfig> Mul for ZZVec<C> {
    type Output = Self;

    // Coefficient wise multiplications with mod reduction.
    fn mul(self, other: Self) -> Self {
        self + &other
    }
}

impl<C: ZZVecConfig> MulAssign for ZZVec<C> {
    // Coefficient wise multiplications with mod reduction.
    fn mul_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl<'a, C: ZZVecConfig> MulAssign<&'a Self> for ZZVec<C> {
    // Coefficient wise multiplications with mod reduction.
    fn mul_assign(&mut self, rhs: &'a Self) {
        self.coeffs
            .par_iter_mut()
            .zip(rhs.coeffs.par_iter())
            .for_each(|(x, y)| *x += y)
    }
}

impl<C, T> Product<T> for ZZVec<C>
where
    C: ZZVecConfig,
    T: core::borrow::Borrow<Self>,
{
    fn product<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, item| acc * item.borrow())
    }
}

// ===========================
// additions
// ===========================
impl<'a, C: ZZVecConfig> Add<&'a Self> for ZZVec<C> {
    type Output = Self;

    // Coefficient wise additions without mod reduction.
    fn add(self, other: &'a Self) -> Self {
        let mut res = self;
        res += other;
        res
    }
}

impl<C: ZZVecConfig> Add for ZZVec<C> {
    type Output = Self;

    // Coefficient wise additions without mod reduction.
    fn add(self, other: Self) -> Self {
        self + &other
    }
}

impl<C: ZZVecConfig> AddAssign for ZZVec<C> {
    // Coefficient wise additions without mod reduction.
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl<'a, C: ZZVecConfig> AddAssign<&'a Self> for ZZVec<C> {
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
impl<'a, C: ZZVecConfig> Sub<&'a Self> for ZZVec<C> {
    type Output = Self;

    // Coefficient wise subtractions without mod reduction.
    fn sub(self, other: &'a Self) -> Self {
        let mut res = self;
        res -= other;
        res
    }
}
impl<C: ZZVecConfig> Sub for ZZVec<C> {
    type Output = Self;

    // Coefficient wise subtractions with mod reduction.
    fn sub(self, other: Self) -> Self {
        self - &other
    }
}

impl<C: ZZVecConfig> SubAssign for ZZVec<C> {
    // Coefficient wise subtractions without mod reduction.
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}

impl<'a, C: ZZVecConfig> SubAssign<&'a Self> for ZZVec<C> {
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
impl<C: ZZVecConfig> Neg for ZZVec<C> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut res = self;
        res.coeffs.par_iter_mut().for_each(|x| *x = -*x);
        res
    }
}

impl<C: ZZVecConfig, T> Sum<T> for ZZVec<C>
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

impl<C: ZZVecConfig> Vector<C> for ZZVec<C> {
    type BaseField = ZZp<C::BaseConfig>;

    /// sample a uniformly random Vector over modulus
    /// if modulus is None, over the modulus of F
    fn random(mut rng: impl RngCore, modulus: Option<Self::BaseField>) -> Self {
        let coeff: Vec<Self::BaseField> = match modulus {
            Some(modulus) => (0..C::DIM)
                .map(|_| Self::BaseField::from(rng.next_u64() % modulus.0.into()))
                .collect(),
            None => (0..C::DIM)
                .map(|_| Self::BaseField::random(&mut rng))
                .collect(),
        };

        Self {
            coeffs: coeff.try_into().unwrap(),
        }
    }

    /// Sample a random binary Vector
    fn random_binary(mut rng: impl RngCore) -> Self {
        let coeff: Vec<Self::BaseField> =
            (0..C::DIM).map(|_| (rng.next_u64() % 2).into()).collect();
        Self {
            coeffs: coeff.try_into().unwrap(),
        }
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

    /// degree of the Vector
    fn dimension(&self) -> usize {
        C::DIM
    }

    /// Expose coefficients as a iter, starting from the constant term (x_0,...x_{d-1})
    fn coefficients(&self) -> Iter<'_, Self::BaseField> {
        self.coeffs.iter()
    }
    /// From coefficients; without checking the range
    fn from_coefficients_vec_unchecked(coeffs: Vec<Self::BaseField>) -> Self {
        Self { coeffs }
    }
}
