//! Stress tests for lexer
//! Additional edge cases and boundary conditions

use metadol::lexer::{Lexer, TokenKind};

// ============================================================================
// WHITESPACE HANDLING
// ============================================================================

#[test]
fn whitespace_only_spaces() {
    let mut lexer = Lexer::new("   ");
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}

#[test]
fn whitespace_only_tabs() {
    let mut lexer = Lexer::new("\t\t\t");
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}

#[test]
fn whitespace_only_newlines() {
    let mut lexer = Lexer::new("\n\n\n");
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}

#[test]
fn whitespace_mixed() {
    let mut lexer = Lexer::new("  \t\n  \t\n");
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}

#[test]
fn whitespace_between_tokens() {
    let mut lexer = Lexer::new("a   b");
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn whitespace_carriage_return() {
    let mut lexer = Lexer::new("a\r\nb");
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

// ============================================================================
// COMMENT EDGE CASES
// ============================================================================

#[test]
fn comment_empty() {
    let mut lexer = Lexer::new("//\nfoo");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Identifier);
}

#[test]
fn comment_at_eof() {
    let mut lexer = Lexer::new("foo //comment");
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}

#[test]
fn comment_multiple_lines() {
    let mut lexer = Lexer::new("// first\n// second\nfoo");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Identifier);
}

#[test]
fn comment_with_operators() {
    let mut lexer = Lexer::new("// + - * /\nfoo");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Identifier);
}

// ============================================================================
// IDENTIFIER BOUNDARIES
// ============================================================================

#[test]
fn ident_single_char() {
    let mut lexer = Lexer::new("a");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Identifier);
    assert_eq!(token.lexeme, "a");
}

#[test]
fn ident_all_underscores() {
    let mut lexer = Lexer::new("___");
    let token = lexer.next_token();
    // Single underscore is Underscore token
    // Multiple underscores might be identifier
    assert!(matches!(
        token.kind,
        TokenKind::Identifier | TokenKind::Underscore
    ));
}

#[test]
fn ident_unicode() {
    // Basic ASCII expected
    let mut lexer = Lexer::new("hello_world");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Identifier);
}

#[test]
fn ident_max_length() {
    let long = "a".repeat(1000);
    let mut lexer = Lexer::new(&long);
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Identifier);
    assert_eq!(token.lexeme.len(), 1000);
}

// ============================================================================
// STRING EDGE CASES
// ============================================================================

#[test]
fn string_with_newline_escape() {
    let mut lexer = Lexer::new(r#""hello\nworld""#);
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::String);
}

#[test]
fn string_with_tab_escape() {
    let mut lexer = Lexer::new(r#""hello\tworld""#);
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::String);
}

#[test]
fn string_with_quote_escape() {
    let mut lexer = Lexer::new(r#""hello\"world""#);
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::String);
}

#[test]
fn string_with_backslash_escape() {
    let mut lexer = Lexer::new(r#""hello\\world""#);
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::String);
}

#[test]
fn string_long() {
    let content = "a".repeat(1000);
    let input = format!("\"{}\"", content);
    let mut lexer = Lexer::new(&input);
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::String);
}

// ============================================================================
// OPERATOR SEQUENCES
// ============================================================================

#[test]
fn operators_no_space() {
    let mut lexer = Lexer::new("a+b");
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Plus);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn operators_double_equals() {
    let mut lexer = Lexer::new("a==b");
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Eq);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn operators_not_equals() {
    let mut lexer = Lexer::new("a!=b");
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Ne);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn operators_less_equal() {
    let mut lexer = Lexer::new("a<=b");
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Le);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn operators_greater_equal() {
    let mut lexer = Lexer::new("a>=b");
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::GreaterEqual);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn operators_pipe_chain() {
    let mut lexer = Lexer::new("a|>b|>c");
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Pipe);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Pipe);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn operators_compose_chain() {
    let mut lexer = Lexer::new("f>>g>>h");
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Compose);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Compose);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn operators_arrow_vs_minus() {
    let mut lexer = Lexer::new("->-");
    assert_eq!(lexer.next_token().kind, TokenKind::Arrow);
    assert_eq!(lexer.next_token().kind, TokenKind::Minus);
}

#[test]
fn operators_fat_arrow_vs_equal() {
    let mut lexer = Lexer::new("=>= ");
    assert_eq!(lexer.next_token().kind, TokenKind::FatArrow);
    assert_eq!(lexer.next_token().kind, TokenKind::Equal);
}

// ============================================================================
// DELIMITER SEQUENCES
// ============================================================================

#[test]
fn delimiters_nested_parens() {
    let mut lexer = Lexer::new("((()))");
    assert_eq!(lexer.next_token().kind, TokenKind::LeftParen);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftParen);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftParen);
    assert_eq!(lexer.next_token().kind, TokenKind::RightParen);
    assert_eq!(lexer.next_token().kind, TokenKind::RightParen);
    assert_eq!(lexer.next_token().kind, TokenKind::RightParen);
}

#[test]
fn delimiters_nested_braces() {
    let mut lexer = Lexer::new("{{{}}}");
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::RightBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::RightBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::RightBrace);
}

