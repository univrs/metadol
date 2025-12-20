# Contributing to Metal DOL

Thank you for your interest in contributing to Metal DOL! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Coding Guidelines](#coding-guidelines)
- [Testing](#testing)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- Rust toolchain 1.75 or later (install from [rustup.rs](https://rustup.rs))
- Git
- A GitHub account

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/metadol.git
   cd metadol
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/univrs/metadol.git
   ```

## Development Setup

### Build the Project

```bash
# Build in debug mode
cargo build

# Build with CLI tools
cargo build --features cli

# Build in release mode
cargo build --release
```

### Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test lexer_tests
cargo test parser_tests
```

### Check Code Quality

```bash
# Format code
cargo fmt

# Check formatting without changes
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Build documentation
cargo doc --open
```

## Making Changes

### Branch Naming

Use descriptive branch names:

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation changes
- `refactor/description` - Code refactoring
- `test/description` - Test additions or changes

### Commit Messages

Write clear, concise commit messages:

- Use the imperative mood ("Add feature" not "Added feature")
- Keep the first line under 72 characters
- Reference issues when applicable (e.g., "Fix #123")

Example:
```
Add version parsing for evolution declarations

- Implement semver parsing in lexer
- Add Version token type
- Update parser to handle @ version syntax
- Add tests for version edge cases

Fixes #42
```

## Coding Guidelines

### Rust Style

- Run `cargo fmt` before every commit
- Zero `clippy` warnings allowed: `cargo clippy -- -D warnings`
- Use `Result<T, E>` for fallible operations
- Prefer explicit error types over `anyhow` in library code

### Documentation

- All public items must have `///` doc comments
- Include examples in doc comments where helpful
- Keep comments concise and meaningful
- Update documentation when changing APIs

### Code Examples

```rust
/// Parses a DOL source file into an AST.
///
/// # Arguments
///
/// * `source` - The DOL source text to parse
///
/// # Returns
///
/// A `Declaration` on success, or a `ParseError` on failure.
///
/// # Example
///
/// ```rust
/// use metadol::parse_file;
///
/// let source = r#"
/// gene example.thing {
///   thing has property
/// }
///
/// exegesis {
///   Example gene for documentation.
/// }
/// "#;
///
/// let result = parse_file(source);
/// assert!(result.is_ok());
/// ```
pub fn parse_file(source: &str) -> Result<Declaration, ParseError> {
    // Implementation
}
```

### Error Handling

- Use `thiserror` for library error types
- Include source location in error messages
- Provide actionable error messages

```rust
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unexpected token at {span:?}: expected {expected}, found {found}")]
    UnexpectedToken {
        expected: String,
        found: String,
        span: Span,
    },
}
```

## Testing

### Test Requirements

- All new functionality must have tests
- Maintain >80% code coverage
- Tests must be deterministic

### Test Categories

1. **Unit Tests** - In the same file as the code
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_feature() {
           // Test implementation
       }
   }
   ```

2. **Integration Tests** - In `tests/` directory
   ```rust
   // tests/integration_test.rs
   use metadol::parse_file;

   #[test]
   fn test_parse_example_file() {
       let source = include_str!("../examples/genes/container.exists.dol");
       assert!(parse_file(source).is_ok());
   }
   ```

### Running Specific Tests

```bash
# Run lexer tests
cargo test lexer

# Run parser tests
cargo test parser

# Run integration tests
cargo test --test integration_tests

# Run tests matching a pattern
cargo test parse_gene
```

## Documentation

### Types of Documentation

1. **API Documentation** - Doc comments on public items
2. **Tutorials** - Step-by-step guides in `docs/tutorials/`
3. **Specification** - Language spec in `docs/specification.md`
4. **Examples** - Working examples in `examples/`

### Building Documentation

```bash
# Build and open docs
cargo doc --open

# Build with private items
cargo doc --document-private-items
```

## Submitting Changes

### Pull Request Process

1. Ensure all tests pass: `cargo test`
2. Ensure code is formatted: `cargo fmt --check`
3. Ensure no clippy warnings: `cargo clippy -- -D warnings`
4. Update documentation if needed
5. Create a pull request with a clear description

### Pull Request Template

```markdown
## Description

Brief description of changes.

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing

- [ ] All tests pass
- [ ] New tests added for new functionality
- [ ] Tested manually with example files

## Checklist

- [ ] Code follows project style guidelines
- [ ] Self-review of code completed
- [ ] Documentation updated
- [ ] No new warnings introduced
```

### Review Process

- PRs require at least one approving review
- CI must pass before merging
- Address all review comments
- Squash commits when merging if appropriate

## Release Process

Releases follow semantic versioning (MAJOR.MINOR.PATCH):

- **PATCH**: Bug fixes, documentation updates
- **MINOR**: New features, backward-compatible changes
- **MAJOR**: Breaking changes

### Creating a Release

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create a git tag: `git tag -a v0.0.1 -m "Release v0.0.1"`
4. Push tag: `git push origin v0.0.1`
5. Create GitHub release with release notes

## Questions?

If you have questions about contributing:

1. Check existing issues and discussions
2. Open a new issue for clarification
3. Reach out to maintainers

Thank you for contributing to Metal DOL!
