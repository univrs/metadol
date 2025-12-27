//! Expression lowering
//!
//! Converts AST expressions to HIR expressions with desugaring.
//!
//! # Desugaring Rules
//!
//! ## Pipe Operator
//! - `x |> f |> g` -> `g(f(x))`
//!
//! ## Short-Circuit Operators
//! - `a && b` -> `if a { b } else { false }`
//! - `a || b` -> `if a { true } else { b }`
//!
//! ## Idiom Brackets
//! - `[| f a b |]` -> `f <$> a <*> b`

use super::LoweringContext;
use crate::hir::*;

impl LoweringContext {
    /// Lower an expression (placeholder)
    ///
    /// This will eventually convert AST expressions to HIR expressions,
    /// applying all desugaring rules.
    pub fn lower_expr(&mut self) -> HirExpr {
        HirExpr::Literal(HirLiteral::Unit)
    }

    /// Lower a literal value
    pub fn lower_literal(&mut self, lit: &crate::ast::Literal) -> HirLiteral {
        match lit {
            crate::ast::Literal::Int(i) => HirLiteral::Int(*i),
            crate::ast::Literal::Float(f) => HirLiteral::Float(*f),
            crate::ast::Literal::String(s) => HirLiteral::String(s.clone()),
            crate::ast::Literal::Bool(b) => HirLiteral::Bool(*b),
            crate::ast::Literal::Char(c) => HirLiteral::String(c.to_string()),
            crate::ast::Literal::Null => HirLiteral::Unit,
        }
    }

    /// Lower a binary operator
    pub fn lower_binary_op(&mut self, op: &crate::ast::BinaryOp) -> Option<HirBinaryOp> {
        match op {
            crate::ast::BinaryOp::Add => Some(HirBinaryOp::Add),
            crate::ast::BinaryOp::Sub => Some(HirBinaryOp::Sub),
            crate::ast::BinaryOp::Mul => Some(HirBinaryOp::Mul),
            crate::ast::BinaryOp::Div => Some(HirBinaryOp::Div),
            crate::ast::BinaryOp::Mod => Some(HirBinaryOp::Mod),
            crate::ast::BinaryOp::Eq => Some(HirBinaryOp::Eq),
            crate::ast::BinaryOp::Ne => Some(HirBinaryOp::Ne),
            crate::ast::BinaryOp::Lt => Some(HirBinaryOp::Lt),
            crate::ast::BinaryOp::Le => Some(HirBinaryOp::Le),
            crate::ast::BinaryOp::Gt => Some(HirBinaryOp::Gt),
            crate::ast::BinaryOp::Ge => Some(HirBinaryOp::Ge),
            crate::ast::BinaryOp::And => Some(HirBinaryOp::And),
            crate::ast::BinaryOp::Or => Some(HirBinaryOp::Or),
            // These operators require desugaring and don't have direct HIR equivalents
            crate::ast::BinaryOp::Pipe
            | crate::ast::BinaryOp::Compose
            | crate::ast::BinaryOp::Apply
            | crate::ast::BinaryOp::Bind
            | crate::ast::BinaryOp::Member
            | crate::ast::BinaryOp::Map
            | crate::ast::BinaryOp::Ap
            | crate::ast::BinaryOp::Implies
            | crate::ast::BinaryOp::Range
            | crate::ast::BinaryOp::Pow => None,
        }
    }

    /// Lower a unary operator
    pub fn lower_unary_op(&mut self, op: &crate::ast::UnaryOp) -> Option<HirUnaryOp> {
        match op {
            crate::ast::UnaryOp::Neg => Some(HirUnaryOp::Neg),
            crate::ast::UnaryOp::Not => Some(HirUnaryOp::Not),
            // These operators require special handling
            crate::ast::UnaryOp::Quote
            | crate::ast::UnaryOp::Reflect
            | crate::ast::UnaryOp::Deref => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;

    #[test]
    fn test_lower_literal_int() {
        let mut ctx = LoweringContext::new();
        let lit = ast::Literal::Int(42);
        let hir_lit = ctx.lower_literal(&lit);
        assert_eq!(hir_lit, HirLiteral::Int(42));
    }

    #[test]
    fn test_lower_literal_bool() {
        let mut ctx = LoweringContext::new();
        let lit = ast::Literal::Bool(true);
        let hir_lit = ctx.lower_literal(&lit);
        assert_eq!(hir_lit, HirLiteral::Bool(true));
    }

    #[test]
    fn test_lower_literal_string() {
        let mut ctx = LoweringContext::new();
        let lit = ast::Literal::String("hello".to_string());
        let hir_lit = ctx.lower_literal(&lit);
        assert_eq!(hir_lit, HirLiteral::String("hello".to_string()));
    }

    #[test]
    fn test_lower_binary_op() {
        let mut ctx = LoweringContext::new();
        assert_eq!(
            ctx.lower_binary_op(&ast::BinaryOp::Add),
            Some(HirBinaryOp::Add)
        );
        assert_eq!(
            ctx.lower_binary_op(&ast::BinaryOp::Eq),
            Some(HirBinaryOp::Eq)
        );
        // Pipe requires desugaring
        assert_eq!(ctx.lower_binary_op(&ast::BinaryOp::Pipe), None);
    }

    #[test]
    fn test_lower_unary_op() {
        let mut ctx = LoweringContext::new();
        assert_eq!(
            ctx.lower_unary_op(&ast::UnaryOp::Neg),
            Some(HirUnaryOp::Neg)
        );
        assert_eq!(
            ctx.lower_unary_op(&ast::UnaryOp::Not),
            Some(HirUnaryOp::Not)
        );
        // Quote requires special handling
        assert_eq!(ctx.lower_unary_op(&ast::UnaryOp::Quote), None);
    }
}
