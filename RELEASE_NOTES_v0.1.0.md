## DOL 2.0 Parser with SEX System

This release marks the first stable version of the DOL (Domain Ontology Language) parser with full DOL 2.0 support and the new SEX (Side Effect eXecution) system.

### Major Features

- **DOL 2.0 Parser** - Full support for modern DOL syntax including:
  - `module` declarations with versioning
  - `pub` visibility modifiers
  - `use` imports with destructuring
  - `fun` function declarations with bodies
  - Inline `constraint` blocks
  - `state` declarations in systems
  - `law` declarations in traits
  - Generic type parameters

- **SEX System** - Side Effect eXecution for explicit effect tracking:
  - `sex fun` - Functions with side effects
  - `sex var` - Mutable global variables
  - `sex { }` - Inline effect blocks
  - `sex extern` - FFI declarations
  - Effect tracking in typechecker
  - Lint rules (E001-E004, W001-W002)

- **Biology Module** - Biomimicry modeling examples:
  - Mycelium network simulation
  - Ecosystem dynamics (Lotka-Volterra)
  - Evolution and speciation
  - Hyphal growth patterns
  - Nutrient transport systems

- **Code Generation**
  - Rust code generator
  - TypeScript code generator
  - JSON Schema generator

### New Keywords

`module`, `use`, `pub`, `fun`, `const`, `var`, `extern`, `sex`, `state`, `law`, `forall`, `migrate`

### Quality

| Metric | Status |
|--------|--------|
| Tests | 626 passing |
| Clippy | 0 warnings |
| Formatting | Clean |

### Installation

```bash
cargo install dol
```

Or build from source:

```bash
git clone https://github.com/univrs/dol.git
cd dol
cargo build --release
```

### Documentation

See the [examples/](examples/) directory for DOL file examples.
