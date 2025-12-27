# DOL v0.3.0 Language Cleanup - Final Decisions

> **Status**: APPROVED
> **Date**: 2025-12-26
> **Target Release**: v0.3.0

---

## Executive Summary

DOL v0.3.0 introduces HIR (High-level Intermediate Representation) and uses it to clean up the language, reducing keywords from 93 to ~55 while improving consistency and readability.

### Key Changes

| Change | Before | After | Rationale |
|--------|--------|-------|-----------|
| **Bindings** | `let`/`mut` | `val`/`var` | Symmetry: val-ue vs var-iable |
| **Types** | `gene` | `type` (gene supported) | Unified, standard term |
| **Quantifiers** | `each`/`all`/`forall` | `forall` | One concept, one keyword |
| **Modules** | `module`/`mod` | `mod` | Shorter |
| **Negation** | `never`/`not` | `not` | Consistent |
| **Inheritance** | `derives from` | `extends` | Standard term |
| **Equality** | `matches` | `==` | Standard operator |
| **Tests** | `given`/`when`/`then` | Standard syntax | Less magic |
| **Evolution** | `adds`/`deprecates`/`removes` | `+`/`~`/`-` | Concise |

---

## 1. Binding Syntax: `val` / `var`

### Design

```
val = immutable VALUE binding
var = mutable VARiable binding
```

### Examples

```dol
// BEFORE (v0.2.x)
let x = 42              // immutable
let mut y = 0           // mutable (awkward!)

// AFTER (v0.3.0)
val x = 42              // immutable value
var y = 0               // mutable variable
```

### With Types

```dol
val name: String = "DOL"
var counter: Int64 = 0
```

### In Functions

```dol
fun process(val input: String, var state: State) -> Result {
    // input is immutable
    // state can be modified
}
```

### Migration

| v0.2.x | v0.3.0 | Status |
|--------|--------|--------|
| `let x = 1` | `val x = 1` | `let` deprecated, warning |
| `let mut x = 1` | `var x = 1` | `mut` deprecated, warning |
| `var x = 1` | `var x = 1` | Already works! |

---

## 2. Type Declaration: `type` (with `gene` compatibility)

### Design

- `type` is the **preferred** keyword for all type declarations
- `gene` remains **supported** for Metal DOL compatibility
- Both desugar to the same HIR form

### Examples

```dol
// PREFERRED (v0.3.0+)
pub type Container {
    id: UInt64
    name: String
    status: ContainerStatus
}

// SUPPORTED (backward compatibility)
pub gene Container {
    has id: UInt64
    has name: String
    has status: ContainerStatus
}

// Both compile to identical HIR!
```

### Unified Property Syntax

```dol
// BEFORE: Multiple predicate styles
entity has identity
entity is running
entity derives from image

// AFTER: Unified field syntax
type Entity {
    identity: Identity
    running: Bool
    source: Image        // 'extends' for inheritance
}
```

### Enum Types

```dol
pub type Status {
    kind: enum {
        Pending,
        Running,
        Complete,
        Failed { reason: String }
    }
}
```

### Migration Path

| Phase | `gene` | `type` |
|-------|--------|--------|
| v0.3.0 | ✅ Works (no warning) | ✅ Preferred |
| v0.4.0 | ⚠️ Soft deprecated | ✅ Preferred |
| v1.0.0 | ⚠️ Warning | ✅ Standard |
| v2.0.0 | ❌ Removed from DOL 2.0 | ✅ Only option |

**Note**: `gene` will always work in Metal DOL context for ontological specifications.

---

## 3. Quantifier Unification: `forall`

### Design

One universal quantifier: `forall`

### Examples

```dol
// BEFORE: Three keywords for same concept
each item in items { validate(item) }
all items satisfy condition
forall x: T. predicate(x)

// AFTER: One keyword
forall item in items { validate(item) }
forall items satisfy condition
forall x: T. predicate(x)
```

### Migration

| v0.2.x | v0.3.0 | Status |
|--------|--------|--------|
| `each x in xs` | `forall x in xs` | `each` deprecated |
| `all xs satisfy p` | `forall xs satisfy p` | `all` deprecated |
| `forall x: T` | `forall x: T` | Already works |

---

## 4. Module Keyword: `mod`

### Examples

```dol
// BEFORE
module dol.parser @ 0.3.0

// AFTER
mod dol.parser @ 0.3.0
```

### Migration

`module` → `mod` with deprecation warning in v0.3.0

---

## 5. Negation: `not`

### Examples

```dol
// BEFORE (Metal DOL)
value never overflows
condition never violated

// AFTER
value not overflows
condition not violated

// Code context (unchanged)
if !condition { }
if not condition { }  // Also valid
```

### Migration

`never` → `not` with deprecation warning

---

## 6. Inheritance: `extends`

### Examples

```dol
// BEFORE
container derives from image

// AFTER
container extends image

// In type declarations
pub type WebServer extends Server {
    port: UInt16
}
```

---

## 7. Equality: Standard `==`

### Examples

