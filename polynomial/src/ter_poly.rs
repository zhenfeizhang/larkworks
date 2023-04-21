use crate::{Polynomial};

#[derive(Debug, Clone, PartialEq)]
// ternary polynomials in coefficient encoding
pub struct TerPolyCoeffEncoding {
    pub(crate) pos: Vec<usize>,
    pub(crate) neg: Vec<usize>,
}

impl From<&Polynomial> for TerPolyCoeffEncoding {
    fn from(poly: &Polynomial) -> Self {
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

impl From<&TerPolyCoeffEncoding> for Polynomial {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Polynomial};
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;
    #[test]
    fn test_ter_mul() {
        let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
        let half_weight = 30 / 2;

        for _ in 0..10 {
            let ter_poly = Polynomial::rand_balanced_ternary(&mut rng, half_weight);
            let bin_poly = Polynomial::rand_binary(&mut rng);
            let ter_poly_coeff_encoding: TerPolyCoeffEncoding = (&ter_poly).into();

            let prod_1 = Polynomial::schoolbook(&bin_poly, &ter_poly);
            let prod_2 = Polynomial::ter_mul_bin(&ter_poly_coeff_encoding, &bin_poly);
            let prod_3 = ter_poly * bin_poly;
          
            assert_eq!(prod_1, prod_3);
            assert_eq!(prod_1, prod_2);
        }
    }
}
