# Tutorial 3: Composing Traits

## What Are Traits?

While genes are atomic units, **traits** are compositions of genes that represent complex behaviors or capabilities. Traits are where your ontology starts to model real-world complexity.

Think of traits as:
- **Genes** = Elementary particles (quarks, electrons)
- **Traits** = Atoms (hydrogen, oxygen)
- **Systems** = Molecules (water, methane)

### Trait Philosophy

Traits follow the **composition over inheritance** principle:
- Traits compose multiple genes
- Traits can depend on other traits
- Traits define behavioral contracts
- Traits emit events and signals

## Trait Syntax

The complete trait syntax:

```dol
trait qualified.identifier @X.Y.Z {
    """
    Mandatory exegesis explaining what this trait represents.
    """

    uses gene.identifier @version
    uses other.trait @version

    has property: type
    is category
    requires dependency @version

    emits event.name
    quantified by measurement
}
```

Let's explore trait-specific features.

## The `uses` Statement

The `uses` statement is how traits compose genes and other traits:

```dol
trait container.lifecycle @1.0.0 {
    """
    Complete lifecycle management for containers.
    """

    uses container.exists @1.0.0
    uses container.lifecycle.started @1.0.0
    uses container.lifecycle.stopped @1.0.0
    uses container.lifecycle.failed @1.0.0

    is lifecycle
    is state_machine
}
```

**Key points:**
- `uses` brings in all properties and categories from the referenced declaration
- Must specify exact versions
- Creates a composition graph
- Order doesn't matter (declarations are declarative)

## Trait-Specific Predicates

Traits support all gene predicates plus two trait-specific ones.

### The `emits` Predicate

Declares events that this trait can emit:

```dol
trait container.monitored @1.0.0 {
    """
    A container that emits monitoring events.
    """

    uses container.exists @1.0.0

    emits container.started
    emits container.stopped
    emits container.health_check
    emits container.error
    emits container.metrics

    has monitoring_interval: integer
    has alert_threshold: float

    is observable
    is event_driven
}
```

Events represent significant state changes or signals that:
- Other systems can listen for
- Are logged for audit trails
- Trigger workflows or reactions
- Are part of the behavioral contract

### The `quantified by` Predicate

Declares measurements or metrics:

```dol
trait container.performance @1.0.0 {
    """
    Performance characteristics of a running container.
    """

    uses container.lifecycle.started @1.0.0

    quantified by cpu_usage
    quantified by memory_usage
    quantified by network_io
    quantified by disk_io
    quantified by response_time

    has measurement_interval: integer

    is measurable
    is optimizable
}
```

Quantifications are continuous measurements that:
- Can be sampled over time
- Have units and ranges
- Can trigger thresholds
- Enable performance analysis

## Building Traits from Genes

Let's build a complete lifecycle trait step by step.

### Step 1: Define Base Genes

First, we need our atomic genes:

```dol
gene container.exists @1.0.0 {
    """
    A container exists in the system.
    """
    has id: string
    has image: string
    is entity
}

gene container.state.created @1.0.0 {
    """
    Container has been created but not started.
    """
    requires container.exists @1.0.0
    has created_at: integer
}

gene container.state.running @1.0.0 {
    """
    Container is actively running.
    """
    requires container.exists @1.0.0
    has pid: integer
    has started_at: integer
}

gene container.state.stopped @1.0.0 {
    """
    Container has been stopped.
    """
    requires container.exists @1.0.0
    has exit_code: integer
    has stopped_at: integer
}
```

### Step 2: Compose into a Trait

Now compose these genes into a lifecycle trait:

```dol
trait container.lifecycle @1.0.0 {
    """
    Complete lifecycle management for containers.

    This trait models the full state machine of container lifecycle:
    created -> running -> stopped. It includes all state transitions
    and emits events at each transition point.

    Implementations must maintain exactly one active state at a time
    and emit events in the correct order.
    """

    uses container.exists @1.0.0
    uses container.state.created @1.0.0
    uses container.state.running @1.0.0
    uses container.state.stopped @1.0.0

    emits lifecycle.created
    emits lifecycle.started
    emits lifecycle.stopped

    has current_state: string
    has state_history: list<string>

    is lifecycle
    is state_machine
    is auditable
}
```

