# DOL v0.3.0 Release - HIR and Language Cleanup

> **Release Date**: December 2025
> **Status**: Released

## What's New in v0.3.0

DOL v0.3.0 introduces the **HIR (High-level Intermediate Representation)**, a major architectural improvement that simplifies the language while making it more consistent and easier to learn.

### Key Highlights

- **Reduced complexity**: 22 canonical node types (down from 50+)
- **Cleaner syntax**: `val`/`var` bindings replace `let`/`let mut`
- **Unified keywords**: `forall` replaces `each`, `all`, and `forall`
- **Standard terms**: `extends` replaces `derives from`
- **Better tooling**: New migration tool for automatic code updates

---

## New Syntax Overview

### Bindings: `val` and `var`

The new binding syntax provides clear symmetry between immutable values and mutable variables:

```dol
// Immutable value (cannot be changed)
val name = "DOL"
val count = 42

// Mutable variable (can be reassigned)
var counter = 0
counter = counter + 1
```

**Why the change?**
- `val` = value (immutable)
- `var` = variable (mutable)
- Clear, symmetric naming

### Type Declarations

While `gene` is still supported for backward compatibility, `type` is now the preferred keyword:

```dol
// Preferred (v0.3.0+)
pub type Container {
    id: UInt64
    name: String
    status: Status
}

// Still works (for compatibility)
pub gene Container {
    has id: UInt64
    has name: String
    has status: Status
}
```

### Inheritance: `extends`

The verbose `derives from` is replaced with the standard `extends`:

```dol
// Before
container derives from image

// After
container extends image

// In type declarations
pub type WebServer extends Server {
    port: UInt16
}
```

### Unified Quantifier: `forall`

Three keywords are unified into one:

```dol
// Before (three ways to say the same thing)
each item in items { validate(item) }
all items satisfy condition
forall x: T. predicate(x)

// After (one way)
forall item in items { validate(item) }
forall items satisfy condition
forall x: T. predicate(x)
```

---

## The HIR Architecture

### What is HIR?

HIR (High-level Intermediate Representation) is an internal representation that sits between parsing and code generation:

```
DOL Source → Parser → AST → Lowering → HIR → Codegen → Rust
```

### Benefits

1. **Simpler Codegen**: Only 22 node types to handle
2. **Canonical Forms**: One representation per concept
3. **Better Errors**: Cleaner error messages
4. **Easier Extensions**: Add new surface syntax without changing codegen

### Node Type Summary

| Category | Count | Examples |
|----------|-------|----------|
| Declarations | 4 | Type, Trait, Function, Module |
| Expressions | 12 | Literal, Call, If, Match, Lambda |
| Statements | 6 | Val, Var, Assign, Return, Break |

---

## Migration Guide

### Automatic Migration

Use the new `dol-migrate` command:

```bash
# Migrate a directory
dol migrate --from 0.2 --to 0.3 src/

# Preview changes first
dol migrate --from 0.2 --to 0.3 --dry-run src/
```

### Migration Rules

| v0.2.x | v0.3.0 |
|--------|--------|
| `let x = 1` | `val x = 1` |
| `let mut x = 1` | `var x = 1` |
| `each x in xs` | `forall x in xs` |
| `module foo` | `mod foo` |
| `derives from x` | `extends x` |
| `never valid` | `not valid` |

### Deprecation Warnings

In v0.3.0, deprecated syntax still works but produces warnings:

```
warning: `let` is deprecated, use `val` instead
  --> src/main.dol:5:1
   |
 5 | let x = 42
   | ^^^ deprecated
   |
   = help: replace with `val x = 42`
```

---

## API Changes

### New Compilation Functions

```rust
use metadol::codegen::{
    compile_to_rust_via_hir,
    compile_with_diagnostics,
};

// Simple compilation
let rust_code = compile_to_rust_via_hir(dol_source)?;

// Get diagnostics too
let (rust_code, warnings) = compile_with_diagnostics(dol_source)?;
```

### Low-Level HIR Access

```rust
use metadol::lower::lower_file;
use metadol::codegen::HirRustCodegen;

// Parse and lower to HIR
let (hir, ctx) = lower_file(source)?;

// Access symbols
let name = ctx.symbols.resolve(hir.name);

// Generate code
let mut codegen = HirRustCodegen::with_symbols(ctx.symbols);
let code = codegen.generate(&hir);
```

---

## Complete Example

### Before (v0.2.x)

```dol
module greeting.service @ 0.2.0

gene greeting.entity {
    entity has identity
    entity has name: String
    entity derives from template

    each greeting in greetings {
        greeting never empty
    }
}

trait entity.greetable {
    uses entity.identity
    greetable can greet
}

test "greeting works" {
    given g = Greeting.new("Hello")
    when g.send()
    then g.delivered == true
}
```

### After (v0.3.0)

```dol
mod greeting.service @ 0.3.0

type greeting.entity {
    identity: Identity
    name: String
    extends template

    forall greeting in greetings {
        not greeting.empty
    }
}

trait entity.greetable {
    use entity.identity
    greet: fun() -> Void
}

test "greeting works" {
    val g = Greeting.new("Hello")
    g.send()
    assert g.delivered == true
}
```

---

## Getting Started

### Installation

```bash
# Update to v0.3.0
cargo install metadol --version 0.3.0

# Or update via cargo
cargo update -p metadol
```

### Quick Start

1. **Migrate existing code**:
   ```bash
   dol migrate --from 0.2 --to 0.3 src/
   ```

2. **Write new code with v0.3.0 syntax**:
   ```dol
   mod myapp @ 0.1.0

   pub type User {
       id: UInt64
       name: String
       email: String
   }

   fun create_user(val name: String, val email: String) -> User {
       User { id: generate_id(), name, email }
   }
   ```

3. **Compile to Rust**:
   ```bash
   dol compile src/myapp.dol --output src/generated/
   ```

---

## Resources

- [HIR Tutorials](./HIR_Tutorials.md) - Detailed HIR documentation
- [Language Decisions](../dol/hir/DOL-v0.3.0-LANGUAGE-DECISIONS.md) - Design rationale
- [Examples](../examples/) - Code examples
- [API Documentation](https://docs.rs/metadol) - Rust API docs

---

## Backward Compatibility

| Feature | v0.3.0 | v0.4.0 | v1.0.0 |
|---------|--------|--------|--------|
| `let`/`mut` | Warning | Warning | Error |
| `gene` | Works | Soft warn | Warning |
| `each`/`all` | Warning | Error | Removed |
| `module` | Warning | Error | Removed |
| `derives from` | Warning | Error | Removed |

The `gene` keyword will continue to work indefinitely for Metal DOL ontological specifications.

---

*Questions? Visit [github.com/univrs/dol](https://github.com/univrs/dol) or join our Discord.*
