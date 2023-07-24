use rand::RngCore;
mod sis_hash;

pub trait Hash {
    type Parameters;
    type Input;
    type Output;

    /// Generate hash parameters
    fn setup(rng: impl RngCore) -> Self::Parameters;

    /// Initialize a haser
    fn init() -> Self;

    /// Update the state of the hash
    fn update(&mut self, param: &Self::Parameters, input: &[Self::Input]);

    /// Finalize the hash and generate the output.
    fn finalize(&self) -> Self::Output;
}
