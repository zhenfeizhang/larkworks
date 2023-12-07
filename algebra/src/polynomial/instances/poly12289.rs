use crate::{ZZpConfig12289, ZZpX, ZZpXConfig};

/// Configuration for ZZ[x]/(x^512+1) mod 12289
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ZZpXConfig12289_512;

impl ZZpXConfig for ZZpXConfig12289_512 {
    /// Config for the base field
    type BaseConfig = ZZpConfig12289;
    /// Number of coefficients in a poly
    const DIM: usize = 512;
}

/// Polynomial used in Kyber with coefficient from ZZ_q where q=3329.
pub type Poly12289_512 = ZZpX<ZZpXConfig12289_512>;

#[test]
fn test_poly() {
    use crate::F12289;
    let coeffs = (0..ZZpXConfig12289_512::DIM)
        .map(|x| F12289::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let poly = Poly12289_512 { coeffs };
    println!("poly {}", poly);

    println!("poly {}", poly.clone() + poly);

    // assert!(false)
}
