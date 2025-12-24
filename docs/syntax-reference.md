# DOL 2.0 Syntax Reference

## Keywords

### Ontological Keywords
| Keyword | Purpose | Example |
|---------|---------|---------|
| `gene` | Define atomic type | `gene Container { }` |
| `trait` | Define interface | `trait Runnable { }` |
| `system` | Define composition | `system Scheduler { }` |
| `constraint` | Define invariant | `constraint valid { }` |
| `evolves` | Define evolution | `evolves V1 > V2 @ 2.0` |
| `exegesis` | Documentation | `exegesis { ... }` |

### Module Keywords
| Keyword | Purpose | Example |
|---------|---------|---------|
| `module` | Declare module | `module path @ version` |
| `use` | Import | `use path.{ items }` |
| `pub` | Public visibility | `pub gene Name { }` |

### Function Keywords
| Keyword | Purpose | Example |
|---------|---------|---------|
| `fun` | Function declaration | `fun name() -> Type { }` |
| `return` | Return value | `return expr` |

### Control Flow
| Keyword | Purpose | Example |
|---------|---------|---------|
| `if`/`else` | Conditional | `if cond { } else { }` |
| `match` | Pattern match | `match x { 0 { } _ { } }` |
| `for` | Iteration | `for x in items { }` |
| `while` | Loop | `while cond { }` |
| `loop` | Infinite loop | `loop { break }` |
| `break` | Exit loop | `break` |
| `continue` | Next iteration | `continue` |

### Type Keywords
| Keyword | Purpose | Example |
|---------|---------|---------|
| `has` | Field declaration | `has name: Type = default` |
| `is` | Method signature | `is method() -> Type` |
| `type` | Type alias | `type Id is UInt64` |

### SEX Keywords (Side Effects)
| Keyword | Purpose | Example |
|---------|---------|---------|
| `sex` | Effect marker | `sex fun io_op() { }` |
| `var` | Mutable variable | `sex var COUNTER: Int64 = 0` |
| `const` | Immutable constant | `const MAX: Int64 = 100` |
| `extern` | FFI declaration | `sex extern fun malloc()` |

### Logic Keywords
| Keyword | Purpose | Example |
|---------|---------|---------|
| `forall` | Universal quantifier | `forall x: T. expr` |
| `exists` | Existential | `exists x: T. expr` |
| `implies` | Implication | `a implies b` |
| `not` | Negation | `not condition` |

### Other Keywords
| Keyword | Purpose | Example |
|---------|---------|---------|
| `state` | System state | `state count: Int64 = 0` |
| `law` | Trait law | `law associativity { }` |
| `uses` | Trait usage | `uses Transport` |
| `requires` | Trait requirement | `requires method()` |
| `provides` | Trait default | `provides method() { }` |
| `where` | Pattern guard | `x where x > 0` |
| `in` | Membership | `for x in list` |
| `as` | Type cast/alias | `x as Type` |
| `migrate` | Evolution migration | `migrate from V1 { }` |

## Operators

### Arithmetic Operators
| Operator | Name | Example |
|----------|------|---------|
| `+` | Addition | `a + b` |
| `-` | Subtraction | `a - b` |
| `*` | Multiplication | `a * b` |
| `/` | Division | `a / b` |
| `%` | Modulo | `a % b` |

### Comparison Operators
| Operator | Name | Example |
|----------|------|---------|
| `==` | Equal | `a == b` |
| `!=` | Not equal | `a != b` |
| `<` | Less than | `a < b` |
| `>` | Greater than | `a > b` |
| `<=` | Less or equal | `a <= b` |
| `>=` | Greater or equal | `a >= b` |

### Logical Operators
| Operator | Name | Example |
|----------|------|---------|
| `&&` | And | `a && b` |
| `\|\|` | Or | `a \|\| b` |
| `!` | Not | `!a` |

### Composition Operators
| Operator | Name | Example | Meaning |
|----------|------|---------|---------|
| `\|>` | Pipe | `x \|> f` | `f(x)` |
| `>>` | Compose | `f >> g` | `(x) -> g(f(x))` |
| `<\|` | Back-pipe | `f <\| x` | `f(x)` |
| `@` | Apply | `f @ x` | `f(x)` |
| `:=` | Bind | `f := (5, _)` | Partial application |

### Meta-Programming Operators
| Operator | Name | Example | Meaning |
|----------|------|---------|---------|
| `'` | Quote | `'expr` | AST of expr |
| `!` | Eval | `!quoted` | Evaluate AST |
| `#` | Macro | `#derive()` | Macro invocation |
| `?` | Reflect | `?Type` | Type info |
| `[\|` `\|]` | Idiom | `[\| f x y \|]` | Applicative |

## Types

### Primitive Types
| DOL Type | Rust | Description |
|----------|------|-------------|
| `Int8`-`Int64` | `i8`-`i64` | Signed integers |
| `UInt8`-`UInt64` | `u8`-`u64` | Unsigned integers |
| `Float32`, `Float64` | `f32`, `f64` | Floating point |
| `Bool` | `bool` | Boolean |
| `String` | `String` | UTF-8 string |
| `Void` | `()` | Unit type |

### Compound Types
| DOL Type | Rust | Example |
|----------|------|---------|
| `List<T>` | `Vec<T>` | `List<Int64>` |
| `Map<K,V>` | `HashMap<K,V>` | `Map<String, Int64>` |
| `Option<T>` | `Option<T>` | `Option<String>` |
| `Result<T,E>` | `Result<T,E>` | `Result<Int64, Error>` |
| `Tuple<A,B>` | `(A, B)` | `Tuple<Int64, String>` |

## Declaration Examples

### Gene Declaration
```dol
pub gene Container<T> {
    has items: List<T> = []
    has capacity: UInt64 = 10

    fun add(item: T) -> Self {
        return Self { items: this.items + [item], ..this }
    }

    constraint valid {
        this.items.length() <= this.capacity
    }

    exegesis {
        A generic container with capacity limits.
    }
}
```

### Trait Declaration
```dol
pub trait Comparable {
    is compare(other: Self) -> Int64

    provides less_than(other: Self) -> Bool {
        return this.compare(other) < 0
    }

    law reflexive {
        forall x: Self. x.compare(x) == 0
    }
}
```

### System Declaration
```dol
pub system TaskScheduler {
    uses Queue<Task>
    uses Logger

    state pending: List<Task> = []
    state running: Option<Task> = None

    fun schedule(task: Task) -> Self {
        return Self { pending: this.pending + [task], ..this }
    }
}
```

### Evolution Declaration
```dol
evolves EntityV1 > EntityV2 @ 2.0.0 {
    added created_at: Int64 = 0
    removed deprecated_field
    renamed old_name -> new_name

    migrate from EntityV1 {
        return EntityV2 {
            ...old,
            created_at: 0
        }
    }
}
```
