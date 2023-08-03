use std::iter::{Product, Sum};
use std::slice::Iter;
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

use rand::RngCore;

use crate::{field::NTTField, Polynomial, PolynomialOps, PolynomialRing, PolynomialRingOps};

impl<F: NTTField, const DEGREE: usize> Default for PolynomialRing<F, DEGREE> {
    fn default() -> Self {
        Self {
            poly: Polynomial::default(),
        }
    }
}

impl<F: NTTField, const DEGREE: usize> Display for PolynomialRing<F, DEGREE> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Polynomial ring:");
        write!(f, "{}", self.poly.coeffs[0])?;
        for (i, e) in self.poly.coeffs.iter().skip(1).take(DEGREE - 2).enumerate() {
            write!(f, " + {}*x^{}", e, i + 1)?;
            if i % 8 == 6 {
                writeln!(f)?;
            }
        }
        writeln!(f, " + {}*x^{}", self.poly.coeffs[DEGREE - 1], DEGREE - 1)
    }
}

// ===========================
// additions
// ===========================

/// todo! use macro. repeated pattern: poly add
impl<F: NTTField, const DEGREE: usize> Add for PolynomialRing<F, DEGREE> {
    type Output = Self;

    // Coefficient wise additions with mod reduction.
    fn add(self, other: Self) -> Self {
        let mut res = self;
        // TODO: parallel iterator
        res.poly
            .coeffs
            .iter_mut()
            .zip(other.poly.coeffs.iter())
            .for_each(|(x, y)| *x += *y);
        res
    }
}

impl<'a, F: NTTField, const DEGREE: usize> Add<&'a Self> for PolynomialRing<F, DEGREE> {
    type Output = Self;

    // Coefficient wise additions without mod reduction.
    fn add(self, other: &'a Self) -> Self {
        self + *other
    }
}

impl<F: NTTField, const DEGREE: usize> AddAssign for PolynomialRing<F, DEGREE> {
    // Coefficient wise additions without mod reduction.
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<'a, F: NTTField, const DEGREE: usize> AddAssign<&'a Self> for PolynomialRing<F, DEGREE> {
    // Coefficient wise additions without mod reduction.
    fn add_assign(&mut self, rhs: &'a Self) {
        *self += *rhs;
    }
}

// ===========================
// subtract
// ===========================
impl<F: NTTField, const DEGREE: usize> Sub for PolynomialRing<F, DEGREE> {
    type Output = Self;

    // Coefficient wise additions with mod reduction.
    fn sub(self, other: Self) -> Self {
        let mut res = self;
        // TODO: parallel iterator
        res.poly
            .coeffs
            .iter_mut()
            .zip(other.poly.coeffs.iter())
            .for_each(|(x, y)| *x -= *y);
        res
    }
}

impl<'a, F: NTTField, const DEGREE: usize> Sub<&'a Self> for PolynomialRing<F, DEGREE> {
    type Output = Self;

    // Coefficient wise subtractions without mod reduction.
    fn sub(self, other: &'a Self) -> Self {
        self - *other
    }
}

impl<F: NTTField, const DEGREE: usize> SubAssign for PolynomialRing<F, DEGREE> {
    // Coefficient wise subtractions without mod reduction.
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<'a, F: NTTField, const DEGREE: usize> SubAssign<&'a Self> for PolynomialRing<F, DEGREE> {
    // Coefficient wise subtractions without mod reduction.
    fn sub_assign(&mut self, rhs: &'a Self) {
        *self -= *rhs;
    }
}

// ===========================
// neg
// ===========================
impl<F: NTTField, const DEGREE: usize> Neg for PolynomialRing<F, DEGREE> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut res = self;
        // TODO: parallel iterator
        res.poly.coeffs.iter_mut().for_each(|x| *x = -*x);
        res
    }
}

impl<F, T, const DEGREE: usize> Sum<T> for PolynomialRing<F, DEGREE>
where
    F: NTTField,
    T: core::borrow::Borrow<Self>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Self::zero(), |acc, item| acc + item.borrow())
    }
}

impl<F: NTTField, const DEGREE: usize> PolynomialOps<F> for PolynomialRing<F, DEGREE> {
    /// Zero element (additive identity)
    fn zero() -> Self {
        Self {
            poly: Polynomial::zero(),
        }
    }

    /// One element (multiplicative identity)
    fn one() -> Self {
        Self {
            poly: Polynomial::one(),
        }
    }
    /// sample a uniformly random polynomial over modulus
    /// if modulus is None, over the modulus of F
    fn random(rng: impl RngCore, modulus: Option<F>) -> Self {
        Self {
            poly: Polynomial::random(rng, modulus),
        }
    }

    /// Sample a random binary polynomial
    fn random_binary(rng: impl RngCore) -> Self {
        Self {
            poly: Polynomial::random_binary(rng)
        }
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
        todo!()
    }

    /// Expose coefficients as a iter, starting from the constant term (x_0,...x_{d-1})
    fn coefficients(&self) -> Iter<'_, F> {
        todo!()
    }
    /// From coefficients; without checking the range
    fn from_coefficients_unchecked(coeff: &[F]) -> Self {
        todo!()
    }

    /// From coefficients; without checking the range
    fn from_coefficients_vec_unchecked(coeff: Vec<F>) -> Self {
        todo!()
    }
}

// impl<F: NTTField, const DEGREE: usize> PolynomialRingOps<F> for PolynomialRing<F, DEGREE> {}
