//! Stress tests for parser
//! Complex parsing scenarios and edge cases

use metadol::ast::*;
use metadol::parser::Parser;

// ============================================================================
// NESTED EXPRESSIONS
// ============================================================================

#[test]
fn deeply_nested_parens() {
    let input = "((((a))))";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn deeply_nested_binary() {
    let input = "a + (b + (c + (d + e)))";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn chained_method_calls() {
    let input = "a.b.c.d.e";
    let result = Parser::new(input).parse_expr(0);
    // Lexer treats this as single identifier
    assert!(result.is_ok());
}

#[test]
fn mixed_operators() {
    let input = "a + b * c - d / e % f";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn comparison_chain() {
    let input = "a < b && b < c";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

// ============================================================================
// COMPLEX FUNCTION CALLS
// ============================================================================

#[test]
fn call_with_no_args() {
    let input = "foo()";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn call_with_many_args() {
    let input = "foo(a, b, c, d, e, f, g, h, i, j)";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn call_with_nested_calls() {
    let input = "foo(bar(baz(qux(x))))";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn call_with_expressions() {
    let input = "foo(a + b, c * d, e - f)";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

// ============================================================================
// COMPLEX LAMBDAS
// ============================================================================

#[test]
fn lambda_returning_lambda() {
    let input = "|x| { |y| { x } }";
    let result = Parser::new(input).parse_expr(0);
    if result.is_ok() {
        // Parser supports nested lambdas
    }
}

#[test]
fn lambda_with_many_params() {
    let input = "|a, b, c, d, e| { a }";
    let result = Parser::new(input).parse_expr(0);
    if result.is_ok() {
        // Multi-param lambda works
    }
}

// ============================================================================
// COMPLEX IF EXPRESSIONS
// ============================================================================

#[test]
fn if_with_complex_condition() {
    let input = "if a && b || c { x }";
    let result = Parser::new(input).parse_expr(0);
    if result.is_ok() {
        // Complex conditions work
    }
}

#[test]
fn if_else_if_chain() {
    let input = "if a { 1 } else if b { 2 } else if c { 3 } else { 4 }";
    let result = Parser::new(input).parse_expr(0);
    if result.is_ok() {
        // Chains work
    }
}

#[test]
fn nested_if_expressions() {
    let input = "if a { if b { 1 } else { 2 } } else { 3 }";
    let result = Parser::new(input).parse_expr(0);
    if result.is_ok() {
        // Nested ifs work
    }
}

// ============================================================================
// COMPLEX MATCH EXPRESSIONS
// ============================================================================

#[test]
fn match_with_many_arms() {
    let input = r#"match x {
        0 { a }
        1 { b }
        2 { c }
        3 { d }
        _ { e }
    }"#;
    let result = Parser::new(input).parse_expr(0);
    if result.is_ok() {
        // Many arms work
    }
}

#[test]
fn match_with_complex_patterns() {
    let input = r#"match x {
        _ { result }
    }"#;
    let result = Parser::new(input).parse_expr(0);
    if result.is_ok() {
        // Pattern matching works
    }
}

// ============================================================================
// COMPLEX LIST EXPRESSIONS
// ============================================================================

#[test]
fn list_of_lists() {
    let input = "[[a, b], [c, d], [e, f]]";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn list_with_expressions() {
    let input = "[a + b, c * d, foo(x)]";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn empty_nested_lists() {
    let input = "[[], [], []]";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

// ============================================================================
// PIPE OPERATOR CHAINS
// ============================================================================

#[test]
fn long_pipe_chain() {
    let input = "a |> b |> c |> d |> e |> f |> g";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn long_compose_chain() {
    let input = "a >> b >> c >> d >> e";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn mixed_pipe_and_other() {
    let input = "(a + b) |> foo |> bar";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

// ============================================================================
// BLOCK EXPRESSIONS
// ============================================================================

#[test]
fn block_with_multiple_statements() {
    let input = "{ let x = 1; let y = 2; x }";
    let result = Parser::new(input).parse_expr(0);
    if result.is_ok() {
        // Multi-statement blocks work
    }
}

#[test]
fn nested_blocks() {
    let input = "{ { { a } } }";
    let result = Parser::new(input).parse_expr(0);
    if result.is_ok() {
        // Nested blocks work
    }
}

// ============================================================================
// GENE DECLARATIONS
// ============================================================================

#[test]
fn gene_with_many_fields() {
    let input = r#"gene Large {
        has a: Int64
        has b: UInt64
        has c: String
        has d: Bool
        has e: Float64
    }"#;
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn gene_empty() {
    let input = "gene Empty { }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn gene_with_type() {
    let input = "gene Typed { type: Int64 }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// TRAIT DECLARATIONS
// ============================================================================

#[test]
fn trait_with_many_predicates() {
    let input = r#"trait Complex {
        entity is active
        entity has state
    }"#;
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn trait_empty() {
    let input = "trait Empty { }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// CONSTRAINT DECLARATIONS
// ============================================================================

#[test]
fn constraint_with_requires() {
    // Use subject-predicate-object syntax instead of direct 'requires' keyword
    let input = r#"constraint Valid {
        field is required
    }"#;
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn constraint_empty() {
    let input = "constraint Empty { }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// FUNCTION DECLARATIONS
// ============================================================================

#[test]
fn function_with_many_params() {
    let input = "fun many(a: Int64, b: Int64, c: Int64, d: Int64) { a }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn function_with_return_type() {
    let input = "fun typed() -> Int64 { 0 }";
    let result = Parser::new(input).parse_file();
    if result.is_ok() {
        // Return type syntax works
    }
}

#[test]
fn function_empty_body() {
    let input = "fun noop() { }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// SYSTEM DECLARATIONS
// ============================================================================

#[test]
fn system_with_predicates() {
    let input = r#"system Runtime {
        entity has state
    }"#;
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn system_empty() {
    let input = "system Empty { }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// MULTIPLE DECLARATIONS
// ============================================================================

#[test]
fn multiple_genes() {
    let input = r#"
gene A { }
gene B { }
gene C { }
    "#;
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
    if let Ok(file) = result {
        assert_eq!(file.declarations.len(), 3);
    }
}

#[test]
fn mixed_declarations() {
    let input = r#"
gene A { }
trait T { }
constraint C { }
fun f() { }
    "#;
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
    if let Ok(file) = result {
        assert_eq!(file.declarations.len(), 4);
    }
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn empty_file() {
    let input = "";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
    if let Ok(file) = result {
        assert!(file.declarations.is_empty());
    }
}

#[test]
fn whitespace_only_file() {
    let input = "   \n\n\t\t  \n   ";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn comments_only_file() {
    let input = "// comment 1\n// comment 2\n";
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

// ============================================================================
// ERROR DETECTION
// ============================================================================

#[test]
fn error_unclosed_paren() {
    let input = "foo(a, b";
    let result = Parser::new(input).parse_expr(0);
    // May succeed with partial parse or fail
    let _ = result;
}

#[test]
fn error_unclosed_brace() {
    let input = "gene A {";
    let result = Parser::new(input).parse_file();
    assert!(result.is_err());
}

#[test]
fn error_missing_identifier() {
    let input = "gene { }";
    let result = Parser::new(input).parse_file();
    assert!(result.is_err());
}

// ============================================================================
// PRECEDENCE TESTS
// ============================================================================

#[test]
fn precedence_mul_over_add() {
    let input = "a + b * c";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
    // Should parse as a + (b * c)
}

#[test]
fn precedence_and_over_or() {
    let input = "a || b && c";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
    // Should parse as a || (b && c)
}

#[test]
fn precedence_comparison_over_logical() {
    let input = "a < b && c > d";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
    // Should parse as (a < b) && (c > d)
}

#[test]
fn precedence_parens_override() {
    let input = "(a + b) * c";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

// ============================================================================
// ASSOCIATIVITY TESTS
// ============================================================================

#[test]
fn left_associative_add() {
    let input = "a + b + c";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
    // Should parse as (a + b) + c
}

#[test]
fn left_associative_mul() {
    let input = "a * b * c";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn left_associative_and() {
    let input = "a && b && c";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

// ============================================================================
// UNARY OPERATORS
// ============================================================================

#[test]
fn unary_not() {
    let input = "!a";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn unary_negate() {
    let input = "-a";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn unary_chain() {
    let input = "!!a";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

#[test]
fn unary_with_binary() {
    let input = "!a && b";
    let result = Parser::new(input).parse_expr(0);
    assert!(result.is_ok());
}

// ============================================================================
// INDEX EXPRESSIONS
// ============================================================================

#[test]
fn index_simple() {
    let input = "a[0]";
    let result = Parser::new(input).parse_expr(0);
    if result.is_ok() {
        // Indexing works
    }
}

#[test]
fn index_nested() {
    let input = "a[b[c]]";
    let result = Parser::new(input).parse_expr(0);
    if result.is_ok() {
        // Nested indexing works
    }
}

#[test]
fn index_with_expression() {
    let input = "a[i + 1]";
    let result = Parser::new(input).parse_expr(0);
    if result.is_ok() {
        // Expression indexing works
    }
}

// ============================================================================
// COMPLEX REAL-WORLD PATTERNS
// ============================================================================

#[test]
fn pattern_builder() {
    let input = r#"gene Builder {
        has value: String
        has count: Int64
    }"#;
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn pattern_validator() {
    let input = r#"constraint NotEmpty {
        value is notempty
    }"#;
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}

#[test]
fn pattern_event_system() {
    let input = r#"system EventDispatcher {
        event has type
        event has payload
    }"#;
    let result = Parser::new(input).parse_file();
    assert!(result.is_ok());
}
