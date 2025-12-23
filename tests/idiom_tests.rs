//! Comprehensive tests for idiom bracket parsing and desugaring.
//!
//! Idiom brackets `[| f a b |]` provide applicative functor syntax.
//! They desugar to `f <$> a <*> b` for applicative style programming.

use metadol::ast::Expr;
use metadol::lexer::{Lexer, TokenKind};
use metadol::parser::Parser;

/// Helper to collect all tokens from input
fn tokenize(input: &str) -> Vec<(TokenKind, String)> {
    Lexer::new(input).map(|t| (t.kind, t.lexeme)).collect()
}

/// Helper to get just token kinds
fn token_kinds(input: &str) -> Vec<TokenKind> {
    Lexer::new(input).map(|t| t.kind).collect()
}

// ============================================
// Lexer Tests
// ============================================

#[test]
fn test_idiom_open_token() {
    let tokens = tokenize("[|");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].0, TokenKind::IdiomOpen);
    assert_eq!(tokens[0].1, "[|");
}

#[test]
fn test_idiom_close_token() {
    let tokens = tokenize("|]");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].0, TokenKind::IdiomClose);
    assert_eq!(tokens[0].1, "|]");
}

#[test]
fn test_complete_idiom_bracket_tokens() {
    let kinds = token_kinds("[| f a b |]");
    assert_eq!(kinds.len(), 5);
    assert_eq!(kinds[0], TokenKind::IdiomOpen);
    assert_eq!(kinds[1], TokenKind::Identifier); // f
    assert_eq!(kinds[2], TokenKind::Identifier); // a
    assert_eq!(kinds[3], TokenKind::Identifier); // b
    assert_eq!(kinds[4], TokenKind::IdiomClose);
}

#[test]
fn test_idiom_bracket_with_whitespace() {
    let kinds = token_kinds("[|   f   a   b   |]");
    assert_eq!(kinds.len(), 5);
    assert_eq!(kinds[0], TokenKind::IdiomOpen);
    assert_eq!(kinds[4], TokenKind::IdiomClose);
}

#[test]
fn test_idiom_bracket_no_space() {
    // Test that tokens are recognized even without spaces
    let kinds = token_kinds("[|f|]");
    assert_eq!(kinds[0], TokenKind::IdiomOpen);
    assert_eq!(kinds[1], TokenKind::Identifier);
    assert_eq!(kinds[2], TokenKind::IdiomClose);
}

// ============================================
// Parser Tests - Valid Cases
// ============================================

#[test]
fn test_parse_idiom_bracket_function_only() {
    // [| f |] - just a function, no arguments
    let input = "[| f |]";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::IdiomBracket { func, args } => {
            assert!(matches!(*func, Expr::Identifier(ref name) if name == "f"));
            assert_eq!(args.len(), 0);
        }
        _ => panic!("Expected IdiomBracket expression, got: {:?}", expr),
    }
}

#[test]
fn test_parse_idiom_bracket_one_argument() {
    // [| f a |] - function with one argument
    let input = "[| f a |]";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::IdiomBracket { func, args } => {
            assert!(matches!(*func, Expr::Identifier(ref name) if name == "f"));
            assert_eq!(args.len(), 1);
            assert!(matches!(args[0], Expr::Identifier(ref name) if name == "a"));
        }
        _ => panic!("Expected IdiomBracket expression, got: {:?}", expr),
    }
}

#[test]
fn test_parse_idiom_bracket_two_arguments() {
    // [| f a b |] - function with two arguments
    let input = "[| f a b |]";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::IdiomBracket { func, args } => {
            assert!(matches!(*func, Expr::Identifier(ref name) if name == "f"));
            assert_eq!(args.len(), 2);
            assert!(matches!(args[0], Expr::Identifier(ref name) if name == "a"));
            assert!(matches!(args[1], Expr::Identifier(ref name) if name == "b"));
        }
        _ => panic!("Expected IdiomBracket expression, got: {:?}", expr),
    }
}

#[test]
fn test_parse_idiom_bracket_three_arguments() {
    // [| f a b c |] - function with three arguments
    let input = "[| f a b c |]";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::IdiomBracket { func, args } => {
            assert!(matches!(*func, Expr::Identifier(ref name) if name == "f"));
            assert_eq!(args.len(), 3);
            assert!(matches!(args[0], Expr::Identifier(ref name) if name == "a"));
            assert!(matches!(args[1], Expr::Identifier(ref name) if name == "b"));
            assert!(matches!(args[2], Expr::Identifier(ref name) if name == "c"));
        }
        _ => panic!("Expected IdiomBracket expression, got: {:?}", expr),
    }
}

