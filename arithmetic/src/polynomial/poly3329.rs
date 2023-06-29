use core::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use ff::Field as FfField;
use rand::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::{field::f3329::F3329, field_common, Field};

use super::Polynomial;

const DEGREE: usize = 256;

pub type Poly3329 = Polynomial<F3329, DEGREE>;

#[test]
fn test_poly() {
    let coeffs = (0..DEGREE)
        .map(|x| F3329::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let poly = Poly3329 { coeffs };
    println!("poly {}", poly);

    println!("poly {}", poly + poly);

    assert!(false)
}
