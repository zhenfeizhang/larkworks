#[macro_export]
macro_rules! impl_field {
    ($field:ident $(<const $modulus: ident $( : u32) + >)) => {
        impl<const $modulus: u32> Add<$field<$modulus>> for $field<$modulus> {
            type Output = $field;

            fn add(self, rhs: $field) -> Self::Output {
                Self {
                    elem: (self.elem + rhs.elem) % $modulus as i32,
                }
            }
        }
    };
}
