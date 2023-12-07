mod definitions;
mod instances;
mod zz_vec;

pub use definitions::{ConfigZZVec, Vector};
pub use instances::{ConfigZZVec12289_512, ConfigZZVec3329_256, Vec12289_512, Vec3329_256};
pub use zz_vec::ZZVec;

// /// Associating the vector with a lattice
// pub trait LatticeVector<F: Field>: Vector<F> {
//     /// parameter for the lattice
//     type LatticeParam;
// }

// /// Associating the vector with an NTT domain and a ring
// pub trait NTTVector<F: NTTField>:
//     Vector<F> + From<Self::RingElement> + Into<Self::RingElement>
// {
//     /// type of ring elements used in the NTT
//     type RingElement: RingElement<F>;
// }
