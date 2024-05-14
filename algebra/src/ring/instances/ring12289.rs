use core::iter::Product;
use std::ops::{Mul, MulAssign};

use crate::{
    ConfigZZVec12289_512, ConfigZZpX12289_512, NTTDomain, Poly12289_512, Polynomial,
    PolynomialRing, ZZVec,
};

/// Ring over ZZ_q/(x^512+1)
pub type Ring12289_512 = Poly12289_512;
/// Configuration for ring over ZZ_q/(x^512+1)
pub type ConfigRing12289_512 = ConfigZZpX12289_512;

// ========================
// multiplications
// ========================
impl Mul for Ring12289_512 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(&rhs)
    }
}

impl<'b> Mul<&'b Ring12289_512> for Ring12289_512 {
    type Output = Ring12289_512;

    #[inline]
    fn mul(self, rhs: &'b Ring12289_512) -> Ring12289_512 {
        let mut res = self;
        res.mul_assign(rhs);
        res
    }
}

impl MulAssign for Ring12289_512 {
    #[inline]
    fn mul_assign(&mut self, rhs: Ring12289_512) {
        self.mul_assign(&rhs)
    }
}

impl<'b> MulAssign<&'b Ring12289_512> for Ring12289_512 {
    #[inline]
    fn mul_assign(&mut self, rhs: &'b Ring12289_512) {
        let a: ZZVec<ConfigZZVec12289_512> = NTTDomain::forward_ntt(self);
        let b: ZZVec<ConfigZZVec12289_512> = NTTDomain::forward_ntt(rhs);
        println!("a: {}", a);
        println!("b: {}", b);
        let c = a * b;
        println!("c: {}", c);
        *self = c.reverse_ntt();
    }
}

impl<T> Product<T> for Ring12289_512
where
    T: core::borrow::Borrow<Self>,
{
    fn product<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, item| acc * item.borrow())
    }
}

impl PolynomialRing<ConfigRing12289_512, ConfigZZVec12289_512> for Ring12289_512 {}

#[cfg(test)]
impl Ring12289_512 {
    /// school book multiplication
    /// output = a(x) * b(x) mod x^N +1 mod MODULUS
    /// using school-book multiplications
    pub fn schoolbook_mul(a: &Self, b: &Self) -> Self {
        use crate::ConfigZZp;
        use crate::ConfigZZpX;

        let a = &a.coeffs;
        let b = &b.coeffs;
        let modulus = <ConfigRing12289_512 as ConfigZZpX>::BaseConfig::MODULUS;
        const N: usize = <ConfigRing12289_512 as ConfigZZpX>::DIM;

        let mut buf = [0u32; N << 1];
        let mut c = [0; N];
        for i in 0..N {
            for j in 0..N {
                buf[i + j] += (a[i].0 as u32 * b[j].0 as u32) % modulus as u32;
            }
        }

        for i in 0..N {
            c[i] =
                ((buf[i] + modulus as u32 - (buf[i + N] % modulus as u32)) % modulus as u32) as u16;
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
    let a = Ring12289_512::random(&mut rng, None);
    let b = Ring12289_512::random(&mut rng, None);
    let c = Ring12289_512::schoolbook_mul(&a, &b);
    let d = a * b;
    assert_eq!(c, d)
}
