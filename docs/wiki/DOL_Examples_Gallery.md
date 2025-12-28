# DOL Examples Gallery

> **Real-World Examples of Ontology-First Development**

This gallery showcases comprehensive DOL examples from simple types to complex simulations.

---

## Quick Examples

### Simple Type Definition

```dol
mod examples.point @ 0.1.0

/// A 2D point
pub type Point2D {
    x: Float64
    y: Float64
}

/// Calculate distance between points
fun distance(val a: Point2D, val b: Point2D) -> Float64 {
    val dx = b.x - a.x
    val dy = b.y - a.y
    (dx * dx + dy * dy).sqrt()
}
```

### Container Definition

```dol
mod container.runtime @ 0.3.0

/// Container status enum
pub type ContainerStatus {
    kind: enum {
        Created,
        Running,
        Paused,
        Stopped,
        Failed { reason: String }
    }
}

/// A runtime container instance
pub type Container {
    id: UInt64
    name: String
    image: String
    status: ContainerStatus
    created_at: Timestamp
    labels: Map<String, String>
}

/// Container lifecycle trait
trait ContainerLifecycle {
    start: fun() -> Result<Void, Error>
    stop: fun() -> Result<Void, Error>
    pause: fun() -> Result<Void, Error>
    resume: fun() -> Result<Void, Error>
}
```

---

## Domain Examples

### E-Commerce: User and Orders

```dol
mod ecommerce.orders @ 0.1.0

pub type UserId = UInt64

pub type User {
    id: UserId
    email: String
    name: String
    created_at: Timestamp
}

pub type OrderStatus {
    kind: enum {
        Pending,
        Confirmed,
        Shipped { tracking: String },
        Delivered,
        Cancelled { reason: String }
    }
}

pub type OrderItem {
    product_id: UInt64
    quantity: UInt32
    unit_price: Decimal
}

pub type Order {
    id: UInt64
    user_id: UserId
    items: List<OrderItem>
    status: OrderStatus
    total: Decimal
    created_at: Timestamp
}

/// Calculate order total
fun calculate_total(val items: List<OrderItem>) -> Decimal {
    items.fold(Decimal.zero(), |acc, item| {
        acc + (item.unit_price * item.quantity)
    })
}

/// Order validation constraint
constraint ValidOrder {
    forall order in orders {
        order.items.length > 0
        order.total == calculate_total(order.items)
    }
}
```

### Cryptographic Identity

```dol
mod identity.cryptographic @ 0.1.0

/// Cryptographic key types
pub type KeyType {
    kind: enum {
        Ed25519,
        Secp256k1,
        RSA { bits: UInt16 }
    }
}

/// A public key
pub type PublicKey {
    key_type: KeyType
    bytes: Bytes
}

/// A cryptographic identity
pub type Identity {
    id: UInt64
    public_key: PublicKey
    created_at: Timestamp
    revoked: Bool
}

/// Signature verification trait
trait Verifiable {
    /// Verify a signature
    verify: fun(message: Bytes, signature: Bytes) -> Bool

    /// Get the public key
    public_key: fun() -> PublicKey
}

/// Identity constraints
constraint IdentityValid {
    forall identity in identities {
        not identity.public_key.bytes.is_empty
        identity.created_at <= Timestamp.now()
    }
}
```

---

## Advanced Example: Mycelium Network Simulation

This is a comprehensive example modeling fungal network behavior. Located in `examples/stdlib/biology/mycelium.dol`:

