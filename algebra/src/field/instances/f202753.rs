//! Field used in Chipmunk's homomorphic vector commitment

use crate::{ConfigZZp, ZZp};

/// Configuration parameter for ZZ mod 202753
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigZZp202753;

impl ConfigZZp for ConfigZZp202753 {
    type PrimitiveType = u32;
    type ProductType = u64;
    const MODULUS: Self::PrimitiveType = 202753;
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

/// ZZ mod 202753
pub type F202753 = ZZp<ConfigZZp202753>;

#[cfg(test)]
mod tests {
    use super::F202753;
    use crate::tests::field::random_field_tests;

    #[test]
    fn test_integer() {
        random_field_tests::<F202753>("F202753".to_string());
    }
}
