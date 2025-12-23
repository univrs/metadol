# Tutorial 2: Writing Genes

## What Are Genes?

In DOL, a **gene** is the atomic unit of your ontology. Genes represent fundamental properties or capabilities that cannot be decomposed further.

Think of genes as the elementary particles of your domain:
- A user **exists**
- A container **has data**
- A file **is readable**
- An entity **derives from** a base type

### Gene Philosophy

Genes follow the **single responsibility principle** for ontology:
- Each gene captures ONE atomic concept
- Genes are immutable once published (use versions for changes)
- Genes compose into traits (covered in Tutorial 3)

## Gene Syntax

The complete gene syntax:

```dol
gene qualified.identifier @X.Y.Z {
    has property: type
    is category
    derives from parent.gene @version
    requires dependency.gene @version
}

exegesis {
    Mandatory exegesis explaining what this gene represents
    and why it exists in your domain ontology.
}
```

Let's explore each component.

## Naming Conventions

### Qualified Identifiers

Gene names use dot notation to express domain hierarchy:

```dol
gene user.exists @1.0.0 { ... }
gene user.authenticated @1.0.0 { ... }
gene user.profile.complete @1.0.0 { ... }
```

**Best practices:**
- Use `domain.property` pattern
- Start with the entity/domain (user, container, file)
- End with the specific property (exists, authenticated, complete)
- Use lowercase with dots (no camelCase or snake_case)

### Version Numbers

All genes require semantic versioning:

```dol
gene container.exists @1.0.0 { ... }    // Initial release
gene container.exists @1.1.0 { ... }    // Backward-compatible addition
gene container.exists @2.0.0 { ... }    // Breaking change
```

**Versioning rules:**
- MAJOR version for breaking changes
- MINOR version for backward-compatible additions
- PATCH version for fixes/clarifications

## The Exegesis Requirement

Every gene MUST include exegesis. This is not optional.

### Good Exegesis

```dol
gene container.lifecycle.started @1.0.0 {
    // ... statements ...
}

exegesis {
    Represents a container that has been started and is currently running.
    A started container has an active process, consumes resources, and can
    receive signals. This state is entered after successful initialization
    and before the container stops or fails.
}
```

### Bad Exegesis

```dol
gene container.lifecycle.started @1.0.0 {
    // ... statements ...
}

exegesis {
    Container is started.
}
// Too brief, no context
```

**Exegesis should answer:**
- What does this gene represent?
- Why does it exist in your ontology?
- When is this property/capability relevant?
- What are the implications of having this gene?

## Predicates

Genes use four predicates to define their ontology.

### 1. The `has` Predicate

Declares typed properties:

```dol
gene container.configured @1.0.0 {
    has image: string
    has memory_limit: integer
    has cpu_shares: integer
    has environment: map<string, string>
    has volumes: list<string>
}

exegesis {
    A container with runtime configuration settings.
}
```

**Supported types:**
- Primitives: `string`, `integer`, `float`, `boolean`
- Collections: `list<T>`, `map<K,V>`, `set<T>`
- References: Other gene identifiers

### 2. The `is` Predicate

Declares categorical membership:

```dol
gene user.account @1.0.0 {
    has username: string
    has email: string

    is entity
    is persistent
    is auditable
    is versioned
}

exegesis {
    A user account in the system.
}
```

Categories are semantic tags that indicate:
- `entity` - A first-class domain object
- `persistent` - Stored beyond process lifetime
- `auditable` - Changes are tracked
- `versioned` - Has version history
- `ephemeral` - Temporary, not persisted

### 3. The `derives from` Predicate

Declares inheritance relationships:

```dol
gene user.admin @1.0.0 {
    derives from user.account @1.0.0

    has permissions: set<string>
    has access_level: integer

    is privileged
}

exegesis {
    An administrative user with elevated privileges.
}
```

**Inheritance rules:**
- A gene inherits all properties from its parent
- Must specify parent version
- Can add new properties
- Can add new categories
- Cannot remove parent properties

### 4. The `requires` Predicate

Declares dependencies on other genes:

```dol
gene container.networked @1.0.0 {
    requires container.exists @1.0.0

    has ip_address: string
    has ports: map<integer, integer>
    has hostname: string

    is networked
}

exegesis {
    A container with network connectivity.
}
```

