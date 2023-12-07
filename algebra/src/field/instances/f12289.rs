use crate::{field::zz_p::ZZp, ConfigZZp};

/// Configuration parameter for ZZ mod 12289
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigZZp12289;

impl ConfigZZp for ConfigZZp12289 {
    type PrimitiveType = u16;
    type ProductType = u32;
    const MODULUS: Self::PrimitiveType = 3329;
}

/// ZZ mod 12289
pub type F12289 = ZZp<ConfigZZp12289>;

#[cfg(test)]
mod tests {
    use super::F12289;
    use crate::tests::field::random_field_tests;

    #[test]
    fn test_integer() {
        random_field_tests::<F12289>("F12289".to_string());
    }
}
