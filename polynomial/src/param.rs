/// degree of polynomial
pub use cpoly::N;
/// q for ring, modulus 202753
pub use cpoly::Q as MODULUS;
/// (q-1)/2
pub const MODULUS_OVER_TWO: i32 = 101376;
/// the largest multiple of q that is smaller than 2^32
pub const SAMPLE_THRESHOLD: u32 = 4294916799;
