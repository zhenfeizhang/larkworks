use crate::{field::zz_p::ZZp, ZZpConfig};

/// Configuration parameter for ZZ mod 8380417
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ZZpConfig8380417;

impl ZZpConfig for ZZpConfig8380417 {
    type PrimitiveType = u32;
    type ProductType = u64;
    const MODULUS: Self::PrimitiveType = 8380417;
}

/// ZZ mod 8380417
pub type F8380417 = ZZp<ZZpConfig8380417>;

#[cfg(test)]
mod tests {
    use super::F8380417;
    use crate::tests::field::random_field_tests;

    #[test]
    fn test_integer() {
        random_field_tests::<F8380417>("F8380417".to_string());
    }
}
