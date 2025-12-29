//! Spirit Compiler Demo
//!
//! Demonstrates the end-to-end DOL to WASM compilation pipeline.
//!
//! Run with:
//! ```bash
//! cargo run --example compiler_demo --features wasm
//! ```

#[cfg(feature = "wasm")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use metadol::compiler::spirit::{compile_source, CompilerError};

    println!("=== Spirit Compiler Demo ===\n");

    // Example 1: Compile a simple module
    println!("Example 1: Compiling empty module");
    let source1 = r#"
module example @ 1.0.0
"#;

    match compile_source(source1, "example.dol") {
        Ok(compiled) => {
            println!("✓ Compiled successfully");
            println!("  WASM size: {} bytes", compiled.wasm.len());
            println!("  Warnings: {}", compiled.warnings.len());
            println!(
                "  WASM magic: {:02x} {:02x} {:02x} {:02x}",
                compiled.wasm[0], compiled.wasm[1], compiled.wasm[2], compiled.wasm[3]
            );
        }
        Err(e) => {
            println!("✗ Compilation failed: {}", e);
        }
    }

    println!();

    // Example 2: Compile a gene
    println!("Example 2: Compiling gene declaration");
    let source2 = r#"
module counter @ 1.0.0

gene Counter {
    has value: Int64

    constraint non_negative {
        this.value >= 0
    }
}

exegesis {
    A simple counter that maintains a non-negative value.
}
"#;

    match compile_source(source2, "counter.dol") {
        Ok(compiled) => {
            println!("✓ Compiled successfully");
            println!("  WASM size: {} bytes", compiled.wasm.len());
            println!("  Warnings: {}", compiled.warnings.len());

            for (i, warning) in compiled.warnings.iter().enumerate() {
                println!("  Warning {}: {}", i + 1, warning.message);
                if let Some((file, line, col)) = &warning.location {
                    println!("    at {}:{}:{}", file, line, col);
                }
            }
        }
        Err(e) => {
            println!("✗ Compilation failed: {}", e);
        }
    }

    println!();

    // Example 3: Compile with functions (DOL 2.0)
    println!("Example 3: Compiling with functions");
    let source3 = r#"
module math @ 1.0.0

fun add(a: Int64, b: Int64) -> Int64 {
    return a + b
}

fun multiply(x: Int64, y: Int64) -> Int64 {
    return x * y
}
"#;

    match compile_source(source3, "math.dol") {
        Ok(compiled) => {
            println!("✓ Compiled successfully");
            println!("  WASM size: {} bytes", compiled.wasm.len());
            println!("  Warnings: {}", compiled.warnings.len());
        }
        Err(e) => {
            println!("✗ Compilation failed: {}", e);
        }
    }

    println!();

    // Example 4: Handle syntax errors
    println!("Example 4: Error handling");
    let source4 = r#"
module broken @ 1.0.0

gene Invalid {
    this is not valid DOL syntax!!!
}
"#;

    match compile_source(source4, "broken.dol") {
        Ok(_) => {
            println!("✗ Should have failed!");
        }
        Err(e) => {
            println!("✓ Correctly detected error");
            match e {
                CompilerError::ParseError(parse_err) => {
                    println!("  Parse error: {}", parse_err);
                }
                other => {
                    println!("  Error: {}", other);
                }
            }
        }
    }

    println!("\n=== Compilation Pipeline ===");
    println!("DOL Source → Lexer → Parser → AST → HIR → MLIR → WASM");
    println!("\nStatus:");
    println!("  ✓ Lexer:  Complete");
    println!("  ✓ Parser: Complete");
    println!("  ✓ AST:    Complete");
    println!("  ✓ HIR:    Complete");
    println!("  ⧗ MLIR:   In progress (skeleton)");
    println!("  ⧗ WASM:   In progress (placeholder)");
    println!("\nNote: Full MLIR → WASM lowering pipeline is complex");
    println!("      and will be completed in future phases.");
    println!("      Current implementation generates valid WASM structure.");

    Ok(())
}

#[cfg(not(feature = "wasm"))]
fn main() {
    eprintln!("This example requires the 'wasm' feature.");
    eprintln!("Run with: cargo run --example compiler_demo --features wasm");
    std::process::exit(1);
}
