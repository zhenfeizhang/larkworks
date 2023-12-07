use std::{
    iter::Product,
    ops::{Mul, MulAssign},
};

use crate::Polynomial;

/// A ring element is a polynomial that also allows for multiplication.
// Although in theory a ring can work on non-NTT friendly field,
// we restrict it to NTTField for convenience.
pub trait PolynomialRing<ConfigPoly, ConfigVec>:
    Polynomial<ConfigPoly>
    + Mul<Output = Self>
    + Product
    + MulAssign
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> Product<&'a Self>
    + for<'a> MulAssign<&'a Self>
{
}
