use crate::{ConfigZZVec, ConfigZZp202753, ZZVec};

/// Configuration for ZZ^n mod 202753
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigZZVec202753_512;

impl ConfigZZVec for ConfigZZVec202753_512 {
    /// Config for the base field
    type BaseConfig = ConfigZZp202753;
    /// Number of coefficients in a vector
    const MAX_DIM: usize = 512;
}

/// Vector with coefficient from ZZ^n mod q=202753.
pub type Vec202753_512 = ZZVec<ConfigZZVec202753_512>;

#[test]
fn test_vec() {
    use crate::F202753;
    let coeffs = (0..ConfigZZVec202753_512::MAX_DIM)
        .map(|x| F202753::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let vec = Vec202753_512 { coeffs };
    println!("vec {}", vec);

    println!("vec {}", vec.clone() + vec);

    // assert!(false)
}
