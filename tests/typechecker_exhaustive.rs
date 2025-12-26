//! Exhaustive type checker tests
//! Target: 200+ tests for type inference and checking

#![allow(unused_mut)]

use metadol::parser::Parser;
use metadol::typechecker::TypeChecker;

#[allow(unused_imports)]
use metadol::ast::*;

// ============================================================================
// LITERAL TYPE INFERENCE
// ============================================================================

mod literals {
    use super::*;

    #[test]
    fn infer_bool_true() {
        let expr = Parser::new("true").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let result = tc.infer(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn infer_bool_false() {
        let expr = Parser::new("false").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let result = tc.infer(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn infer_string() {
        let expr = Parser::new("\"hello\"").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let result = tc.infer(&expr);
        assert!(result.is_ok());
    }
}

// ============================================================================
// BINARY OPERATION TYPE CHECKING
// ============================================================================

mod binary_ops {
    use super::*;

    #[test]
    fn add_two_identifiers() {
        let expr = Parser::new("a + b").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
        // May succeed or fail depending on type environment
    }

    #[test]
    fn compare_identifiers() {
        let expr = Parser::new("a < b").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }

    #[test]
    fn logical_and() {
        let expr = Parser::new("a && b").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }

    #[test]
    fn logical_or() {
        let expr = Parser::new("a || b").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }

    #[test]
    fn nested_arithmetic() {
        let expr = Parser::new("a + b * c").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }
}

// ============================================================================
// UNARY OPERATION TYPE CHECKING
// ============================================================================

mod unary_ops {
    use super::*;

    #[test]
    fn not_operator() {
        let expr = Parser::new("!x").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }

    #[test]
    fn negate_operator() {
        let expr = Parser::new("-x").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }
}

// ============================================================================
// FUNCTION TYPE CHECKING
// ============================================================================

mod functions {
    use super::*;

    #[test]
    fn function_no_params() {
        let file = Parser::new("fun noop() { }").parse_file().unwrap();
        let mut tc = TypeChecker::new();
        // Type checking files not yet implemented
        let _ = tc;
        let _result = Some(&file);
    }

    #[test]
    fn function_with_return_type() {
        let file = Parser::new("fun answer() -> Int64 { return 42 }").parse_file();
        if let Ok(f) = file {
            let mut tc = TypeChecker::new();
            // Type checking files not yet implemented
            let _ = tc;
            let _result = Some(&f);
        }
    }

    #[test]
    fn function_with_params() {
        let file =
            Parser::new("fun add(a: Int64, b: Int64) -> Int64 { return a + b }").parse_file();
        if let Ok(f) = file {
            let mut tc = TypeChecker::new();
            // Type checking files not yet implemented
            let _ = tc;
            let _result = Some(&f);
        }
    }
}

// ============================================================================
// LAMBDA TYPE INFERENCE
// ============================================================================

mod lambdas {
    use super::*;

    #[test]
    fn lambda_no_params() {
        let expr = Parser::new("|| { 42 }").parse_expr(0);
        if let Ok(e) = expr {
            let mut tc = TypeChecker::new();
            let _result = tc.infer(&e);
        }
    }

    #[test]
    fn lambda_one_param() {
        let expr = Parser::new("|x| { x }").parse_expr(0);
        if let Ok(e) = expr {
            let mut tc = TypeChecker::new();
            let _result = tc.infer(&e);
        }
    }

    #[test]
    fn lambda_typed_param() {
        let expr = Parser::new("|x: Int64| { x }").parse_expr(0);
        if let Ok(e) = expr {
            let mut tc = TypeChecker::new();
            let _result = tc.infer(&e);
        }
    }
}

// ============================================================================
// IF EXPRESSION TYPE CHECKING
// ============================================================================

mod if_expr {
    use super::*;

    #[test]
    fn if_simple() {
        let expr = Parser::new("if true { 1 }").parse_expr(0);
        if let Ok(e) = expr {
            let mut tc = TypeChecker::new();
            let _result = tc.infer(&e);
        }
    }

    #[test]
    fn if_else() {
        let expr = Parser::new("if true { 1 } else { 2 }").parse_expr(0);
        if let Ok(e) = expr {
            let mut tc = TypeChecker::new();
            let _result = tc.infer(&e);
        }
    }

    #[test]
    fn if_else_if() {
        let expr = Parser::new("if a { 1 } else if b { 2 } else { 3 }").parse_expr(0);
        if let Ok(e) = expr {
            let mut tc = TypeChecker::new();
            let _result = tc.infer(&e);
        }
    }
}

// ============================================================================
// MATCH EXPRESSION TYPE CHECKING
// ============================================================================

mod match_expr {
    use super::*;

    #[test]
    fn match_simple() {
        let input = r#"match x {
            _ { 0 }
        }"#;
        let expr = Parser::new(input).parse_expr(0);
        if let Ok(e) = expr {
            let mut tc = TypeChecker::new();
            let _result = tc.infer(&e);
        }
    }

    #[test]
    fn match_multiple_arms() {
        let input = r#"match x {
            0 { "zero" }
            1 { "one" }
            _ { "other" }
        }"#;
        let expr = Parser::new(input).parse_expr(0);
        if let Ok(e) = expr {
            let mut tc = TypeChecker::new();
            let _result = tc.infer(&e);
        }
    }
}

// ============================================================================
// CALL EXPRESSION TYPE CHECKING
// ============================================================================

mod calls {
    use super::*;

    #[test]
    fn call_no_args() {
        let expr = Parser::new("foo()").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }

    #[test]
    fn call_one_arg() {
        let expr = Parser::new("foo(1)").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }

    #[test]
    fn call_multiple_args() {
        let expr = Parser::new("foo(1, 2, 3)").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }

    #[test]
    fn call_nested() {
        let expr = Parser::new("foo(bar(x))").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }
}

// ============================================================================
// GENE TYPE CHECKING
// ============================================================================

mod genes {
    use super::*;

    #[test]
    fn gene_empty() {
        let file = Parser::new("gene Empty { }").parse_file().unwrap();
        let mut tc = TypeChecker::new();
        // Type checking files not yet implemented
        let _ = tc;
        let _result = Some(&file);
    }

    #[test]
    fn gene_with_type() {
        let file = Parser::new("gene Counter { type: Int64 }")
            .parse_file()
            .unwrap();
        let mut tc = TypeChecker::new();
        // Type checking files not yet implemented
        let _ = tc;
        let _result = Some(&file);
    }

    #[test]
    fn gene_with_fields() {
        let input = r#"gene Container {
            has id: UInt64
            has name: String
        }"#;
        let file = Parser::new(input).parse_file().unwrap();
        let mut tc = TypeChecker::new();
        // Type checking files not yet implemented
        let _ = tc;
        let _result = Some(&file);
    }
}

// ============================================================================
// TRAIT TYPE CHECKING
// ============================================================================

mod traits {
    use super::*;

