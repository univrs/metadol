# Meta DOL

[![Build Status](https://github.com/univrs/metadol/workflows/CI/badge.svg)](https://github.com/univrs/metadol/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

**A declarative DSL for ontology-first development**

---

## Overview

Meta DOL (Domain Ontology Language) is a production-ready DSL toolchain that enables ontology-first development. Instead of writing code and hoping it aligns with your domain model, Meta DOL lets you declare your domain's fundamental units, behaviors, and constraints explicitly. The toolchain then validates, tests, and generates code that provably satisfies your ontological commitments.

Meta DOL treats ontology as a first-class concern. You define **genes** (atomic units of meaning), **traits** (composable behaviors), **constraints** (invariants that must hold), **systems** (top-level compositions), and **evolutions** (version-tracked changes). Each declaration includes mandatory exegesis—human-readable explanations that bridge the gap between formal specification and domain understanding.

This approach reduces ontological drift, improves code maintainability, and creates a single source of truth for domain semantics. Whether you're building distributed systems, domain-driven applications, or knowledge graphs, Meta DOL provides the foundation for rigorous, validated domain modeling.

## Quick Start

### Prerequisites

- Rust toolchain 1.70 or later ([install from rustup.rs](https://rustup.rs))

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/univrs/metadol.git
cd metadol

# Build the project
cargo build --release

# Run tests
cargo test
```

### Create Your First .dol File

Create a file named `example.dol`:

```dol
gene user.account {
  user has identity
  user has credentials
  user has profile
}

exegesis {
  A user account represents an authenticated entity in the system.
  Every user must have a unique identity, authentication credentials,
  and a profile containing user preferences and settings.
}
```

### Parse It

```bash
cargo run --features cli --bin dol-parse -- example.dol
```

### Expected Output

```
✓ example.dol (user.account)
    user.account gene with 3 statements

Summary
  Total:    1
  Success:  1
```

For JSON output, use `--format json`:

```bash
cargo run --features cli --bin dol-parse -- --format json example.dol
```

## Installation

### From Source

```bash
# Clone and build
git clone https://github.com/univrs/metadol.git
cd metadol
cargo build --release

# Install binaries to ~/.cargo/bin (requires cli feature)
cargo install --path . --features cli

# Verify installation
dol-parse --version
dol-check --version
dol-test --version
```

### For Development

```bash
# Clone repository
git clone https://github.com/univrs/metadol.git
cd metadol

# Run in development mode
cargo run --bin dol-parse -- examples/genes/container.exists.dol

# Run tests
cargo test

# Check formatting and lints
cargo fmt --check
cargo clippy -- -D warnings

# Build documentation
cargo doc --open
```

## CLI Tools

Meta DOL provides three command-line tools for working with DOL files:

### `dol-parse`

Parses DOL files and outputs a JSON representation of the AST. Useful for validation, debugging, and integration with other tools.

```bash
dol-parse <file.dol>
dol-parse --format json examples/genes/container.exists.dol
```

### `dol-check`

Validates DOL files for correctness, ensuring all declarations have required exegesis and all dependencies are satisfied. Ideal for CI/CD pipelines.

```bash
dol-check examples/
dol-check --require-exegesis src/**/*.dol
```

### `dol-test`

Generates Rust test code from `.dol.test` files, enabling property-based testing of ontological commitments.

```bash
dol-test examples/traits/container.lifecycle.dol.test
dol-test --output tests/ examples/**/*.dol.test
```

## Language Overview

Meta DOL provides five core declaration types:

### Genes

Atomic units of meaning that cannot be decomposed further. Genes represent fundamental concepts in your domain.

```dol
gene container.exists {
  container has identity
  container has state
  container has boundaries
}

exegesis {
  A container is the fundamental unit of workload isolation.
  Every container has an immutable identity, discrete state,
  and enforced resource boundaries.
}
```

### Traits

Composable behaviors that build on genes and other traits. Traits define what things can do.

```dol
trait container.lifecycle {
  uses container.exists

  container is created
  container is running
  container is stopped
  each transition emits event
}

exegesis {
  The container lifecycle defines state transitions from
  creation through running to termination.
}
```

### Constraints

Invariants that must hold across your domain. Constraints express rules that cannot be violated.

```dol
constraint container.integrity {
  identity matches original identity
  state never undefined
  boundaries never violated
}

exegesis {
  Container integrity ensures that identity is immutable,
  state is always defined, and boundaries are enforced.
}
```

### Systems

Top-level compositions that bring genes, traits, and constraints together with versioned requirements.

```dol
system univrs.orchestrator @ 0.0.1 {
  requires container.exists >= 0.0.1
  requires container.lifecycle >= 0.0.1
  each container has supervisor
  all containers is monitored
}

exegesis {
  The orchestrator system manages container lifecycles and
  ensures all containers are supervised and monitored.
}
```

### Evolutions

Version-tracked changes that document how your ontology evolves over time.

```dol
evolves container.lifecycle @ 0.0.2 > 0.0.1 {
  adds container is paused
  adds container is resumed
  because "workload migration requires state preservation"
}

exegesis {
  Version 0.0.2 adds pause and resume states to support
  live migration between nodes.
}
```

## Examples

The `examples/` directory contains comprehensive examples organized by declaration type:

- **`examples/genes/`** - Atomic domain concepts
- **`examples/traits/`** - Composable behaviors
- **`examples/constraints/`** - Domain invariants
- **`examples/systems/`** - Complete system definitions
- **`examples/evolutions/`** - Version migration examples

Each example includes detailed exegesis and demonstrates best practices for ontology-first development.

## Documentation

Comprehensive documentation is available in the `docs/` directory:

- **[Language Specification](docs/specification.md)** - Complete formal specification of Meta DOL
- **[Grammar (EBNF)](docs/grammar.ebnf)** - Formal EBNF grammar definition
- **[Tutorials](docs/tutorials/)** - Step-by-step guides for common workflows
  - Getting Started with Genes
  - Composing Traits
  - Defining Constraints
  - Building Systems
  - Managing Evolutions

Additional resources:

- **API Documentation** - Run `cargo doc --open` to browse the Rust API
- **Contributing Guide** - See `CONTRIBUTING.md` for development guidelines

## Project Status

**Current Version**: 0.0.1

Meta DOL is production-ready with a complete toolchain for ontology-first development.

### Roadmap

- **Phase 1** ✅ Complete: Foundation - 47 tests passing, clean clippy/fmt
- **Phase 2** ✅ Complete: Tooling - CLI tools (dol-parse, dol-check, dol-test)
- **Phase 3** ✅ Complete: Documentation - tutorials, examples, specification
- **Phase 4** (Planned): Community - first release, contribution guidelines

For detailed roadmap and progress tracking, see [docs/PHASE2_ROADMAP.md](docs/PHASE2_ROADMAP.md).

### Contributing

Contributions are welcome! This project follows standard Rust conventions:

- Run `cargo fmt` before committing
- Ensure `cargo clippy -- -D warnings` passes
- Add tests for new functionality
- Update documentation for API changes

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

**Built with Rust. Powered by Ontology. Driven by Clarity.**
