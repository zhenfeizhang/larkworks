use super::ntt_param::{INV_NTT_TABLE, NTT_TABLE};
use crate::param::{MODULUS, N, ONE_OVER_N};
use crate::poly::Poly;
use crate::{impl_ntt_poly, impl_poly};

#[derive(Debug, Clone, Copy)]
// polynomials in NTT encoding
pub struct NTTPoly {
    pub(crate) coeffs: [i32; N as usize],
}

impl_poly!(NTTPoly, MODULUS, N);
impl_ntt_poly!(Poly, NTTPoly, MODULUS, N, ONE_OVER_N);

#[cfg(test)]
mod test {
    use super::*;
    use super::{inv_ntt, ntt};
    use crate::impl_ntt_poly_tests;
    use crate::lift;
    use crate::param::MODULUS;
    use crate::poly::Poly;
    use crate::Polynomial;
    use ark_std::{end_timer, start_timer};
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    impl_ntt_poly_tests!(Poly, NTTPoly, MODULUS);
}
