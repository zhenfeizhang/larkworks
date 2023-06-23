//! Lattice interfaces

use crate::matrix::Matrix;

pub trait Lattice {
    type Field;
    type LatticeVector;
    type Basis: Matrix;

    /// Compute the determinant
    fn determinant(&self) -> Self::Field;

    /// dimension of the lattice
    fn dimension(&self) -> usize;

    /// Build a lattice from a basis
    fn from_bases(basis: &Self::Basis) ->Self;

    /// extract a bases for the lattice
    fn bases(&self) -> Self::Basis;

    /// Gaussian expected length
    fn gaussian_length() -> Self::Field;

    /// If a vector is in the lattice
    fn contains(&self, vec: &Self::LatticeVector) -> bool;
}

pub trait LatticeReduction: Lattice {
    type GramSchmidtBasis;

    /// GramSchmidt bases
    fn gram_schmidt_bases(&self) -> Self::GramSchmidtBasis;

    /// Babai nearest plane algorithm
    fn babai(&self, vec: &Self::LatticeVector) -> Self::LatticeVector;

    /// LLL lattice reduction
    /// (prob not going to implement for phase 1)
    fn lll(base: &Self::Basis) -> Self::Basis;

    /// BKZ lattice reduction
    /// (prob not going to implement for phase 1)
    fn bkz(base: &Self::Basis) -> Self::Basis;
}

pub trait IdealLattice: Lattice {
    /// Underlying ring for the ideal lattice
    type Ring;

    /// Build the lattice from the ring
    fn from_ring_element(elem: &Self::Ring) ->Self;
}

pub trait ModularLattice: Lattice {
    /// Underlying ring for the ideal lattice
    type Ring;

    /// Get the rank of the lattice
    fn rank(&self) -> usize;

    /// Build the lattice from the ring
    fn from_ring_elements(elem: &[Self::Ring]) ->Self;
}