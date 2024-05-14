use ark_std::rand::RngCore;

pub trait AlgebraicHash {
    /// Configuration of the hash function
    type Configuration;

    /// public parameters, if any
    type Parameters;

    /// Input to the hash
    type Preimages;

    /// Output to the hash
    type Digests;

    /// Generate the config file
    fn config() -> Self::Configuration;

    /// Setup public parameters
    fn setup(config: &Self::Configuration, rng: impl RngCore) -> Self::Parameters;

    /// hash preimages to the digests
    fn hash(inputs: &Self::Preimages, param: &Self::Parameters) -> Self::Digests;
}
