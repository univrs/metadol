# Getting Started with DOL 2.0

## Installation

```bash
# Clone the repository
git clone https://github.com/univrs/dol.git
cd dol

# Build with all features
cargo build --release --all-features

# Install CLI tools
cargo install --path . --features cli
```

## Your First DOL File

Create `hello.dol`:

```dol
module hello @ 1.0.0

/// A simple greeting gene
pub gene Greeting {
    has message: String = "Hello, World!"
    has recipient: String = "Universe"

    fun greet() -> String {
        return this.message + ", " + this.recipient + "!"
    }

    constraint non_empty {
        this.message.length() > 0 && this.recipient.length() > 0
    }

    exegesis {
        Greeting demonstrates basic DOL 2.0 syntax:
        - Module declaration with version
        - Public visibility
        - Typed fields with defaults
        - Functions with bodies
        - Named constraints
    }
}
```

## Parse and Check

```bash
# Parse the file
dol-parse hello.dol

# Validate the file
dol-check hello.dol

# Generate Rust code
dol-codegen hello.dol -o generated/
```

## CLI Tools

| Tool | Purpose | Example |
|------|---------|---------|
| `dol-parse` | Parse DOL to AST | `dol-parse file.dol` |
| `dol-check` | Validate DOL files | `dol-check examples/` |
| `dol-codegen` | Generate code | `dol-codegen file.dol -o out/` |
| `dol-test` | Run DOL tests | `dol-test tests/` |

## Next Steps

- Read the [Syntax Reference](./syntax-reference.md) for complete language documentation
- Learn about the [SEX System](./sex-system.md) for side effects
- Explore the [Biology Examples](./examples/biology.md) for real-world patterns
- Check the [Specification](./specification.md) for formal language definition
