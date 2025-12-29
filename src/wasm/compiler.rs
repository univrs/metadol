//! # WASM Compiler
//!
//! Compiles Metal DOL modules to WebAssembly bytecode.
//!
//! This implementation uses direct WASM emission via the `wasm-encoder` crate:
//!
//! ```text
//! DOL AST → WASM bytecode
//! ```
//!
//! For the full MLIR-based pipeline (DOL → MLIR → LLVM → WASM),
//! enable the `wasm-mlir` feature (requires LLVM 18 installed).
//!
//! ## Example
//!
//! ```rust,ignore
//! use metadol::wasm::WasmCompiler;
//! use metadol::parse_file;
//!
//! let source = r#"
//! gene container.exists {
//!   container has identity
//! }
//!
//! exegesis {
//!   A container exists.
//! }
//! "#;
//!
//! let module = parse_file(source)?;
//! let compiler = WasmCompiler::new()
//!     .with_optimization(true);
//!
//! let wasm_bytes = compiler.compile(&module)?;
//! ```

#[cfg(feature = "wasm")]
use crate::ast::Declaration;
#[cfg(feature = "wasm")]
use crate::wasm::WasmError;
#[cfg(feature = "wasm")]
use std::path::Path;
#[cfg(feature = "wasm")]
use wasm_encoder;

/// WASM compiler for Metal DOL modules.
///
/// The `WasmCompiler` transforms DOL declarations into WebAssembly bytecode.
/// It provides control over optimization levels and debug information.
///
/// # Example
///
/// ```rust,ignore
/// use metadol::wasm::WasmCompiler;
///
/// let compiler = WasmCompiler::new()
///     .with_optimization(true)
///     .with_debug_info(false);
/// ```
#[cfg(feature = "wasm")]
#[derive(Debug, Clone)]
pub struct WasmCompiler {
    /// Enable LLVM optimizations
    optimize: bool,
    /// Include debug information in WASM
    debug_info: bool,
}

#[cfg(feature = "wasm")]
impl WasmCompiler {
    /// Create a new WASM compiler with default settings.
    ///
    /// Default settings:
    /// - Optimization: disabled
    /// - Debug info: enabled
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use metadol::wasm::WasmCompiler;
    ///
    /// let compiler = WasmCompiler::new();
    /// ```
    pub fn new() -> Self {
        Self {
            optimize: false,
            debug_info: true,
        }
    }

    /// Enable or disable optimizations.
    ///
    /// When enabled, LLVM will run optimization passes on the IR before
    /// generating WASM bytecode. This produces smaller and faster WASM
    /// modules but increases compilation time.
    ///
    /// # Arguments
    ///
    /// * `optimize` - `true` to enable optimizations, `false` to disable
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use metadol::wasm::WasmCompiler;
    ///
    /// let compiler = WasmCompiler::new().with_optimization(true);
    /// ```
    pub fn with_optimization(mut self, optimize: bool) -> Self {
        self.optimize = optimize;
        self
    }

    /// Enable or disable debug information.
    ///
    /// When enabled, the WASM module will include source location information
    /// for debugging. This increases module size but improves debuggability.
    ///
    /// # Arguments
    ///
    /// * `debug_info` - `true` to include debug info, `false` to omit
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use metadol::wasm::WasmCompiler;
    ///
    /// let compiler = WasmCompiler::new().with_debug_info(false);
    /// ```
    pub fn with_debug_info(mut self, debug_info: bool) -> Self {
        self.debug_info = debug_info;
        self
    }

