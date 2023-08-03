use std::iter::{Product, Sum};
use std::slice::Iter;
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

use rand::RngCore;

use crate::{Field, Polynomial, PolynomialOps};

impl<F: Field, const DEGREE: usize> Default for Polynomial<F, DEGREE> {
    fn default() -> Self {
        Self {
            coeffs: [F::default(); DEGREE],
        }
    }
}

impl<F: Field, const DEGREE: usize> Display for Polynomial<F, DEGREE> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Polynomial:");
        write!(f, "{}", self.coeffs[0])?;
        for (i, e) in self.coeffs.iter().skip(1).take(DEGREE - 2).enumerate() {
            write!(f, " + {}*x^{}", e, i + 1)?;
            if i % 8 == 6 {
                writeln!(f)?;
            }
        }
        writeln!(f, " + {}*x^{}", self.coeffs[DEGREE - 1], DEGREE - 1)
    }
}

// ===========================
// additions
// ===========================

/// todo! use macro. repeated pattern: poly add
impl<F: Field, const DEGREE: usize> Add for Polynomial<F, DEGREE> {
    type Output = Self;

    // Coefficient wise additions without mod reduction.
    fn add(self, other: Self) -> Self {
        let mut res = self;
        // TODO: parallel iterator
        res.coeffs
            .iter_mut()
            .zip(other.coeffs.iter())
            .for_each(|(x, y)| *x += *y);
        res
    }
}

impl<'a, F: Field, const DEGREE: usize> Add<&'a Self> for Polynomial<F, DEGREE> {
    type Output = Self;

    // Coefficient wise additions without mod reduction.
    fn add(self, other: &'a Self) -> Self {
        self + *other
    }
}

impl<F: Field, const DEGREE: usize> AddAssign for Polynomial<F, DEGREE> {
    // Coefficient wise additions without mod reduction.
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<'a, F: Field, const DEGREE: usize> AddAssign<&'a Self> for Polynomial<F, DEGREE> {
    // Coefficient wise additions without mod reduction.
    fn add_assign(&mut self, rhs: &'a Self) {
        *self += *rhs;
    }
}

// ===========================
// subtract
// ===========================
impl<F: Field, const DEGREE: usize> Sub for Polynomial<F, DEGREE> {
    type Output = Self;

    // Coefficient wise additions with mod reduction.
    fn sub(self, other: Self) -> Self {
        let mut res = self;
        // TODO: parallel iterator
        res.coeffs
            .iter_mut()
            .zip(other.coeffs.iter())
            .for_each(|(x, y)| *x -= *y);
        res
    }
}

impl<'a, F: Field, const DEGREE: usize> Sub<&'a Self> for Polynomial<F, DEGREE> {
    type Output = Self;

    // Coefficient wise subtractions without mod reduction.
    fn sub(self, other: &'a Self) -> Self {
        self - *other
    }
}

impl<F: Field, const DEGREE: usize> SubAssign for Polynomial<F, DEGREE> {
    // Coefficient wise subtractions without mod reduction.
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<'a, F: Field, const DEGREE: usize> SubAssign<&'a Self> for Polynomial<F, DEGREE> {
    // Coefficient wise subtractions without mod reduction.
    fn sub_assign(&mut self, rhs: &'a Self) {
        *self -= *rhs;
    }
}

// ===========================
// neg
// ===========================
impl<F: Field, const DEGREE: usize> Neg for Polynomial<F, DEGREE> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut res = self;
        // TODO: parallel iterator
        res.coeffs.iter_mut().for_each(|x| *x = -*x);
        res
    }
}

impl<F, T, const DEGREE: usize> Sum<T> for Polynomial<F, DEGREE>
where
    F: Field,
    T: core::borrow::Borrow<Self>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Self::zero(), |acc, item| acc + item.borrow())
    }
}

impl<F: Field, const DEGREE: usize> PolynomialOps<F> for Polynomial<F, DEGREE> {
    /// Zero element (additive identity)
    fn zero() -> Self {
        Self {
            coeffs: [F::ZERO; DEGREE],
        }
    }
    /// One element (multiplicative identity)
    fn one() -> Self {
        let mut res = Self::zero();
        res.coeffs[0] = F::ONE;
        res
    }
    /// sample a uniformly random polynomial over modulus
    /// if modulus is None, over the modulus of F
    fn random(mut rng: impl RngCore, modulus: Option<F>) -> Self {
        let coeff: Vec<F> = match modulus {
            Some(modulus) => (0..DEGREE)
                .map(|_| (rng.next_u64() % modulus.into()).into())
                .collect(),
            None => (0..DEGREE).map(|_| F::random(&mut rng)).collect(),
        };

        Self {
            coeffs: coeff.try_into().unwrap(),
        }
    }

    /// Sample a random binary polynomial
    fn random_binary(mut rng: impl RngCore) -> Self {
        let coeff: Vec<F> = (0..DEGREE).map(|_| (rng.next_u64() % 2).into()).collect();
        Self {
            coeffs: coeff.try_into().unwrap(),
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
