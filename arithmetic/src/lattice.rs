//! Lattice interfaces

use crate::{matrix::Matrix};

pub trait Lattice {
    type Field;
    type LatticeVector;
    type Bases: Matrix;
    type GramSchmidtBasis;

    /// extract a bases for the lattice
    fn bases(&self) -> Self::Bases;

    /// Gaussian expected length
    fn gaussian_length() -> Self::Field;

    /// GramSchmidt bases
    fn gram_schmidt_bases(&self) -> Self::GramSchmidtBasis;

    /// If a vector is in the lattice
    fn contains(&self, vec: &Self::LatticeVector) -> bool;

    /// LLL lattice reduction
    /// (prob not going to implement for phase 1)
    fn lll(base: &Self::Bases) -> Self::Bases;

    /// BKZ lattice reduction
    /// (prob not going to implement for phase 1)
    fn bkz(base: &Self::Bases) -> Self::Bases;
}