    /// Compile a DOL module to WASM bytecode.
    ///
    /// Takes a DOL declaration AST and transforms it to WebAssembly bytecode.
    /// This implementation uses direct WASM emission via wasm-encoder.
    ///
    /// # Arguments
    ///
    /// * `module` - The DOL module to compile
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the WASM bytecode on success, or a `WasmError`
    /// if compilation fails.
    ///
    /// # Supported Constructs
    ///
    /// - Function declarations with basic types (i32, i64, f32, f64)
    /// - Integer and float literals
    /// - Binary operations (add, sub, mul, div, etc.)
    /// - Function calls
    /// - Return statements
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use metadol::wasm::WasmCompiler;
    /// use metadol::parse_file;
    ///
    /// let source = r#"
    /// fun add(a: i64, b: i64) -> i64 {
    ///   return a + b
    /// }
    ///
    /// exegesis {
    ///   Adds two integers.
    /// }
    /// "#;
    ///
    /// let module = parse_file(source)?;
    /// let compiler = WasmCompiler::new();
    /// let wasm_bytes = compiler.compile(&module)?;
    /// ```
    pub fn compile(&self, module: &Declaration) -> Result<Vec<u8>, WasmError> {
        use wasm_encoder::{
            CodeSection, ExportKind, ExportSection, Function, FunctionSection, Module, TypeSection,
            ValType,
        };

        // Extract function declarations from the module
        let functions = self.extract_functions(module)?;

        if functions.is_empty() {
            return Err(WasmError::new(
                "No functions found in module - only function declarations are currently supported for WASM compilation",
            ));
        }

        // Build WASM module
        let mut wasm_module = Module::new();

        // Type section: function signatures
        let mut types = TypeSection::new();
        let mut type_indices = Vec::new();

        for func in &functions {
            let params: Vec<ValType> = func
                .params
                .iter()
                .map(|p| self.dol_type_to_wasm(&p.type_ann))
                .collect::<Result<Vec<_>, _>>()?;

            let results = if let Some(ref ret_type) = func.return_type {
                vec![self.dol_type_to_wasm(ret_type)?]
            } else {
                vec![]
            };

            types.function(params, results);
            type_indices.push(type_indices.len() as u32);
        }

        wasm_module.section(&types);

        // Function section: function indices
        let mut funcs = FunctionSection::new();
        for type_idx in &type_indices {
            funcs.function(*type_idx);
        }
        wasm_module.section(&funcs);

        // Export section: export all functions
        let mut exports = ExportSection::new();
        for (idx, func) in functions.iter().enumerate() {
            exports.export(&func.name, ExportKind::Func, idx as u32);
        }
        wasm_module.section(&exports);

        // Code section: function bodies
        let mut code = CodeSection::new();
        for func in &functions {
            let mut function = Function::new(vec![]); // No locals for now
            self.emit_function_body(&mut function, func)?;
            code.function(&function);
        }
        wasm_module.section(&code);

        Ok(wasm_module.finish())
    }

