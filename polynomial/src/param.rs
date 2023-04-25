/// degree of polynomial
pub const N: usize = 512;
/// q for small ring, HVC modulus 202753
pub const MODULUS: i32 = 202753;
/// 1/N mod q
pub const ONE_OVER_N: i32 = 202357;
/// (q-1)/2
pub const MODULUS_OVER_TWO: i32 = 101376;
/// the largest multiple of q that is smaller than 2^32
pub const SAMPLE_THRESHOLD: u32 = 4294916799;
/// hamming weight of the hash of the message
pub const ALPHA_H: usize = 37;
