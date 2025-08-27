//! Identifier types.

// Intentionally not `pub`, so that we're free to experiment with the internal structure.
mod compound;
mod core;
mod qualification;
mod quote;
#[cfg(test)]
mod tests;

pub use compound::*;
pub use core::*;
pub use qualification::*;
pub use quote::*;