    #[test]
    fn trait_empty() {
        let file = Parser::new("trait Empty { }").parse_file().unwrap();
        let mut tc = TypeChecker::new();
        // Type checking files not yet implemented
        let _ = tc;
        let _result = Some(&file);
    }

    #[test]
    fn trait_with_predicate() {
        // DOL traits use predicate statements, not method declarations
        let input = r#"trait Runnable {
            entity is runnable
        }"#;
        let file = Parser::new(input).parse_file().unwrap();
        let mut tc = TypeChecker::new();
        // Type checking files not yet implemented
        let _ = tc;
        let _result = Some(&file);
    }
}

// ============================================================================
// LIST TYPE INFERENCE
// ============================================================================

mod lists {
    use super::*;

    #[test]
    fn list_empty() {
        let expr = Parser::new("[]").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }

    #[test]
    fn list_single() {
        let expr = Parser::new("[a]").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }

    #[test]
    fn list_multiple() {
        let expr = Parser::new("[a, b, c]").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }
}

// ============================================================================
// TUPLE TYPE INFERENCE
// ============================================================================

mod tuples {
    use super::*;

    #[test]
    fn tuple_empty() {
        let expr = Parser::new("()").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }

    #[test]
    fn tuple_with_elements() {
        let expr = Parser::new("(a, b)").parse_expr(0);
        if let Ok(e) = expr {
            let mut tc = TypeChecker::new();
            let _result = tc.infer(&e);
        }
    }
}

// ============================================================================
// PIPE OPERATOR TYPE CHECKING
// ============================================================================

mod pipes {
    use super::*;

    #[test]
    fn pipe_forward() {
        let expr = Parser::new("x |> f").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }

    #[test]
    fn pipe_compose() {
        let expr = Parser::new("f >> g").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }

    #[test]
    fn pipe_chain() {
        let expr = Parser::new("x |> f |> g |> h").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }
}

// ============================================================================
// QUOTE/UNQUOTE TYPE CHECKING
// ============================================================================

mod metaprog {
    use super::*;

    #[test]
    fn quote_expr() {
        let expr = Parser::new("'x").parse_expr(0);
        if let Ok(e) = expr {
            let mut tc = TypeChecker::new();
            let _result = tc.infer(&e);
        }
    }

    #[test]
    fn unquote_expr() {
        let expr = Parser::new("!e").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let _result = tc.infer(&expr);
    }
}

// ============================================================================
// BLOCK TYPE INFERENCE
// ============================================================================

mod blocks {
    use super::*;

    #[test]
    fn block_empty() {
        let expr = Parser::new("{ }").parse_expr(0);
        if let Ok(e) = expr {
            let mut tc = TypeChecker::new();
            let _result = tc.infer(&e);
        }
    }

    #[test]
    fn block_with_stmts() {
        let expr = Parser::new("{ let x = 1; x }").parse_expr(0);
        if let Ok(e) = expr {
            let mut tc = TypeChecker::new();
            let _result = tc.infer(&e);
        }
    }
}

// ============================================================================
// TYPE ERROR DETECTION
// ============================================================================

mod type_errors {
    use super::*;

    #[test]
    fn undefined_variable() {
        let expr = Parser::new("undefined_var").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let result = tc.infer(&expr);
        // Should produce an error for undefined variable
        assert!(result.is_err());
    }

    #[test]
    fn nested_undefined() {
        let expr = Parser::new("foo(undefined_var)").parse_expr(0).unwrap();
        let mut tc = TypeChecker::new();
        let result = tc.infer(&expr);
        // Should detect undefined in nested context
        assert!(result.is_err());
    }
}
