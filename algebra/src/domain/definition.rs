use crate::{Field, Polynomial, Vector};

/// NTT domain
pub trait NTTDomain<ConfigPoly, ConfigVec>: Vector<ConfigVec> {
    /// 1/DIM mod q
    const ONE_OVER_N: <Self::BaseField as Field>::PrimitiveType;

    /// Polynomial
    type Polynomial: Polynomial<ConfigPoly>;

    /// (Inverse) NTT Table
    type Table;

    /// Get the forward table
    fn table() -> Self::Table;

    /// Get the reverse table
    fn inv_table() -> Self::Table;

    /// convert polynomial to vector
    fn forward_ntt(poly: &Self::Polynomial) -> Self;

    /// convert the vector to polynomial
    fn reverse_ntt(&self) -> Self::Polynomial;
}
