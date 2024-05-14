use crate::{ConfigZZVec, ConfigZZpGoldilocks, ZZVec};

/// Configuration for ZZ^n mod 12289
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigZZVecGoldilocks256;

impl ConfigZZVec for ConfigZZVecGoldilocks256 {
    /// Config for the base field
    type BaseConfig = ConfigZZpGoldilocks;
    /// Number of coefficients in a vector
    const MAX_DIM: usize = 256;
}

/// Vector with coefficient from ZZ^n mod q=2^64-2^32+1.
pub type VecGoldilocks256 = ZZVec<ConfigZZVecGoldilocks256>;

#[test]
fn test_vec() {
    use crate::Goldilocks;
    let coeffs = (0..ConfigZZVecGoldilocks256::MAX_DIM)
        .map(|x| Goldilocks::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let vec = VecGoldilocks256 { coeffs };
    println!("vec {}", vec);

    println!("vec {}", vec.clone() + vec);

    // assert!(false)
}
