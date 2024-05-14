use crate::{ConfigZZVec, ConfigZZp12289, ZZVec};

/// Configuration for ZZ^n mod 12289
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigZZVec12289_512;

impl ConfigZZVec for ConfigZZVec12289_512 {
    /// Config for the base field
    type BaseConfig = ConfigZZp12289;
    /// Number of coefficients in a vector
    const MAX_DIM: usize = 256;
}

/// Vector with coefficient from ZZ^n mod q=12289.
pub type Vec12289_512 = ZZVec<ConfigZZVec12289_512>;

#[test]
fn test_vec() {
    use crate::F12289;
    let coeffs = (0..ConfigZZVec12289_512::MAX_DIM)
        .map(|x| F12289::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let vec = Vec12289_512 { coeffs };
    println!("vec {}", vec);

    println!("vec {}", vec.clone() + vec);

    // assert!(false)
}
