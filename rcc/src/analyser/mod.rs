//! Semantic analysis includes:
//! - Symbol resolving
//! - Type check
//! - Flow control check
//!

mod sym_resolver;
pub mod scope;
#[cfg(test)]
mod tests;
