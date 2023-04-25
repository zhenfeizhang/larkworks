#[macro_export]
macro_rules! impl_poly {
    ($poly:ident, $modulus:ident, $dim:ident) => {
        impl PartialEq for $poly {
            fn eq(&self, rhs: &$poly) -> bool {
                self.coeffs
                    .iter()
                    .zip(rhs.coeffs.iter())
                    .map(|(&x, &y)| crate::lift(x, $modulus) == crate::lift(y, $modulus))
                    .fold(true, |acc, x| acc & x)
            }
        }

        impl Default for $poly {
            fn default() -> Self {
                Self {
                    coeffs: [0i32; $dim as usize],
                }
            }
        }

        impl std::fmt::Display for $poly {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "{:#16x} {:#16x} {:#16x} {:#16x}",
                    self.coeffs[0], self.coeffs[1], self.coeffs[0], self.coeffs[1]
                )
            }
        }

        impl std::ops::Add for $poly {
            type Output = Self;

            // Coefficient wise additions without mod reduction.
            fn add(self, other: Self) -> Self {
                let mut res = self;
                res += other;
                res
            }
        }

        impl std::ops::AddAssign for $poly {
            // Coefficient wise additions with mod reduction.
            fn add_assign(&mut self, other: Self) {
                self.coeffs
                    .iter_mut()
                    .zip(other.coeffs)
                    .for_each(|(x, y)| *x = (*x + y) % $modulus as i32)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_signed_poly {
    ($poly:ident, $modulus:ident, $dim:ident) => {
        crate::impl_poly!($poly, $modulus, $dim);

        impl From<&$poly> for TerPolyCoeffEncoding {
            fn from(poly: &$poly) -> Self {
                #[cfg(debug_assertions)]
                assert!(poly.is_ternary());

                let mut pos = vec![];
                let mut neg = vec![];
                for (index, &coeff) in poly.coeffs.iter().enumerate() {
                    if coeff == 1 {
                        pos.push(index);
                    }
                    if coeff == -1 {
                        neg.push(index);
                    }
                }

                Self { pos, neg }
            }
        }

        impl From<&TerPolyCoeffEncoding> for $poly {
            fn from(poly: &TerPolyCoeffEncoding) -> Self {
                let mut res = Self::default();
                for p in poly.pos.iter() {
                    res.coeffs[*p] = 1;
                }
                for n in poly.neg.iter() {
                    res.coeffs[*n] = -1;
                }
                res
            }
        }
    };
}

#[macro_export]
macro_rules! impl_ntt_poly {
    ($poly:ident, $ntt_poly:ident, $modulus:ident, $dim:ident, $one_over_n:ident) => {
        impl From<&$poly> for $ntt_poly {
            fn from(poly: &$poly) -> Self {
                let mut coeffs = poly.coeffs;
                ntt(&mut coeffs);
                Self { coeffs }
            }
        }

        impl From<$poly> for $ntt_poly {
            fn from(poly: $poly) -> Self {
                Self::from(&poly)
            }
        }

        impl From<&$ntt_poly> for $poly {
            fn from(poly: &$ntt_poly) -> Self {
                let mut coeffs = poly.coeffs;
                inv_ntt(&mut coeffs);
                $poly { coeffs }
            }
        }

        impl From<$ntt_poly> for $poly {
            fn from(poly: $ntt_poly) -> Self {
                $poly::from(&poly)
            }
        }

        impl std::ops::Mul for $ntt_poly {
            type Output = Self;

            // Coefficient-wise multiplication over the NTT domain.
            fn mul(self, other: Self) -> Self {
                let mut res = Self::default();
                for (e, (f, g)) in res
                    .coeffs
                    .iter_mut()
                    .zip(self.coeffs.iter().zip(other.coeffs.iter()))
                {
                    *e = (((*f as i64) * (*g as i64)) % $modulus as i64) as i32
                }

                res
            }
        }

        fn ntt(p: &mut [i32; $dim]) {
            let mut t = $dim;
            for l in 0..9 {
                let m = 1 << l;
                let ht = t >> 1;
                let mut i = 0;
                let mut j1 = 0;
                while i < m {
                    let s = NTT_TABLE[m + i];
                    let j2 = j1 + ht;
                    let mut j = j1;
                    while j < j2 {
                        let u = p[j];
                        let v = ((p[j + ht] as i64) * (s as i64) % $modulus as i64) as i32;
                        p[j] = (u + v) % $modulus;
                        p[j + ht] = (u + $modulus - v) % $modulus;
                        j += 1;
                    }
                    i += 1;
                    j1 += t;
                }
                t = ht;
            }
        }

        fn inv_ntt(p: &mut [i32; $dim]) {
            let mut t = 1;
            let mut m = N;

            while m > 1 {
                let hm = m >> 1;
                let dt = t << 1;
                let mut i = 0usize;
                let mut j1 = 0;
                while i < hm {
                    let j2 = j1 + t;
                    let s = INV_NTT_TABLE[hm + i];
                    let mut j = j1;
                    while j < j2 {
                        let u = p[j];
                        let v = p[j + t];
                        p[j] = (u + v) % $modulus;
                        p[j + t] =
                            (((u + $modulus - v) as i64) * (s as i64) % $modulus as i64) as i32;
                        j += 1;
                    }
                    i += 1;
                    j1 += dt;
                }
                t = dt;
                m = hm;
            }
            for e in p.iter_mut() {
                *e = (*e as i64 * $one_over_n as i64 % $modulus as i64) as i32;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_signed_poly_functions {
    ($poly:ident, $ntt_poly:ident, $modulus:ident, $dim:ident, $modulus_over_2:ident, $sample_threshold:ident) => {
        use crate::param::ALPHA_H;
        use rand::SeedableRng;

        impl Polynomial for $poly {
            const DIM: usize = $dim;
            const MODULUS: i32 = $modulus;
            const MODULUS_OVER_2: i32 = $modulus_over_2;
            const SAMPLE_THRESHOLD: u32 = $sample_threshold;

            type NTTPoly = $ntt_poly;

            // school book multiplication
            // slow. only used for correctness checking
            fn schoolbook(a: &Self, b: &Self) -> Self {
                let mut buf = [0i32; $dim * 2];
                let mut c = [0; $dim];
                for i in 0..$dim {
                    for j in 0..$dim {
                        buf[i + j] +=
                            ((a.coeffs[i] as i64) * (b.coeffs[j] as i64) % $modulus as i64) as i32;
                    }
                }
                for i in 0..$dim {
                    c[i] = crate::poly::lift(buf[i] - buf[i + $dim as usize], $modulus) as i32;
                }
                Self { coeffs: c }
            }

            /// sample a uniformly random polynomial with coefficients between 0 and q-1
            fn rand_poly<R: Rng>(rng: &mut R) -> Self {
                let mut res = Self::default();
                for e in res.coeffs.iter_mut() {
                    let mut tmp = rng.next_u32();
                    while tmp >= $sample_threshold {
                        tmp = rng.next_u32();
                    }
                    *e = (tmp % $modulus as u32) as i32 - $modulus_over_2
                }
                res
            }

            /// A 32 bytes digest of the polynomial
            fn digest(&self) -> [u8; 32] {
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

            /// If the polynomial's coefficients are ternary
            fn is_ternary(&self) -> bool {
                for &e in self.coeffs.iter() {
                    if e != 1 && e != -1 {
                        return false;
                    }
                }
                true
            }

            /// Sample a random ternary polynomial with a fixed weight
            fn rand_balanced_ternary<R: Rng>(rng: &mut R, half_weight: usize) -> Self {
                let mut ct = 0;
                let mut coeffs = [0; $dim];
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

            /// Sample a random binary polynomial
            fn rand_binary<R: Rng>(rng: &mut R) -> Self {
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

            /// Sample a random ternary polynomial
            fn rand_ternary<R: Rng>(rng: &mut R, weight: usize) -> Self {
                let mut res = Self::default();
                let mut ct = 0;
                // todo: improve sampling rates
                while ct < weight {
                    let tmp = rng.next_u32() as usize;
                    if res.coeffs[(tmp % $dim) as usize] == 0 {
                        ct += 1;
                        if (tmp >> 9) & 1 == 1 {
                            res.coeffs[(tmp % $dim) as usize] = 1;
                        } else {
                            res.coeffs[(tmp % $dim) as usize] = -1;
                        }
                    }
                }
                res
            }

            /// Sample a random polynomial with coefficients between [-p, p]
            fn rand_mod_p<R: Rng>(rng: &mut R, p: u32) -> Self {
                // todo: improve sampling rates
                let mut res = Self::default();
                let modulus = (p * 2 + 1) as u32;
                let threshold = u32::MAX / modulus * modulus;
                for e in res.coeffs.iter_mut() {
                    let mut tmp = rng.next_u32();
                    while tmp > threshold {
                        tmp = rng.next_u32();
                    }

                    *e = (tmp % modulus) as i32 - p as i32;
                }

                res
            }

            /// Hash a blob into a message polynomial
            fn from_hash_message(msg: &[u8]) -> Self {
                let mut hasher = Sha256::new();
                hasher.update(msg);
                let seed = hasher.finalize().into();
                let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed);

                Self::rand_ternary(&mut rng, ALPHA_H)
            }

            /// Infinity norm of the polynomial
            fn infinity_norm(&self) -> u32 {
                let mut norm = 0;
                self.coeffs
                    .iter()
                    .for_each(|&x| norm = std::cmp::max(norm, std::cmp::max(x, -x)));
                norm as u32
            }

            /// Normalize self into a polynomial within [-MODULUS_OVER_2, MODULUS_OVER_2)
            fn lift(&mut self) {
                self.coeffs.iter_mut().for_each(|x| {
                    *x = lift(*x, $modulus);
                });
            }

            /// Normalize self into a polynomial within (-MODULUS, MODULUS)
            fn normalize(&mut self) {
                self.coeffs.iter_mut().for_each(|x| {
                    *x = normalize(*x, $modulus);
                });
            }
        }
    };
}
