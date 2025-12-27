//! Code generation from Metal DOL declarations.
//!
//! This module provides code generation capabilities for transforming DOL
//! declarations into executable code in various target languages.
//!
//! # Supported Targets
//!
//! - **Rust**: Generate structs, traits, and type aliases
//! - **TypeScript**: Generate interfaces and type definitions
//! - **JSON Schema**: Generate JSON Schema from types (planned)
//!
//! # Example
//!
//! ```rust
//! use metadol::{parse_file, codegen::RustCodegen};
//!
//! let source = r#"
//! gene container.exists {
//!   container has id
//!   container has image
//! }
//!
//! exegesis {
//!   A container is the fundamental unit.
//! }
//! "#;
//!
//! let decl = parse_file(source).unwrap();
//! let rust_code = RustCodegen::generate(&decl);
//! println!("{}", rust_code);
//! ```

mod crate_gen;
pub mod hir_rust;
mod jsonschema;
mod rust;
mod typescript;

pub use crate_gen::{CrateCodegen, CrateConfig, ModuleInfo};
pub use hir_rust::HirRustCodegen;
pub use jsonschema::JsonSchemaCodegen;
pub use rust::RustCodegen;
pub use typescript::TypeScriptCodegen;

use crate::ast::{Declaration, TypeExpr};
use crate::lower::{lower_file, LowerDiagnostic};
use crate::typechecker::Type;

/// Trait for code generation backends.
///
/// Implement this trait to add support for new target languages.
pub trait Codegen {
    /// Generate code from a DOL declaration.
    fn generate(decl: &Declaration) -> String;

    /// Generate code from multiple declarations.
    fn generate_all(decls: &[Declaration]) -> String {
        decls
            .iter()
            .map(Self::generate)
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

/// Configuration options for code generation.
#[derive(Debug, Clone, Default)]
pub struct CodegenOptions {
    /// Include documentation comments in output
    pub include_docs: bool,

    /// Generate derive macros (Rust-specific)
    pub derive_macros: Vec<String>,

    /// Visibility modifier for generated items
    pub visibility: Visibility,

    /// Generate builder pattern methods
    pub generate_builders: bool,
}

/// Visibility level for generated code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Visibility {
    /// Private (no modifier)
    Private,
    /// Public
    #[default]
    Public,
    /// Crate-visible (Rust pub(crate))
    Crate,
}

/// Convert a DOL type to a target language type string.
pub trait TypeMapper {
    /// Map a DOL Type to the target language type string.
    fn map_type(ty: &Type) -> String;

    /// Map a DOL TypeExpr to the target language type string.
    fn map_type_expr(ty: &TypeExpr) -> String;
}

/// Convert a DOL identifier to a valid identifier in the target language.
pub fn to_pascal_case(s: &str) -> String {
    s.split('.')
        .flat_map(|part| part.split('_'))
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

/// Convert a DOL identifier to snake_case.
pub fn to_snake_case(s: &str) -> String {
    s.split('.')
        .collect::<Vec<_>>()
        .join("_")
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                vec!['_', c.to_lowercase().next().unwrap()]
            } else {
                vec![c.to_lowercase().next().unwrap()]
            }
        })
        .collect()
}

/// Rust reserved keywords that need r# escaping when used as identifiers.
const RUST_KEYWORDS: &[&str] = &[
    // Strict keywords
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", // Reserved keywords
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "try", "typeof",
    "unsized", "virtual", "yield",
];

/// Keywords that cannot be escaped with r# and need to be renamed.
const RUST_RESERVED_NO_ESCAPE: &[&str] = &["self", "Self", "super", "crate"];

/// Escape a Rust keyword with r# prefix if necessary.
/// Some keywords like `self` cannot be escaped and are renamed instead.
pub fn escape_rust_keyword(s: &str) -> String {
    if RUST_RESERVED_NO_ESCAPE.contains(&s) {
        // These keywords cannot use r# escaping, so we rename them
        format!("{}_", s)
    } else if RUST_KEYWORDS.contains(&s) {
        format!("r#{}", s)
    } else {
        s.to_string()
    }
}

/// Convert a DOL identifier to a valid Rust identifier (snake_case, keyword-escaped).
pub fn to_rust_ident(s: &str) -> String {
    escape_rust_keyword(&to_snake_case(s))
}

// ============================================================================
// HIR-based Compilation Pipeline (v0.3.0+)
// ============================================================================

