//! Macro system for Metal DOL 2.0.
//!
//! This module provides compile-time metaprogramming through macros,
//! enabling code generation and transformation during parsing.
//!
//! # Macro Types
//!
//! DOL supports two forms of macros:
//!
//! 1. **Attribute macros**: Applied to declarations using `#[macro_name(args)]`
//! 2. **Expression macros**: Used inline with `#macro_name(args)`
//!
//! # Built-in Macros
//!
//! - `#derive`: Generate trait implementations
//! - `#stringify`: Convert expression to string literal
//! - `#concat`: Concatenate string literals
//! - `#env`: Access environment variables
//! - `#cfg`: Conditional compilation
//!
//! # Example
//!
//! ```dol
//! #[derive(Debug, Clone)]
//! gene container.exists {
//!   container has identity
//! }
//!
//! // Expression macro usage
//! let name = #stringify(container.exists);
//! let combined = #concat("prefix_", name);
//! ```
//!
//! # Custom Macros
//!
//! Implement the [`Macro`] trait to create custom macros:
//!
//! ```rust,ignore
//! use metadol::macros::{Macro, MacroInput, MacroOutput, MacroError, MacroContext};
//!
//! struct MyMacro;
//!
//! impl Macro for MyMacro {
//!     fn name(&self) -> &str { "my_macro" }
//!
//!     fn expand(&self, input: MacroInput, ctx: &MacroContext) -> Result<MacroOutput, MacroError> {
//!         // Transform input into output
//!         Ok(MacroOutput::Expr(Box::new(input.as_expr().clone())))
//!     }
//! }
//! ```

pub mod builtin;
pub mod expand;

use crate::ast::{Declaration, Expr, Span, Stmt, TypeExpr};
use std::collections::HashMap;
use std::fmt;

/// Error type for macro operations.
#[derive(Debug, Clone, PartialEq)]
pub struct MacroError {
    /// Error message
    pub message: String,
    /// Source location where the error occurred
    pub span: Option<Span>,
}

impl MacroError {
    /// Creates a new macro error.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span: None,
        }
    }

    /// Creates a macro error with source location.
    pub fn with_span(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span: Some(span),
        }
    }

    /// Creates an undefined macro error.
    pub fn undefined(name: &str) -> Self {
        Self::new(format!("undefined macro: #{}", name))
    }

    /// Creates an invalid argument error.
    pub fn invalid_argument(msg: &str) -> Self {
        Self::new(format!("invalid macro argument: {}", msg))
    }

    /// Creates an arity mismatch error.
    pub fn arity_mismatch(name: &str, expected: usize, actual: usize) -> Self {
        Self::new(format!(
            "macro #{} expects {} argument(s), got {}",
            name, expected, actual
        ))
    }

    /// Creates a type error.
    pub fn type_error(expected: &str, actual: &str) -> Self {
        Self::new(format!(
            "macro type error: expected {}, got {}",
            expected, actual
        ))
    }
}

impl fmt::Display for MacroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(span) = &self.span {
            write!(
                f,
                "macro error at line {}, column {}: {}",
                span.line, span.column, self.message
            )
        } else {
            write!(f, "macro error: {}", self.message)
        }
    }
}

impl std::error::Error for MacroError {}

/// Input to a macro invocation.
///
/// Represents the various forms of input a macro can receive.
#[derive(Debug, Clone, PartialEq)]
pub enum MacroInput {
    /// Empty input (no arguments)
    Empty,

    /// Single expression argument
    Expr(Box<Expr>),

    /// Multiple expression arguments
    ExprList(Vec<Expr>),

    /// Token stream (raw tokens for procedural macros)
    Tokens(Vec<MacroToken>),

    /// Declaration being annotated (for attribute macros)
    Declaration(Box<Declaration>),

    /// Key-value pairs (for configuration macros)
    Config(HashMap<String, MacroValue>),

    /// Identifier argument
    Ident(String),

    /// List of identifiers
    IdentList(Vec<String>),
}

impl MacroInput {
    /// Creates an empty input.
    pub fn empty() -> Self {
        MacroInput::Empty
    }

    /// Creates an expression input.
    pub fn expr(expr: Expr) -> Self {
        MacroInput::Expr(Box::new(expr))
    }

    /// Creates an expression list input.
    pub fn expr_list(exprs: Vec<Expr>) -> Self {
        MacroInput::ExprList(exprs)
    }

