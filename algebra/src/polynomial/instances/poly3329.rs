use crate::{ConfigZZp3329, ConfigZZpX, ZZpX};

/// Configuration for ZZ[x]/(x^256+1) mod 3329
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigZZpX3329_256;

impl ConfigZZpX for ConfigZZpX3329_256 {
    /// Config for the base field
    type BaseConfig = ConfigZZp3329;
    /// Number of coefficients in a poly
    const DIM: usize = 256;
}

/// Polynomial used in Kyber with coefficient from ZZ_q where q=3329.
pub type Poly3329_256 = ZZpX<ConfigZZpX3329_256>;

#[test]
fn test_poly() {
    use crate::F3329;
    let coeffs = (0..ConfigZZpX3329_256::DIM)
        .map(|x| F3329::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let poly = Poly3329_256 { coeffs };
    println!("poly {}", poly);
    println!("poly {}", poly.clone() + poly);
}
