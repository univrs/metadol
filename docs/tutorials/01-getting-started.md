# Tutorial 1: Getting Started with Metal DOL

## Welcome to Metal DOL

Metal DOL is a Domain-Specific Language (DSL) for **ontology-first development**. Instead of writing code and hoping it matches your mental model, you define your domain's ontology first, then generate code that perfectly aligns with it.

### What is the DOL-First Paradigm?

Traditional development follows a code-first approach:
1. Write code
2. Add comments/docs
3. Hope the implementation matches the design

DOL-first inverts this:
1. **Define your ontology** in Metal DOL (genes, traits, systems)
2. **Generate tests** that validate the ontology
3. **Implement code** that passes the tests

This ensures your implementation always matches your domain model.

### What Can You Build with Metal DOL?

Metal DOL excels at modeling:
- **Domain entities** (users, products, orders)
- **System behaviors** (lifecycle states, event flows)
- **Complex constraints** (business rules, invariants)
- **Evolutionary systems** (versioned APIs, migrations)

## Installation

### Prerequisites

You'll need Rust installed. If you don't have it:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Building Metal DOL

Clone and build the toolchain:

```bash
git clone https://github.com/univrs/metadol.git
cd metadol
cargo build --release
```

This compiles three CLI tools:
- `dol-parse` - Parse and validate DOL files
- `dol-test` - Generate tests from DOL specifications
- `dol-check` - CI validation gate

### Verify Installation

```bash
cargo test
cargo run --bin dol-parse -- --version
```

## Your First DOL File

Let's create a simple gene that models a container's existence.

Create `my-first.dol`:

```dol
gene container.exists @1.0.0 {
    """
    A container exists in the system and has a unique identifier.
    This is the most basic property of any container.
    """

    has identifier: string
    is entity
    is persistent
}
```

Let's break this down:

### Declaration Header

```dol
gene container.exists @1.0.0 {
```

- `gene` - Declares an atomic ontological unit
- `container.exists` - Qualified identifier (domain.property)
- `@1.0.0` - Semantic version
- Opening brace starts the body

### Exegesis (Required)

```dol
"""
A container exists in the system and has a unique identifier.
This is the most basic property of any container.
"""
```

Every DOL declaration **must** include exegesis (triple-quoted docstring). This is a core philosophy: your ontology must be self-documenting.

### Predicates

```dol
has identifier: string
is entity
is persistent
```

Predicates define properties and relationships:
- `has` - Declares typed properties
- `is` - Declares categorical membership
- `derives from` - Declares inheritance
- `requires` - Declares dependencies

## Running dol-parse

Parse your file:

```bash
cargo run --bin dol-parse -- my-first.dol
```

Success looks like:

```
Parsed successfully: my-first.dol
1 declaration(s) found

Gene: container.exists @1.0.0
  Properties:
    - identifier: string
  Categories:
    - entity
    - persistent
```

### Common Errors

**Missing exegesis:**

```dol
gene container.exists @1.0.0 {
    has identifier: string
}
```

```
Error: Missing exegesis for gene 'container.exists'
  --> my-first.dol:1:1
```

**Invalid version:**

```dol
gene container.exists @1.0 {
```

```
Error: Invalid version format, expected X.Y.Z
  --> my-first.dol:1:23
```

## Understanding the Output

When you run `dol-parse` with `--json`:

```bash
cargo run --bin dol-parse -- my-first.dol --json
```

You get a structured AST:

```json
{
  "declarations": [
    {
      "type": "Gene",
      "name": "container.exists",
      "version": "1.0.0",
      "exegesis": "A container exists...",
      "statements": [
        {
          "type": "Has",
          "property": "identifier",
          "property_type": "string"
        },
        {
          "type": "Is",
          "category": "entity"
        },
        {
          "type": "Is",
          "category": "persistent"
        }
      ]
    }
  ]
}
```

This AST can be used to:
- Generate Rust structs
- Generate database schemas
- Generate API specifications
- Generate tests

## Next Steps

Now that you can write and parse basic genes, explore:

1. **Tutorial 2: Writing Genes** - Deep dive into gene syntax and predicates
2. **Tutorial 3: Composing Traits** - Learn to build complex behaviors
3. **Tutorial 4: System Design** - Architect complete systems

### Explore Examples

Check out the examples directory:

```bash
ls examples/genes/
ls examples/traits/
ls examples/systems/
```

Parse them all:

```bash
find examples -name "*.dol" -exec cargo run --bin dol-parse -- {} \;
```

### Read the Specification

For complete language reference:

```bash
cat docs/specification.md
cat docs/grammar.ebnf
```

## Key Takeaways

1. **DOL-first development** means ontology before implementation
2. **Exegesis is mandatory** - your ontology documents itself
3. **Qualified identifiers** use dot notation (domain.property)
4. **Semantic versioning** is required for all declarations
5. **The parser validates** your ontology before code generation

Welcome to ontology-first development!