#[test]
fn test_parse_nested_idiom_brackets() {
    // [| [| f a |] b |] - nested idiom brackets
    let input = "[| [| f a |] b |]";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::IdiomBracket { func, args } => {
            // The function should be an idiom bracket itself
            match *func {
                Expr::IdiomBracket {
                    func: inner_func,
                    args: inner_args,
                } => {
                    assert!(matches!(*inner_func, Expr::Identifier(ref name) if name == "f"));
                    assert_eq!(inner_args.len(), 1);
                    assert!(matches!(inner_args[0], Expr::Identifier(ref name) if name == "a"));
                }
                _ => panic!("Expected nested IdiomBracket in function position"),
            }
            assert_eq!(args.len(), 1);
            assert!(matches!(args[0], Expr::Identifier(ref name) if name == "b"));
        }
        _ => panic!("Expected IdiomBracket expression, got: {:?}", expr),
    }
}

#[test]
fn test_parse_idiom_bracket_with_qualified_identifier() {
    // [| map.async transform.data list.items |]
    let input = "[| map.async transform.data list.items |]";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::IdiomBracket { func, args } => {
            assert!(matches!(*func, Expr::Identifier(ref name) if name == "map.async"));
            assert_eq!(args.len(), 2);
            assert!(matches!(args[0], Expr::Identifier(ref name) if name == "transform.data"));
            assert!(matches!(args[1], Expr::Identifier(ref name) if name == "list.items"));
        }
        _ => panic!("Expected IdiomBracket expression, got: {:?}", expr),
    }
}

#[test]
fn test_parse_idiom_bracket_with_expressions() {
    // [| add (x) (y) |] - with parenthesized expressions
    // Note: The parser will parse 'add(x)(y)' as curried function calls,
    // so the entire expression becomes the function with no additional args
    let input = "[| add (x) (y) |]";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::IdiomBracket { func, args } => {
            // The parser treats 'add(x)(y)' as the function expression
            // This is curried function application
            match *func {
                Expr::Call { .. } => {
                    // The entire 'add(x)(y)' is parsed as nested calls
                    // No additional arguments after the function
                    assert_eq!(args.len(), 0);
                }
                Expr::Identifier(ref name) if name == "add" => {
                    // If somehow 'add' is the function, then (x) and (y) should be the args
                    assert_eq!(args.len(), 2);
                }
                _ => panic!(
                    "Expected either identifier or call as function, got: {:?}",
                    func
                ),
            }
        }
        _ => panic!("Expected IdiomBracket expression, got: {:?}", expr),
    }
}

#[test]
fn test_idiom_bracket_in_pipeline() {
    // data |> [| f a |] |> result
    let input = "data |> [| f a |] |> result";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    // The overall expression should be a pipe
    match expr {
        Expr::Binary {
            op: metadol::ast::BinaryOp::Pipe,
            ..
        } => {
            // Success - the idiom bracket is part of the pipeline
        }
        _ => panic!("Expected pipe expression at top level"),
    }
}

#[test]
fn test_idiom_bracket_with_many_arguments() {
    // [| func a b c d e f |] - many arguments
    let input = "[| func a b c d e f |]";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::IdiomBracket { func, args } => {
            assert!(matches!(*func, Expr::Identifier(ref name) if name == "func"));
            assert_eq!(args.len(), 6);
        }
        _ => panic!("Expected IdiomBracket expression"),
    }
}

#[test]
fn test_idiom_bracket_with_complex_function() {
    // [| (f >> g) a b |] - composed function
    let input = "[| (f >> g) a b |]";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::IdiomBracket { func, args } => {
            // Function should be a composition
            match *func {
                Expr::Binary {
                    op: metadol::ast::BinaryOp::Compose,
                    ..
                } => {}
                _ => panic!("Expected composition in function position"),
            }
            assert_eq!(args.len(), 2);
        }
        _ => panic!("Expected IdiomBracket expression"),
    }
}

// ============================================
// Error Cases
// ============================================

#[test]
fn test_unclosed_idiom_bracket() {
    // [| f a - missing closing |]
    let input = "[| f a";
    let mut parser = Parser::new(input);
    let result = parser.parse_expr(0);

    assert!(
        result.is_err(),
        "Should fail on unclosed idiom bracket, but got: {:?}",
        result
    );
}

#[test]
fn test_empty_idiom_bracket() {
    // [| |] - empty idiom bracket (no function)
    let input = "[| |]";
    let mut parser = Parser::new(input);
    let result = parser.parse_expr(0);

    // This should fail because we expect at least a function expression
    assert!(
        result.is_err(),
        "Should fail on empty idiom bracket, but got: {:?}",
        result
    );
}

