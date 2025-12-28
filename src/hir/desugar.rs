//! AST → HIR Desugaring
//!
//! Transforms DOL surface syntax to canonical HIR forms.

use super::types::*;
use crate::ast::{Declaration, Expr, Stmt};

/// Desugar AST to HIR
pub fn desugar(ast: &[Declaration]) -> Result<Vec<HirNode>, DesugarError> {
    // TODO: Implement desugaring rules
    todo!("Implement AST → HIR desugaring")
}

#[derive(Debug, Clone)]
pub enum DesugarError {
    UnsupportedConstruct(String),
    InvalidSyntax(String),
}

impl std::fmt::Display for DesugarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DesugarError::UnsupportedConstruct(s) => write!(f, "unsupported: {}", s),
            DesugarError::InvalidSyntax(s) => write!(f, "invalid syntax: {}", s),
        }
    }
}

impl std::error::Error for DesugarError {}