Dependencies establish prerequisites:
- Required genes must be present in any implementation
- Creates a dependency graph
- Enables dependency ordering in code generation

## Complete Example: Building a User Gene

Let's model a complete user authentication gene:

```dol
gene user.authenticated @1.0.0 {
    // Core identity
    has user_id: string
    has username: string
    has email: string

    // Session tracking
    has session_token: string
    has session_expiry: integer
    has last_login: integer

    // Security metadata
    has ip_address: string
    has user_agent: string

    // Dependencies
    requires user.exists @1.0.0

    // Categories
    is authenticated
    is session
    is auditable
    is temporal
}

exegesis {
    Represents an authenticated user session in the system.

    An authenticated user has verified their identity through credentials
    and has been granted a session token. This gene is the foundation for
    all authorization decisions and audit trails.

    Authentication is distinct from authorization - this gene only confirms
    identity, not permissions.
}
```

## Best Practices

### 1. Keep Genes Atomic

Bad (too much in one gene):

```dol
gene user.complete @1.0.0 {
    has username: string
    has email: string
    has profile_picture: string
    has posts: list<string>
    has followers: list<string>
    has settings: map<string, string>
}
```

Good (separate concerns):

```dol
gene user.exists @1.0.0 {
    has username: string
    has email: string
}

gene user.profile @1.0.0 {
    requires user.exists @1.0.0
    has profile_picture: string
}

gene user.social @1.0.0 {
    requires user.exists @1.0.0
    has posts: list<string>
    has followers: list<string>
}
```

### 2. Use Explicit Dependencies

Make dependencies explicit:

```dol
gene container.stopped @1.0.0 {
    """
    A container that has been stopped.
    """

    requires container.exists @1.0.0
    requires container.lifecycle.started @1.0.0

    has exit_code: integer
    has stopped_at: integer
}
```

### 3. Version Carefully

When updating a gene, create a new version:

```dol
// Version 1.0.0
gene user.profile @1.0.0 {
    has name: string
    has email: string
}

// Version 2.0.0 - added avatar (backward compatible)
gene user.profile @2.0.0 {
    has name: string
    has email: string
    has avatar: string  // New property
}
```

### 4. Document Edge Cases

Use exegesis to clarify edge cases:

```dol
gene container.failed @1.0.0 {
    """
    A container that failed to start or crashed during execution.

    Edge cases:
    - A container that fails during initialization is considered failed
    - A container that exits with code 0 is NOT failed (use container.stopped)
    - A container killed by signal is failed if non-zero exit code
    """

    requires container.exists @1.0.0

    has error_message: string
    has exit_code: integer
    has failed_at: integer
}
```

## Testing Your Genes

Parse your genes to validate syntax:

```bash
cargo run --bin dol-parse -- my-genes.dol
```

Generate tests (covered in detail in later tutorials):

```bash
cargo run --bin dol-test -- my-genes.dol.test
```

## Common Patterns

### The Existence Pattern

Every domain entity should have an "exists" gene:

```dol
gene entity.exists @1.0.0 {
    has id: string
    is entity
}

exegesis {
    Base existence for any entity.
}
```

### The State Pattern

Model states as separate genes:

```dol
gene task.pending @1.0.0 { ... }
gene task.running @1.0.0 { ... }
gene task.completed @1.0.0 { ... }
gene task.failed @1.0.0 { ... }
```

### The Capability Pattern

Model capabilities as genes:

```dol
gene file.readable @1.0.0 { ... }
gene file.writable @1.0.0 { ... }
gene file.executable @1.0.0 { ... }
```

## Next Steps

You now understand genes, the atomic units of Metal DOL. Next:

- **Tutorial 3: Composing Traits** - Learn to combine genes into complex behaviors
- **Tutorial 4: System Design** - Architect complete systems from traits

## Key Takeaways

1. **Genes are atomic** - one concept per gene
2. **Exegesis is mandatory** - document your ontology
3. **Use qualified identifiers** - domain.property naming
4. **Version everything** - semantic versioning required
5. **Four predicates** - has, is, derives from, requires
6. **Dependencies are explicit** - use requires
7. **Keep genes focused** - single responsibility principle
