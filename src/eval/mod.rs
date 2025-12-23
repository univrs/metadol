//! Expression evaluation for Metal DOL.
//!
//! This module provides runtime evaluation of DOL 2.0 expressions,
//! supporting functional programming features including first-class
//! functions, closures, quote/eval metaprogramming, and reflection.
//!
//! # Example
//!
//! ```rust
//! use metadol::eval::Interpreter;
//! use metadol::ast::{Expr, Literal};
//!
//! let mut interpreter = Interpreter::new();
//! let expr = Expr::Literal(Literal::Int(42));
//! let result = interpreter.eval(&expr).unwrap();
//! ```

pub mod builtins;
pub mod interpreter;
pub mod value;

pub use interpreter::Interpreter;
pub use value::{Environment, EvalError, Value};
