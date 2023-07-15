use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, AddAssign},
};

use crate::{Field, Polynomial};

impl<F: Field, const DEGREE: usize> Default for Polynomial<F, DEGREE> {
    fn default() -> Self {
        Self {
            coeffs: [F::default(); DEGREE],
        }
    }
}

impl<F: Field, const DEGREE: usize> Display for Polynomial<F, DEGREE> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.coeffs[0])?;
        for (i, e) in self.coeffs.iter().skip(1).take(DEGREE - 2).enumerate() {
            write!(f, " + {}*x^{}", e, i + 1)?;
            if i % 8 == 6 {
                writeln!(f, "")?;
            }
        }
        writeln!(f, " + {}*x^{}", self.coeffs[DEGREE - 1], DEGREE - 1)
    }
}

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
        *self += *self + *rhs;
    }
}

// impl<F: Field, const DEGREE: usize> PolynomialOps<F> for Polynomial<F, DEGREE> {}