    /// Extract function declarations from a DOL module.
    ///
    /// Currently only supports top-level function declarations.
    fn extract_functions<'a>(
        &self,
        module: &'a Declaration,
    ) -> Result<Vec<&'a crate::ast::FunctionDecl>, WasmError> {
        use crate::ast::Declaration;

        match module {
            Declaration::Function(func) => Ok(vec![func.as_ref()]),
            _ => {
                // For now, we only support direct function declarations
                // Future: extract functions from Gene/Trait/System bodies
                Ok(vec![])
            }
        }
    }

    /// Convert a DOL type expression to a WASM value type.
    ///
    /// Maps DOL types to their WASM equivalents:
    /// - i32, i64 → i64
    /// - f32, f64 → f64
    /// - bool → i32
    fn dol_type_to_wasm(
        &self,
        type_expr: &crate::ast::TypeExpr,
    ) -> Result<wasm_encoder::ValType, WasmError> {
        use crate::ast::TypeExpr;
        use wasm_encoder::ValType;

        match type_expr {
            TypeExpr::Named(name) => match name.as_str() {
                "i32" | "i64" | "int" => Ok(ValType::I64),
                "f32" | "f64" | "float" => Ok(ValType::F64),
                "bool" => Ok(ValType::I32),
                other => Err(WasmError::new(format!(
                    "Unsupported type for WASM compilation: {}",
                    other
                ))),
            },
            TypeExpr::Generic { .. } => Err(WasmError::new(
                "Generic types not yet supported in WASM compilation",
            )),
            TypeExpr::Function { .. } => Err(WasmError::new(
                "Function types not yet supported in WASM compilation",
            )),
            TypeExpr::Tuple(_) => Err(WasmError::new(
                "Tuple types not yet supported in WASM compilation",
            )),
            TypeExpr::Enum { .. } => Err(WasmError::new(
                "Enum types not yet supported in WASM compilation",
            )),
            TypeExpr::Never => Err(WasmError::new(
                "Never type not supported in WASM compilation",
            )),
        }
    }

    /// Emit the body of a function as WASM instructions.
    fn emit_function_body(
        &self,
        function: &mut wasm_encoder::Function,
        func_decl: &crate::ast::FunctionDecl,
    ) -> Result<(), WasmError> {
        use wasm_encoder::Instruction;

        // Emit each statement in the function body
        for stmt in &func_decl.body {
            self.emit_statement(function, stmt, func_decl)?;
        }

        // If no explicit return, add an end instruction
        function.instruction(&Instruction::End);

        Ok(())
    }

    /// Emit a statement as WASM instructions.
    fn emit_statement(
        &self,
        function: &mut wasm_encoder::Function,
        stmt: &crate::ast::Stmt,
        func_decl: &crate::ast::FunctionDecl,
    ) -> Result<(), WasmError> {
        use crate::ast::Stmt;
        use wasm_encoder::Instruction;

        match stmt {
            Stmt::Return(expr_opt) => {
                if let Some(expr) = expr_opt {
                    self.emit_expression(function, expr, func_decl)?;
                }
                function.instruction(&Instruction::Return);
            }
            Stmt::Expr(expr) => {
                self.emit_expression(function, expr, func_decl)?;
                // Drop the result if it's an expression statement
                function.instruction(&Instruction::Drop);
            }
            Stmt::Let { .. } => {
                return Err(WasmError::new(
                    "Let bindings not yet supported in WASM compilation",
                ))
            }
            Stmt::Assign { .. } => {
                return Err(WasmError::new(
                    "Assignments not yet supported in WASM compilation",
                ))
            }
            Stmt::For { .. } | Stmt::While { .. } | Stmt::Loop { .. } => {
                return Err(WasmError::new(
                    "Loops not yet supported in WASM compilation",
                ))
            }
            Stmt::Break | Stmt::Continue => {
                return Err(WasmError::new(
                    "Break/continue not yet supported in WASM compilation",
                ))
            }
        }

        Ok(())
    }

    /// Emit an expression as WASM instructions.
    fn emit_expression(
        &self,
        function: &mut wasm_encoder::Function,
        expr: &crate::ast::Expr,
        func_decl: &crate::ast::FunctionDecl,
    ) -> Result<(), WasmError> {
        use crate::ast::{Expr, Literal};
        use wasm_encoder::Instruction;

        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Int(i) => {
                    function.instruction(&Instruction::I64Const(*i));
                }
                Literal::Float(f) => {
                    function.instruction(&Instruction::F64Const(*f));
                }
                Literal::Bool(b) => {
                    function.instruction(&Instruction::I32Const(if *b { 1 } else { 0 }));
                }
                Literal::String(_) => {
                    return Err(WasmError::new(
                        "String literals not yet supported in WASM compilation",
                    ))
                }
                Literal::Char(_) => {
                    return Err(WasmError::new(
                        "Char literals not yet supported in WASM compilation",
                    ))
                }
                Literal::Null => {
                    return Err(WasmError::new(
                        "Null literals not yet supported in WASM compilation",
                    ))
                }
            },
            Expr::Identifier(name) => {
                // Look up the parameter index
                let param_idx = func_decl
                    .params
                    .iter()
                    .position(|p| p.name == *name)
                    .ok_or_else(|| WasmError::new(format!("Unknown identifier: {}", name)))?;
                function.instruction(&Instruction::LocalGet(param_idx as u32));
            }
            Expr::Binary { left, op, right } => {
                // Emit left operand
                self.emit_expression(function, left, func_decl)?;
                // Emit right operand
                self.emit_expression(function, right, func_decl)?;
                // Emit operation
                self.emit_binary_op(function, *op)?;
            }
            Expr::Call { callee, args } => {
                // For now, only support direct function calls (not expressions)
                if let Expr::Identifier(_func_name) = callee.as_ref() {
                    // Emit arguments
                    for arg in args {
                        self.emit_expression(function, arg, func_decl)?;
                    }
                    // TODO: Look up function index - for now, assume index 0
                    // This is a simplification; proper implementation needs a symbol table
                    function.instruction(&Instruction::Call(0));
                } else {
                    return Err(WasmError::new(
                        "Only direct function calls are supported in WASM compilation",
                    ));
                }
            }
            Expr::If { .. } => {
                return Err(WasmError::new(
                    "If expressions not yet supported in WASM compilation",
                ))
            }
            Expr::Block { .. } => {
                return Err(WasmError::new(
                    "Block expressions not yet supported in WASM compilation",
                ))
            }
            Expr::Match { .. } => {
                return Err(WasmError::new(
                    "Match expressions not yet supported in WASM compilation",
                ))
            }
            Expr::Lambda { .. } => {
                return Err(WasmError::new(
                    "Lambda expressions not yet supported in WASM compilation",
                ))
            }
            Expr::Member { .. } => {
                return Err(WasmError::new(
                    "Member access not yet supported in WASM compilation",
                ))
            }
            Expr::Unary { .. } => {
                return Err(WasmError::new(
                    "Unary expressions not yet supported in WASM compilation",
                ))
            }
            Expr::List(_) | Expr::Tuple(_) => {
                return Err(WasmError::new(
                    "List/tuple literals not yet supported in WASM compilation",
                ))
            }
            Expr::Forall { .. } | Expr::Exists { .. } => {
                return Err(WasmError::new(
                    "Quantifier expressions not yet supported in WASM compilation",
                ))
            }
            Expr::Quote(_) | Expr::Unquote(_) | Expr::Reflect(_) => {
                return Err(WasmError::new(
                    "Metaprogramming expressions not yet supported in WASM compilation",
                ))
            }
            Expr::SexBlock { .. } => {
                return Err(WasmError::new(
                    "Sex blocks not yet supported in WASM compilation",
                ))
            }
            Expr::Cast { .. } => {
                return Err(WasmError::new(
                    "Type casts not yet supported in WASM compilation",
                ))
            }
            Expr::Try(_) => {
                return Err(WasmError::new(
                    "Try expressions not yet supported in WASM compilation",
                ))
            }
            Expr::QuasiQuote(_) | Expr::Eval(_) => {
                return Err(WasmError::new(
                    "Quasi-quote/eval expressions not yet supported in WASM compilation",
                ))
            }
            Expr::IdiomBracket { .. } => {
                return Err(WasmError::new(
                    "Idiom bracket expressions not yet supported in WASM compilation",
                ))
            }
            Expr::Implies { .. } => {
                return Err(WasmError::new(
                    "Implies expressions not yet supported in WASM compilation",
                ))
            }
        }

        Ok(())
    }

    /// Emit a binary operation as a WASM instruction.
    fn emit_binary_op(
        &self,
        function: &mut wasm_encoder::Function,
        op: crate::ast::BinaryOp,
    ) -> Result<(), WasmError> {
        use crate::ast::BinaryOp;
        use wasm_encoder::Instruction;

        // For simplicity, assume i64 operations
        // A full implementation would track types through the expression tree
        match op {
            BinaryOp::Add => {
                function.instruction(&Instruction::I64Add);
            }
            BinaryOp::Sub => {
                function.instruction(&Instruction::I64Sub);
            }
            BinaryOp::Mul => {
                function.instruction(&Instruction::I64Mul);
            }
            BinaryOp::Div => {
                function.instruction(&Instruction::I64DivS);
            }
            BinaryOp::Mod => {
                function.instruction(&Instruction::I64RemS);
            }
            BinaryOp::Eq => {
                function.instruction(&Instruction::I64Eq);
            }
            BinaryOp::Ne => {
                function.instruction(&Instruction::I64Ne);
            }
            BinaryOp::Lt => {
                function.instruction(&Instruction::I64LtS);
            }
            BinaryOp::Le => {
                function.instruction(&Instruction::I64LeS);
            }
            BinaryOp::Gt => {
                function.instruction(&Instruction::I64GtS);
            }
            BinaryOp::Ge => {
                function.instruction(&Instruction::I64GeS);
            }
            BinaryOp::And => {
                function.instruction(&Instruction::I64And);
            }
            BinaryOp::Or => {
                function.instruction(&Instruction::I64Or);
            }
            BinaryOp::Pow => {
                return Err(WasmError::new(
                    "Exponentiation not supported in basic WASM (requires math functions)",
                ))
            }
            BinaryOp::Pipe
            | BinaryOp::Compose
            | BinaryOp::Apply
            | BinaryOp::Bind
            | BinaryOp::Member
            | BinaryOp::Map
            | BinaryOp::Ap
            | BinaryOp::Implies
            | BinaryOp::Range => {
                return Err(WasmError::new(format!(
                    "Operator {:?} not supported in WASM compilation",
                    op
                )))
            }
        }

        Ok(())
    }

    /// Compile a DOL module to WASM and write to a file.
    ///
    /// Convenience method that calls [`compile`](WasmCompiler::compile) and
    /// writes the resulting bytecode to a file.
    ///
    /// # Arguments
    ///
    /// * `module` - The DOL module to compile
    /// * `output_path` - Path to write the WASM file
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or a `WasmError` if compilation or writing fails.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use metadol::wasm::WasmCompiler;
    /// use metadol::parse_file;
    ///
    /// let module = parse_file(source)?;
    /// let compiler = WasmCompiler::new();
    /// compiler.compile_to_file(&module, "output.wasm")?;
    /// ```
    pub fn compile_to_file(
        &self,
        module: &Declaration,
        output_path: impl AsRef<Path>,
    ) -> Result<(), WasmError> {
        let wasm_bytes = self.compile(module)?;
        std::fs::write(output_path, wasm_bytes)?;
        Ok(())
    }
}

