#[macro_export]
macro_rules! field_common {
    (
        $field:ident, $modulus:ident, $primitive_type:ident
    ) => {
        impl std::fmt::Display for $field {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<'b> Sub<&'b $field> for $field {
            type Output = $field;

            #[inline]
            fn sub(self, rhs: &'b $field) -> $field {
                self.sub(*rhs)
            }
        }

        impl SubAssign for $field {
            #[inline]
            fn sub_assign(&mut self, rhs: $field) {
                *self = (*self).sub(rhs)
            }
        }

        impl<'b> SubAssign<&'b $field> for $field {
            #[inline]
            fn sub_assign(&mut self, rhs: &'b $field) {
                *self = (*self).sub(rhs)
            }
        }

        // ========================
        // additions
        // ========================

        impl<'b> Add<&'b $field> for $field {
            type Output = $field;

            #[inline]
            fn add(self, rhs: &'b $field) -> $field {
                self.add(*rhs)
            }
        }

        impl AddAssign for $field {
            #[inline]
            fn add_assign(&mut self, rhs: $field) {
                *self = (*self).add(rhs)
            }
        }

        impl<'b> AddAssign<&'b $field> for $field {
            #[inline]
            fn add_assign(&mut self, rhs: &'b $field) {
                *self = (*self).add(rhs)
            }
        }

        impl<T> Sum<T> for $field
        where
            T: core::borrow::Borrow<Self>,
        {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = T>,
            {
                iter.fold(Self::ZERO, |acc, item| acc + item.borrow())
            }
        }

        // ========================
        // multiplications
        // ========================

        impl<'b> Mul<&'b $field> for $field {
            type Output = $field;

            #[inline]
            fn mul(self, rhs: &'b $field) -> $field {
                self.mul(*rhs)
            }
        }

        impl MulAssign for $field {
            #[inline]
            fn mul_assign(&mut self, rhs: $field) {
                *self = self.clone().mul(rhs)
            }
        }

        impl<'b> MulAssign<&'b $field> for $field {
            #[inline]
            fn mul_assign(&mut self, rhs: &'b $field) {
                *self = self.clone().mul(rhs)
            }
        }

        impl<T> Product<T> for $field
        where
            T: core::borrow::Borrow<Self>,
        {
            fn product<I: Iterator<Item = T>>(iter: I) -> Self {
                iter.fold(Self::ONE, |acc, item| acc * item.borrow())
            }
        }

        // ========================
        // conversion
        // ========================

        impl From<u64> for $field {
            fn from(value: u64) -> Self {
                assert!(value < $modulus as u64);
                Self(value as $primitive_type)
            }
        }

        impl Into<u64> for $field {
            fn into(self) -> u64 {
                self.0 as u64
            }
        }

        // ========================
        // misc
        // ========================

        impl ConstantTimeEq for $field {
            fn ct_eq(&self, other: &Self) -> subtle::Choice {
                self.0.ct_eq(&other.0)
            }
        }

        impl ConditionallySelectable for $field {
            fn conditional_select(a: &Self, b: &Self, choice: subtle::Choice) -> Self {
                Self($primitive_type::conditional_select(&a.0, &b.0, choice))
            }
        }

        impl ff::Field for $field {
            /// The zero element of the field, the additive identity.
            const ZERO: Self = Self(0);

            /// The one element of the field, the multiplicative identity.
            const ONE: Self = Self(1);

            /// Returns an element chosen uniformly at random using a user-provided RNG.
            fn random(mut rng: impl RngCore) -> Self {
                Self((rng.next_u64() % $modulus as u64) as $primitive_type)
            }

            /// Squares this element.
            #[must_use]
            fn square(&self) -> Self {
                *self * *self
            }

            /// Doubles this element.
            #[must_use]
            fn double(&self) -> Self {
                *self + *self
            }

            /// Computes the multiplicative inverse of this element,
            /// failing if the element is zero.
            fn invert(&self) -> CtOption<Self> {
                let tmp = self.pow_vartime(&[($modulus - 2) as u64]);
                CtOption::new(tmp, !self.ct_eq(&Self::ZERO))
            }

            /// Computes:
            ///
            /// - $(\textsf{true}, \sqrt{\textsf{num}/\textsf{div}})$, if $\textsf{num}$ and
            ///   $\textsf{div}$ are nonzero and $\textsf{num}/\textsf{div}$ is a square in the
            ///   field;
            /// - $(\textsf{true}, 0)$, if $\textsf{num}$ is zero;
            /// - $(\textsf{false}, 0)$, if $\textsf{num}$ is nonzero and $\textsf{div}$ is zero;
            /// - $(\textsf{false}, \sqrt{G_S \cdot \textsf{num}/\textsf{div}})$, if
            ///   $\textsf{num}$ and $\textsf{div}$ are nonzero and $\textsf{num}/\textsf{div}$ is
            ///   a nonsquare in the field;
            ///
            /// where $G_S$ is a non-square.
            ///
            /// # Warnings
            ///
            /// - The choice of root from `sqrt` is unspecified.
            /// - The value of $G_S$ is unspecified, and cannot be assumed to have any specific
            ///   value in a generic context.
            fn sqrt_ratio(_num: &Self, _div: &Self) -> (Choice, Self) {
                todo!()
            }
        }
    };
}
