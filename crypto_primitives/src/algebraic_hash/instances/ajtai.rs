use ark_std::rand::RngCore;
use lark_algebra::{ConfigZZVecGoldilocks256, NTTDomain, RingGoldilock256, Vector, ZZVec};

use crate::algebraic_hash::definitions::AlgebraicHash;

/// number of input ring elements
const M: usize = 4;

pub struct AjtaiHash;

impl AlgebraicHash for AjtaiHash {
    /// Configuration of the hash function
    type Configuration = ();

    /// public parameters, stored in their NTT form
    type Parameters = [ZZVec<ConfigZZVecGoldilocks256>; M];

    /// Input to the hash
    type Preimages = [RingGoldilock256; M];

    /// Output to the hash
    type Digests = RingGoldilock256;

    /// Generate the config file
    fn config() -> Self::Configuration {}

    /// Setup public parameters
    fn setup(_config: &Self::Configuration, mut rng: impl RngCore) -> Self::Parameters {
        (0..M)
            .map(|_| ZZVec::random(&mut rng, None))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    /// hash preimages to the digests
    fn hash(inputs: &Self::Preimages, param: &Self::Parameters) -> Self::Digests {
        let digest_vec: ZZVec<ConfigZZVecGoldilocks256> = inputs
            .iter()
            .zip(param.iter())
            .map(|(input_poly, param_vec)| {
                let input_vec: ZZVec<ConfigZZVecGoldilocks256> = NTTDomain::forward_ntt(input_poly);
                input_vec * param_vec
            })
            .sum();

        NTTDomain::reverse_ntt(&digest_vec)
    }
}
