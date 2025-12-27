//! Lowering context for AST -> HIR conversion

use super::{DiagnosticKind, LowerDiagnostic};
use crate::ast;
use crate::hir::{HirId, Symbol, SymbolTable};

/// Context for lowering AST to HIR
pub struct LoweringContext {
    /// Symbol table for interning identifiers
    pub symbols: SymbolTable,
    /// Next HIR ID to assign
    next_id: u32,
    /// Diagnostics collected during lowering
    diagnostics: Vec<LowerDiagnostic>,
}

impl LoweringContext {
    /// Create a new lowering context
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::new(),
            next_id: 0,
            diagnostics: Vec::new(),
        }
    }

    /// Generate a fresh HIR ID
    pub fn fresh_id(&mut self) -> HirId {
        self.next_id += 1;
        HirId::new()
    }

    /// Intern a string as a symbol
    pub fn intern(&mut self, s: &str) -> Symbol {
        self.symbols.intern(s)
    }

    /// Resolve a symbol back to its string representation
    pub fn resolve(&self, sym: Symbol) -> Option<&str> {
        self.symbols.resolve(sym)
    }

    /// Emit a deprecation warning
    pub fn emit_deprecation(&mut self, old: &str, new: &str, span: ast::Span) {
        self.diagnostics.push(LowerDiagnostic {
            kind: DiagnosticKind::Deprecation,
            message: format!("'{}' is deprecated", old),
            span: Some(span),
            suggestion: Some(format!("use '{}' instead", new)),
        });
    }

    /// Emit a warning
    pub fn emit_warning(&mut self, message: &str, span: Option<ast::Span>) {
        self.diagnostics.push(LowerDiagnostic {
            kind: DiagnosticKind::Warning,
            message: message.to_string(),
            span,
            suggestion: None,
        });
    }

    /// Emit an error
    pub fn emit_error(&mut self, message: &str, span: Option<ast::Span>) {
        self.diagnostics.push(LowerDiagnostic {
            kind: DiagnosticKind::Error,
            message: message.to_string(),
            span,
            suggestion: None,
        });
    }

    /// Get all diagnostics
    pub fn diagnostics(&self) -> &[LowerDiagnostic] {
        &self.diagnostics
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.kind == DiagnosticKind::Error)
    }

    /// Take diagnostics, consuming them
    pub fn take_diagnostics(&mut self) -> Vec<LowerDiagnostic> {
        std::mem::take(&mut self.diagnostics)
    }
}

impl Default for LoweringContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let ctx = LoweringContext::new();
        assert!(!ctx.has_errors());
        assert!(ctx.diagnostics().is_empty());
    }

    #[test]
    fn test_symbol_interning() {
        let mut ctx = LoweringContext::new();
        let foo1 = ctx.intern("foo");
        let foo2 = ctx.intern("foo");
        assert_eq!(foo1, foo2);
    }

    #[test]
    fn test_fresh_id() {
        let mut ctx = LoweringContext::new();
        let id1 = ctx.fresh_id();
        let id2 = ctx.fresh_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_deprecation_warning() {
        let mut ctx = LoweringContext::new();
        ctx.emit_deprecation("let", "val", ast::Span::default());
        assert_eq!(ctx.diagnostics().len(), 1);
        assert_eq!(ctx.diagnostics()[0].kind, DiagnosticKind::Deprecation);
    }

    #[test]
    fn test_error_detection() {
        let mut ctx = LoweringContext::new();
        assert!(!ctx.has_errors());

        ctx.emit_warning("just a warning", None);
        assert!(!ctx.has_errors());

        ctx.emit_error("an error occurred", None);
        assert!(ctx.has_errors());
    }

    #[test]
    fn test_take_diagnostics() {
        let mut ctx = LoweringContext::new();
        ctx.emit_warning("warning 1", None);
        ctx.emit_error("error 1", None);

        let diags = ctx.take_diagnostics();
        assert_eq!(diags.len(), 2);
        assert!(ctx.diagnostics().is_empty());
    }

    #[test]
    fn test_symbol_resolution() {
        let mut ctx = LoweringContext::new();
        let sym = ctx.intern("test_symbol");
        assert_eq!(ctx.resolve(sym), Some("test_symbol"));
    }
}
