// use std::iter::Product;
// use std::ops::{Mul, MulAssign};

// use crate::{
//     field::f202753::F202753, polynomial::Poly202753, NTTField, Polynomial, PolynomialRing,
// };

// const DEGREE: usize = 1024;

// pub struct PolyRing202753{
//     pub polynomial: Poly202753
// }

// impl Mul for Poly202753 {}
// impl<'b> Mul<&'b Poly202753> for Poly202753 {
//     type Output = Poly202753;

//     #[inline]
//     fn mul(self, rhs: &'b Poly202753) -> Poly202753 {
//         self.mul(*rhs)
//     }
// }

// impl MulAssign for Poly202753 {}

// impl<'b> MulAssign<&'b Poly202753> for Poly202753 {
//     #[inline]
//     fn mul_assign(&mut self, rhs: &'b Poly202753) {
//         *self = self.clone().mul(rhs)
//     }
// }

// impl Product for Poly202753 {}

// // impl PolynomialRing<F202753, DEGREE> for Poly202753 {}
