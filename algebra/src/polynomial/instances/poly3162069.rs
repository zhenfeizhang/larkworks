use crate::{ConfigZZp3168257, ConfigZZpX, ZZpX};

/// Configuration for ZZ[x]/(x^512+1) mod 3168257
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigZZpX3168257_512;

impl ConfigZZpX for ConfigZZpX3168257_512 {
    /// Config for the base field
    type BaseConfig = ConfigZZp3168257;
    /// Number of coefficients in a poly
    const DIM: usize = 512;
}

/// Polynomial with coefficient from ZZ_q where q=3168257.
pub type Poly3168257_512 = ZZpX<ConfigZZpX3168257_512>;

#[test]
fn test_poly() {
    use crate::F3168257;
    let coeffs = (0..ConfigZZpX3168257_512::DIM)
        .map(|x| F3168257::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let poly = Poly3168257_512 { coeffs };
    println!("poly {}", poly);
    println!("poly {}", poly.clone() + poly);
}