#[cfg(feature = "wasm")]
impl Default for WasmCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(feature = "wasm")]
mod tests {
    use super::*;
    use crate::ast::{
        BinaryOp, Expr, FunctionDecl, FunctionParam, Literal, Purity, Span, Stmt, TypeExpr,
        Visibility,
    };

    #[test]
    fn test_compiler_new() {
        let compiler = WasmCompiler::new();
        assert!(!compiler.optimize);
        assert!(compiler.debug_info);
    }

    #[test]
    fn test_compiler_with_optimization() {
        let compiler = WasmCompiler::new().with_optimization(true);
        assert!(compiler.optimize);
    }

    #[test]
    fn test_compiler_with_debug_info() {
        let compiler = WasmCompiler::new().with_debug_info(false);
        assert!(!compiler.debug_info);
    }

    #[test]
    fn test_compiler_chaining() {
        let compiler = WasmCompiler::new()
            .with_optimization(true)
            .with_debug_info(false);
        assert!(compiler.optimize);
        assert!(!compiler.debug_info);
    }

    #[test]
    fn test_compiler_default() {
        let compiler = WasmCompiler::default();
        assert!(!compiler.optimize);
        assert!(compiler.debug_info);
    }

    #[test]
    fn test_compile_simple_function() {
        // Create a simple function: fun add(a: i64, b: i64) -> i64 { return a + b }
        let func = FunctionDecl {
            visibility: Visibility::Public,
            purity: Purity::Pure,
            name: "add".to_string(),
            type_params: None,
            params: vec![
                FunctionParam {
                    name: "a".to_string(),
                    type_ann: TypeExpr::Named("i64".to_string()),
                },
                FunctionParam {
                    name: "b".to_string(),
                    type_ann: TypeExpr::Named("i64".to_string()),
                },
            ],
            return_type: Some(TypeExpr::Named("i64".to_string())),
            body: vec![Stmt::Return(Some(Expr::Binary {
                left: Box::new(Expr::Identifier("a".to_string())),
                op: BinaryOp::Add,
                right: Box::new(Expr::Identifier("b".to_string())),
            }))],
            exegesis: "Adds two numbers".to_string(),
            span: Span::default(),
        };

        let decl = Declaration::Function(Box::new(func));
        let compiler = WasmCompiler::new();
        let wasm_bytes = compiler.compile(&decl).expect("Compilation failed");

        // Verify WASM magic number (0x00 0x61 0x73 0x6D) and version (0x01 0x00 0x00 0x00)
        assert!(wasm_bytes.len() >= 8, "WASM output too short");
        assert_eq!(
            &wasm_bytes[0..4],
            &[0x00, 0x61, 0x73, 0x6D],
            "Invalid WASM magic number"
        );
        assert_eq!(
            &wasm_bytes[4..8],
            &[0x01, 0x00, 0x00, 0x00],
            "Invalid WASM version"
        );
    }

