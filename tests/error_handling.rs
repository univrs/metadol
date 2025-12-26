//! Error handling tests
//! Tests error detection and recovery

use metadol::lexer::{Lexer, TokenKind};
use metadol::parser::Parser;

// ============================================================================
// PARSER ERROR DETECTION
// ============================================================================

#[test]
fn error_missing_gene_name() {
    let result = Parser::new("gene { }").parse_file();
    assert!(result.is_err());
}

#[test]
fn error_missing_gene_brace() {
    let result = Parser::new("gene Test").parse_file();
    assert!(result.is_err());
}

#[test]
fn error_unclosed_gene() {
    let result = Parser::new("gene Test {").parse_file();
    assert!(result.is_err());
}

#[test]
fn error_missing_trait_name() {
    let result = Parser::new("trait { }").parse_file();
    assert!(result.is_err());
}

#[test]
fn error_missing_constraint_name() {
    let result = Parser::new("constraint { }").parse_file();
    assert!(result.is_err());
}

#[test]
fn error_missing_function_name() {
    let result = Parser::new("fun () { }").parse_file();
    assert!(result.is_err());
}

#[test]
fn error_missing_system_name() {
    let result = Parser::new("system { }").parse_file();
    assert!(result.is_err());
}

// ============================================================================
// UNCLOSED DELIMITERS
// ============================================================================

#[test]
fn error_unclosed_paren_in_call() {
    let result = Parser::new("fun test() { foo(a, b }").parse_file();
    assert!(result.is_err());
}

#[test]
fn error_unclosed_bracket_in_list() {
    let result = Parser::new("fun test() { [a, b }").parse_file();
    assert!(result.is_err());
}

#[test]
fn error_extra_close_brace() {
    let result = Parser::new("gene Test { } }").parse_file();
    // May produce error or handle gracefully
    let _ = result;
}

#[test]
fn error_extra_close_paren() {
    let result = Parser::new("fun test() { (a)) }").parse_file();
    // May produce error or handle gracefully
    let _ = result;
}

// ============================================================================
// MISSING KEYWORDS
// ============================================================================

#[test]
fn error_missing_return_keyword() {
    let result = Parser::new("fun test() { x }").parse_file();
    // May be valid (expression as final statement)
    let _ = result;
}

#[test]
fn error_missing_if_body() {
    let result = Parser::new("fun test() { if true }").parse_file();
    assert!(result.is_err());
}

#[test]
fn error_missing_match_scrutinee() {
    let result = Parser::new("fun test() { match { } }").parse_file();
    assert!(result.is_err());
}

// ============================================================================
// TYPE ERRORS
// ============================================================================

#[test]
fn error_missing_field_type() {
    // DOL allows untyped 'has x' statements (produces Statement::Has)
    // But 'has x:' without a type after colon is an error
    let result = Parser::new("gene Test { has x: }").parse_file();
    assert!(result.is_err());
}

#[test]
fn error_missing_param_type() {
    let result = Parser::new("fun test(x) { }").parse_file();
    assert!(result.is_err());
}

// ============================================================================
// LEXER ERROR HANDLING
// ============================================================================

#[test]
fn lexer_unknown_char() {
    // Unknown characters should produce Error token or be skipped
    let mut lexer = Lexer::new("$");
    let token = lexer.next_token();
    // Should not crash
    assert!(matches!(token.kind, TokenKind::Error | TokenKind::Eof));
}

#[test]
fn lexer_unclosed_string() {
    let mut lexer = Lexer::new("\"hello");
    let token = lexer.next_token();
    // Should handle gracefully
    let _ = token;
}

#[test]
fn lexer_empty_input() {
    let mut lexer = Lexer::new("");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Eof);
}

// ============================================================================
// ERROR MESSAGE QUALITY
// ============================================================================

#[test]
fn error_message_has_span() {
    let result = Parser::new("gene { }").parse_file();
    if let Err(e) = result {
        let msg = format!("{:?}", e);
        // Error should include location info
        assert!(msg.contains("span") || msg.contains("line") || msg.contains("Span"));
    }
}

#[test]
fn error_message_describes_expected() {
    let result = Parser::new("gene { }").parse_file();
    if let Err(e) = result {
        let msg = format!("{:?}", e);
        // Error should mention what was expected
        assert!(msg.contains("expected") || msg.contains("Expected") || msg.contains("identifier"));
    }
}

// ============================================================================
// RECOVERY TESTS
// ============================================================================

#[test]
fn recover_and_continue_parsing() {
    // Even with one error, parser may be able to recover
    // This depends on error recovery implementation
    let input = "gene A { gene B { }"; // Missing close brace for A
    let result = Parser::new(input).parse_file();
    // May fail or partially succeed
    let _ = result;
}

#[test]
fn multiple_errors_detected() {
    let input = "gene { } trait { }"; // Two missing names
    let result = Parser::new(input).parse_file();
    assert!(result.is_err());
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn error_only_keyword() {
    let result = Parser::new("gene").parse_file();
    assert!(result.is_err());
}

#[test]
fn error_only_brace() {
    let result = Parser::new("{").parse_file();
    assert!(result.is_err());
}

#[test]
fn error_random_tokens() {
    let result = Parser::new("+ - * /").parse_file();
    assert!(result.is_err());
}

#[test]
fn error_mismatched_braces() {
    let result = Parser::new("gene Test { ] }").parse_file();
    assert!(result.is_err());
}

// ============================================================================
// GRACEFUL DEGRADATION
// ============================================================================

#[test]
fn graceful_on_large_error() {
    let input = "{{{{{{{{{{";
    let result = Parser::new(input).parse_file();
    // Should not crash or hang
    let _ = result;
}

#[test]
fn graceful_on_deep_nesting_error() {
    let input = "(((((((((( ))))))))))";
    let result = Parser::new(input).parse_expr(0);
    // Should handle gracefully
    let _ = result;
}

#[test]
fn graceful_on_long_input() {
    let input = "x ".repeat(10000);
    let result = Parser::new(&input).parse_file();
    // Should not crash
    let _ = result;
}
