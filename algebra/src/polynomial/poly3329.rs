use crate::field::f3329::F3329;

use super::Polynomial;

const DEGREE: usize = 256;

/// Polynomial used in Kyber with coefficient from ZZ_q where q=3329.
pub type Poly3329 = Polynomial<F3329, DEGREE>;

#[test]
fn test_poly() {
    let coeffs = (0..DEGREE)
        .map(|x| F3329::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let poly = Poly3329 { coeffs };
    println!("poly {}", poly);

    println!("poly {}", poly + poly);

    // assert!(false)
}
