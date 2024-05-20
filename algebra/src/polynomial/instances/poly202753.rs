use crate::{ConfigZZp202753, ConfigZZpX, ZZpX};

/// Configuration for ZZ[x]/(x^512+1) mod 202753
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigZZpX202753_512;

impl ConfigZZpX for ConfigZZpX202753_512 {
    /// Config for the base field
    type BaseConfig = ConfigZZp202753;
    /// Number of coefficients in a poly
    const DIM: usize = 512;
}

/// Polynomial with coefficient from ZZ_q where q=202753.
pub type Poly202753_512 = ZZpX<ConfigZZpX202753_512>;

#[test]
fn test_poly() {
    use crate::F202753;
    let coeffs = (0..ConfigZZpX202753_512::DIM)
        .map(|x| F202753::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let poly = Poly202753_512 { coeffs };
    println!("poly {}", poly);
    println!("poly {}", poly.clone() + poly);
}
