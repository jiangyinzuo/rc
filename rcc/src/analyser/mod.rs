//! Semantic analysis includes:
//! - Symbol resolving
//! - Type check
//! - Flow control check
//!

pub mod sym_resolver;
pub mod scope;
#[cfg(test)]
mod tests;