    /// Creates an identifier input.
    pub fn ident(name: impl Into<String>) -> Self {
        MacroInput::Ident(name.into())
    }

    /// Creates an identifier list input.
    pub fn ident_list(names: Vec<String>) -> Self {
        MacroInput::IdentList(names)
    }

    /// Returns true if the input is empty.
    pub fn is_empty(&self) -> bool {
        matches!(self, MacroInput::Empty)
    }

    /// Attempts to get the input as a single expression.
    pub fn as_expr(&self) -> Option<&Expr> {
        match self {
            MacroInput::Expr(e) => Some(e),
            _ => None,
        }
    }

    /// Attempts to get the input as an expression list.
    pub fn as_expr_list(&self) -> Option<&[Expr]> {
        match self {
            MacroInput::ExprList(list) => Some(list),
            MacroInput::Expr(_) => None, // Single expr is not a list
            _ => None,
        }
    }

    /// Attempts to get the input as an identifier.
    pub fn as_ident(&self) -> Option<&str> {
        match self {
            MacroInput::Ident(s) => Some(s),
            _ => None,
        }
    }

    /// Attempts to get the input as an identifier list.
    pub fn as_ident_list(&self) -> Option<&[String]> {
        match self {
            MacroInput::IdentList(list) => Some(list),
            _ => None,
        }
    }

    /// Returns the number of arguments.
    pub fn arg_count(&self) -> usize {
        match self {
            MacroInput::Empty => 0,
            MacroInput::Expr(_) => 1,
            MacroInput::ExprList(list) => list.len(),
            MacroInput::Tokens(tokens) => tokens.len(),
            MacroInput::Declaration(_) => 1,
            MacroInput::Config(map) => map.len(),
            MacroInput::Ident(_) => 1,
            MacroInput::IdentList(list) => list.len(),
        }
    }
}

/// A token in a macro token stream.
#[derive(Debug, Clone, PartialEq)]
pub struct MacroToken {
    /// Token kind
    pub kind: MacroTokenKind,
    /// Token text
    pub text: String,
    /// Source location
    pub span: Span,
}

/// Kind of macro token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroTokenKind {
    /// Identifier
    Ident,
    /// Keyword
    Keyword,
    /// Literal (string, number, etc.)
    Literal,
    /// Punctuation
    Punct,
    /// Delimiter
    Delim,
}

/// Value type for macro configuration.
#[derive(Debug, Clone, PartialEq)]
pub enum MacroValue {
    /// Boolean value
    Bool(bool),
    /// Integer value
    Int(i64),
    /// String value
    String(String),
    /// List of values
    List(Vec<MacroValue>),
}

impl MacroValue {
    /// Returns the value as a boolean, if it is one.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            MacroValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the value as a string, if it is one.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            MacroValue::String(s) => Some(s),
            _ => None,
        }
    }
}

/// Output from a macro expansion.
///
/// Represents the various forms of output a macro can produce.
#[derive(Debug, Clone, PartialEq)]
pub enum MacroOutput {
    /// No output (macro had side effect only)
    None,

    /// Single expression
    Expr(Box<Expr>),

    /// Multiple expressions
    ExprList(Vec<Expr>),

    /// Statement
    Stmt(Box<Stmt>),

    /// Multiple statements
    StmtList(Vec<Stmt>),

    /// Declaration (for attribute macros)
    Declaration(Box<Declaration>),

    /// Multiple declarations
    DeclarationList(Vec<Declaration>),

    /// Type expression
    Type(Box<TypeExpr>),

    /// Token stream (for procedural macros)
    Tokens(Vec<MacroToken>),
}

impl MacroOutput {
    /// Creates an empty output.
    pub fn none() -> Self {
        MacroOutput::None
    }

    /// Creates an expression output.
    pub fn expr(expr: Expr) -> Self {
        MacroOutput::Expr(Box::new(expr))
    }

    /// Creates an expression list output.
    pub fn expr_list(exprs: Vec<Expr>) -> Self {
        MacroOutput::ExprList(exprs)
    }

    /// Creates a statement output.
    pub fn stmt(stmt: Stmt) -> Self {
        MacroOutput::Stmt(Box::new(stmt))
    }

    /// Creates a declaration output.
    pub fn declaration(decl: Declaration) -> Self {
        MacroOutput::Declaration(Box::new(decl))
    }

