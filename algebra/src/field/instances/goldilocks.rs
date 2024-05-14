use crate::{ConfigZZp, ZZp};

/// 2^32 - 1
const EPSILON: u64 = 0xffffffff;

/// Configuration parameter for Goldilocks
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigGoldilocks;

impl ConfigZZp for ConfigGoldilocks {
    type PrimitiveType = u64;
    type ProductType = u128;
    const MODULUS: Self::PrimitiveType = 0xffffffff00000001;

    /// The place where the multiplication algorithm is actually implemented.
    fn mul_internal(a: &Self::PrimitiveType, b: &Self::PrimitiveType) -> Self::PrimitiveType {
        util::reduce128((*a as u128) * (*b as u128))
    }

    /// The place where the addition algorithm is actually implemented.
    fn add_internal(a: &Self::PrimitiveType, b: &Self::PrimitiveType) -> Self::PrimitiveType {
        let (sum, over) = a.overflowing_add(*b);
        let (mut sum, over) = sum.overflowing_add((over as u64) * EPSILON);
        if over {
            // NB: self.0 > Self::ORDER && rhs.0 > Self::ORDER is necessary but not sufficient for
            // double-overflow.
            // This assume does two things:
            //  1. If compiler knows that either self.0 or rhs.0 <= ORDER, then it can skip this
            //     check.
            //  2. Hints to the compiler how rare this double-overflow is (thus handled better with
            //     a branch).
            util::assume(*a > Self::MODULUS && *b > Self::MODULUS);
            util::branch_hint();
            sum += EPSILON; // Cannot overflow.
        }
        sum
    }

    fn eq_internal(a: &Self::PrimitiveType, b: &Self::PrimitiveType) -> bool {
        *a == *b || *a + Self::MODULUS == *b || *a == *b + Self::MODULUS
    }
}

/// Goldilocks field with modulus 2^64 - 2^32 + 1.
/// A Goldilocks field may store a non-canonical form of the element
/// where the value can be between 0 and 2^64.
//
// ISSUE: This uses macro impl of mod reduction and will be slow.
// TODO: reload the multiplication algorithm as in
// https://github.com/zhenfeizhang/Goldilocks/blob/master/src/primefield/fp.rs#L472
pub type Goldilocks = ZZp<ConfigGoldilocks>;

mod util {

    #[inline(always)]
    pub fn assume(p: bool) {
        debug_assert!(p);
        if !p {
            unsafe {
                std::hint::unreachable_unchecked();
            }
        }
    }

    /// Try to force Rust to emit a branch. Example:
    ///     if x > 2 {
    ///         y = foo();
    ///         branch_hint();
    ///     } else {
    ///         y = bar();
    ///     }
    /// This function has no semantics. It is a hint only.
    #[inline(always)]
    pub(crate) fn branch_hint() {
        unsafe {
            std::arch::asm!("", options(nomem, nostack, preserves_flags));
        }
    }

    /// Reduces to a 64-bit value. The result might not be in canonical form; it could be in between the
    /// field order and `2^64`.
    #[inline]
    pub(crate) fn reduce128(x: u128) -> u64 {
        let (x_lo, x_hi) = split(x); // This is a no-op
        let x_hi_hi = x_hi >> 32;
        let x_hi_lo = x_hi & super::EPSILON;

        let (mut t0, borrow) = x_lo.overflowing_sub(x_hi_hi);
        if borrow {
            branch_hint(); // A borrow is exceedingly rare. It is faster to branch.
            t0 -= super::EPSILON; // Cannot underflow.
        }
        let t1 = x_hi_lo * super::EPSILON;
        unsafe { add_no_canonicalize_trashing_input(t0, t1) }
    }

    #[inline]
    pub(crate) fn split(x: u128) -> (u64, u64) {
        (x as u64, (x >> 64) as u64)
    }

    /// Fast addition modulo ORDER for x86-64.
    /// This function is marked unsafe for the following reasons:
    ///   - It is only correct if x + y < 2**64 + ORDER = 0x1ffffffff00000001.
    ///   - It is only faster in some circumstances. In particular, on x86 it overwrites both inputs in
    ///     the registers, so its use is not recommended when either input will be used again.
    #[inline(always)]
    #[cfg(target_arch = "x86_64")]
    pub(crate) unsafe fn add_no_canonicalize_trashing_input(x: u64, y: u64) -> u64 {
        let res_wrapped: u64;
        let adjustment: u64;
        std::arch:: asm!(
            "add {0}, {1}",
            // Trick. The carry flag is set iff the addition overflowed.
            // sbb x, y does x := x - y - CF. In our case, x and y are both {1:e}, so it simply does
            // {1:e} := 0xffffffff on overflow and {1:e} := 0 otherwise. {1:e} is the low 32 bits of
            // {1}; the high 32-bits are zeroed on write. In the end, we end up with 0xffffffff in {1}
            // on overflow; this happens be EPSILON.
            // Note that the CPU does not realize that the result of sbb x, x does not actually depend
            // on x. We must write the result to a register that we know to be ready. We have a
            // dependency on {1} anyway, so let's use it.
            "sbb {1:e}, {1:e}",
            inlateout(reg) x => res_wrapped,
            inlateout(reg) y => adjustment,
            options(pure, nomem, nostack),
        );
        assume(x != 0 || (res_wrapped == y && adjustment == 0));
        assume(y != 0 || (res_wrapped == x && adjustment == 0));
        // Add EPSILON == subtract ORDER.
        // Cannot overflow unless the assumption if x + y < 2**64 + ORDER is incorrect.
        res_wrapped + adjustment
    }

    #[inline(always)]
    #[cfg(not(target_arch = "x86_64"))]
    pub(crate) unsafe fn add_no_canonicalize_trashing_input(x: u64, y: u64) -> u64 {
        let (res_wrapped, carry) = x.overflowing_add(y);
        // Below cannot overflow unless the assumption if x + y < 2**64 + ORDER is incorrect.
        res_wrapped + super::EPSILON * (carry as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::Goldilocks;
    use crate::tests::field::random_field_tests;

    #[test]
    fn test_integer() {
        random_field_tests::<Goldilocks>("Goldilocks".to_string());
    }
}