```dol
mod biology.mycelium @ 0.1.0

// ============================================================================
// CORE TYPES
// ============================================================================

/// 3D position in the simulation space
pub type Vec3 {
    x: Float64
    y: Float64
    z: Float64
}

/// A node in the mycelium network
pub type MyceliumNode {
    id: UInt64
    position: Vec3
    connections: List<NodeConnection>
    nutrients: Float64
    age: Duration
    health: Float64
}

/// Connection between mycelium nodes
pub type NodeConnection {
    target_id: UInt64
    strength: Float64
    flow_rate: Float64
    bidirectional: Bool
}

/// The complete mycelium network
pub type MyceliumNetwork {
    nodes: Map<UInt64, MyceliumNode>
    total_nutrients: Float64
    age: Duration
    growth_rate: Float64
}

// ============================================================================
// BEHAVIORS
// ============================================================================

/// Network growth behavior
trait NetworkGrowth {
    /// Attempt to grow a new node
    grow_node: fun(
        val parent: UInt64,
        val direction: Vec3
    ) -> Option<UInt64>

    /// Extend existing connection
    extend_connection: fun(
        val from: UInt64,
        val to: UInt64,
        val strength: Float64
    ) -> Bool

    /// Prune weak connections
    prune: fun(val threshold: Float64) -> UInt32
}

/// Nutrient distribution behavior
trait NutrientFlow {
    /// Distribute nutrients from source
    distribute: fun(
        val source: UInt64,
        val amount: Float64
    ) -> Map<UInt64, Float64>

    /// Calculate flow between nodes
    calculate_flow: fun(
        val from: UInt64,
        val to: UInt64
    ) -> Float64

    /// Find nutrient-rich path
    find_nutrient_path: fun(
        val from: UInt64,
        val to: UInt64
    ) -> Option<List<UInt64>>
}

/// Network intelligence
trait NetworkIntelligence {
    /// Find shortest path
    shortest_path: fun(
        val from: UInt64,
        val to: UInt64
    ) -> Option<List<UInt64>>

    /// Detect network partitions
    find_partitions: fun() -> List<List<UInt64>>

    /// Calculate network centrality
    centrality: fun(val node: UInt64) -> Float64
}

// ============================================================================
// CONSTRAINTS
// ============================================================================

/// Physical constraints on the network
constraint PhysicalLimits {
    forall node in network.nodes {
        // Nutrients cannot be negative
        node.nutrients >= 0.0

        // Health must be in valid range
        node.health >= 0.0
        node.health <= 1.0

        // Connection strength bounded
        forall conn in node.connections {
            conn.strength >= 0.0
            conn.strength <= 1.0
        }
    }
}

/// Network topology constraints
constraint TopologyRules {
    forall node in network.nodes {
        // Each node must have at least one connection
        // (except during initial growth)
        node.age > Duration.seconds(10) implies
            node.connections.length > 0

        // Maximum connection limit (biological constraint)
        node.connections.length <= 8
    }
}

/// Conservation of nutrients
constraint NutrientConservation {
    // Total nutrients in system is conserved
    // (minus decay and consumption)
    val total_before = network.total_nutrients
    val total_after = sum(network.nodes, |n| n.nutrients)
    val decay = calculate_decay(network)

    abs(total_before - total_after - decay) < 0.001
}

// ============================================================================
// SIMULATION
// ============================================================================

/// Simulation state
pub type SimulationState {
    network: MyceliumNetwork
    time: Duration
    step: UInt64
    rng: RandomState
}

/// Simulation configuration
pub type SimConfig {
    growth_probability: Float64
    prune_threshold: Float64
    nutrient_decay_rate: Float64
    time_step: Duration
    max_nodes: UInt32
}

/// Run one simulation step
fun simulate_step(
    var state: SimulationState,
    val config: SimConfig
) -> SimulationState {
    // 1. Age all nodes
    forall node in state.network.nodes {
        node.age = node.age + config.time_step
    }

    // 2. Distribute nutrients
    distribute_nutrients(state.network, config.nutrient_decay_rate)

    // 3. Attempt growth
    if state.rng.next_float() < config.growth_probability {
        attempt_growth(state.network, config.max_nodes)
    }

    // 4. Prune weak connections
    prune_connections(state.network, config.prune_threshold)

    // 5. Update simulation state
    state.time = state.time + config.time_step
    state.step = state.step + 1

    state
}

/// Run simulation for specified duration
fun simulate(
    val initial: MyceliumNetwork,
    val config: SimConfig,
    val duration: Duration
) -> List<SimulationState> {
    var state = SimulationState {
        network: initial,
        time: Duration.zero(),
        step: 0,
        rng: RandomState.new()
    }

    var history = []

    while state.time < duration {
        history.push(state.clone())
        state = simulate_step(state, config)
    }

    history
}

// ============================================================================
// ANALYSIS
// ============================================================================

/// Network statistics
pub type NetworkStats {
    node_count: UInt32
    edge_count: UInt32
    average_degree: Float64
    clustering_coefficient: Float64
    diameter: UInt32
    total_nutrients: Float64
}

/// Calculate network statistics
fun analyze(val network: MyceliumNetwork) -> NetworkStats {
    val nodes = network.nodes.values()
    val edges = nodes.flat_map(|n| n.connections)

    NetworkStats {
        node_count: nodes.length,
        edge_count: edges.length / 2,  // undirected
        average_degree: edges.length / nodes.length,
        clustering_coefficient: calculate_clustering(network),
        diameter: calculate_diameter(network),
        total_nutrients: nodes.fold(0.0, |acc, n| acc + n.nutrients)
    }
}
```