### Step 3: Add Monitoring

Extend with monitoring capabilities:

```dol
trait container.lifecycle.monitored @1.0.0 {
    """
    Lifecycle management with health monitoring.
    """

    uses container.lifecycle @1.0.0

    emits health.check
    emits health.failed
    emits health.recovered

    quantified by health_score
    quantified by uptime

    has health_check_interval: integer
    has health_check_timeout: integer
    has health_check_retries: integer

    is monitored
    is self_healing
}
```

## Example: Building a Complete User Session Trait

Let's build a realistic user session trait.

### Define Supporting Genes

```dol
gene user.exists @1.0.0 {
    """A user account exists in the system."""
    has user_id: string
    has username: string
    is entity
}

gene user.credentials.verified @1.0.0 {
    """User credentials have been verified."""
    requires user.exists @1.0.0
    has verified_at: integer
    has verification_method: string
}

gene session.token.issued @1.0.0 {
    """A session token has been issued."""
    has token: string
    has issued_at: integer
    has expires_at: integer
}
```

### Compose the Trait

```dol
trait user.session.authenticated @1.0.0 {
    """
    A complete authenticated user session.

    This trait represents an active, authenticated user session with
    token management, activity tracking, and security monitoring.

    Sessions have a limited lifetime and can be explicitly terminated.
    All session activity is logged for security auditing.
    """

    // Core composition
    uses user.exists @1.0.0
    uses user.credentials.verified @1.0.0
    uses session.token.issued @1.0.0

    // Session lifecycle events
    emits session.created
    emits session.renewed
    emits session.terminated
    emits session.expired

    // Security events
    emits security.suspicious_activity
    emits security.token_refreshed
    emits security.concurrent_login

    // Session metrics
    quantified by session_duration
    quantified by activity_frequency
    quantified by api_call_rate

    // Session properties
    has last_activity: integer
    has activity_count: integer
    has ip_address: string
    has user_agent: string

    // Session configuration
    has max_idle_time: integer
    has max_session_time: integer
    has allow_concurrent: boolean

    // Categories
    is authenticated
    is temporal
    is auditable
    is revocable
    is renewable
}
```

## Trait Composition Patterns

### The Layered Pattern

Build traits in layers of abstraction:

```dol
// Layer 1: Base
trait entity.basic @1.0.0 {
    uses entity.exists @1.0.0
}

// Layer 2: Persistence
trait entity.persistent @1.0.0 {
    uses entity.basic @1.0.0
    uses entity.storable @1.0.0
}

// Layer 3: Full-featured
trait entity.managed @1.0.0 {
    uses entity.persistent @1.0.0
    uses entity.auditable @1.0.0
    uses entity.versioned @1.0.0
}
```

### The Aspect Pattern

Model cross-cutting concerns as traits:

```dol
trait aspect.auditable @1.0.0 {
    """Audit trail tracking for any entity."""
    emits audit.created
    emits audit.updated
    emits audit.deleted
    has audit_log: list<string>
}

trait aspect.cacheable @1.0.0 {
    """Caching behavior for any entity."""
    has cache_key: string
    has cache_ttl: integer
    is cacheable
}

trait aspect.observable @1.0.0 {
    """Observable pattern for any entity."""
    emits state.changed
    has observers: list<string>
}
```

Then apply aspects to domain traits:

```dol
trait user.profile.managed @1.0.0 {
    uses user.profile @1.0.0
    uses aspect.auditable @1.0.0
    uses aspect.cacheable @1.0.0
    uses aspect.observable @1.0.0
}
```

### The State Machine Pattern

Model state machines explicitly:

