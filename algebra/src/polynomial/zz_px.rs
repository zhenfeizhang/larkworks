use core::slice::Iter;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use rand::RngCore;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::ConfigZZpX;
use crate::Field;
use crate::Polynomial;
use crate::ZZp;

/// ZZ_p[X]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZZpX<C: ConfigZZpX> {
    pub(crate) coeffs: Vec<ZZp<C::BaseConfig>>,
}

impl<C: ConfigZZpX> Default for ZZpX<C> {
    fn default() -> Self {
        Self {
            coeffs: vec![ZZp::<C::BaseConfig>::default(); C::DIM],
        }
    }
}

impl<C: ConfigZZpX> Display for ZZpX<C> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Polynomial:")?;
        write!(f, "{}", self.coeffs[0])?;
        for (i, e) in self.coeffs.iter().skip(1).take(C::DIM - 2).enumerate() {
            write!(f, " + {}*x^{}", e, i + 1)?;
            if i % 8 == 6 {
                writeln!(f)?;
            }
        }
        writeln!(f, " + {}*x^{}", self.coeffs[C::DIM - 1], C::DIM - 1)
    }
}

// ===========================
// additions
// ===========================
impl<'a, C: ConfigZZpX> Add<&'a Self> for ZZpX<C> {
    type Output = Self;

    // Coefficient wise additions without mod reduction.
    fn add(self, other: &'a Self) -> Self {
        let mut res = self;
        res += other;
        res
    }
}

impl<C: ConfigZZpX> Add for ZZpX<C> {
    type Output = Self;

    // Coefficient wise additions without mod reduction.
    fn add(self, other: Self) -> Self {
        self + &other
    }
}

impl<C: ConfigZZpX> AddAssign for ZZpX<C> {
    // Coefficient wise additions without mod reduction.
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl<'a, C: ConfigZZpX> AddAssign<&'a Self> for ZZpX<C> {
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
impl<'a, C: ConfigZZpX> Sub<&'a Self> for ZZpX<C> {
    type Output = Self;

    // Coefficient wise subtractions without mod reduction.
    fn sub(self, other: &'a Self) -> Self {
        let mut res = self;
        res -= other;
        res
    }
}
impl<C: ConfigZZpX> Sub for ZZpX<C> {
    type Output = Self;

    // Coefficient wise subtractions with mod reduction.
    fn sub(self, other: Self) -> Self {
        self - &other
    }
}

impl<C: ConfigZZpX> SubAssign for ZZpX<C> {
    // Coefficient wise subtractions without mod reduction.
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}

impl<'a, C: ConfigZZpX> SubAssign<&'a Self> for ZZpX<C> {
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
impl<C: ConfigZZpX> Neg for ZZpX<C> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut res = self;
        res.coeffs.par_iter_mut().for_each(|x| *x = -*x);
        res
    }
}

impl<C: ConfigZZpX, T> Sum<T> for ZZpX<C>
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

impl<C: ConfigZZpX> Polynomial<C> for ZZpX<C> {
    type BaseField = ZZp<C::BaseConfig>;

    /// Zero element (additive identity)
    fn zero() -> Self {
        Self {
            coeffs: vec![Self::BaseField::zero(); C::DIM],
        }
    }
    /// One element (multiplicative identity)
    fn one() -> Self {
        let mut res = Self::zero();
        res.coeffs[0] = Self::BaseField::one();
        res
    }

    /// sample a uniformly random polynomial over modulus
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

        Self { coeffs: coeff }
    }

    /// Sample a random binary polynomial
    fn random_binary(mut rng: impl RngCore) -> Self {
        let coeff: Vec<Self::BaseField> =
            (0..C::DIM).map(|_| (rng.next_u64() % 2).into()).collect();
        Self { coeffs: coeff }
    }

    /// A 32 bytes digest of the polynomial
    fn digest(&self) -> [u8; 32] {
        todo!()
    }

    /// Infinity norm of the polynomial
    fn infinity_norm(&self) -> u32 {
        todo!()
    }

    /// L2 norm of the polynomial
    fn l2_norm(&self) -> u32 {
        todo!()
    }

    /// degree of the polynomial
    fn degree(&self) -> usize {
        C::DIM - 1
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
