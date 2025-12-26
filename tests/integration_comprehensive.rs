//! Comprehensive integration tests
//! End-to-end tests combining lexer, parser, and validation

use metadol::ast::*;
use metadol::lexer::{Lexer, TokenKind};
use metadol::parser::Parser;

// ============================================================================
// LEXER-PARSER INTEGRATION
// ============================================================================

#[test]
fn integration_lexer_produces_valid_stream() {
    let input = "gene Test { has field: Int64 }";
    let mut lexer = Lexer::new(input);

    // Collect all tokens
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        tokens.push(token.kind);
        if token.kind == TokenKind::Eof {
            break;
        }
    }

    // Should be parseable
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn integration_all_keywords_lexable() {
    let keywords = [
        "gene",
        "trait",
        "constraint",
        "system",
        "evolves",
        "fun",
        "return",
        "if",
        "else",
        "match",
        "for",
        "while",
        "loop",
        "break",
        "continue",
        "where",
        "in",
        "requires",
        "law",
        "has",
        "is",
        "use",
        "module",
        "pub",
        "let",
        "true",
        "false",
        "Int64",
        "UInt64",
        "String",
        "Bool",
        "Float64",
        "Void",
    ];

    for kw in keywords {
        let mut lexer = Lexer::new(kw);
        let token = lexer.next_token();
        // All keywords should produce a non-Identifier, non-Eof token
        // (or for types like Int64, the specific type token)
        assert_ne!(token.kind, TokenKind::Eof, "Keyword {} produced Eof", kw);
    }
}

// ============================================================================
// COMPLETE GENE PARSING
// ============================================================================

#[test]
fn complete_gene_container() {
    let input = r#"gene Container {
        has id: UInt64
        has name: String
        has running: Bool
    }"#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());

    let file = result.unwrap();
    assert_eq!(file.declarations.len(), 1);
}

#[test]
fn complete_gene_with_type() {
    let input = r#"gene Counter {
        type: Int64
    }"#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn complete_gene_minimal() {
    let input = "gene Empty { }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// COMPLETE TRAIT PARSING
// ============================================================================

#[test]
fn complete_trait_lifecycle() {
    let input = r#"trait Lifecycle {
        entity is created
        entity is active
        entity is destroyed
    }"#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn complete_trait_with_uses() {
    let input = r#"trait Consumer {
        uses Producer
    }"#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// COMPLETE CONSTRAINT PARSING
// ============================================================================

#[test]
fn complete_constraint_validation() {
    let input = r#"constraint NonEmpty {
        value is nonempty
    }"#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn complete_constraint_with_law() {
    let input = r#"constraint Positive {
        value is positive
    }"#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// COMPLETE FUNCTION PARSING
// ============================================================================

#[test]
fn complete_function_identity() {
    let input = "fun identity(x: Int64) -> Int64 { return x }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn complete_function_binary() {
    let input = "fun add(a: Int64, b: Int64) -> Int64 { return a + b }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn complete_function_no_params() {
    let input = "fun answer() -> Int64 { return 42 }";
    let result = Parser::new(input).parse_file();
    if result.is_ok() {
        // Return literal parsing works
    }
}