```dol
// BEFORE
entity matches expectation

// AFTER
entity == expectation
```

---

## 8. Test Syntax: Standard Assertions

### Examples

```dol
// BEFORE: Special test keywords
test "user creation" {
    given user = User.new()
    when user.activate()
    then user.is_active == true
}

// AFTER: Standard syntax
test "user creation" {
    val user = User.new()
    user.activate()
    assert user.is_active == true
}
```

---

## 9. Evolution Syntax: Operators

### Examples

```dol
// BEFORE: Verbose keywords
evolves UserV1 > UserV2 @ 2.0.0 {
    adds email: String = ""
    deprecates legacy_id
    removes temp_field
    because "adding email support"
}

// AFTER: Concise operators
evolves UserV1 > UserV2 @ 2.0.0 {
    + email: String = ""      // Added
    ~ legacy_id               // Deprecated  
    - temp_field              // Removed
    // "adding email support"  // Comment for rationale
}
```

---

## Keyword Summary

### Removed Keywords (14)

```
each        → forall
all         → forall
module      → mod
never       → not
derives     → extends
from        → (part of extends)
matches     → ==
given       → (removed)
when        → (removed)
then        → (removed)
adds        → +
deprecates  → ~
removes     → -
because     → // comment
```

### Changed Keywords (2)

```
let         → val (deprecated, then removed)
mut         → var (deprecated, then removed)
```

### New Keywords (2)

```
val         = immutable binding
extends     = inheritance/derivation
```

### Keyword Count

| Version | Count | Change |
|---------|-------|--------|
| v0.2.x | 93 | - |
| v0.3.0 | 81 | -12 (deprecations) |
| v1.0.0 | ~55 | -38 (removals) |

---

## Complete Syntax Comparison

### Before (v0.2.x)

```dol
module greeting.service @ 0.1.0

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

constraint bounds.valid {
    value never overflows
    min matches expected_min
}

test "greeting works" {
    given g = Greeting.new("Hello")
    when g.send()
    then g.delivered == true
}

evolves GreetingV1 > GreetingV2 @ 2.0.0 {
    adds timestamp: Int64 = 0
    removes legacy_format
    because "modernizing"
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

constraint bounds.valid {
    not value.overflows
    min == expected_min
}

test "greeting works" {
    val g = Greeting.new("Hello")
    g.send()
    assert g.delivered == true
}

evolves GreetingV1 > GreetingV2 @ 2.0.0 {
    + timestamp: Int64 = 0
    - legacy_format
    // "modernizing"
}
```

---

## Implementation Phases

### Phase 1: HIR Core (Week 1)
- Define HIR types with `Val`/`Var` statements
- Symbol interning
- Type representation

### Phase 2: Lowering (Week 2)
- AST → HIR conversion
- Desugaring rules for all deprecated syntax
- Both old and new syntax produce same HIR

### Phase 3: Parser Updates (Week 3)
- Add `val`, `var`, `type`, `extends` keywords
- Add deprecation warnings for old keywords
- Both syntaxes accepted

### Phase 4: Codegen Refactor (Week 4)
- Update Rust codegen to use HIR
- Simpler code (canonical forms only)

### Phase 5: Self-Hosting Update (Week 5)
- Update DOL compiler sources to new syntax
- Verify stage2/stage3 still work
- Release v0.3.0

---

## Auto-Migration Tool

```bash
# Migrate DOL files from v0.2 to v0.3 syntax
dol migrate --from 0.2 --to 0.3 src/

# Preview changes without applying
dol migrate --from 0.2 --to 0.3 --dry-run src/

# Migrate single file
dol migrate --from 0.2 --to 0.3 path/to/file.dol
```

### Migration Rules

```
let x        → val x
let mut x    → var x
gene Foo     → type Foo (optional)
each x in    → forall x in
all x        → forall x
module       → mod
never        → not
derives from → extends
matches      → ==
given        → val (in test)
when         → (inline statement)
then         → assert
adds         → +
deprecates   → ~
removes      → -
because "x"  → // x
```

---

## Backward Compatibility

| Syntax | v0.3.0 | v0.4.0 | v1.0.0 |
|--------|--------|--------|--------|
| `let` | ⚠️ Warning | ⚠️ Warning | ❌ Error |
| `mut` | ⚠️ Warning | ⚠️ Warning | ❌ Error |
| `gene` | ✅ Works | ⚠️ Soft warn | ⚠️ Warning |
| `each` | ⚠️ Warning | ❌ Error | ❌ Removed |
| `all` (quantifier) | ⚠️ Warning | ❌ Error | ❌ Removed |
| `module` | ⚠️ Warning | ❌ Error | ❌ Removed |
| `never` | ⚠️ Warning | ❌ Error | ❌ Removed |
| `derives from` | ⚠️ Warning | ❌ Error | ❌ Removed |
| `matches` | ⚠️ Warning | ❌ Error | ❌ Removed |
| `given/when/then` | ⚠️ Warning | ❌ Error | ❌ Removed |

---

*"Simplicity is the ultimate sophistication."* — Leonardo da Vinci
