//! This crate provides arithemtic backend for algebraic objects used in
//! lattic-based crypto.

#![warn(unused, future_incompatible, nonstandard_style)]
#![deny(missing_docs)]

mod field;
mod lattice;
mod macros;
mod matrix;
mod polynomial;
mod polynomial_ring;
mod vector;

#[cfg(test)]
mod tests;

mod prelude;

use log::warn;
pub use prelude::*;
