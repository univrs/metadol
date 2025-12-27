//! Statement lowering
//!
//! Converts AST statements to HIR statements.
//! Handles val/var bindings with deprecation warnings for let/mut.
//!
//! # Desugaring Rules
//!
//! ## Bindings
//! - `let x = e` -> `HirStmt::Val` (deprecated, emits warning)
//! - `val x = e` -> `HirStmt::Val` (preferred in v0.3.0)
//! - `let mut x = e` -> `HirStmt::Var` (deprecated)
//! - `var x = e` -> `HirStmt::Var` (preferred in v0.3.0)
//!
//! ## Loops
//! - `for x in xs { body }` -> `loop { match iter.next() { Some(x) => body, None => break } }`
//! - `while cond { body }` -> `loop { if cond { body } else { break } }`

use super::LoweringContext;
use crate::hir::*;

impl LoweringContext {
    /// Lower a statement (placeholder)
    ///
    /// This will eventually convert AST statements to HIR statements,
    /// applying all desugaring rules including loop transformations.
    pub fn lower_stmt(&mut self) -> HirStmt {
        HirStmt::Expr(self.lower_expr())
    }

    /// Lower a DOL statement (from gene/trait bodies)
    pub fn lower_dol_statement(&mut self, stmt: &crate::ast::Statement) -> HirStatement {
        let id = self.fresh_id();
        let kind = match stmt {
            crate::ast::Statement::Has {
                subject, property, ..
            } => HirStatementKind::Has {
                subject: self.intern(subject),
                property: self.intern(property),
            },
            crate::ast::Statement::Is { subject, state, .. } => HirStatementKind::Is {
                subject: self.intern(subject),
                type_name: self.intern(state),
            },
            crate::ast::Statement::DerivesFrom {
                subject, origin, ..
            } => HirStatementKind::DerivesFrom {
                subject: self.intern(subject),
                parent: self.intern(origin),
            },
            crate::ast::Statement::Requires {
                subject,
                requirement,
                ..
            } => HirStatementKind::Requires {
                subject: self.intern(subject),
                dependency: self.intern(requirement),
            },
            crate::ast::Statement::Uses { reference, .. } => {
                // For uses statements, we treat the reference as both subject and resource
                HirStatementKind::Uses {
                    subject: self.intern("self"),
                    resource: self.intern(reference),
                }
            }
            crate::ast::Statement::Emits { action, event, .. } => {
                // Map emits to uses for now (simplified)
                HirStatementKind::Uses {
                    subject: self.intern(action),
                    resource: self.intern(event),
                }
            }
            crate::ast::Statement::Matches {
                subject, target, ..
            } => {
                // Map matches to requires for now (simplified)
                HirStatementKind::Requires {
                    subject: self.intern(subject),
                    dependency: self.intern(target),
                }
            }
            crate::ast::Statement::Never {
                subject, action, ..
            } => {
                // Map never to requires with negation marker (simplified)
                HirStatementKind::Requires {
                    subject: self.intern(subject),
                    dependency: self.intern(&format!("!{}", action)),
                }
            }
            crate::ast::Statement::Quantified { phrase, .. } => {
                // Map quantified to has for now (simplified)
                HirStatementKind::Has {
                    subject: self.intern("quantified"),
                    property: self.intern(phrase),
                }
            }
            crate::ast::Statement::HasField(field) => HirStatementKind::Has {
                subject: self.intern("self"),
                property: self.intern(&field.name),
            },
            crate::ast::Statement::Function(func) => {
                // Functions in statement position become method declarations
                HirStatementKind::Has {
                    subject: self.intern("self"),
                    property: self.intern(&func.name),
                }
            }
        };

        HirStatement { id, kind }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;

    #[test]
    fn test_lower_has_statement() {
        let mut ctx = LoweringContext::new();
        let stmt = ast::Statement::Has {
            subject: "container".to_string(),
            property: "identity".to_string(),
            span: ast::Span::default(),
        };

        let hir_stmt = ctx.lower_dol_statement(&stmt);
        match hir_stmt.kind {
            HirStatementKind::Has { subject, property } => {
                assert_eq!(ctx.resolve(subject), Some("container"));
                assert_eq!(ctx.resolve(property), Some("identity"));
            }
            _ => panic!("Expected Has statement"),
        }
    }

    #[test]
    fn test_lower_is_statement() {
        let mut ctx = LoweringContext::new();
        let stmt = ast::Statement::Is {
            subject: "container".to_string(),
            state: "created".to_string(),
            span: ast::Span::default(),
        };

        let hir_stmt = ctx.lower_dol_statement(&stmt);
        match hir_stmt.kind {
            HirStatementKind::Is { subject, type_name } => {
                assert_eq!(ctx.resolve(subject), Some("container"));
                assert_eq!(ctx.resolve(type_name), Some("created"));
            }
            _ => panic!("Expected Is statement"),
        }
    }

    #[test]
    fn test_lower_derives_from_statement() {
        let mut ctx = LoweringContext::new();
        let stmt = ast::Statement::DerivesFrom {
            subject: "identity".to_string(),
            origin: "keypair".to_string(),
            span: ast::Span::default(),
        };

        let hir_stmt = ctx.lower_dol_statement(&stmt);
        match hir_stmt.kind {
            HirStatementKind::DerivesFrom { subject, parent } => {
                assert_eq!(ctx.resolve(subject), Some("identity"));
                assert_eq!(ctx.resolve(parent), Some("keypair"));
            }
            _ => panic!("Expected DerivesFrom statement"),
        }
    }

    #[test]
    fn test_lower_requires_statement() {
        let mut ctx = LoweringContext::new();
        let stmt = ast::Statement::Requires {
            subject: "identity".to_string(),
            requirement: "no authority".to_string(),
            span: ast::Span::default(),
        };

        let hir_stmt = ctx.lower_dol_statement(&stmt);
        match hir_stmt.kind {
            HirStatementKind::Requires {
                subject,
                dependency,
            } => {
                assert_eq!(ctx.resolve(subject), Some("identity"));
                assert_eq!(ctx.resolve(dependency), Some("no authority"));
            }
            _ => panic!("Expected Requires statement"),
        }
    }

    #[test]
    fn test_lower_uses_statement() {
        let mut ctx = LoweringContext::new();
        let stmt = ast::Statement::Uses {
            reference: "container.exists".to_string(),
            span: ast::Span::default(),
        };

        let hir_stmt = ctx.lower_dol_statement(&stmt);
        match hir_stmt.kind {
            HirStatementKind::Uses { subject, resource } => {
                assert_eq!(ctx.resolve(subject), Some("self"));
                assert_eq!(ctx.resolve(resource), Some("container.exists"));
            }
            _ => panic!("Expected Uses statement"),
        }
    }
}
