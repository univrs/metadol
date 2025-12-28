# DOL Self-Hosting: The DOL-in-DOL Compiler

> **Building a Language That Compiles Itself**

## Overview

DOL is a **self-hosting** language - the DOL compiler is written in DOL itself. This document explains how the self-hosting compiler works and how to contribute to it.

---

## Architecture

### The Bootstrap Problem

Every self-hosting compiler faces a chicken-and-egg problem: you need a compiler to compile the compiler. DOL solves this with a three-stage bootstrap:

```
Stage 0: Rust Implementation (src/)
         ├── Lexer (logos)
         ├── Parser (recursive descent)
         ├── HIR (canonical representation)
         └── Codegen (Rust output)
              │
              ▼
Stage 1: DOL Compiler in DOL (dol/)
         ├── dol/ast.dol (AST definitions)
         ├── dol/parser.dol (parser logic)
         ├── dol/hir/ (HIR types)
         └── dol/codegen.dol (code generation)
              │
              ▼
Stage 2: Self-Compiled DOL Compiler
         (Compiler compiled by Stage 1)
              │
              ▼
Stage 3: Verification
         (Stage 2 compiles itself, output matches Stage 2)
```

### Directory Structure

```
dol/                          # Self-hosted DOL compiler
├── ast.dol                   # AST type definitions (1033 lines)
├── parser.dol                # Parser implementation
├── lexer.dol                 # Lexer/tokenizer
├── codegen.dol               # Code generation
├── types.dol                 # Type system
├── hir/                      # HIR module
│   ├── dol-hir-types-v2.dol  # HIR type definitions
│   └── DOL-v0.3.0-LANGUAGE-DECISIONS.md
└── stdlib/                   # Standard library in DOL
    ├── functor.dol
    ├── applicative.dol
    └── monad.dol
```

---

## The AST in DOL

The AST is defined in `dol/ast.dol`. Here's a simplified view:

```dol
mod dol.ast @ 0.3.0

/// A DOL source file
pub type SourceFile {
    path: String
    declarations: List<Declaration>
    module_decl: Option<ModuleDecl>
}

/// Top-level declaration
pub type Declaration {
    kind: DeclKind
    span: Span
}

pub type DeclKind {
    kind: enum {
        Gene {
            name: QualifiedName
            statements: List<Statement>
            exegesis: Option<Exegesis>
            extends: Option<QualifiedName>
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
        System {
            name: QualifiedName
            components: List<SystemComponent>
            exegesis: Option<Exegesis>
        },
        Evolution {
            from_type: QualifiedName
            to_type: QualifiedName
            version: Version
            changes: List<EvolutionChange>
        },
        Function {
            name: Identifier
            params: List<Parameter>
            return_type: Option<TypeExpr>
            body: Option<Expression>
        }
    }
}
```

### Statement Types

```dol
/// Statement within a gene/trait/constraint
pub type Statement {
    kind: enum {
        // entity has property
        Has {
            subject: Identifier
            property: Identifier
            type_: Option<TypeExpr>
        },
        // entity is type_name
        Is {
            subject: Identifier
            type_name: Identifier
        },
        // entity extends parent (v0.3.0)
        Extends {
            subject: Identifier
            parent: Identifier
        },
        // entity requires dependency
        Requires {
            subject: Identifier
            dependency: Identifier
        },
        // entity uses resource
        Uses {
            subject: Identifier
            resource: Identifier
        },
        // entity can capability
        Can {
            subject: Identifier
            capability: Identifier
        }
    }
    span: Span
}
```

---

## The HIR in DOL

The HIR provides a canonical representation. From `dol/hir/dol-hir-types-v2.dol`:

```dol
mod dol.hir @ 0.1.0

// ============================================================================
// SYMBOLS & IDENTIFIERS
// ============================================================================

/// Interned string symbol for fast comparison
pub type Symbol {
    id: UInt32  // Index into symbol table
}

/// Qualified name (e.g., dol.hir.Symbol)
pub type QualName {
    segments: List<Symbol>
}

// ============================================================================
// TYPES (8 forms)
// ============================================================================

pub type HirType {
    kind: enum {
        // Primitives
        Void,
        Bool,
        Int { bits: UInt8, signed: Bool },
        Float { bits: UInt8 },
        String,

        // Compound
        Array { elem: Box<HirType> },
        Tuple { elems: List<HirType> },
        Record { fields: List<(Symbol, HirType)> },
        Function { params: List<HirType>, ret: Box<HirType> },

        // Named
        Named { name: QualName, args: List<HirType> },

        // Generic
        Generic { name: Symbol },

        // Inference placeholder
        Infer { id: UInt32 }
    }
}

// ============================================================================
// STATEMENTS (6 forms) - Note val/var naming!
// ============================================================================

pub type HirStmt {
    kind: enum {
        // Bindings
        Val { name: Symbol, ty: HirType, value: HirExpr },  // Immutable
        Var { name: Symbol, ty: HirType, value: HirExpr },  // Mutable

        // Effects
        Assign { target: HirExpr, value: HirExpr },
        Expr { expr: HirExpr },

        // Control
        Return { value: Option<HirExpr> },
        Break { value: Option<HirExpr>, label: Option<Symbol> }
    }
}
```

