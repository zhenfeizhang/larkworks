use std::iter::Sum;
use std::ops::{Add, AddAssign, Sub, SubAssign};

// use crate::additive_ops;
use crate::{field::f202753::F202753, PolynomialOps};

use super::Polynomial;

const DEGREE: usize = 1024;

/// Polynomial used in Kyber with coefficient from ZZ_q where q=3329.
pub type Poly202753 = Polynomial<F202753, DEGREE>;

// impl Add for Poly202753 {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self::Output {
//         todo!()
//     }

// }
// impl Sub for Poly202753 {}
// additive_ops!(Poly202753);

// impl PolynomialOps<F202753> for Poly202753 {}

#[test]
fn test_poly() {
    let coeffs = (0..DEGREE)
        .map(|x| F202753::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let poly = Poly202753 { coeffs };
    println!("poly {}", poly);

    println!("poly {}", poly + poly);

    // assert!(false)
}
