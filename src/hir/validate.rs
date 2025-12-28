//! HIR Validation
//!
//! Type checking and semantic validation of HIR.

use super::types::*;

/// Validate HIR for type correctness
pub fn validate(hir: &[HirNode]) -> Result<(), ValidationError> {
    // TODO: Implement validation
    todo!("Implement HIR validation")
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    UndefinedVariable { name: String },
    TypeMismatch { expected: String, found: String },
    MissingReturn { function: String },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::UndefinedVariable { name } => {
                write!(f, "undefined variable: {}", name)
            }
            ValidationError::TypeMismatch { expected, found } => {
                write!(f, "type mismatch: expected {}, found {}", expected, found)
            }
            ValidationError::MissingReturn { function } => {
                write!(f, "missing return in function: {}", function)
            }
        }
    }
}

impl std::error::Error for ValidationError {}
