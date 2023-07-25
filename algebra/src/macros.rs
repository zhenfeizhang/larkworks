// #[macro_export]
// macro_rules! additive_ops {
//     (
//         $type:ident
//     ) => {
//         // ========================
//         // subtractions
//         // ========================
//         impl<'b> Sub<&'b $type> for $type {
//             type Output = $type;

//             #[inline]
//             fn sub(self, rhs: &'b $type) -> $type {
//                 self.sub(*rhs)
//             }
//         }

//         impl SubAssign for $type {
//             #[inline]
//             fn sub_assign(&mut self, rhs: $type) {
//                 *self = (*self).sub(rhs)
//             }
//         }

//         impl<'b> SubAssign<&'b $type> for $type {
//             #[inline]
//             fn sub_assign(&mut self, rhs: &'b $type) {
//                 *self = (*self).sub(rhs)
//             }
//         }

//         // ========================
//         // additions
//         // ========================

//         impl<'b> Add<&'b $type> for $type {
//             type Output = $type;

//             #[inline]
//             fn add(self, rhs: &'b $type) -> $type {
//                 self.add(*rhs)
//             }
//         }

//         impl AddAssign for $type {
//             #[inline]
//             fn add_assign(&mut self, rhs: $type) {
//                 *self = (*self).add(rhs)
//             }
//         }

//         impl<'b> AddAssign<&'b $type> for $type {
//             #[inline]
//             fn add_assign(&mut self, rhs: &'b $type) {
//                 *self = (*self).add(rhs)
//             }
//         }
//         // ========================
//         // sum
//         // ========================
//         impl<T> Sum<T> for $type
//         where
//             T: core::borrow::Borrow<Self>,
//         {
//             fn sum<I>(iter: I) -> Self
//             where
//                 I: Iterator<Item = T>,
//             {
//                 iter.fold(Self::ZERO, |acc, item| acc + item.borrow())
//             }
//         }
//     };
// }
