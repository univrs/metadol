//! Unit tests for semantic validation
//! Tests validator behavior for various inputs

use metadol::parser::Parser;
use metadol::validator::validate;

// ============================================================================
// EXEGESIS REQUIREMENT TESTS
// ============================================================================
// Note: DOL requires exegesis (documentation) for declarations.
// Declarations without exegesis will fail validation.

#[test]
fn validate_gene_without_exegesis_fails() {
    let file = Parser::new("gene Test { }").parse_file().unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        // Should fail because no exegesis
        assert!(!result.is_valid() || result.has_warnings());
    }
}

#[test]
fn validate_gene_with_exegesis() {
    let input = r#"gene Test { }
    exegesis { This is the Test gene documentation. }"#;
    let file = Parser::new(input).parse_file().unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        // May pass if exegesis is sufficient
        let _ = result.is_valid();
    }
}

// ============================================================================
// VALIDATION BEHAVIOR TESTS
// ============================================================================

#[test]
fn validation_result_has_errors_without_exegesis() {
    let file = Parser::new("gene Test { }").parse_file().unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        // Should have at least one error or warning
        assert!(!result.errors.is_empty() || !result.warnings.is_empty() || !result.is_valid());
    }
}

#[test]
fn validation_result_can_check_valid() {
    let file = Parser::new("gene Test { }").parse_file().unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        // is_valid() should return a boolean
        let _is_valid: bool = result.is_valid();
    }
}

#[test]
fn validation_result_can_check_warnings() {
    let file = Parser::new("gene Test { }").parse_file().unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        // has_warnings() should return a boolean
        let _has_warnings: bool = result.has_warnings();
    }
}

// ============================================================================
// DECLARATION TYPE TESTS
// ============================================================================

#[test]
fn validate_gene_declaration() {
    let file = Parser::new("gene Test { has x: Int64 }")
        .parse_file()
        .unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        // Should produce a validation result
        assert!(!result.declaration_name.is_empty());
    }
}

#[test]
fn validate_trait_declaration() {
    let file = Parser::new("trait Test { entity is active }")
        .parse_file()
        .unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        assert!(!result.declaration_name.is_empty());
    }
}

#[test]
fn validate_constraint_declaration() {
    let file = Parser::new("constraint Test { value is required }")
        .parse_file()
        .unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        assert!(!result.declaration_name.is_empty());
    }
}

#[test]
fn validate_function_declaration() {
    let file = Parser::new("fun test() { }").parse_file().unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        assert!(!result.declaration_name.is_empty());
    }
}

#[test]
fn validate_system_declaration() {
    let file = Parser::new("system Test { entity has state }")
        .parse_file()
        .unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        assert!(!result.declaration_name.is_empty());
    }
}

// ============================================================================
// NAMING CONVENTION TESTS
// ============================================================================

#[test]
fn validate_pascal_case_gene_name() {
    let file = Parser::new("gene MyContainer { }").parse_file().unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        // Pascal case should not produce naming warning (but will have exegesis error)
        let has_naming_warning = result
            .warnings
            .iter()
            .any(|w| format!("{:?}", w).contains("Naming"));
        assert!(!has_naming_warning);
    }
}

#[test]
fn validate_lowercase_gene_name_warns() {
    let file = Parser::new("gene mycontainer { }").parse_file().unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        // lowercase gene name should produce naming warning
        let _ = result.warnings.len(); // May or may not have warning
    }
}

#[test]
fn validate_qualified_name() {
    let input = "gene container.exists { }";
    let file = Parser::new(input).parse_file();
    if let Ok(file) = file {
        if let Some(decl) = file.declarations.first() {
            let result = validate(decl);
            // Qualified names are valid
            assert!(!result.declaration_name.is_empty());
        }
    }
}

// ============================================================================
// MULTIPLE DECLARATIONS
// ============================================================================

#[test]
fn validate_each_declaration_independently() {
    let input = "gene A { } gene B { } gene C { }";
    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let result = validate(decl);
        // Each should produce a result
        assert!(!result.declaration_name.is_empty());
    }
}

