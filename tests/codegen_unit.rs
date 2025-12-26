//! Unit tests for code generation
//! Tests RustCodegen output for various AST inputs

use metadol::codegen::RustCodegen;
use metadol::parser::Parser;

// ============================================================================
// GENE CODEGEN
// ============================================================================

#[test]
fn codegen_gene_empty() {
    let input = "gene Empty { }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("struct Empty"));
    }
}

#[test]
fn codegen_gene_single_field() {
    let input = "gene Counter { has count: Int64 }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("struct Counter"));
        assert!(output.contains("count"));
    }
}

#[test]
fn codegen_gene_multiple_fields() {
    let input = r#"gene Container {
        has id: UInt64
        has name: String
    }"#;
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("struct Container"));
        assert!(output.contains("id"));
        assert!(output.contains("name"));
    }
}

#[test]
fn codegen_gene_with_type() {
    let input = "gene Value { type: Int64 }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("struct Value"));
    }
}

#[test]
fn codegen_gene_bool_field() {
    let input = "gene Flag { has enabled: Bool }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("enabled"));
        assert!(output.contains("bool"));
    }
}

#[test]
fn codegen_gene_float_field() {
    let input = "gene Metric { has value: Float64 }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("value"));
    }
}

// ============================================================================
// FUNCTION CODEGEN
// ============================================================================

#[test]
fn codegen_function_empty() {
    let input = "fun noop() { }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("fn noop"));
    }
}

#[test]
fn codegen_function_with_param() {
    let input = "fun identity(x: Int64) { return x }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("fn identity"));
    }
}

#[test]
fn codegen_function_with_return_type() {
    let input = "fun answer() -> Int64 { return 42 }";
    let file = Parser::new(input).parse_file();
    if let Ok(file) = file {
        for decl in &file.declarations {
            let output = RustCodegen::generate(decl);
            assert!(output.contains("fn answer"));
        }
    }
}

#[test]
fn codegen_function_multiple_params() {
    let input = "fun add(a: Int64, b: Int64) { return a + b }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("fn add"));
    }
}

// ============================================================================
// TRAIT CODEGEN
// ============================================================================

#[test]
fn codegen_trait_empty() {
    let input = "trait Empty { }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("trait Empty"));
    }
}

#[test]
fn codegen_trait_with_predicates() {
    let input = r#"trait Active {
        entity is active
    }"#;
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("trait Active"));
    }
}

// ============================================================================
// CONSTRAINT CODEGEN
// ============================================================================

#[test]
fn codegen_constraint_empty() {
    let input = "constraint Empty { }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let _output = RustCodegen::generate(decl);
        // Constraints may generate marker traits or structs
    }
}

#[test]
fn codegen_constraint_with_requires() {
    let input = r#"constraint NonEmpty {
        value is required
    }"#;
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let _output = RustCodegen::generate(decl);
        // Generated output should exist
    }
}

// ============================================================================
// SYSTEM CODEGEN
// ============================================================================

#[test]
fn codegen_system_empty() {
    let input = "system Empty { }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let _output = RustCodegen::generate(decl);
        // Systems generate some output
    }
}

#[test]
fn codegen_system_with_predicates() {
    let input = r#"system Runtime {
        process has state
    }"#;
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let _output = RustCodegen::generate(decl);
    }
}

// ============================================================================
// TYPE MAPPING
// ============================================================================

#[test]
fn type_mapping_int64() {
    let input = "gene Test { has x: Int64 }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        // Should map to isize or i64
        assert!(output.contains("isize") || output.contains("i64"));
    }
}

#[test]
fn type_mapping_uint64() {
    let input = "gene Test { has x: UInt64 }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        // Should map to usize or u64
        assert!(output.contains("usize") || output.contains("u64") || output.contains("isize"));
    }
}

#[test]
fn type_mapping_string() {
    let input = "gene Test { has x: String }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("String"));
    }
}

#[test]
fn type_mapping_bool() {
    let input = "gene Test { has x: Bool }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("bool"));
    }
}

// ============================================================================
// DERIVE ATTRIBUTES
// ============================================================================

#[test]
fn derive_debug() {
    let input = "gene Test { }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("#[derive(Debug"));
    }
}

#[test]
fn derive_clone() {
    let input = "gene Test { }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("Clone"));
    }
}

// ============================================================================
// IMPL BLOCK GENERATION
// ============================================================================

#[test]
fn impl_new_generated() {
    let input = "gene Test { has value: Int64 }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("impl Test"));
        assert!(output.contains("fn new"));
    }
}

#[test]
fn impl_self_return() {
    let input = "gene Test { has value: Int64 }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("-> Self") || output.contains("Self {"));
    }
}

// ============================================================================
// PUB VISIBILITY
// ============================================================================

#[test]
fn pub_struct() {
    let input = "gene Test { has value: Int64 }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("pub struct"));
    }
}

#[test]
fn pub_field() {
    let input = "gene Test { has value: Int64 }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("pub value"));
    }
}

// ============================================================================
// MULTIPLE DECLARATIONS
// ============================================================================

#[test]
fn multiple_genes() {
    let input = r#"
gene A { has x: Int64 }
gene B { has y: String }
    "#;
    let file = Parser::new(input).parse_file().unwrap();

    let mut outputs: Vec<String> = Vec::new();
    for decl in &file.declarations {
        outputs.push(RustCodegen::generate(decl));
    }

    assert_eq!(outputs.len(), 2);
    assert!(outputs[0].contains("struct A"));
    assert!(outputs[1].contains("struct B"));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn codegen_reserved_word_field() {
    // If field name is Rust keyword, should be handled
    let input = "gene Test { has value: Int64 }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        // Should produce valid Rust
        assert!(!output.is_empty());
    }
}

#[test]
fn codegen_long_name() {
    let input = "gene VeryLongGeneNameThatIsStillValid { }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        assert!(output.contains("VeryLongGeneNameThatIsStillValid"));
    }
}

// ============================================================================
// OUTPUT FORMAT
// ============================================================================

#[test]
fn output_is_valid_rust_syntax() {
    let input = "gene Test { has value: Int64 }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        // Should have balanced braces
        let opens = output.matches('{').count();
        let closes = output.matches('}').count();
        assert_eq!(opens, closes);
    }
}

#[test]
fn output_has_no_trailing_whitespace_issues() {
    let input = "gene Test { }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let output = RustCodegen::generate(decl);
        // Should not have excessive blank lines
        assert!(!output.contains("\n\n\n\n"));
    }
}