---

## Type Evolution Example

```dol
mod versioning.example @ 0.2.0

/// Version 1 of User type
pub type UserV1 {
    id: UInt64
    name: String
    email: String
}

/// Version 2 with additional fields
pub type UserV2 {
    id: UInt64
    name: String
    email: String
    avatar_url: Option<String>
    created_at: Timestamp
    settings: UserSettings
}

/// Evolution declaration
evolves UserV1 > UserV2 @ 2.0.0 {
    + avatar_url: Option<String> = None
    + created_at: Timestamp = Timestamp.now()
    + settings: UserSettings = UserSettings.default()
    // "Adding user customization features"
}

/// Automatic migration function
fun migrate_user(val v1: UserV1) -> UserV2 {
    UserV2 {
        id: v1.id,
        name: v1.name,
        email: v1.email,
        avatar_url: None,
        created_at: Timestamp.now(),
        settings: UserSettings.default()
    }
}
```

---

## Trait Hierarchies

```dol
mod stdlib.collections @ 0.1.0

/// Base collection trait
trait Collection<T> {
    length: fun() -> UInt64
    is_empty: fun() -> Bool = { self.length() == 0 }
}

/// Iterable collections
trait Iterable<T> extends Collection<T> {
    iter: fun() -> Iterator<T>

    forall: fun(f: fun(T) -> Void) -> Void = {
        var it = self.iter()
        while let Some(item) = it.next() {
            f(item)
        }
    }
}

/// Indexed access
trait Indexed<T> extends Collection<T> {
    get: fun(index: UInt64) -> Option<T>
    get_unchecked: fun(index: UInt64) -> T

    first: fun() -> Option<T> = { self.get(0) }
    last: fun() -> Option<T> = {
        if self.is_empty() { None }
        else { self.get(self.length() - 1) }
    }
}

/// Mutable collections
trait MutableCollection<T> extends Collection<T> {
    push: fun(item: T) -> Void
    pop: fun() -> Option<T>
    clear: fun() -> Void
}

/// Searchable collections
trait Searchable<T> extends Iterable<T> {
    find: fun(predicate: fun(T) -> Bool) -> Option<T>
    contains: fun(item: T) -> Bool
    position: fun(item: T) -> Option<UInt64>
}
```

---

## System Definition

```dol
mod infrastructure.microservices @ 0.1.0

/// Define a microservices system
system OrderProcessing {
    components {
        api_gateway: ApiGateway
        order_service: OrderService
        inventory_service: InventoryService
        payment_service: PaymentService
        notification_service: NotificationService
    }

    connections {
        api_gateway -> order_service: HTTP
        order_service -> inventory_service: gRPC
        order_service -> payment_service: gRPC
        order_service -> notification_service: AMQP
    }

    constraints {
        // All services must be healthy
        forall service in components {
            service.health_check() == Healthy
        }

        // Payment must respond within 5 seconds
        payment_service.latency_p99 < Duration.seconds(5)

        // Inventory updates must be consistent
        inventory_service.consistency == Strong
    }
}

exegesis {
    Order processing system with event-driven architecture.
    The API gateway routes requests to the order service,
    which coordinates with inventory, payment, and notifications.
}
```

---

## Running the Examples

```bash
# Parse and validate
dol check examples/

# Generate Rust code
dol compile examples/container.dol --output generated/

# Run tests
dol test examples/

# Interactive exploration
dol repl
> :load examples/biology/mycelium.dol
> analyze(sample_network)
```

---

## More Examples

Find more examples in the repository:

- `examples/genes/` - Gene definitions
- `examples/traits/` - Trait definitions
- `examples/constraints/` - Constraint examples
- `examples/systems/` - System definitions
- `examples/evolutions/` - Type evolution examples
- `examples/stdlib/` - Standard library examples
- `dol/` - Self-hosted compiler (DOL in DOL!)

---

*"Programs must be written for people to read, and only incidentally for machines to execute."* — Harold Abelson
