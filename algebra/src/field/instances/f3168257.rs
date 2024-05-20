//! Field used in Chipmunk's one time signature scheme

use crate::{ConfigZZp, ZZp};

/// Configuration parameter for ZZ mod 3168257
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigZZp3168257;

impl ConfigZZp for ConfigZZp3168257 {
    type PrimitiveType = u32;
    type ProductType = u64;
    const MODULUS: Self::PrimitiveType = 3168257;
    /// The place where the multiplication algorithm is actually implemented.
    fn mul_internal(a: &Self::PrimitiveType, b: &Self::PrimitiveType) -> Self::PrimitiveType {
        (*a as Self::ProductType * *b as Self::ProductType % Self::MODULUS as Self::ProductType)
            as Self::PrimitiveType
    }

    /// The place where the addition algorithm is actually implemented.
    fn add_internal(a: &Self::PrimitiveType, b: &Self::PrimitiveType) -> Self::PrimitiveType {
        let mut tmp = a + b;
        if tmp >= Self::MODULUS {
            tmp -= Self::MODULUS
        }
        tmp
    }

    /// The place where the subtraction algorithm is actually implemented.
    fn sub_internal(a: &Self::PrimitiveType, b: &Self::PrimitiveType) -> Self::PrimitiveType {
        if a >= b {
            a - b
        } else {
            a + Self::MODULUS - b
        }
    }

    fn eq_internal(a: &Self::PrimitiveType, b: &Self::PrimitiveType) -> bool {
        a % Self::MODULUS == b % Self::MODULUS
    }
}

/// ZZ mod 3168257
pub type F3168257 = ZZp<ConfigZZp3168257>;

#[cfg(test)]
mod tests {
    use super::F3168257;
    use crate::tests::field::random_field_tests;

    #[test]
    fn test_integer() {
        random_field_tests::<F3168257>("F3168257".to_string());
    }
}