#[test]
fn test_mismatched_brackets() {
    // [| f a ] - wrong closing bracket
    let input = "[| f a ]";
    let mut parser = Parser::new(input);
    let result = parser.parse_expr(0);

    assert!(
        result.is_err(),
        "Should fail on mismatched brackets, but got: {:?}",
        result
    );
}

#[test]
fn test_only_opening_bracket() {
    // [| - just the opening
    let input = "[|";
    let mut parser = Parser::new(input);
    let result = parser.parse_expr(0);

    assert!(
        result.is_err(),
        "Should fail with only opening bracket, but got: {:?}",
        result
    );
}

// ============================================
// Integration Tests
// ============================================

#[test]
fn test_idiom_bracket_multiple_in_expression() {
    // [| f a |] |> [| g b |]
    let input = "[| f a |] |> [| g b |]";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::Binary {
            op: metadol::ast::BinaryOp::Pipe,
            left,
            right,
        } => {
            assert!(matches!(*left, Expr::IdiomBracket { .. }));
            assert!(matches!(*right, Expr::IdiomBracket { .. }));
        }
        _ => panic!("Expected pipe with idiom brackets"),
    }
}

#[test]
fn test_idiom_bracket_as_function_argument() {
    // map([| f a |], list)
    let input = "map([| f a |], list)";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::Call { args, .. } => {
            assert_eq!(args.len(), 2);
            assert!(matches!(args[0], Expr::IdiomBracket { .. }));
        }
        _ => panic!("Expected function call with idiom bracket"),
    }
}

#[test]
fn test_idiom_bracket_in_let_binding() {
    // let result = [| add x y |];
    let input = "let result = [| add x y |];";
    let mut parser = Parser::new(input);
    let stmt = parser.parse_stmt().unwrap();

    match stmt {
        metadol::ast::Stmt::Let { value, .. } => {
            assert!(matches!(value, Expr::IdiomBracket { .. }));
        }
        _ => panic!("Expected let statement with idiom bracket"),
    }
}

#[test]
fn test_idiom_bracket_in_if_condition() {
    // if [| pred a |] { true } else { false }
    let input = "if [| pred a |] { true } else { false }";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::If { condition, .. } => {
            assert!(matches!(*condition, Expr::IdiomBracket { .. }));
        }
        _ => panic!("Expected if expression with idiom bracket condition"),
    }
}

#[test]
fn test_idiom_bracket_deeply_nested() {
    // [| [| [| f a |] b |] c |]
    let input = "[| [| [| f a |] b |] c |]";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    // Verify it's deeply nested
    match expr {
        Expr::IdiomBracket { func, .. } => match *func {
            Expr::IdiomBracket {
                func: inner_func, ..
            } => {
                assert!(matches!(*inner_func, Expr::IdiomBracket { .. }));
            }
            _ => panic!("Expected nested idiom bracket"),
        },
        _ => panic!("Expected IdiomBracket expression"),
    }
}

// ============================================
// Whitespace Handling Tests
// ============================================

#[test]
fn test_idiom_bracket_with_newlines() {
    let input = r#"[|
        f
        a
        b
    |]"#;
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::IdiomBracket { func, args } => {
            assert!(matches!(*func, Expr::Identifier(ref name) if name == "f"));
            assert_eq!(args.len(), 2);
        }
        _ => panic!("Expected IdiomBracket expression"),
    }
}

#[test]
fn test_idiom_bracket_minimal_spacing() {
    // [|f a b|] - minimal spacing
    let input = "[|f a b|]";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::IdiomBracket { func: _, args } => {
            assert_eq!(args.len(), 2);
        }
        _ => panic!("Expected IdiomBracket expression"),
    }
}

// ============================================
// Applicative Semantics Tests (Documentation)
// ============================================

/// This test documents the intended desugaring semantics.
/// [| f a b |] should desugar to: fmap(f, a) <*> b
/// Or in Haskell notation: f <$> a <*> b
#[test]
fn test_idiom_bracket_applicative_semantics_documentation() {
    // The AST captures the structure that will be desugared
    let input = "[| add x y |]";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0).unwrap();

    match expr {
        Expr::IdiomBracket { func, args } => {
            // Semantically, this represents:
            // fmap(add, x) <*> y
            // or: add <$> x <*> y
            assert!(matches!(*func, Expr::Identifier(ref name) if name == "add"));
            assert_eq!(args.len(), 2);

            // The desugaring would happen in a later compilation phase
            // For now, we just verify the AST structure is correct
        }
        _ => panic!("Expected IdiomBracket for applicative desugaring"),
    }
}
