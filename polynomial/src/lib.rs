mod poly;
mod poly_ntt;
mod ter_poly;
mod param;

pub use ter_poly::*;
pub use poly::*;
pub use param::*;
pub use poly_ntt::*;

#[inline]
fn lift(a: i32, modulus: u32) -> u32 {
    (a % modulus as i32 + modulus as i32) as u32 % modulus
}
