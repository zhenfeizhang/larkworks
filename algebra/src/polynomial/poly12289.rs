use crate::field::f12289::F12289;

use super::DensePolynomial;

const DEGREE: usize = 256;

/// Polynomial used in Kyber with coefficient from ZZ_q where q=3329.
pub type P12289 = DensePolynomial<F12289, DEGREE>;

#[test]
fn test_poly() {
    let coeffs = (0..DEGREE)
        .map(|x| F12289::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let poly = P12289 { coeffs };
    println!("poly {}", poly);
    println!("poly {}", poly + poly);
}
