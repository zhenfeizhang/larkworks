use crate::{ConfigZZp, ZZp};

/// Configuration parameter for Goldilocks
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigGoldilocks;

impl ConfigZZp for ConfigGoldilocks {
    type PrimitiveType = u64;
    type ProductType = u128;
    const MODULUS: Self::PrimitiveType = 0xffffffff00000001;
}

/// Goldilocks field with modulus 2^64 - 2^32 + 1.
/// A Goldilocks field may store a non-canonical form of the element
/// where the value can be between 0 and 2^64.
//
// ISSUE: This uses macro impl of mod reduction and will be slow.
// TODO: reload the multiplication algorithm as in
// https://github.com/zhenfeizhang/Goldilocks/blob/master/src/primefield/fp.rs#L472
pub type Goldilocks = ZZp<ConfigGoldilocks>;

#[cfg(test)]
mod tests {
    use super::Goldilocks;
    use crate::tests::field::random_field_tests;

    #[test]
    fn test_integer() {
        random_field_tests::<Goldilocks>("Goldilocks".to_string());
    }
}
