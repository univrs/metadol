//! Runtime values for DOL expression evaluation.
//!
//! This module defines the value representation used during expression
//! evaluation, including primitives, functions, quoted AST, and reflection data.

use crate::ast::Expr;
use std::collections::HashMap;
use std::fmt;

/// Runtime value representation.
///
/// Values are the results of evaluating expressions at runtime.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Void/unit value (no meaningful result)
    Void,

    /// Boolean value
    Bool(bool),

    /// Signed 64-bit integer
    Int(i64),

    /// 64-bit floating point number
    Float(f64),

    /// String value
    String(String),

    /// Quoted AST (captured expression for metaprogramming)
    Quoted(Box<Expr>),

    /// Function closure
    Function {
        /// Parameter names
        params: Vec<String>,
        /// Function body expression
        body: Box<Expr>,
        /// Captured environment (closure)
        env: Environment,
    },

    /// Built-in function reference
    Builtin(String),

    /// Type metadata from reflection
    TypeInfo {
        /// Type name
        name: String,
        /// Type kind (e.g., "struct", "enum", "primitive")
        kind: String,
        /// Fields/members (name, type)
        fields: Vec<(String, String)>,
    },

    /// Array/list of values
    Array(Vec<Value>),

    /// Record/struct (key-value pairs)
    Record(HashMap<String, Value>),
}

impl Value {
    /// Returns true if this is a truthy value.
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Void => false,
            Value::Int(0) => false,
            Value::Float(f) if *f == 0.0 => false,
            Value::String(s) if s.is_empty() => false,
            Value::Array(a) if a.is_empty() => false,
            _ => true,
        }
    }

    /// Attempts to convert this value to an integer.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(n) => Some(*n),
            Value::Float(f) => Some(*f as i64),
            Value::Bool(true) => Some(1),
            Value::Bool(false) => Some(0),
            _ => None,
        }
    }

    /// Attempts to convert this value to a float.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Int(n) => Some(*n as f64),
            _ => None,
        }
    }

    /// Attempts to convert this value to a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the type name of this value.
    pub fn type_name(&self) -> &str {
        match self {
            Value::Void => "Void",
            Value::Bool(_) => "Bool",
            Value::Int(_) => "Int",
            Value::Float(_) => "Float",
            Value::String(_) => "String",
            Value::Quoted(_) => "Quoted",
            Value::Function { .. } => "Function",
            Value::Builtin(_) => "Builtin",
            Value::TypeInfo { .. } => "TypeInfo",
            Value::Array(_) => "Array",
            Value::Record(_) => "Record",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Void => write!(f, "()"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Quoted(_) => write!(f, "'<expr>"),
            Value::Function { params, .. } => {
                write!(f, "<function({})>", params.join(", "))
            }
            Value::Builtin(name) => write!(f, "<builtin:{}>", name),
            Value::TypeInfo { name, .. } => write!(f, "<type:{}>", name),
            Value::Array(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Record(fields) => {
                write!(f, "{{")?;
                for (i, (k, v)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

/// Runtime environment for variable bindings.
///
/// Environments form a tree structure through parent pointers,
/// supporting lexical scoping and closures.
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    /// Variable bindings in current scope
    bindings: HashMap<String, Value>,
    /// Parent scope (if any)
    parent: Option<Box<Environment>>,
}

impl Environment {
    /// Creates a new empty environment.
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    /// Creates a child environment with this as parent.
    pub fn child(&self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(Box::new(self.clone())),
        }
    }

    /// Binds a variable to a value in the current scope.
    pub fn bind(&mut self, name: impl Into<String>, value: Value) {
        self.bindings.insert(name.into(), value);
    }

    /// Looks up a variable's value, searching parent scopes if needed.
    pub fn lookup(&self, name: &str) -> Option<&Value> {
        self.bindings
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.lookup(name)))
    }

    /// Updates a variable's value in the scope where it's defined.
    pub fn update(&mut self, name: &str, value: Value) -> Result<(), EvalError> {
        if self.bindings.contains_key(name) {
            self.bindings.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.update(name, value)
        } else {
            Err(EvalError::undefined_variable(name))
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

/// Evaluation error.
#[derive(Debug, Clone, PartialEq)]
pub struct EvalError {
    /// Error message
    pub message: String,
}

impl EvalError {
    /// Creates a new evaluation error.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Creates a type error.
    pub fn type_error(expected: &str, actual: &str) -> Self {
        Self::new(format!(
            "type error: expected {}, found {}",
            expected, actual
        ))
    }

    /// Creates an undefined variable error.
    pub fn undefined_variable(name: &str) -> Self {
        Self::new(format!("undefined variable: {}", name))
    }

    /// Creates an arity mismatch error.
    pub fn arity_mismatch(expected: usize, actual: usize) -> Self {
        Self::new(format!(
            "arity mismatch: expected {} arguments, found {}",
            expected, actual
        ))
    }

    /// Creates a division by zero error.
    pub fn division_by_zero() -> Self {
        Self::new("division by zero")
    }

    /// Creates an invalid operation error.
    pub fn invalid_operation(op: &str, left: &str, right: &str) -> Self {
        Self::new(format!(
            "invalid operation: {} on types {} and {}",
            op, left, right
        ))
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for EvalError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_truthy() {
        assert!(Value::Bool(true).is_truthy());
        assert!(!Value::Bool(false).is_truthy());
        assert!(!Value::Void.is_truthy());
        assert!(!Value::Int(0).is_truthy());
        assert!(Value::Int(1).is_truthy());
        assert!(!Value::Float(0.0).is_truthy());
        assert!(Value::Float(1.5).is_truthy());
    }

    #[test]
    fn test_environment_binding() {
        let mut env = Environment::new();
        env.bind("x", Value::Int(42));

        assert_eq!(env.lookup("x"), Some(&Value::Int(42)));
        assert_eq!(env.lookup("y"), None);
    }

    #[test]
    fn test_environment_child() {
        let mut parent = Environment::new();
        parent.bind("x", Value::Int(1));

        let mut child = parent.child();
        child.bind("y", Value::Int(2));

        assert_eq!(child.lookup("x"), Some(&Value::Int(1)));
        assert_eq!(child.lookup("y"), Some(&Value::Int(2)));
        assert_eq!(parent.lookup("y"), None);
    }

    #[test]
    fn test_value_type_name() {
        assert_eq!(Value::Int(42).type_name(), "Int");
        assert_eq!(Value::Float(1.5).type_name(), "Float");
        assert_eq!(Value::Bool(true).type_name(), "Bool");
        assert_eq!(Value::String("hello".to_string()).type_name(), "String");
    }
}