    /// Returns true if the output is empty.
    pub fn is_none(&self) -> bool {
        matches!(self, MacroOutput::None)
    }

    /// Attempts to get the output as a single expression.
    pub fn as_expr(&self) -> Option<&Expr> {
        match self {
            MacroOutput::Expr(e) => Some(e),
            _ => None,
        }
    }

    /// Converts the output to an expression, if possible.
    pub fn into_expr(self) -> Option<Expr> {
        match self {
            MacroOutput::Expr(e) => Some(*e),
            _ => None,
        }
    }
}

/// Context provided to macros during expansion.
///
/// Contains information about the expansion environment that macros
/// may need to access.
#[derive(Debug, Clone)]
pub struct MacroContext {
    /// Current file being processed
    pub file_path: Option<String>,

    /// Line number of macro invocation
    pub line: usize,

    /// Column number of macro invocation
    pub column: usize,

    /// Environment variables available to macros
    pub env_vars: HashMap<String, String>,

    /// Configuration flags for conditional compilation
    pub cfg_flags: HashMap<String, bool>,

    /// Feature flags enabled
    pub features: Vec<String>,
}

impl MacroContext {
    /// Creates a new macro context.
    pub fn new() -> Self {
        Self {
            file_path: None,
            line: 0,
            column: 0,
            env_vars: std::env::vars().collect(),
            cfg_flags: HashMap::new(),
            features: Vec::new(),
        }
    }

    /// Creates a context with source location.
    pub fn with_location(file: Option<String>, line: usize, column: usize) -> Self {
        Self {
            file_path: file,
            line,
            column,
            ..Self::new()
        }
    }

    /// Sets a configuration flag.
    pub fn set_cfg(&mut self, key: impl Into<String>, value: bool) {
        self.cfg_flags.insert(key.into(), value);
    }

    /// Gets a configuration flag.
    pub fn get_cfg(&self, key: &str) -> bool {
        self.cfg_flags.get(key).copied().unwrap_or(false)
    }

    /// Adds a feature flag.
    pub fn add_feature(&mut self, feature: impl Into<String>) {
        self.features.push(feature.into());
    }

    /// Checks if a feature is enabled.
    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.iter().any(|f| f == feature)
    }

    /// Gets an environment variable.
    pub fn get_env(&self, key: &str) -> Option<&str> {
        self.env_vars.get(key).map(|s| s.as_str())
    }
}

impl Default for MacroContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for implementing macros.
///
/// Implement this trait to create custom macros for DOL.
///
/// # Example
///
/// ```rust,ignore
/// use metadol::macros::{Macro, MacroInput, MacroOutput, MacroError, MacroContext};
/// use metadol::ast::{Expr, Literal};
///
/// struct AnswerMacro;
///
/// impl Macro for AnswerMacro {
///     fn name(&self) -> &str { "answer" }
///
///     fn expand(&self, _input: MacroInput, _ctx: &MacroContext) -> Result<MacroOutput, MacroError> {
///         Ok(MacroOutput::expr(Expr::Literal(Literal::Int(42))))
///     }
/// }
/// ```
pub trait Macro: Send + Sync {
    /// Returns the name of this macro (without the `#` prefix).
    fn name(&self) -> &str;

    /// Expands the macro with the given input and context.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to the macro
    /// * `ctx` - The expansion context
    ///
    /// # Returns
    ///
    /// The expanded output, or an error if expansion fails.
    fn expand(&self, input: MacroInput, ctx: &MacroContext) -> Result<MacroOutput, MacroError>;

    /// Returns a description of this macro for documentation.
    fn description(&self) -> &str {
        ""
    }

    /// Returns whether this macro can be used as an attribute macro.
    fn is_attribute_macro(&self) -> bool {
        false
    }

    /// Returns whether this macro can be used as an expression macro.
    fn is_expr_macro(&self) -> bool {
        true
    }

    /// Returns the minimum number of arguments this macro accepts.
    fn min_args(&self) -> usize {
        0
    }

    /// Returns the maximum number of arguments this macro accepts.
    /// Returns None if there's no maximum.
    fn max_args(&self) -> Option<usize> {
        None
    }