---

## Standard Library Traits

DOL's standard library includes foundational type classes. From `dol/stdlib/`:

### Functor

```dol
mod dol.stdlib.functor @ 0.1.0

/// Functor - types that can be mapped over
///
/// Laws:
/// 1. Identity: map(id) == id
/// 2. Composition: map(f >> g) == map(f) >> map(g)
pub trait Functor<F> {
    /// Map a function over the functor
    map: fun<A, B>(fa: F<A>, f: fun(A) -> B) -> F<B>
}

// Example instances
impl Functor for List {
    map = |list, f| list.map_items(f)
}

impl Functor for Option {
    map = |opt, f| match opt {
        Some(a) => Some(f(a)),
        None => None
    }
}
```

### Monad

```dol
mod dol.stdlib.monad @ 0.1.0

/// Monad - sequencing computations
///
/// Laws:
/// 1. Left identity: pure(a) >>= f == f(a)
/// 2. Right identity: m >>= pure == m
/// 3. Associativity: (m >>= f) >>= g == m >>= (|x| f(x) >>= g)
pub trait Monad<M> extends Applicative<M> {
    /// Bind/flatMap
    bind: fun<A, B>(ma: M<A>, f: fun(A) -> M<B>) -> M<B>

    /// Also known as >>=
    >>= : fun<A, B>(ma: M<A>, f: fun(A) -> M<B>) -> M<B> = bind
}
```

---

## Contributing to Self-Hosting

### Development Workflow

1. **Make changes to Stage 0 (Rust)**:
   ```bash
   # Edit src/*.rs files
   cargo test
   cargo build
   ```

2. **Update Stage 1 (DOL) to match**:
   ```bash
   # Edit dol/*.dol files
   dol check dol/
   ```

3. **Verify bootstrap**:
   ```bash
   # Stage 1 compiles itself
   dol compile dol/ --output stage2/

   # Stage 2 compiles itself
   ./stage2/dol compile dol/ --output stage3/

   # Verify stage2 == stage3
   diff -r stage2/ stage3/
   ```

### Adding New Syntax

When adding new syntax to DOL:

1. **Update the Rust lexer** (`src/lexer.rs`)
2. **Update the Rust parser** (`src/parser.rs`)
3. **Update the Rust AST** (`src/ast.rs`)
4. **Add HIR support** (`src/hir/types.rs`)
5. **Update lowering** (`src/lower/`)
6. **Update codegen** (`src/codegen/`)
7. **Mirror in DOL** (`dol/ast.dol`, `dol/parser.dol`, etc.)
8. **Run full test suite**:
   ```bash
   cargo test
   dol test
   ```

### Testing Self-Hosting

```bash
# Run all tests including self-hosting verification
cargo test --features self-hosting

# Specific self-hosting tests
cargo test bootstrap

# Verify AST definitions match
dol verify-sync src/ast.rs dol/ast.dol
```

---

## Real Example: Adding `extends`

Here's how the `extends` keyword was added to v0.3.0:

### 1. Rust Lexer

```rust
// src/lexer.rs
#[derive(Logos, Debug, Clone, PartialEq)]
pub enum TokenKind {
    // ...
    #[token("extends")]
    Extends,
}
```

### 2. Rust AST

```rust
// src/ast.rs
pub struct Gene {
    pub name: String,
    pub statements: Vec<Statement>,
    pub exegesis: Option<Exegesis>,
    pub extends: Option<String>,  // NEW
}
```

### 3. Rust Parser

```rust
// src/parser.rs
fn parse_gene(&mut self) -> Result<Gene> {
    // ...
    let extends = if self.check(TokenKind::Extends) {
        self.advance();
        Some(self.parse_identifier()?)
    } else {
        None
    };
    // ...
}
```

### 4. HIR Type

```rust
// src/hir/types.rs
pub struct HirTypeDecl {
    // ...
    pub extends: Option<Symbol>,  // NEW
}
```

### 5. DOL AST

```dol
// dol/ast.dol
pub type DeclKind {
    kind: enum {
        Gene {
            name: QualifiedName
            statements: List<Statement>
            exegesis: Option<Exegesis>
            extends: Option<QualifiedName>  // NEW
        }
        // ...
    }
}
```

### 6. DOL HIR

```dol
// dol/hir/dol-hir-types-v2.dol
pub type HirDecl {
    kind: enum {
        Type {
            name: Symbol
            extends: Option<QualName>  // NEW
            // ...
        }
    }
}
```

---

## Resources

- [HIR Tutorials](./HIR_Tutorials.md) - HIR architecture details
- [Language Decisions](../dol/hir/DOL-v0.3.0-LANGUAGE-DECISIONS.md) - v0.3.0 design
- [dol/ast.dol](../dol/ast.dol) - Full AST in DOL
- [dol/hir/](../dol/hir/) - HIR types in DOL

---

*"A language that doesn't affect the way you think about programming is not worth knowing."* — Alan Perlis
