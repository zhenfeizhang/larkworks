//! This crate provides arithemtic backend for algebraic objects used in
//! lattic-based crypto.

#![warn(unused, future_incompatible, nonstandard_style)]
#![deny(missing_docs)]

mod field;
mod lattice;
mod matrix;
mod polynomial;
mod ring;
mod vector;

#[cfg(test)]
mod tests;

mod prelude;

use log::warn;
pub use prelude::*;