    /// Validates the input before expansion.
    fn validate(&self, input: &MacroInput) -> Result<(), MacroError> {
        let count = input.arg_count();
        let min = self.min_args();

        if count < min {
            return Err(MacroError::arity_mismatch(self.name(), min, count));
        }

        if let Some(max) = self.max_args() {
            if count > max {
                return Err(MacroError::arity_mismatch(self.name(), max, count));
            }
        }

        Ok(())
    }
}

/// Macro invocation AST node.
///
/// Represents a macro call in the source code.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MacroInvocation {
    /// Macro name (without #)
    pub name: String,
    /// Arguments to the macro
    pub args: Vec<Expr>,
    /// Source location
    pub span: Span,
}

impl MacroInvocation {
    /// Creates a new macro invocation.
    pub fn new(name: impl Into<String>, args: Vec<Expr>, span: Span) -> Self {
        Self {
            name: name.into(),
            args,
            span,
        }
    }

    /// Creates a macro invocation with no arguments.
    pub fn simple(name: impl Into<String>, span: Span) -> Self {
        Self::new(name, Vec::new(), span)
    }
}

/// Attribute macro on a declaration.
///
/// Represents a `#[macro_name(args)]` annotation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MacroAttribute {
    /// Macro name (without #)
    pub name: String,
    /// Arguments as identifiers or key-value pairs
    pub args: Vec<AttributeArg>,
    /// Source location
    pub span: Span,
}

/// Argument to an attribute macro.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeArg {
    /// Simple identifier: `Debug`
    Ident(String),
    /// Key-value pair: `name = "value"`
    KeyValue {
        /// The key name
        key: String,
        /// The value expression
        value: Expr,
    },
    /// Nested attribute: `serde(rename = "foo")`
    Nested {
        /// The nested attribute name
        name: String,
        /// The nested arguments
        args: Vec<AttributeArg>,
    },
}

impl MacroAttribute {
    /// Creates a new macro attribute.
    pub fn new(name: impl Into<String>, args: Vec<AttributeArg>, span: Span) -> Self {
        Self {
            name: name.into(),
            args,
            span,
        }
    }

    /// Creates an attribute with no arguments.
    pub fn simple(name: impl Into<String>, span: Span) -> Self {
        Self::new(name, Vec::new(), span)
    }

    /// Creates an attribute with identifier arguments.
    pub fn with_idents(name: impl Into<String>, idents: Vec<String>, span: Span) -> Self {
        let args = idents.into_iter().map(AttributeArg::Ident).collect();
        Self::new(name, args, span)
    }
}

// Re-export commonly used items
pub use builtin::BuiltinMacros;
pub use expand::MacroExpander;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Literal;

    #[test]
    fn test_macro_error() {
        let err = MacroError::undefined("test");
        assert!(err.message.contains("undefined macro"));

        let err = MacroError::arity_mismatch("test", 2, 1);
        assert!(err.message.contains("expects 2 argument(s)"));
    }

    #[test]
    fn test_macro_input() {
        let input = MacroInput::empty();
        assert!(input.is_empty());
        assert_eq!(input.arg_count(), 0);

        let input = MacroInput::expr(Expr::Literal(Literal::Int(42)));
        assert!(!input.is_empty());
        assert_eq!(input.arg_count(), 1);
        assert!(input.as_expr().is_some());
    }

    #[test]
    fn test_macro_output() {
        let output = MacroOutput::none();
        assert!(output.is_none());

        let output = MacroOutput::expr(Expr::Literal(Literal::String("hello".to_string())));
        assert!(!output.is_none());
        assert!(output.as_expr().is_some());
    }

    #[test]
    fn test_macro_context() {
        let mut ctx = MacroContext::new();

        ctx.set_cfg("debug", true);
        assert!(ctx.get_cfg("debug"));
        assert!(!ctx.get_cfg("release"));

        ctx.add_feature("async");
        assert!(ctx.has_feature("async"));
        assert!(!ctx.has_feature("sync"));
    }

    #[test]
    fn test_macro_invocation() {
        let invoc = MacroInvocation::simple("stringify", Span::default());
        assert_eq!(invoc.name, "stringify");
        assert!(invoc.args.is_empty());
    }

    #[test]
    fn test_macro_attribute() {
        let attr = MacroAttribute::with_idents(
            "derive",
            vec!["Debug".to_string(), "Clone".to_string()],
            Span::default(),
        );
        assert_eq!(attr.name, "derive");
        assert_eq!(attr.args.len(), 2);
    }
}