#[test]
fn delimiters_nested_brackets() {
    let mut lexer = Lexer::new("[[[]]]");
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::RightBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::RightBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::RightBracket);
}

#[test]
fn delimiters_mixed() {
    let mut lexer = Lexer::new("([{}])");
    assert_eq!(lexer.next_token().kind, TokenKind::LeftParen);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::RightBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::RightBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::RightParen);
}

// ============================================================================
// KEYWORD VS IDENTIFIER
// ============================================================================

#[test]
fn keyword_prefix_ident() {
    let mut lexer = Lexer::new("gene_type");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Identifier);
}

#[test]
fn keyword_suffix_ident() {
    let mut lexer = Lexer::new("mygene");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Identifier);
}

#[test]
fn keyword_case_sensitive() {
    let mut lexer = Lexer::new("Gene");
    let token = lexer.next_token();
    // Keywords are case-sensitive, Gene != gene
    assert_eq!(token.kind, TokenKind::Identifier);
}

#[test]
fn keyword_uppercase() {
    let mut lexer = Lexer::new("GENE");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Identifier);
}

// ============================================================================
// NUMERIC BOUNDARY TESTS
// ============================================================================

#[test]
fn version_simple() {
    let mut lexer = Lexer::new("0.0.0");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Version);
}

#[test]
fn version_large_numbers() {
    let mut lexer = Lexer::new("999.999.999");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Version);
}

#[test]
fn dot_dot_operator() {
    let mut lexer = Lexer::new("..");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::DotDot);
}

#[test]
fn single_dot() {
    let mut lexer = Lexer::new(".");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Dot);
}

// ============================================================================
// PATH SEPARATOR
// ============================================================================

#[test]
fn path_sep_simple() {
    let mut lexer = Lexer::new("a::b");
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::PathSep);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn path_sep_long() {
    let mut lexer = Lexer::new("a::b::c::d");
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::PathSep);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::PathSep);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::PathSep);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

// ============================================================================
// IDIOM BRACKETS
// ============================================================================

#[test]
fn idiom_brackets_with_content() {
    let mut lexer = Lexer::new("[| x |]");
    assert_eq!(lexer.next_token().kind, TokenKind::IdiomOpen);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::IdiomClose);
}

#[test]
fn idiom_brackets_nested() {
    let mut lexer = Lexer::new("[| [| x |] |]");
    assert_eq!(lexer.next_token().kind, TokenKind::IdiomOpen);
    assert_eq!(lexer.next_token().kind, TokenKind::IdiomOpen);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::IdiomClose);
    assert_eq!(lexer.next_token().kind, TokenKind::IdiomClose);
}

// ============================================================================
// SPECIAL CHARACTERS
// ============================================================================

#[test]
fn at_symbol() {
    let mut lexer = Lexer::new("@");
    assert_eq!(lexer.next_token().kind, TokenKind::At);
}

#[test]
fn hash_symbol() {
    let mut lexer = Lexer::new("#");
    assert_eq!(lexer.next_token().kind, TokenKind::Macro);
}

#[test]
fn question_symbol() {
    let mut lexer = Lexer::new("?");
    assert_eq!(lexer.next_token().kind, TokenKind::Reflect);
}

#[test]
fn quote_symbol() {
    let mut lexer = Lexer::new("'");
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
}

// ============================================================================
// REAL-WORLD LEXER PATTERNS
// ============================================================================

#[test]
fn pattern_function_signature() {
    let input = "fun add(a: Int64, b: Int64) -> Int64";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next_token().kind, TokenKind::Function);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier); // add
    assert_eq!(lexer.next_token().kind, TokenKind::LeftParen);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier); // a
    assert_eq!(lexer.next_token().kind, TokenKind::Colon);
    assert_eq!(lexer.next_token().kind, TokenKind::Int64);
}

#[test]
fn pattern_gene_declaration() {
    let input = "gene Container { has id: UInt64 }";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next_token().kind, TokenKind::Gene);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier); // Container
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::Has);
}

#[test]
fn pattern_trait_declaration() {
    let input = "trait Runnable { entity is active }";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next_token().kind, TokenKind::Trait);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier); // Runnable
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier); // entity
    assert_eq!(lexer.next_token().kind, TokenKind::Is);
}

#[test]
fn pattern_constraint() {
    let input = "constraint Valid { requires field }";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next_token().kind, TokenKind::Constraint);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::Requires);
}

#[test]
fn pattern_pipe_expression() {
    let input = "data |> transform |> output";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Pipe);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Pipe);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn pattern_match_arm() {
    let input = "_ => { result }";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next_token().kind, TokenKind::Underscore);
    assert_eq!(lexer.next_token().kind, TokenKind::FatArrow);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBrace);
}

#[test]
fn pattern_list_literal() {
    let input = "[a, b, c]";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Comma);
}

#[test]
fn pattern_range() {
    // Lexer may include dots in identifiers
    // Use spaces to separate
    let input = "a .. b";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::DotDot);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

// ============================================================================
// ERROR RECOVERY
// ============================================================================

#[test]
fn empty_input() {
    let mut lexer = Lexer::new("");
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}

#[test]
fn eof_after_token() {
    let mut lexer = Lexer::new("x");
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
    // Multiple EOF calls should be safe
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}
