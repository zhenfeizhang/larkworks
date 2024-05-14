//! This crate provides arithmetic backend for algebraic objects used in
//! lattice-based crypto.

#![warn(unused, future_incompatible, nonstandard_style)]
#![deny(missing_docs)]

/// NTT domain
mod domain;
/// Fields
mod field;
/// Polynomials
mod polynomial;
/// Polynomial ring elements
mod ring;
/// Vector space
mod vector;

// mod lattice;
// mod matrix;

#[cfg(test)]
mod tests;

/// Re-expose all APIs
mod prelude;

use log::warn;
pub use prelude::*;