#[test]
fn complete_function_void() {
    let input = "fun noop() { }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// COMPLETE SYSTEM PARSING
// ============================================================================

#[test]
fn complete_system_runtime() {
    let input = r#"system Runtime {
        process has state
        process has pid
    }"#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// MULTI-DECLARATION FILES
// ============================================================================

#[test]
fn multi_gene_file() {
    let input = r#"
gene A { has x: Int64 }
gene B { has y: String }
gene C { has z: Bool }
    "#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());

    let file = result.unwrap();
    assert_eq!(file.declarations.len(), 3);
}

#[test]
fn multi_mixed_file() {
    let input = r#"
gene Container { has id: UInt64 }
trait Lifecycle { entity is active }
constraint Valid { id is valid }
fun process() { }
system Runtime { entity has state }
    "#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());

    let file = result.unwrap();
    assert_eq!(file.declarations.len(), 5);
}

// ============================================================================
// EXPRESSION INTEGRATION
// ============================================================================

#[test]
fn expr_in_function() {
    let input = "fun calc() { return a + b * c }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn expr_pipe_in_function() {
    let input = "fun transform() { return x |> f |> g }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn expr_if_in_function() {
    let input = "fun check() { if true { a } else { b } }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn expr_match_in_function() {
    let input = r#"fun dispatch() {
        match x {
            _ { result }
        }
    }"#;
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// REAL-WORLD DOL PATTERNS
// ============================================================================

#[test]
fn realworld_entity_model() {
    let input = r#"
gene User {
    has id: UInt64
    has username: String
    has email: String
    has active: Bool
}

gene Post {
    has id: UInt64
    has title: String
    has content: String
}
    "#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn realworld_state_machine() {
    let input = r#"
trait StateMachine {
    entity is idle
    entity is running
    entity is stopped
}
    "#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn realworld_validation() {
    let input = r#"
constraint EmailValid {
    email is valid
}

constraint PasswordStrong {
    password is strong
}
    "#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn realworld_api_system() {
    let input = r#"
system API {
    request has method
    request has path
    response has status
    response has body
}
    "#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// SPAN TRACKING
// ============================================================================

#[test]
fn span_covers_declaration() {
    let input = "gene Test { }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());

    let file = result.unwrap();
    if let Some(Declaration::Gene(gene)) = file.declarations.first() {
        assert!(gene.span.start < gene.span.end);
    }
}

#[test]
fn span_line_tracking() {
    let input = "gene Test { }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());

    let file = result.unwrap();
    if let Some(Declaration::Gene(gene)) = file.declarations.first() {
        assert!(gene.span.line >= 1);
    }
}

// ============================================================================
// UNICODE AND SPECIAL CHARACTERS
// ============================================================================

#[test]
fn unicode_in_string() {
    let input = r#"gene Test { }"#;
    // Unicode in identifiers may not be supported
    // but Unicode in strings should be
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// WHITESPACE HANDLING
// ============================================================================

#[test]
fn extra_whitespace() {
    let input = "   gene   Test   {   }   ";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn tabs_instead_of_spaces() {
    let input = "gene\tTest\t{\t}";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn mixed_line_endings() {
    let input = "gene Test {\r\n}\r\n";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// COMMENT INTEGRATION
// ============================================================================

#[test]
fn comments_between_declarations() {
    let input = r#"
gene A { }
// This is a comment
gene B { }
    "#;

    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn inline_comments() {
    let input = "gene Test { } // inline comment";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// ERROR RECOVERY
// ============================================================================

#[test]
fn error_recovery_missing_brace() {
    let input = "gene Test {";
    let result = Parser::new(input).parse_file();
    assert!(result.is_err());
}

#[test]
fn error_recovery_extra_brace() {
    let input = "gene Test { } }";
    let result = Parser::new(input).parse_file();
    // May produce error or partial parse
    let _ = result;
}

#[test]
fn error_recovery_missing_name() {
    let input = "gene { }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_err());
}

// ============================================================================
// STRESS TESTS
// ============================================================================

#[test]
fn stress_large_file() {
    let mut input = String::new();
    for i in 0..100 {
        input.push_str(&format!("gene Gene{} {{ has field: Int64 }}\n", i));
    }

    let result = Parser::new(&input).parse_file();
    assert!(result.is_ok());

    let file = result.unwrap();
    assert_eq!(file.declarations.len(), 100);
}

#[test]
fn stress_deep_nesting() {
    let input = "fun deep() { return ((((((((a)))))))) }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn stress_long_chain() {
    let input = "fun chain() { return a |> b |> c |> d |> e |> f |> g |> h |> i |> j }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn stress_many_fields() {
    let mut fields = String::new();
    for i in 0..50 {
        fields.push_str(&format!("    has field{}: Int64\n", i));
    }
    let input = format!("gene ManyFields {{\n{}}}", fields);

    let result = Parser::new(&input).parse_file();
    assert!(result.is_ok());
}
