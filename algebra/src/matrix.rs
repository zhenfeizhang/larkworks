//! Matrix operations.

use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
    slice::Iter,
};

use crate::{field::NTTField, PolynomialRing};

pub trait Matrix:
    Sized
    + Eq
    + Copy
    + Clone
    + Default
    + Debug
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + AddAssign
    + SubAssign
    + Sum
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> Sum<&'a Self>
{
    /// A matrix's element can be either a field element F
    /// or a ring element R.
    type Element;

    /// Number of rows and columns.
    fn dim(&self) -> (usize, usize);

    /// Number of rows
    fn number_of_rows(&self) -> usize;

    /// Number of columns
    fn number_of_columns(&self) -> usize;

    /// Expose rows as vectors
    fn row_vectors(&self) -> Iter<'_, Vec<Self::Element>>;

    /// Build a matrix from row vectors
    fn from_row_vectors(rows: Vec<Vec<Self::Element>>) -> Self;

    /// Expose columns as vectors
    fn column_vectors(&self) -> Iter<'_, Vec<Self::Element>>;

    /// Build a matrix from column vectors
    fn from_column_vectors(rows: Vec<Vec<Self::Element>>) -> Self;

    /// Transpose a matrix
    fn transpose(&self) -> Self;

    /// Compute the determinant over F or R
    fn determinant_modulo_p(&self) -> Self::Element;

    /// multiply a matrix by left
    fn mul_by_left(&self, lhs: &Self) -> Self;

    /// multiply a matrix by left
    fn mul_by_right(&self, rhs: &Self) -> Self;
}

/// An anti-circulant matrix can be obtained from a ring element
pub trait AnitCirculantMatrix<R>: Matrix + From<R> + Into<R>
where
    R: PolynomialRing<Self::Element>,
    Self::Element: NTTField,
{
}
