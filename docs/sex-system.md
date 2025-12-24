# SEX: Side Effect eXecution

## Philosophy

DOL defaults to **pure** code - no side effects, referentially transparent.
When you need effects (I/O, mutation, FFI), you explicitly mark it with `sex`.

## Safety Hierarchy

```
PURE (default)  → No side effects, safe to parallelize
PUB (public)    → Exported, still pure unless in sex context
SEX (effects)   → Can mutate, do I/O, call FFI - explicitly marked
```

## Syntax

### Sex Functions

```dol
// Pure function (default)
fun add(a: Int64, b: Int64) -> Int64 {
    return a + b
}

// Sex function (has side effects)
sex fun log(msg: String) -> Void {
    println(msg)  // I/O is a side effect
}
```

### Sex Variables (Mutable Globals)

```dol
// Mutable global - requires sex
sex var COUNTER: Int64 = 0

// Immutable constant - no sex needed
const MAX_VALUE: Int64 = 1000

sex fun increment() -> Int64 {
    COUNTER += 1
    return COUNTER
}
```

### Sex Blocks

```dol
// Mostly pure function with small effectful section
fun process(data: List<Int64>) -> Int64 {
    result = data.sum()

    sex {
        // This block can have side effects
        log("Processed " + data.length() + " items")
    }

    return result
}
```

### FFI (Foreign Function Interface)

```dol
// Declare external C function
sex extern "C" fun malloc(size: UInt64) -> Ptr<Void>
sex extern "C" fun free(ptr: Ptr<Void>)

// Platform-specific FFI
#cfg(target.wasm)
sex extern "wasi" fun fd_write(fd: Int32, ...) -> Int32
```

## File Detection

Files are automatically in sex context if:

1. **Extension**: `*.sex.dol`
2. **Directory**: Inside `sex/` folder

```
src/
├── types.dol          # Pure
├── math.dol           # Pure
├── io.sex.dol         # Sex (by extension)
└── sex/
    ├── globals.dol    # Sex (by directory)
    └── ffi.dol        # Sex (by directory)
```

## Compiler Enforcement

### Errors
| Error | Code | Description |
|-------|------|-------------|
| Sex in pure context | E001 | Calling sex function from pure function |
| Mutable global outside sex | E002 | `sex var` in non-sex file |
| FFI outside sex | E003 | `extern` without sex |
| I/O outside sex | E004 | println/file ops without sex |

### Warnings
| Warning | Code | Description |
|---------|------|-------------|
| Large sex block | W001 | Consider extracting to sex function |
| Undocumented sex | W002 | Sex function should document effects |

## Generated Code

### Rust Output

DOL:
```dol
sex var COUNTER: Int64 = 0

sex fun increment() -> Int64 {
    COUNTER += 1
    return COUNTER
}
```

Generated Rust:
```rust
use std::sync::atomic::{AtomicI64, Ordering};

static COUNTER: AtomicI64 = AtomicI64::new(0);

pub fn increment() -> i64 {
    COUNTER.fetch_add(1, Ordering::SeqCst) + 1
}
```

### Thread Safety

When generating Rust code, `sex var` globals use atomic types for thread safety:
- `Int64` → `AtomicI64`
- `Bool` → `AtomicBool`
- Complex types → `RwLock<T>`

## Best Practices

1. **Minimize sex scope** - Keep effectful code as small as possible
2. **Document effects** - Add exegesis explaining what side effects occur
3. **Isolate I/O** - Put all I/O in dedicated sex modules
4. **Test pure first** - Pure functions are easier to test
5. **Use sex blocks sparingly** - Prefer dedicated sex functions

## Examples

### Logger Module

```dol
// src/sex/logger.dol

sex var LOG_LEVEL: Int64 = 1  // 0=ERROR, 1=INFO, 2=DEBUG

sex fun set_log_level(level: Int64) -> Void {
    LOG_LEVEL = level
}

sex fun log(level: Int64, msg: String) -> Void {
    if level <= LOG_LEVEL {
        println("[" + level_name(level) + "] " + msg)
    }
}

// Pure helper - no sex needed
fun level_name(level: Int64) -> String {
    match level {
        0 { "ERROR" }
        1 { "INFO" }
        2 { "DEBUG" }
        _ { "UNKNOWN" }
    }
}
```

### File I/O

```dol
// src/io.sex.dol

sex extern "C" fun fopen(path: Ptr<Int8>, mode: Ptr<Int8>) -> Ptr<Void>
sex extern "C" fun fclose(file: Ptr<Void>) -> Int32
sex extern "C" fun fread(buf: Ptr<Void>, size: UInt64, count: UInt64, file: Ptr<Void>) -> UInt64

sex fun read_file(path: String) -> Result<String, String> {
    file = fopen(path.as_ptr(), "r".as_ptr())
    if file.is_null() {
        return Err("Failed to open: " + path)
    }

    // Read contents...
    content = read_all(file)
    fclose(file)

    return Ok(content)
}
```
