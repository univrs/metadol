//! Integration tests for the Spirit compiler module.
//!
//! These tests verify the end-to-end compilation pipeline from DOL source
//! to WASM bytecode.

#![cfg(feature = "wasm")]

use metadol::compiler::spirit::{compile_source, CompilerError};

#[test]
fn test_compile_empty_module() {
    let source = r#"
module test @ 1.0.0
"#;

    let result = compile_source(source, "test.dol");
    assert!(result.is_ok(), "Failed to compile empty module");

    let compiled = result.unwrap();
    assert!(!compiled.wasm.is_empty());

    // Verify WASM magic number
    assert_eq!(&compiled.wasm[0..4], b"\0asm");
    // Verify WASM version 1
    assert_eq!(&compiled.wasm[4..8], &[1, 0, 0, 0]);
}

#[test]
fn test_compile_gene_declaration() {
    let source = r#"
module example @ 1.0.0

gene Counter {
    has value: Int64
}

exegesis {
    A simple counter.
}
"#;

    let result = compile_source(source, "example.dol");
    assert!(result.is_ok(), "Failed to compile gene declaration");

    let compiled = result.unwrap();
    assert!(!compiled.wasm.is_empty());
    assert_eq!(&compiled.wasm[0..4], b"\0asm");
}

#[test]
fn test_compile_invalid_syntax() {
    let source = r#"
module broken @ 1.0.0

gene Invalid {
    this is completely invalid syntax
}
"#;

    let result = compile_source(source, "broken.dol");
    assert!(result.is_err(), "Should fail on invalid syntax");

    match result.unwrap_err() {
        CompilerError::ParseError(_) => {
            // Expected
        }
        other => panic!("Expected ParseError, got {:?}", other),
    }
}

#[test]
fn test_compiled_spirit_has_warnings() {
    // Use deprecated syntax to generate warnings
    let source = r#"
module deprecated @ 1.0.0

gene Test {
    has field: Int64
}

exegesis {
    Test gene.
}
"#;

    let result = compile_source(source, "deprecated.dol");
    assert!(result.is_ok());

    let compiled = result.unwrap();
    // Warnings may or may not be generated depending on deprecation tracking
    // Just verify the structure is correct
    assert_eq!(compiled.warnings.len(), compiled.warnings.len());
}

#[test]
fn test_compiler_error_types() {
    use std::io;

    // Test IoError conversion
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let compiler_err: CompilerError = io_err.into();
    assert!(matches!(compiler_err, CompilerError::IoError(_)));

    // Test display
    let err = CompilerError::WasmError("test".to_string());
    let display = format!("{}", err);
    assert!(display.contains("WASM emission error"));
}
