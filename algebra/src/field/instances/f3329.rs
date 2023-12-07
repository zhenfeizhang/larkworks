use crate::{field::zz::ZZp, ZZpConfig};

/// Configuration parameter for ZZ mod 3329
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Param3329;

impl ZZpConfig for Param3329 {
    type PrimitiveType = u16;
    type ProductType = u32;
    const MODULUS: Self::PrimitiveType = 3329;
}

///  ZZ mod 3329
pub type F3329 = ZZp<Param3329>;

#[cfg(test)]
mod tests {
    use super::F3329;
    use crate::tests::field::random_field_tests;

    #[test]
    fn test_integer() {
        random_field_tests::<F3329>("F3329".to_string());
    }
}