```dol
trait order.lifecycle @1.0.0 {
    """
    Order processing state machine.
    """

    uses order.state.pending @1.0.0
    uses order.state.processing @1.0.0
    uses order.state.completed @1.0.0
    uses order.state.cancelled @1.0.0

    emits order.created
    emits order.processing_started
    emits order.completed
    emits order.cancelled

    has current_state: string
    has allowed_transitions: map<string, list<string>>

    is state_machine
    is workflow
}
```

## Best Practices

### 1. Single Behavioral Concern

Each trait should model ONE cohesive behavior:

Bad (mixing concerns):

```dol
trait user.everything @1.0.0 {
    uses user.authentication @1.0.0
    uses user.authorization @1.0.0
    uses user.profile @1.0.0
    uses user.preferences @1.0.0
    uses user.notifications @1.0.0
}
```

Good (focused traits):

```dol
trait user.auth.complete @1.0.0 {
    uses user.authentication @1.0.0
    uses user.authorization @1.0.0
}

trait user.personalization @1.0.0 {
    uses user.profile @1.0.0
    uses user.preferences @1.0.0
}
```

### 2. Explicit Event Contracts

Document what events mean:

```dol
trait payment.processed @1.0.0 {
    """
    Payment processing with event-driven workflow.

    Events:
    - payment.initiated: Payment request received
    - payment.validated: Payment details verified
    - payment.authorized: Payment authorized by provider
    - payment.captured: Funds captured
    - payment.failed: Payment failed at any stage
    """

    emits payment.initiated
    emits payment.validated
    emits payment.authorized
    emits payment.captured
    emits payment.failed

    ...
}
```

### 3. Meaningful Quantifications

Only quantify what matters:

```dol
trait api.endpoint @1.0.0 {
    """API endpoint with performance tracking."""

    quantified by response_time      // Critical
    quantified by error_rate         // Critical
    quantified by throughput         // Critical
    quantified by cpu_usage          // Useful
    // Don't quantify everything - focus on what matters
}
```

### 4. Version Trait Dependencies Carefully

When updating a trait, consider dependency versions:

```dol
// Original
trait container.managed @1.0.0 {
    uses container.lifecycle @1.0.0
    uses container.monitored @1.0.0
}

// Update - new monitoring version
trait container.managed @2.0.0 {
    uses container.lifecycle @1.0.0    // Same version OK
    uses container.monitored @2.0.0    // Updated version
}
```

## Testing Your Traits

Parse your traits:

```bash
cargo run --bin dol-parse -- my-traits.dol
```

Validate composition:

```bash
cargo run --bin dol-check -- my-traits.dol --check-dependencies
```

## Common Pitfalls

### Circular Dependencies

Avoid circular trait dependencies:

```dol
// BAD - circular dependency
trait a @1.0.0 {
    uses b @1.0.0
}

trait b @1.0.0 {
    uses a @1.0.0  // CIRCULAR!
}
```

### Over-Composition

Don't create mega-traits:

```dol
// BAD - too many concerns
trait user.everything @1.0.0 {
    uses gene1 @1.0.0
    uses gene2 @1.0.0
    // ... 20 more uses
}
```

### Missing Events

Declare all significant events:

```dol
// BAD - state changes without events
trait lifecycle @1.0.0 {
    has state: string
    // Missing emits for state transitions
}

// GOOD
trait lifecycle @1.0.0 {
    has state: string
    emits state.changed
    emits state.transitioned
}
```

## Next Steps

You now understand trait composition. Next:

- **Tutorial 4: System Design** - Compose traits into complete systems
- **Advanced Topics** - Constraints, evolution tracking, version management

## Key Takeaways

1. **Traits compose genes** - build complexity from simplicity
2. **Use `uses` for composition** - bring in genes and other traits
3. **Emit events** - define behavioral contracts
4. **Quantify metrics** - measure what matters
5. **Single behavioral concern** - one trait, one behavior
6. **Explicit dependencies** - always version your uses statements
7. **Think in layers** - build traits progressively
