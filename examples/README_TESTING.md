# DOL Examples Testing Documentation

This document tracks testing procedures, fixes, and known issues for the DOL examples.

## Testing Procedures

### 1. Build DOL Toolchain

```bash
# Build with CLI feature (required for dol-parse, dol-check, dol-test)
cargo build --release --features cli
```

### 2. Parse All Examples

```bash
# Run dol-parse on all .dol files
find examples -name "*.dol" -type f -exec \
  cargo run --release --features cli --bin dol-parse -- {} \;
```

### 3. Check All Examples (with validation)

```bash
# Run dol-check for validation and exegesis checking
cargo run --release --features cli --bin dol-check -- examples/
```

### 4. Run Full Test Suite

```bash
# Standard tests
cargo test

# With WASM compilation tests
cargo test --features wasm
```

### 5. Test DOL Test Generator

```bash
# Show what tests would be generated
cargo run --release --features cli --bin dol-test -- plan examples/container.lifecycle.dol.test

# Generate stub tests
cargo run --release --features cli --bin dol-test -- generate examples/container.lifecycle.dol.test --stubs --force
```

---

## Fixes Applied

### 2026-01-03: echo.dol - Reserved Keyword Fix

**File**: `examples/echo.dol`

**Issue**: Variable name `from` is a reserved keyword in DOL, causing parse error:
```
Parse error: expected expression, found 'from' at line 13, column 18
```

**Fix**: Renamed variable `from` to `sender_id`:
```dol
// Before (line 10-13):
let from = sender()
println("Echo received: " + msg)
send(from, msg)

// After:
let sender_id = sender()
println("Echo received: " + msg)
send(sender_id, msg)
```

**Verification**:
```bash
cargo run --release --features cli --bin dol-parse -- examples/echo.dol
# Output: ✓ examples/echo.dol (EchoSpirit)
```

---

## Test Results Summary (2026-01-03)

### Examples Parsing
- **Total**: 37
- **Passed**: 37 (100%)
- **Failed**: 0

### Unit Tests (cargo test)
- **Total**: 1,200+
- **Passed**: All
- **Failed**: 0

### WASM Tests (cargo test --features wasm)
- **Total**: 47
- **Passed**: 43
- **Failed**: 1
- **Ignored**: 3

---

## Known Issues

### 1. WASM Enum Comparison Bug

**Test**: `test_enum_type_mapping`
**Location**: `tests/wasm_execution.rs:944`
**Error**: `WasmError { message: "Wasmtime error: WebAssembly translation error" }`

**Description**: Comparing enum types with equality operator (`a == b`) in a function produces invalid WASM bytecode. This is an edge case in the WASM compiler.

**Workaround**: Avoid direct enum comparison in functions compiled to WASM. Use match expressions instead:
```dol
// Instead of: if a == b { ... }
// Use:
match a {
    AccountType.Node => match b {
        AccountType.Node => 1,
        _ => 0,
    },
    // ...
}
```

**Status**: Open - documented as known limitation

---

## Reserved Keywords

The following are reserved keywords in DOL and cannot be used as identifiers:

| Keyword | Category |
|---------|----------|
| `from` | Import/messaging |
| `in` | Loop iteration |
| `is` | Type checking |
| `has` | Field declaration |
| `if`, `else`, `match` | Control flow |
| `for`, `while`, `loop` | Loops |
| `break`, `continue`, `return` | Control flow |
| `fun`, `gene`, `trait`, `constraint`, `system` | Declarations |
| `true`, `false` | Boolean literals |
| `pub`, `use`, `module` | Visibility/imports |

---

## CLI Quick Reference

| Command | Description |
|---------|-------------|
| `dol-parse <file>` | Parse DOL file, output AST summary |
| `dol-check <dir>` | Validate DOL files with semantic checks |
| `dol-check --require-exegesis <dir>` | Require exegesis blocks |
| `dol-check --typecheck <dir>` | Enable DOL 2.0 type checking |
| `dol-test plan <file>` | Show test generation plan |
| `dol-test generate <file>` | Generate Rust tests from .dol.test |

---

## Adding New Examples

When adding new `.dol` examples:

1. **Avoid reserved keywords** as variable names
2. **Include exegesis blocks** for documentation
3. **Test parsing**: `cargo run --release --features cli --bin dol-parse -- <file>`
4. **Run validation**: `cargo run --release --features cli --bin dol-check -- <file>`
5. **Update this document** if fixes or workarounds are needed
