use crate::{ConfigZZVec, ConfigZZp3168257, ZZVec};

/// Configuration for ZZ^n mod 3168257
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigZZVec3168257_512;

impl ConfigZZVec for ConfigZZVec3168257_512 {
    /// Config for the base field
    type BaseConfig = ConfigZZp3168257;
    /// Number of coefficients in a vector
    const MAX_DIM: usize = 512;
}

/// Vector with coefficient from ZZ^n mod q=3168257.
pub type Vec3168257_512 = ZZVec<ConfigZZVec3168257_512>;

#[test]
fn test_vec() {
    use crate::F3168257;
    let coeffs = (0..ConfigZZVec3168257_512::MAX_DIM)
        .map(|x| F3168257::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let vec = Vec3168257_512 { coeffs };
    println!("vec {}", vec);

    println!("vec {}", vec.clone() + vec);

    // assert!(false)
}
