//! HIR - High-level Intermediate Representation
//!
//! Canonical forms for DOL compilation.
//! All surface syntax desugars to these 22 node types.

pub mod desugar;
pub mod types;
pub mod validate;

pub use desugar::desugar;
pub use types::*;
pub use validate::validate;
