use crate::{ConfigZZpGoldilocks, ConfigZZpX, ZZpX};

/// Configuration for ZZ[x]/(x^512+1) mod 12289
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigZZpXGoldilocks256;

impl ConfigZZpX for ConfigZZpXGoldilocks256 {
    /// Config for the base field
    type BaseConfig = ConfigZZpGoldilocks;
    /// Number of coefficients in a poly
    const DIM: usize = 512;
}

/// Polynomial with coefficient from ZZ_q where q=12289.
pub type PolyGoldilock256 = ZZpX<ConfigZZpXGoldilocks256>;

#[test]
fn test_poly() {
    use crate::Goldilocks;
    let coeffs = (0..ConfigZZpXGoldilocks256::DIM)
        .map(|x| Goldilocks::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let poly = PolyGoldilock256 { coeffs };
    println!("poly {}", poly);
    println!("poly {}", poly.clone() + poly);
}
