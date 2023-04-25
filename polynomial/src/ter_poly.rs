#[derive(Debug, Clone, PartialEq)]
// ternary polynomials in coefficient encoding
pub struct TerPolyCoeffEncoding {
    pub(crate) pos: Vec<usize>,
    pub(crate) neg: Vec<usize>,
}

#[cfg(test)]
mod test {
    use crate::{param::ALPHA_H, poly::Poly, Polynomial};

    use super::TerPolyCoeffEncoding;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;
    #[test]
    fn test_ter_mul() {
        let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
        let half_weight = ALPHA_H / 2;

        for _ in 0..10 {
            let ter_poly = Poly::rand_balanced_ternary(&mut rng, half_weight);
            let bin_poly = Poly::rand_binary(&mut rng);
            let ter_poly_coeff_encoding: TerPolyCoeffEncoding = (&ter_poly).into();

            let prod_1 = Poly::schoolbook(&bin_poly, &ter_poly);
            let prod_2 = Poly::ter_mul_bin(&ter_poly_coeff_encoding, &bin_poly);
            let prod_3 = ter_poly * bin_poly;
            assert_eq!(prod_1, prod_3);
            assert_eq!(prod_1, prod_2);
        }
    }
}