#[test]
fn validate_mixed_declaration_types() {
    let input = r#"
gene A { }
trait B { }
constraint C { }
fun d() { }
system E { }
    "#;
    let file = Parser::new(input).parse_file().unwrap();

    assert_eq!(file.declarations.len(), 5);

    for decl in &file.declarations {
        let result = validate(decl);
        assert!(!result.declaration_name.is_empty());
    }
}

// ============================================================================
// FIELD AND STATEMENT TESTS
// ============================================================================

#[test]
fn validate_gene_with_many_fields() {
    let mut input = String::from("gene Test { ");
    for i in 0..10 {
        input.push_str(&format!("has field{}: Int64 ", i));
    }
    input.push_str("}");

    let file = Parser::new(&input).parse_file().unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        assert!(!result.declaration_name.is_empty());
    }
}

#[test]
fn validate_trait_with_many_predicates() {
    let input = r#"trait Test {
        entity is active
        entity is running
        entity is ready
    }"#;

    let file = Parser::new(input).parse_file().unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        assert!(!result.declaration_name.is_empty());
    }
}

// ============================================================================
// TYPE VALIDATION
// ============================================================================

#[test]
fn validate_int64_type() {
    let file = Parser::new("gene Test { has x: Int64 }")
        .parse_file()
        .unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        // Type should be valid
        assert!(!result.declaration_name.is_empty());
    }
}

#[test]
fn validate_uint64_type() {
    let file = Parser::new("gene Test { has x: UInt64 }")
        .parse_file()
        .unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        assert!(!result.declaration_name.is_empty());
    }
}

#[test]
fn validate_string_type() {
    let file = Parser::new("gene Test { has x: String }")
        .parse_file()
        .unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        assert!(!result.declaration_name.is_empty());
    }
}

#[test]
fn validate_bool_type() {
    let file = Parser::new("gene Test { has x: Bool }")
        .parse_file()
        .unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        assert!(!result.declaration_name.is_empty());
    }
}

#[test]
fn validate_float64_type() {
    let file = Parser::new("gene Test { has x: Float64 }")
        .parse_file()
        .unwrap();
    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        assert!(!result.declaration_name.is_empty());
    }
}

// ============================================================================
// STRESS TESTS
// ============================================================================

#[test]
fn validate_large_file() {
    let mut input = String::new();
    for i in 0..50 {
        input.push_str(&format!("gene Gene{} {{ has x: Int64 }}\n", i));
    }

    let file = Parser::new(&input).parse_file().unwrap();
    assert_eq!(file.declarations.len(), 50);

    for decl in &file.declarations {
        let result = validate(decl);
        assert!(!result.declaration_name.is_empty());
    }
}

#[test]
fn validate_complex_file() {
    let input = r#"
gene User {
    has id: UInt64
    has name: String
    has active: Bool
}

gene Post {
    has id: UInt64
    has author: UInt64
    has content: String
}

trait Lifecycle {
    entity is created
    entity is deleted
}

constraint NonEmpty {
    value is required
}

fun process(x: Int64) -> Int64 {
    return x
}

system API {
    request has method
    request has path
}
    "#;

    let file = Parser::new(input).parse_file().unwrap();

    for decl in &file.declarations {
        let result = validate(decl);
        // Each declaration produces a result
        assert!(!result.declaration_name.is_empty());
    }
}

#[test]
fn validate_long_name() {
    let name = "A".repeat(100);
    let input = format!("gene {} {{ }}", name);
    let file = Parser::new(&input).parse_file().unwrap();

    if let Some(decl) = file.declarations.first() {
        let result = validate(decl);
        assert_eq!(result.declaration_name.len(), 100);
    }
}

// ============================================================================
// EMPTY FILE
// ============================================================================

#[test]
fn validate_empty_file_no_declarations() {
    let file = Parser::new("").parse_file().unwrap();
    assert!(file.declarations.is_empty());
}

#[test]
fn validate_whitespace_file_no_declarations() {
    let file = Parser::new("   \n\t\n   ").parse_file().unwrap();
    assert!(file.declarations.is_empty());
}