    #[test]
    fn test_compile_function_with_literals() {
        // Create a function that returns a constant: fun get_answer() -> i64 { return 42 }
        let func = FunctionDecl {
            visibility: Visibility::Public,
            purity: Purity::Pure,
            name: "get_answer".to_string(),
            type_params: None,
            params: vec![],
            return_type: Some(TypeExpr::Named("i64".to_string())),
            body: vec![Stmt::Return(Some(Expr::Literal(Literal::Int(42))))],
            exegesis: "Returns the answer to everything".to_string(),
            span: Span::default(),
        };

        let decl = Declaration::Function(Box::new(func));
        let compiler = WasmCompiler::new();
        let wasm_bytes = compiler.compile(&decl).expect("Compilation failed");

        // Verify valid WASM output
        assert!(wasm_bytes.len() >= 8);
        assert_eq!(&wasm_bytes[0..4], &[0x00, 0x61, 0x73, 0x6D]);
        assert_eq!(&wasm_bytes[4..8], &[0x01, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_compile_non_function_declaration_fails() {
        use crate::ast::Gene;

        // Try to compile a Gene (not supported)
        let gene = Gene {
            name: "test.gene".to_string(),
            extends: None,
            statements: vec![],
            exegesis: "Test gene".to_string(),
            span: Span::default(),
        };

        let decl = Declaration::Gene(gene);
        let compiler = WasmCompiler::new();
        let result = compiler.compile(&decl);

        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("No functions found"));
    }
}
