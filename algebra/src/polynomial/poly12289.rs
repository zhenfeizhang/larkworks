use crate::field::f12289::F12289;

use super::Polynomial;

const DEGREE: usize = 512;

/// Polynomial used in Kyber with coefficient from ZZ_q where q=3329.
pub type Poly12289 = Polynomial<F12289, DEGREE>;

// todo! implement polynomial trait for Poly12289

#[test]
fn test_poly() {
    let coeffs = (0..DEGREE)
        .map(|x| F12289::from(x as u64))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let poly = Poly12289 { coeffs };
    println!("poly {}", poly);

    println!("poly {}", poly + poly);

    // assert!(false)
}
