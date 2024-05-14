use crate::{field::zz_p::ZZp, ConfigZZp};

/// Configuration parameter for ZZ mod 8380417
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigZZp8380417;

impl ConfigZZp for ConfigZZp8380417 {
    type PrimitiveType = u32;
    type ProductType = u64;
    const MODULUS: Self::PrimitiveType = 8380417;

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

/// ZZ mod 8380417
pub type F8380417 = ZZp<ConfigZZp8380417>;

#[cfg(test)]
mod tests {
    use super::F8380417;
    use crate::tests::field::random_field_tests;

    #[test]
    fn test_integer() {
        random_field_tests::<F8380417>("F8380417".to_string());
    }
}
