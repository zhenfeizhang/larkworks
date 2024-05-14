use core::iter::Product;
use std::ops::{Mul, MulAssign};

use crate::{
    ConfigZZVecGoldilocks256, ConfigZZpXGoldilocks256, NTTDomain, PolyGoldilock256, Polynomial,
    PolynomialRing, ZZVec,
};

/// Ring over ZZ_q/(x^512+1)
pub type RingGoldilock256 = PolyGoldilock256;
/// Configuration for ring over ZZ_q/(x^512+1)
pub type ConfigRingGoldilocks256 = ConfigZZpXGoldilocks256;

// ========================
// multiplications
// ========================
impl Mul for RingGoldilock256 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(&rhs)
    }
}

impl<'b> Mul<&'b RingGoldilock256> for RingGoldilock256 {
    type Output = RingGoldilock256;

    #[inline]
    fn mul(self, rhs: &'b RingGoldilock256) -> RingGoldilock256 {
        let mut res = self;
        res.mul_assign(rhs);
        res
    }
}

impl MulAssign for RingGoldilock256 {
    #[inline]
    fn mul_assign(&mut self, rhs: RingGoldilock256) {
        self.mul_assign(&rhs)
    }
}

impl<'b> MulAssign<&'b RingGoldilock256> for RingGoldilock256 {
    #[inline]
    fn mul_assign(&mut self, rhs: &'b RingGoldilock256) {
        let a: ZZVec<ConfigZZVecGoldilocks256> = NTTDomain::forward_ntt(self);
        let b: ZZVec<ConfigZZVecGoldilocks256> = NTTDomain::forward_ntt(rhs);
        println!("a: {}", a);
        println!("b: {}", b);
        let c = a * b;
        println!("c: {}", c);
        *self = c.reverse_ntt();
    }
}

impl<T> Product<T> for RingGoldilock256
where
    T: core::borrow::Borrow<Self>,
{
    fn product<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, item| acc * item.borrow())
    }
}

impl PolynomialRing<ConfigRingGoldilocks256, ConfigZZpXGoldilocks256> for RingGoldilock256 {}

#[cfg(test)]
impl RingGoldilock256 {
    /// school book multiplication
    /// output = a(x) * b(x) mod x^N +1 mod MODULUS
    /// using school-book multiplications
    pub fn schoolbook_mul(a: &Self, b: &Self) -> Self {
        use crate::ConfigZZp;
        use crate::ConfigZZpX;

        let a = &a.coeffs;
        let b = &b.coeffs;
        let modulus = <ConfigRingGoldilocks256 as ConfigZZpX>::BaseConfig::MODULUS;
        const N: usize = <ConfigRingGoldilocks256 as ConfigZZpX>::DIM;

        let mut buf = [0u128; N << 1];
        let mut c = [0; N];
        for i in 0..N {
            for j in 0..N {
                buf[i + j] += (a[i].0 as u128 * b[j].0 as u128) % modulus as u128;
            }
        }

        for i in 0..N {
            c[i] = ((buf[i] + modulus as u128 - (buf[i + N] % modulus as u128)) % modulus as u128)
                as u64;
        }
        Self::from_primitive_types(&c)
    }
}

#[test]
fn test_ring_mul() {
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);
    let a = RingGoldilock256::random(&mut rng, None);
    let b = RingGoldilock256::random(&mut rng, None);
    let c = RingGoldilock256::schoolbook_mul(&a, &b);
    let d = a * b;
    assert_eq!(c, d)
}
