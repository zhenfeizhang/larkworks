use crate::{ConfigZZVec, ConfigZZp3329, ZZVec};

/// Configuration for ZZ^n mod  3329
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigZZVec3329_256;

impl ConfigZZVec for ConfigZZVec3329_256 {
    /// Config for the base field
    type BaseConfig = ConfigZZp3329;
    /// Number of coefficients in a Vector
    const DIM: usize = 256;
}

/// Vector with coefficient from ZZ^n mod q=3329.
pub type Vec3329_256 = ZZVec<ConfigZZVec3329_256>;

#[test]
fn test_vec() {
    use crate::F3329;
    let coeffs = (0..ConfigZZVec3329_256::DIM)
        .map(|x| F3329::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let vec = Vec3329_256 { coeffs };
    println!("vec {}", vec);

    println!("vec {}", vec.clone() + vec);

    // assert!(false)
}