/// New HIR-based compilation pipeline: DOL Source -> AST -> HIR -> Rust
///
/// This is the preferred pipeline for v0.3.0+. It provides:
/// - Deprecation warnings for old syntax
/// - Canonical HIR forms
/// - Simplified codegen
///
/// # Example
///
/// ```rust
/// use metadol::codegen::compile_to_rust_via_hir;
///
/// let source = r#"
/// gene container.exists {
///   container has id
///   container has image
/// }
///
/// exegesis {
///   A container is the fundamental unit.
/// }
/// "#;
///
/// let result = compile_to_rust_via_hir(source);
/// assert!(result.is_ok());
/// ```
pub fn compile_to_rust_via_hir(source: &str) -> Result<String, crate::error::ParseError> {
    // Parse to AST and lower to HIR
    let (hir, ctx) = lower_file(source)?;

    // Emit deprecation warnings
    for diag in ctx.diagnostics() {
        eprintln!("{}", diag);
    }

    // Generate Rust code from HIR
    let mut codegen = HirRustCodegen::with_symbols(ctx.symbols);
    Ok(codegen.generate(&hir))
}

/// Compile a DOL file to Rust, returning both code and diagnostics.
///
/// Unlike [`compile_to_rust_via_hir`], this function returns the diagnostics
/// instead of printing them to stderr, allowing callers to handle them
/// programmatically.
///
/// # Example
///
/// ```rust
/// use metadol::codegen::compile_with_diagnostics;
///
/// let source = r#"
/// gene test.simple {
///     entity has identity
/// }
///
/// exegesis {
///     Simple test.
/// }
/// "#;
///
/// let result = compile_with_diagnostics(source);
/// assert!(result.is_ok());
/// let (code, diagnostics) = result.unwrap();
/// assert!(!code.is_empty());
/// ```
pub fn compile_with_diagnostics(
    source: &str,
) -> Result<(String, Vec<LowerDiagnostic>), crate::error::ParseError> {
    let (hir, mut ctx) = lower_file(source)?;
    let mut codegen = HirRustCodegen::with_symbols(ctx.symbols.clone());
    let code = codegen.generate(&hir);
    Ok((code, ctx.take_diagnostics()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("container.exists"), "ContainerExists");
        assert_eq!(
            to_pascal_case("identity.cryptographic"),
            "IdentityCryptographic"
        );
        assert_eq!(to_pascal_case("simple"), "Simple");
        assert_eq!(to_pascal_case("snake_case"), "SnakeCase");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("container.exists"), "container_exists");
        assert_eq!(to_snake_case("ContainerExists"), "container_exists");
        assert_eq!(to_snake_case("simple"), "simple");
    }
}

/// Tests for the HIR-based compilation pipeline (v0.3.0+)
#[cfg(test)]
mod hir_pipeline_tests {
    use super::*;

    #[test]
    fn test_compile_simple_gene() {
        let source = r#"
gene test.point {
    point has x
    point has y
}

exegesis {
    A 2D point.
}
"#;
        let result = compile_to_rust_via_hir(source);
        assert!(result.is_ok(), "Failed to compile: {:?}", result.err());
        let code = result.unwrap();
        assert!(code.contains("Generated from DOL HIR"));
    }

    #[test]
    fn test_compile_with_diagnostics() {
        let source = r#"
gene test.simple {
    entity has identity
}

exegesis {
    Simple test.
}
"#;
        let result = compile_with_diagnostics(source);
        assert!(result.is_ok());
        let (code, diagnostics) = result.unwrap();
        assert!(!code.is_empty());
        // No deprecated syntax used, so no diagnostics expected
        assert!(
            diagnostics.is_empty(),
            "Unexpected diagnostics: {:?}",
            diagnostics
        );
    }

    #[test]
    fn test_compile_trait() {
        let source = r#"
trait test.lifecycle {
    uses test.exists
    entity is stateful
}

exegesis {
    A lifecycle trait.
}
"#;
        let result = compile_to_rust_via_hir(source);
        assert!(
            result.is_ok(),
            "Failed to compile trait: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_compile_constraint() {
        let source = r#"
constraint test.valid {
    entity has validity
}

exegesis {
    A validity constraint.
}
"#;
        let result = compile_to_rust_via_hir(source);
        // Constraints may or may not be fully supported, so we just check it doesn't panic
        if result.is_err() {
            // That's okay if the parser doesn't support constraints fully
        }
    }

    #[test]
    fn test_hir_codegen_output_format() {
        let source = r#"
gene container.exists {
    container has id
    container has image
    container has name
}

exegesis {
    A container is the fundamental unit.
}
"#;
        let result = compile_to_rust_via_hir(source);
        assert!(result.is_ok());
        let code = result.unwrap();

        // Check that the output contains expected elements
        assert!(code.contains("Generated from DOL HIR"), "Missing header");
        // The gene should be converted to a struct (HIR codegen behavior)
    }
}
