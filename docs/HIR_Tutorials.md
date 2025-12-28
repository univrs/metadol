# DOL v0.3.0 HIR Tutorials

> **High-level Intermediate Representation for Ontology-First Development**

## Table of Contents

1. [Introduction to HIR](#introduction-to-hir)
2. [The Compilation Pipeline](#the-compilation-pipeline)
3. [HIR Node Types](#hir-node-types)
4. [Desugaring Rules](#desugaring-rules)
5. [DOL-in-DOL Development](#dol-in-dol-development)
6. [Code Generation](#code-generation)
7. [Migration Guide](#migration-guide)
8. [Examples](#examples)

---

## Introduction to HIR

HIR (High-level Intermediate Representation) is the canonical representation for DOL programs. It serves as the bridge between the surface syntax (what developers write) and code generation (what gets compiled).

### Why HIR?

| Aspect | Before (AST) | After (HIR) |
|--------|--------------|-------------|
| Node Types | 50+ | 22 |
| Keywords | 93 | ~55 |
| Representations per concept | Multiple | One |
| Codegen complexity | High | Low |

### Design Principles

1. **Minimal**: 22 node types cover all language constructs
2. **Canonical**: One representation per concept
3. **Typed**: All expressions carry type information
4. **Desugared**: No syntactic sugar remains

---

## The Compilation Pipeline

```
┌─────────────────────────────────────────────────────────────────────┐
│                    DOL v0.3.0 Compilation Pipeline                  │
└─────────────────────────────────────────────────────────────────────┘

    ┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
    │   DOL    │     │   AST    │     │   HIR    │     │   Rust   │
    │  Source  │ ──► │  (Parse) │ ──► │ (Lower)  │ ──► │ (Codegen)│
    └──────────┘     └──────────┘     └──────────┘     └──────────┘
         │                │                │                │
         │                │                │                │
         ▼                ▼                ▼                ▼
    .dol files       50+ nodes        22 nodes        .rs files
    val/var          let/mut          Val/Var         let/let mut
    forall           each/all         Loop+Match      for/while
    extends          derives from     Extends         : Parent
```

### Pipeline Stages

#### Stage 1: Parsing (Source → AST)

The parser accepts both old and new syntax:

```dol
// Both of these parse to the same AST:
let x = 42          // v0.2.x syntax (deprecated)
val x = 42          // v0.3.0 syntax (preferred)
```

#### Stage 2: Lowering (AST → HIR)

Lowering transforms the AST into canonical HIR form:

```rust
use metadol::lower::lower_file;

let source = r#"
gene point.2d {
    point has x
    point has y
}

exegesis {
    A 2D point with x and y coordinates.
}
"#;

let (hir, ctx) = lower_file(source)?;

// ctx contains:
// - Symbol table (interned strings)
// - Span map (source locations)
// - Diagnostics (deprecation warnings)
```

#### Stage 3: Code Generation (HIR → Rust)

The HIR codegen produces clean Rust code:

```rust
use metadol::codegen::compile_to_rust_via_hir;

let rust_code = compile_to_rust_via_hir(source)?;
// Produces:
// pub struct Point2d {
//     pub x: String,
//     pub y: String,
// }
```

---

## HIR Node Types

### Overview (22 Total)

| Category | Count | Types |
|----------|-------|-------|
| Declarations | 4 | Type, Trait, Function, Module |
| Expressions | 12 | Literal, Var, Binary, Unary, Call, MethodCall, Field, Index, Block, If, Match, Lambda |
| Statements | 6 | Val, Var, Assign, Expr, Return, Break |
| Types | 8 | Named, Tuple, Array, Function, Ref, Optional, Var, Error |
| Patterns | 6 | Wildcard, Var, Literal, Constructor, Tuple, Or |

### Declaration Forms

```dol
// All declarations desugar to one of 4 forms:

// 1. Type Declaration
pub type Container {
    id: UInt64
    name: String
    status: ContainerStatus
}

// 2. Trait Declaration
trait Lifecycle {
    start: fun() -> Void
    stop: fun() -> Void
}

// 3. Function Declaration
fun process(val input: String) -> Result {
    // ...
}

// 4. Module Declaration
mod container.runtime @ 0.3.0 {
    // nested declarations
}
```

### Expression Forms

```dol
// 12 expression forms in HIR:

// Atoms
val x = 42                      // Literal
val y = x                       // Var

// Compound
val sum = a + b                 // Binary
val neg = -x                    // Unary
val result = process(data)      // Call
val len = list.length()         // MethodCall
val name = user.name            // Field
val first = items[0]            // Index

// Control
val value = { stmt1; stmt2; expr }  // Block
val max = if a > b { a } else { b } // If
val msg = match status {            // Match
    Ok(v) => v,
    Err(e) => "error"
}

// Functions
val double = |x| x * 2          // Lambda
```

### Statement Forms

```dol
// 6 statement forms (note val/var, not let/mut!):

val x = 42              // Immutable binding
var counter = 0         // Mutable binding
counter = counter + 1   // Assignment
process(data)           // Expression statement
return result           // Return
break                   // Break (with optional value)
```

---

## Desugaring Rules

### Binding Syntax

| Surface Syntax | HIR Form | Status |
|----------------|----------|--------|
| `let x = 1` | `Val { name: x, ... }` | Deprecated |
| `val x = 1` | `Val { name: x, ... }` | Preferred |
| `let mut x = 1` | `Var { name: x, ... }` | Deprecated |
| `var x = 1` | `Var { name: x, ... }` | Preferred |

### Control Flow

| Surface Syntax | HIR Form |
|----------------|----------|
| `for x in xs { body }` | `Loop { Match(iter.next(), Some(x) => body, None => Break) }` |
| `while cond { body }` | `Loop { If(cond, body, Break) }` |
| `each x in xs { body }` | Same as `for` (deprecated) |

### Operators

| Surface Syntax | HIR Form |
|----------------|----------|
| `x \|> f \|> g` | `Call(g, Call(f, x))` |
| `f >> g` | `Lambda { Call(g, Call(f, param)) }` |
| `a && b` | `If(a, b, false)` |
| `a \|\| b` | `If(a, true, b)` |
| `x += 1` | `Assign(x, Binary(x, Add, 1))` |

### Keywords

| Before | After | HIR |
|--------|-------|-----|
| `each` | `forall` | Loop+Match |
| `all` | `forall` | Loop+Match |
| `module` | `mod` | Module |
| `never` | `not` | Unary(Not) |
| `derives from` | `extends` | Extends field |
| `matches` | `==` | Binary(Eq) |

---

## DOL-in-DOL Development

DOL is self-hosting - the compiler is written in DOL itself. Here's how the HIR types are defined in DOL:

### HIR Types in DOL Syntax

```dol
mod dol.hir @ 0.1.0

// Symbol interning for fast comparison
pub type Symbol {
    id: UInt32  // Index into symbol table
}

// HIR expression with attached type
pub type HirExpr {
    ty: HirType           // Every expression has a type
    kind: HirExprKind
}

// Expression kinds (12 forms)
pub type HirExprKind {
    kind: enum {
        // Atoms
        Lit { value: HirLit },
        Var { name: Symbol },

        // Compound
        Binary { left: Box<HirExpr>, op: BinOp, right: Box<HirExpr> },
        Unary { op: UnOp, operand: Box<HirExpr> },
        Call { func: Box<HirExpr>, args: List<HirExpr> },
        Field { expr: Box<HirExpr>, field: Symbol },
        Index { expr: Box<HirExpr>, index: Box<HirExpr> },

        // Control
        If { cond: Box<HirExpr>, then_: Box<HirExpr>, else_: Box<HirExpr> },
        Match { scrutinee: Box<HirExpr>, arms: List<HirArm> },
        Loop { body: Box<HirExpr>, label: Option<Symbol> },

        // Functions
        Lambda { params: List<HirParam>, body: Box<HirExpr> }
    }
}

// Statements (note val/var naming!)
pub type HirStmt {
    kind: enum {
        Val { name: Symbol, ty: HirType, value: HirExpr },  // Immutable
        Var { name: Symbol, ty: HirType, value: HirExpr },  // Mutable
        Assign { target: HirExpr, value: HirExpr },
        Expr { expr: HirExpr },
        Return { value: Option<HirExpr> },
        Break { value: Option<HirExpr>, label: Option<Symbol> }
    }
}
```

### Self-Hosted Compiler Example

From `dol/ast.dol` - the AST definitions in DOL:

```dol
mod dol.ast @ 0.3.0

/// DOL declaration - top-level construct
pub type Declaration {
    kind: enum {
        Gene {
            name: QualifiedName
            statements: List<Statement>
            exegesis: Option<Exegesis>
            extends: Option<QualifiedName>  // NEW in v0.3.0
        },
        Trait {
            name: QualifiedName
            statements: List<Statement>
            exegesis: Option<Exegesis>
        },
        Constraint {
            name: QualifiedName
            statements: List<Statement>
            exegesis: Option<Exegesis>
        },
        // ... more variants
    }
    span: Span
}

/// Statement within a declaration
pub type Statement {
    kind: enum {
        Has { subject: Identifier, property: Identifier, type_: Option<TypeExpr> },
        Is { subject: Identifier, type_name: Identifier },
        DerivesFrom { subject: Identifier, parent: Identifier },
        Requires { subject: Identifier, dependency: Identifier },
        Uses { subject: Identifier, resource: Identifier },
        Can { subject: Identifier, capability: Identifier }
    }
    span: Span
}
```

---

## Code Generation

### Using the HIR Codegen

```rust
use metadol::codegen::{compile_to_rust_via_hir, compile_with_diagnostics};

// Simple compilation
let rust_code = compile_to_rust_via_hir(dol_source)?;

// With diagnostics (for deprecation warnings)
let (rust_code, diagnostics) = compile_with_diagnostics(dol_source)?;
for diag in diagnostics {
    eprintln!("Warning: {}", diag);
}
```

### Generated Code Example

**Input (DOL):**
```dol
gene container.runtime {
    container has id
    container has name
    container has status
    container extends image
}

exegesis {
    A runtime container instance.
}
```

**Output (Rust):**
```rust
// Generated from DOL HIR
// Source: container.runtime

/// A runtime container instance.
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerRuntime {
    pub id: String,
    pub name: String,
    pub status: String,
}
```

---

## Migration Guide

### Automatic Migration

Use the `dol-migrate` tool:

```bash
# Migrate all files in a directory
dol migrate --from 0.2 --to 0.3 src/

# Preview changes without applying
dol migrate --from 0.2 --to 0.3 --dry-run src/

# Migrate a single file
dol migrate --from 0.2 --to 0.3 path/to/file.dol
```

### Manual Migration Checklist

| Before (v0.2.x) | After (v0.3.0) |
|-----------------|----------------|
| `let x = 1` | `val x = 1` |
| `let mut x = 1` | `var x = 1` |
| `each x in xs` | `forall x in xs` |
| `all xs satisfy p` | `forall xs satisfy p` |
| `module foo` | `mod foo` |
| `never empty` | `not empty` |
| `derives from parent` | `extends parent` |
| `matches expected` | `== expected` |
| `given x = ...` | `val x = ...` |
| `then assert` | `assert` |

### Deprecation Timeline

| Syntax | v0.3.0 | v0.4.0 | v1.0.0 |
|--------|--------|--------|--------|
| `let` | Warning | Warning | Error |
| `mut` | Warning | Warning | Error |
| `gene` | Works | Soft warn | Warning |
| `each` | Warning | Error | Removed |
| `module` | Warning | Error | Removed |

---

## Examples

### Complete Gene with HIR

```dol
mod biology.mycelium @ 0.1.0

/// Fungal network node representing a mycelium junction
pub type MyceliumNode {
    id: UInt64
    position: Vec3
    connections: List<NodeConnection>
    nutrients: Float64
    age: Duration
}

/// Connection between mycelium nodes
pub type NodeConnection {
    target: UInt64
    strength: Float64
    flow_rate: Float64
}

/// Network operations trait
trait NetworkOps {
    /// Find path between nodes
    find_path: fun(from: UInt64, to: UInt64) -> Option<List<UInt64>>

    /// Distribute nutrients across network
    distribute: fun(amount: Float64) -> Void

    /// Prune weak connections
    prune: fun(threshold: Float64) -> UInt32
}

/// Simulation constraints
constraint NetworkHealth {
    forall node in nodes {
        not node.nutrients.is_negative
        node.connections.length > 0
    }
}
```

### Trait with Default Implementations

```dol
trait Lifecycle {
    /// Start the entity
    start: fun() -> Result<Void, Error>

    /// Stop the entity
    stop: fun() -> Result<Void, Error>

    /// Check if running
    is_running: fun() -> Bool = { false }  // default impl

    /// Restart (default uses start/stop)
    restart: fun() -> Result<Void, Error> = {
        self.stop()?
        self.start()
    }
}
```

### Evolution Declaration

```dol
evolves ContainerV1 > ContainerV2 @ 2.0.0 {
    + created_at: Timestamp = Timestamp.now()  // Added
    + labels: Map<String, String> = {}         // Added
    ~ legacy_id                                 // Deprecated
    - temp_storage                              // Removed
    // "Adding timestamp and labels support"
}
```

---

## Quick Reference

### HIR Node Count: 22

```
Declarations:  4  (Type, Trait, Function, Module)
Expressions:  12  (Lit, Var, Binary, Unary, Call, MethodCall,
                   Field, Index, Block, If, Match, Lambda)
Statements:    6  (Val, Var, Assign, Expr, Return, Break)
```

### Keyword Changes Summary

```
let      → val     (immutable value)
let mut  → var     (mutable variable)
gene     → type    (gradual migration)
each/all → forall  (unified quantifier)
module   → mod     (shorter)
never    → not     (consistent)
derives from → extends (standard term)
matches  → ==      (standard operator)
```

### Compilation API

```rust
// Simple
let code = compile_to_rust_via_hir(source)?;

// With diagnostics
let (code, warnings) = compile_with_diagnostics(source)?;

// Low-level access
let (hir, ctx) = lower_file(source)?;
let mut codegen = HirRustCodegen::with_symbols(ctx.symbols);
let code = codegen.generate(&hir);
```

---

*"Simplicity is the ultimate sophistication."* — Leonardo da Vinci
