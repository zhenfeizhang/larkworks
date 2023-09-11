use rand::RngCore;

use std::cmp::max;
use std::iter::{Product, Sum};
use std::slice::Iter;
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{Field, VectorOps};

#[derive(Debug, Clone, Eq, Default, PartialEq)]
pub struct Vector<F: Field> {
    pub(crate) coeffs: Vec<F>,
}

impl<F: Field> Display for Vector<F> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Vector:");
        write!(f, "{:?}", self.coeffs)
    }
}

// ===========================
// additions
// ===========================

/// todo! use macro. repeated pattern
impl<F: Field> Add for Vector<F> {
    type Output = Self;

    // Coefficient wise additions with mod reduction.
    fn add(self, other: Self) -> Self {
        let len = max(self.coeffs.len(), other.coeffs.len());

        let mut res: Vec<F> = self
            .coeffs
            .iter()
            .cloned()
            .chain(std::iter::repeat(Default::default()))
            .take(len)
            .collect();
        // TODO: parallel iterator
        res.iter_mut()
            .zip(
                other
                    .coeffs
                    .iter()
                    .cloned()
                    .chain(std::iter::repeat(Default::default()))
                    .take(len),
            )
            .for_each(|(x, y)| *x += y);
        Self { coeffs: res }
    }
}

impl<'a, F: Field> Add<&'a Self> for Vector<F> {
    type Output = Self;

    // Coefficient wise additions without mod reduction.
    fn add(self, other: &'a Self) -> Self {
        self + *other
    }
}

impl<F: Field> AddAssign for Vector<F> {
    // Coefficient wise additions without mod reduction.
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<'a, F: Field> AddAssign<&'a Self> for Vector<F> {
    // Coefficient wise additions without mod reduction.
    fn add_assign(&mut self, rhs: &'a Self) {
        *self += *rhs;
    }
}
impl<F, T> Sum<T> for Vector<F>
where
    F: Field,
    T: core::borrow::Borrow<Self>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Self::default(), |acc, item| acc + item.borrow())
    }
}

// ===========================
// subtract
// ===========================

/// todo! use macro. repeated pattern
impl<F: Field> Sub for Vector<F> {
    type Output = Self;

    // Coefficient wise subitions with mod reduction.
    fn sub(self, other: Self) -> Self {
        let len = max(self.coeffs.len(), other.coeffs.len());

        let mut res = self
            .coeffs
            .iter()
            .chain(std::iter::repeat(&Default::default()))
            .take(len);

        // TODO: parallel iterator
        res.zip(
            other
                .coeffs
                .iter()
                .chain(std::iter::repeat(&Default::default()))
                .take(len),
        )
        .for_each(|(x, y)| *x -= *y);
        Self {
            coeffs: res.cloned().collect(),
        }
    }
}

impl<'a, F: Field> Sub<&'a Self> for Vector<F> {
    type Output = Self;

    // Coefficient wise subitions without mod reduction.
    fn sub(self, other: &'a Self) -> Self {
        self - *other
    }
}

impl<F: Field> SubAssign for Vector<F> {
    // Coefficient wise subitions without mod reduction.
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<'a, F: Field> SubAssign<&'a Self> for Vector<F> {
    // Coefficient wise subitions without mod reduction.
    fn sub_assign(&mut self, rhs: &'a Self) {
        *self -= *rhs;
    }
}

// ===========================
// neg
// ===========================
impl<F: Field> Neg for Vector<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut res = self;
        // TODO: parallel iterator
        res.coeffs.iter_mut().for_each(|x| *x = -*x);
        res
    }
}

// ===========================
// mul
// ===========================

/// todo! use macro. repeated pattern
impl<F: Field> Mul for Vector<F> {
    type Output = Self;

    // Coefficient wise mulitions with mod reduction.
    fn mul(self, other: Self) -> Self {
        let len = max(self.coeffs.len(), other.coeffs.len());

        let mut res = self;
        // TODO: parallel iterator
        res.coeffs
            .iter_mut()
            .zip(other.coeffs.iter())
            .for_each(|(x, y)| *x *= *y);
        res.coeffs = res
            .coeffs
            .iter()
            .chain(std::iter::repeat(&Default::default()))
            .take(len)
            .cloned()
            .collect();
        res
    }
}

impl<'a, F: Field> Mul<&'a Self> for Vector<F> {
    type Output = Self;

    // Coefficient wise mulitions without mod reduction.
    fn mul(self, other: &'a Self) -> Self {
        self * *other
    }
}

impl<F: Field> MulAssign for Vector<F> {
    // Coefficient wise mulitions without mod reduction.
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<'a, F: Field> MulAssign<&'a Self> for Vector<F> {
    // Coefficient wise mulitions without mod reduction.
    fn mul_assign(&mut self, rhs: &'a Self) {
        *self *= *rhs;
    }
}
impl<F, T> Product<T> for Vector<F>
where
    F: Field,
    T: core::borrow::Borrow<Self>,
{
    fn product<I: Iterator<Item = T>>(iter: I) -> Self {
        todo!()
        // iter.fold(Self::one(), |acc, item| acc * item.borrow())
    }
}

// // ===========================
// // div
// // ===========================

// /// todo! use macro. repeated pattern
// impl<F: Field> Div for Vector<F> {
//     type Output = Self;

//     // Coefficient wise divitions with mod reduction.
//     fn div(self, other: Self) -> Self {
//         assert!(
//             self.coeffs.len() <= other.coeffs.len(),
//             "cannot divide by 0"
//         );
//         other
//             .coeffs
//             .iter()
//             .for_each(|x| assert_ne!(*x, F::ZERO, "cannot divide by 0"));

//         let mut res = self
//             .coeffs
//             .iter()
//             .chain(std::iter::repeat(&Default::default()))
//             .take(other.coeffs.len());

//         // TODO: parallel iterator
//         res
//             .zip(
//                 other
//                     .coeffs
//                     .iter()
//                     ,
//             )
//             .for_each(|(x, y)| *x /= *y);
//         Self{ coeffs:res}
//     }
// }

// impl<'a, F: Field> Div<&'a Self> for Vector<F> {
//     type Output = Self;

//     // Coefficient wise divitions without mod reduction.
//     fn div(self, other: &'a Self) -> Self {
//         self + *other
//     }
// }

// impl<F: Field> DivAssign for Vector<F> {
//     // Coefficient wise divitions without mod reduction.
//     fn div_assign(&mut self, rhs: Self) {
//         *self = *self + rhs;
//     }
// }

// impl<'a, F: Field> DivAssign<&'a Self> for Vector<F> {
//     // Coefficient wise divitions without mod reduction.
//     fn div_assign(&mut self, rhs: &'a Self) {
//         *self += *rhs;
//     }
// }

impl<F: Field> VectorOps<F> for Vector<F> {
    /// sample a uniformly random vector over modulus
    /// if modulus is None, over the modulus of F
    fn random(rng: impl RngCore, modulus: Option<F>, dim: usize) -> Self {
        todo!()
    }

    /// Return the dimension of the vector
    fn dimension(&self) -> usize {
        todo!()
    }

    /// Infinity norm of the vector
    fn infinity_norm(&self) -> u32 {
        todo!()
    }

    /// L2 norm of the vector
    fn l2_norm(&self) -> u32 {
        todo!()
    }

    /// Expose coefficients as a iter
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
