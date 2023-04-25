use super::lift;
use super::normalize;
use crate::impl_signed_poly;
use crate::impl_signed_poly_functions;
use crate::param::MODULUS;
use crate::param::MODULUS_OVER_TWO;
use crate::param::N;
use crate::param::SAMPLE_THRESHOLD;
use crate::ter_poly::TerPolyCoeffEncoding;
use crate::Polynomial;

use rand::Rng;
use sha2::Digest;
use sha2::Sha256;
use std::ops::Mul;

use crate::poly_ntt::NTTPoly;

#[derive(Debug, Clone, Copy)]
// polynomials in canonical encoding
pub struct Poly {
    pub(crate) coeffs: [i32; N],
}

impl_signed_poly!(Poly, MODULUS, N);
impl_signed_poly_functions!(
    Poly,
    NTTPoly,
    MODULUS,
    N,
    MODULUS_OVER_TWO,
    SAMPLE_THRESHOLD
);

impl Mul for Poly {
    type Output = Self;

    // Ring multiplication
    fn mul(self, other: Self) -> Self {
        (&(NTTPoly::from(&self) * NTTPoly::from(&other))).into()
    }
}

impl Poly {
    // multiply a ternary with a binary poly
    pub fn ter_mul_bin(ter: &TerPolyCoeffEncoding, bin: &Self) -> Self {
        Self::from(ter) * *bin

        // TODO: use the following AVX code
        // #[cfg(debug_assertions)]
        // assert!(bin.is_binary());

        // let mut res = Self::default();
        // let mut tmp = [0i8; N];
        // let mut buf = [0u8; 2 * N];
        // let bin: Vec<i8> = bin.coeffs.iter().map(|&x| x as i8).collect();
        // let ter: Vec<u8> = ter.indices.iter().map(|&x| x as u8).collect();

        // unsafe {
        //     ternary_mul(
        //         tmp.as_mut_ptr(),
        //         buf.as_mut_ptr(),
        //         bin.as_ptr(),
        //         ter.as_ptr(),
        //     );
        // }
        // for (e, f) in res.coeffs.iter_mut().zip(tmp.iter()) {
        //     *e = *f as i32
        // }
        // res
    }
}

#[cfg(test)]
mod test {
    use super::Poly;
    use crate::impl_poly_tests;
    use crate::{param::MODULUS, poly::Polynomial};
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    impl_poly_tests!(Poly, MODULUS);
}
