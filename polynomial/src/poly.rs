use rand::Rng;
use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Mul},
};

use sha2::Digest;
use sha2::Sha256;

use crate::{
    TerPolyCoeffEncoding, MODULUS, MODULUS_OVER_TWO, SAMPLE_THRESHOLD, N,
    
};

use super::{poly_ntt::NTTPolynomial, lift};

#[derive(Debug, Clone, Copy)]
// Polynomials in canonical encoding
pub struct Polynomial {
    pub(crate) coeffs: [i32; N as usize],
}

impl PartialEq for Polynomial {
    fn eq(&self, rhs: &Polynomial) -> bool {
        self.coeffs
            .iter()
            .zip(rhs.coeffs.iter())
            .map(|(&x, &y)| lift(x, MODULUS) == lift(y, MODULUS))
            .fold(true, |acc, x| acc & x)
    }
}

impl Default for Polynomial {
    fn default() -> Self {
        Self {
            coeffs: [0i32; N as usize],
        }
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:#16x} {:#16x} {:#16x} {:#16x}",
            self.coeffs[0], self.coeffs[1], self.coeffs[0], self.coeffs[1]
        )
    }
}

impl Add for Polynomial {
    type Output = Self;

    // Coefficient wise additions without mod reduction.
    fn add(self, other: Self) -> Self {
        let mut res = self;
        res += other;
        res
    }
}

impl AddAssign for Polynomial {
    // Coefficient wise additions with mod reduction.
    fn add_assign(&mut self, other: Self) {
        self.coeffs
            .iter_mut()
            .zip(other.coeffs)
            .for_each(|(x, y)| *x = (*x + y) % MODULUS as i32)
    }
}

impl Mul for Polynomial {
    type Output = Self;

    // Ring multiplication
    fn mul(self, other: Self) -> Self {
        (&(NTTPolynomial::from(&self) * NTTPolynomial::from(&other))).into()
    }
}

impl Polynomial {
    // school book multiplication
    // slow. only used for correctness checking
    #[cfg(test)]
    pub(crate) fn schoolbook(a: &Self, b: &Self) -> Self {
        let mut buf = [0i32; N as usize * 2];
        let mut c = [0; N as usize];
        for i in 0..N as usize {
            for j in 0..N as usize {
                buf[i + j] +=
                    ((a.coeffs[i] as i64) * (b.coeffs[j] as i64) % MODULUS as i64) as i32;
            }
        }
        for i in 0..N as usize {
            c[i] = lift(buf[i] - buf[i + N as usize], MODULUS) as i32;
        }
        Self { coeffs: c }
    }

    /// sample a uniformly random polynomial with coefficients between 0 and q-1
    pub fn rand_poly<R: Rng>(rng: &mut R) -> Self {
        let mut res = Self::default();
        for e in res.coeffs.iter_mut() {
            let mut tmp = rng.next_u32();
            while tmp >= SAMPLE_THRESHOLD {
                tmp = rng.next_u32();
            }
            *e = (tmp % MODULUS) as i32 - MODULUS_OVER_TWO
        }
        res
    }

    /// Normalize self into a polynomial within [-MODULUS_OVER_2, MODULUS_OVER_2)
    pub fn normalize(&mut self) {
        self.coeffs.iter_mut().for_each(|x| {
            *x = *x % MODULUS as i32;
            if *x > MODULUS_OVER_TWO {
                *x -= MODULUS as i32
            }
            if *x < -MODULUS_OVER_TWO {
                *x += MODULUS as i32
            }
        });
    }

    /// A 256 digest of the polynomial
    pub fn digest(&self) -> [u8; 32] {
        let mut inputs = Vec::new();
        for e in self.coeffs {
            inputs.push((e & 0xFF) as u8);
            inputs.push(((e >> 8) & 0xFF) as u8);
        }
        let mut hasher = Sha256::new();
        hasher.update(inputs);
        let result = hasher.finalize();
        result.into()
    }

    pub(crate) fn is_ternary(&self) -> bool {
        for &e in self.coeffs.iter() {
            if e != 1 && e != -1 {
                return false;
            }
        }
        true
    }

    // sample a random ternary polynomial with a fixed weight
    pub fn rand_balanced_ternary<R: Rng>(rng: &mut R, half_weight: usize) -> Self {
        let mut ct = 0;
        let mut coeffs = [0; N as usize];
        let mut rng_ct = 0;
        let mut tmp = rng.next_u32();

        while ct < half_weight {
            let index = (tmp & 0xFF) as usize;
            tmp >>= 9;
            rng_ct += 1;
            if rng_ct == 3 {
                tmp = rng.next_u32();
                rng_ct = 0;
            }
            if coeffs[index] == 0 {
                ct += 1;
                coeffs[index] = 1
            }
        }
        ct = 0;
        while ct < half_weight {
            let index = (tmp & 0xFF) as usize;
            tmp >>= 9;
            rng_ct += 1;
            if rng_ct == 3 {
                tmp = rng.next_u32();
                rng_ct = 0;
            }

            if coeffs[index] == 0 {
                ct += 1;
                coeffs[index] = -1
            }
        }
        Self { coeffs }
    }

    // sample a random binary polynomial
    pub fn rand_binary<R: Rng>(rng: &mut R) -> Self {
        let mut res = Self::default();
        for i in 0..16 {
            let mut tmp = rng.next_u32();
            for j in 0..32 {
                res.coeffs[i * 32 + j] = (tmp & 1) as i32;
                tmp >>= 1;
            }
        }

        res
    }

    // sample a random ternary polynomial
    pub fn rand_ternary<R: Rng>(rng: &mut R, weight: usize) -> Self {
        let mut res = Self::default();
        let mut ct = 0;
        // todo: improve sampling rates
        while ct < weight {
            let tmp = rng.next_u32();
            if res.coeffs[(tmp % N as u32) as usize] == 0 {
                ct += 1;
                if (tmp >> 9) & 1 == 1 {
                    res.coeffs[(tmp % N as u32) as usize] = 1;
                } else {
                    res.coeffs[(tmp % N as u32) as usize] = -1;
                }
            }
        }
        res
    }

    pub fn infinity_norm(&self) -> u32 {
        let mut norm = 0;
        self.coeffs
            .iter()
            .for_each(|&x| norm = std::cmp::max(norm, std::cmp::max(x, -x)));
        norm as u32
    }

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
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    use crate::{Polynomial, MODULUS};

    #[test]
    fn test_normalization() {
        let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
        for _ in 0..10 {
            let mut poly = Polynomial::rand_poly(&mut rng);
            let mut poly2 = poly.clone();

            poly.normalize();
            poly2
                .coeffs
                .iter_mut()
                .for_each(|x| *x = *x + ((rng.next_u32() % 100) * MODULUS) as i32);
            poly2.normalize();
            assert_eq!(poly, poly2);

            for (e, f) in poly.coeffs.iter().zip(poly2.coeffs.iter()) {
                assert_eq!(e, f)
            }
        }
    }
}
