use std::ops::{Add, AddAssign, Mul};

use cpoly::{poly_inv_ntt, poly_ntt};

use crate::{MODULUS, MODULUS_OVER_TWO, N};

use super::poly::Polynomial;

#[derive(Debug, Clone, PartialEq, Copy)]
// Polynomials in NTT encoding
pub struct NTTPolynomial {
    pub(crate) coeffs: [u32; N as usize],
}

impl Default for NTTPolynomial {
    fn default() -> Self {
        Self {
            coeffs: [0u32; N as usize],
        }
    }
}

impl From<&Polynomial> for NTTPolynomial {
    // convert poly into its ntt form. Requires that coefficients are between 0 and 12289
    fn from(poly: &Polynomial) -> Self {
        let mut coeffs = [0u32; N as usize];
        coeffs
            .iter_mut()
            .zip(poly.coeffs.iter())
            .for_each(|(r, x)| *r = (x + MODULUS as i32) as u32 % MODULUS);

        unsafe {
            poly_ntt(coeffs.as_mut_ptr());
        }
        Self { coeffs }
    }
}

impl From<&NTTPolynomial> for Polynomial {
    fn from(poly: &NTTPolynomial) -> Self {
        let mut coeffs = poly.coeffs;
        unsafe {
            poly_inv_ntt(coeffs.as_mut_ptr());
        }
        let mut res = [0i32; N as usize];
        res.iter_mut().zip(coeffs.iter()).for_each(|(r, &f)| {
            *r = if f >= MODULUS_OVER_TWO as u32 {
                f as i32 - MODULUS as i32
            } else {
                f as i32
            }
        });

        Polynomial { coeffs: res }
    }
}

impl Add for NTTPolynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = Self::default();
        for (e, (f, g)) in res
            .coeffs
            .iter_mut()
            .zip(self.coeffs.iter().zip(other.coeffs.iter()))
        {
            *e = (f + g) % MODULUS
        }

        res
    }
}

impl AddAssign for NTTPolynomial {
    fn add_assign(&mut self, other: NTTPolynomial) {
        for (x, y) in self.coeffs.iter_mut().zip(other.coeffs) {
            *x = (*x + y) % MODULUS
        }
    }
}

impl Mul for NTTPolynomial {
    type Output = Self;

    // Coefficient-wise multiplication over the NTT domain.
    fn mul(self, other: Self) -> Self {
        let mut res = Self::default();
        for (e, (f, g)) in res
            .coeffs
            .iter_mut()
            .zip(self.coeffs.iter().zip(other.coeffs.iter()))
        {
            *e = (((*f as u64) * (*g as u64)) % MODULUS as u64) as u32
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_conversion() {
        let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
        for _ in 0..10 {
            let poly = Polynomial::rand_poly(&mut rng);
            let poly_ntt: NTTPolynomial = (&poly).into();
            let poly_rec: Polynomial = (&poly_ntt).into();

            assert_eq!(poly, poly_rec)
        }
    }

    #[test]
    fn test_arithmetic() {
        let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
        for _ in 0..10 {
            let a = Polynomial::rand_poly(&mut rng);
            let a_ntt: NTTPolynomial = (&a).into();
            let b = Polynomial::rand_poly(&mut rng);
            let b_ntt: NTTPolynomial = (&b).into();

            {
                // test correctness of ntt multiplications
                let c_ntt = a_ntt * b_ntt;
                let c: Polynomial = (&c_ntt).into();
                let c_rec = Polynomial::schoolbook(&a, &b);

                assert_eq!(c, c_rec);
            }
            {
                // test correctness of ntt additions
                let d_ntt = a_ntt + b_ntt;
                let d: Polynomial = (&d_ntt).into();
                let d_rec = a + b;

                assert_eq!(d, d_rec)
            }
        }
    }
}
