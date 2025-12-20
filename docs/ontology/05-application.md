# Application: How Univrs Maps to the Ontology

> "In theory, theory and practice are the same. In practice, they are not."
> — Attributed to many

This document bridges the conceptual foundations to the concrete Univrs 
implementation. We show how each component of Univrs instantiates the 
ontological primitives, respects the foundational constraints, and 
participates in the information flows.

## Univrs as Ontological System

Univrs is a container orchestration platform. Viewed through our ontology:

```
┌────────────────────────────────────────────────────────────────────┐
│                          UNIVRS SYSTEM                             │
├────────────────────────────────────────────────────────────────────┤
│  CONTINUANTS                                                       │
│  ├── Containers: Workload units with identity and lifecycle        │
│  ├── Nodes: Physical/virtual machines hosting containers           │
│  ├── Cluster: Collection of nodes forming a coherent system        │
│  ├── Identities: Cryptographic keypairs for authentication         │
│  └── Resources: CPU, memory, storage, network allocations          │
├────────────────────────────────────────────────────────────────────┤
│  OCCURRENTS                                                        │
│  ├── Lifecycle Events: Create, start, stop, remove                 │
│  ├── Cluster Events: Node join, node leave, partition, heal        │
│  ├── Scheduling Decisions: Place, migrate, evict                   │
│  ├── Consensus Rounds: Raft elections, log replication             │
│  └── Reconciliation Cycles: Sense, compare, actuate                │
├────────────────────────────────────────────────────────────────────┤
│  RELATIONS                                                         │
│  ├── Placement: Container placed_on Node                           │
│  ├── Membership: Node member_of Cluster                            │
│  ├── Dependency: Container depends_on Container                    │
│  ├── Ownership: Identity owns Container                            │
│  └── Composition: Cluster composed_of Nodes                        │
├────────────────────────────────────────────────────────────────────┤
│  TRANSFORMATIONS                                                   │
│  ├── Container Lifecycle: State transitions                        │
│  ├── Cluster Membership: Join/leave transformations                │
│  ├── Resource Allocation: Binding resources to containers          │
│  └── State Reconciliation: Converging actual to desired            │
└────────────────────────────────────────────────────────────────────┘
```

## Core Continuants

### Container

The container is Univrs's primary workload primitive:

```dol
gene univrs.container {
  uses continuant.entity
  uses continuant.identity
  uses continuant.boundary
  uses physical.system
  
  container is continuant
  
  container has identity
  identity derives from ed25519 keypair
  identity is self-sovereign
  identity requires no central authority
  
  container has boundaries
  boundaries include namespace isolation
  boundaries include resource limits
  boundaries include network segmentation
  
  container has state
  state is observable
  state is one of: created, starting, running, pausing, paused, stopping, stopped, removing, removed
  
  container has resources
  resources include cpu shares
  resources include memory limit
  resources include storage quota
  resources include network bandwidth
}

exegesis {
  A container embodies the ontological concept of a bounded,
  identifiable continuant. It persists through state changes
  (starting, stopping) while maintaining its identity.
  
  The Ed25519 cryptographic identity ensures that identity is
  intrinsic to the container, not assigned by external authority.
  This aligns with Univrs's self-sovereign design philosophy.
}
```

### Node

The node is the physical substrate on which containers run:

```dol
gene univrs.node {
  uses continuant.entity
  uses continuant.identity
  uses physical.system
  
  node is continuant
  
  node has identity
  identity derives from ed25519 keypair
  
  node has physical substrate
  substrate is machine (physical or virtual)
  
  node has capacity
  capacity includes cpu cores
  capacity includes memory bytes
  capacity includes storage bytes
  capacity includes network bandwidth
  
  node has allocation
  allocation is resources assigned to containers
  allocation is less than or equal to capacity
  
  node has health
  health is observable
  health affects scheduling decisions
}

exegesis {
  The node is where abstraction meets physical reality. It has
  finite capacity (conservation), consumes energy, and can fail
  (entropy). Understanding nodes as physical systems keeps our
  orchestration grounded.
}
```

### Cluster

The cluster is the emergent whole composed of nodes:

```dol
gene univrs.cluster {
  uses continuant.entity
  uses transformation.composition
  
  cluster is continuant
  cluster is composed of nodes
  
  cluster has identity
  identity is derived from member identities
  
  cluster has state
  state is consensus of member states
  
  cluster has invariants
  invariants hold despite member changes
  
  cluster has emergent properties
  emergent includes fault tolerance
  emergent includes scalability
  emergent includes availability
}

exegesis {
  The cluster demonstrates ontological composition—a whole with
  properties its parts (individual nodes) lack. A single node
  cannot be fault-tolerant; a cluster can.
  
  The cluster's identity persists even as members change,
  exemplifying continuant persistence through change.
}
```

### Identity

Cryptographic identity is the foundation of authentication:

```dol
gene univrs.identity {
  uses continuant.identity
  uses information.encoding
  
  identity has keypair
  keypair is ed25519
  
  keypair has public key
  keypair has private key
  
  public key is shareable
  private key is secret
  private key never leaves owner
  
  identity enables signing
  signing proves possession of private key
  
  identity enables verification
  verification confirms signature validity
  
  identity enables encryption
  encryption ensures confidentiality
}

exegesis {
  Identity in Univrs is cryptographic, not administrative.
  There is no central identity provider to trust (or compromise).
  Each entity generates its own keypair and proves identity
  through cryptographic operations.
  
  This maps to the ontological principle that identity is
  intrinsic, not assigned.
}
```

## Core Occurrents

### Container Lifecycle Events

```dol
trait univrs.container.lifecycle {
  uses univrs.container
  uses occurrent.event
  uses transformation.state_transition
  
  // Events are occurrents with participants
  creation is event
  creation has participant container as patient
  creation has participant orchestrator as agent
  creation has participant node as location
  
  start is event
  start requires container in created state
  start transitions container to starting then running
  
  stop is event
  stop requires container in running state
  stop transitions container to stopping then stopped
  
  removal is event
  removal requires container in stopped state
  removal transitions container to removing then removed
  removal is destruction of container
  
  // Temporal ordering
  creation precedes start
  start precedes stop
  stop precedes removal
  
  // Each event is logged
  each event emits log entry
  each log entry is signed by agent
}

exegesis {
  Lifecycle events are the occurrents that transform containers.
  The explicit temporal ordering (creation precedes start) and
  participant roles (container as patient, orchestrator as agent)
  provide precise semantics for reasoning about system behavior.
}
```

### Cluster Events

```dol
trait univrs.cluster.events {
  uses univrs.cluster
  uses univrs.node
  uses occurrent.event
  
  node_join is event
  node_join adds node to cluster
  node_join requires authentication
  node_join triggers rebalancing
  
  node_leave is event
  node_leave removes node from cluster
  node_leave may be graceful or abrupt
  graceful allows container migration
  abrupt triggers failover
  
  partition is event
  partition splits cluster into groups
  groups cannot communicate
  each group may continue independently
  
  heal is event
  heal reconnects partitioned groups
  heal requires conflict resolution
  heal restores cluster invariants
}

exegesis {
  Cluster events capture the dynamics of distributed systems.
  Partitions are not bugs but physical reality (network failures).
  The ontology makes this explicit rather than assuming
  perfect connectivity.
}
```

### Scheduling Decisions

```dol
trait univrs.scheduling {
  uses univrs.container
  uses univrs.node
  uses univrs.cluster
  uses occurrent.process
  
  scheduling is process
  scheduling has goal: place containers optimally
  
  scheduling has phases:
    filter_nodes: exclude unsuitable nodes
    score_nodes: rank suitable nodes
    select_node: choose best node
    bind_container: assign container to node
  
  scheduling respects constraints:
    resource constraints: capacity >= requested
    affinity constraints: preferred co-location
    anti-affinity constraints: required separation
    topology constraints: zone/region awareness
  
  scheduling optimizes objectives:
    resource utilization
    workload spread
    failure domain distribution
}

exegesis {
  Scheduling is a transformation that establishes the placement
  relation between containers and nodes. It respects conservation
  (can't allocate more than capacity) and is deterministic
  given the same inputs (for auditability).
}
```

### Consensus Rounds

```dol
trait univrs.consensus {
  uses univrs.cluster
  uses occurrent.event
  uses information.channel
  
  consensus establishes agreement
  agreement is on cluster state
  
  // Raft consensus
  consensus has leader
  leader is single node
  leader handles writes
  
  consensus has followers
  followers replicate leader state
  
  consensus has election
  election chooses new leader
  election triggered by leader failure
  election requires majority
  
  consensus has log replication
  replication ensures durability
  replication requires acknowledgment
  acknowledgment from majority
}

exegesis {
  Consensus is how distributed continuants maintain coherent
  state despite physical separation and potential failures.
  The requirement for majority acknowledges the impossibility
  of instantaneous global consistency (speed of light limits).
}
```

## Information Flows

### Event Stream

```dol
system univrs.events @ 0.1.0 {
  uses information.channel
  uses information.encoding
  
  event_stream is channel
  event_stream carries events
  
  events are encoded as protocol buffers
  events are signed by source
  events have monotonic sequence numbers
  
  consumers subscribe to event_stream
  subscriptions may filter by type
  subscriptions may replay from sequence
  
  event_stream has durability
  events persisted for retention period
  older events may be compacted
}

exegesis {
  The event stream is Univrs's information backbone. All state
  changes flow through it, enabling observation, replay, and
  audit. Monotonic sequence numbers establish causal ordering.
}
```

### State Store

```dol
system univrs.state @ 0.1.0 {
  uses information.storage
  uses univrs.consensus
  
  state_store is storage
  state_store holds cluster state
  
  state is key-value pairs
  keys are hierarchical paths
  values are serialized structures
  
  writes go through consensus
  reads may be from leader or follower
  stale reads are possible from follower
  
  state has versions
  versions enable optimistic concurrency
  conflicts detected by version mismatch
}

exegesis {
  The state store is how Univrs persists information. Consensus
  ensures durability despite failures. Versioning enables
  concurrent access without locks.
}
```

### Metrics Pipeline

```dol
system univrs.metrics @ 0.1.0 {
  uses information.sensing
  uses information.channel
  
  metrics are sensing results
  metrics capture cluster state
  
  metrics include:
    container_cpu_usage
    container_memory_usage
    node_capacity_available
    cluster_health_score
  
  metrics are sampled
  sampling has interval
  sampling has precision
  
  metrics are transmitted
  transmission has latency
  metrics may be aggregated
  aggregation reduces volume
  aggregation loses detail
}

exegesis {
  Metrics are how Univrs senses physical reality. The pipeline
  acknowledges information-theoretic limits: sampling loses
  temporal detail, aggregation loses spatial detail.
}
```

## Transformation Patterns

### Reconciliation Loop

The core of Kubernetes-style orchestration:

```dol
trait univrs.reconciliation {
  uses transformation.structure
  uses information.control_loop
  
  reconciliation is control loop
  
  sense: read current state from cluster
  compare: diff current against desired
  plan: compute actions to reduce diff
  actuate: execute planned actions
  repeat: continue until converged or timeout
  
  reconciliation has period
  period is time between iterations
  
  reconciliation has tolerance
  tolerance is acceptable diff
  
  reconciliation may not converge
  non-convergence indicates problem
}

exegesis {
  Reconciliation is the transformation pattern that makes
  declarative orchestration work. Instead of imperative
  commands, users declare desired state; the system
  continuously transforms current state toward desired.
}
```

### Migration

```dol
trait univrs.migration {
  uses univrs.container
  uses univrs.node
  uses transformation.transfer
  
  migration is transfer
  migration moves container between nodes
  
  migration has source node
  migration has target node
  migration has container
  
  migration phases:
    checkpoint: save container state
    transfer: move state to target
    restore: recreate container from state
    cleanup: remove from source
  
  migration may be live
  live migration minimizes downtime
  live migration has pre-copy and post-copy phases
  
  migration may fail
  failure requires rollback
  rollback restores to source
}

exegesis {
  Migration demonstrates the physical grounding of our ontology.
  Moving a container means moving information (state) through
  a channel (network) with latency, bandwidth, and fidelity
  constraints. The phases acknowledge physical reality.
}
```

## Constraint Enforcement

```dol
constraint univrs.integrity {
  uses univrs.container
  uses univrs.node
  uses univrs.cluster
  uses conservation.resources
  
  // Resource integrity
  total allocation on node <= node capacity
  // No over-commitment beyond policy
  
  // Identity integrity
  container identity never changes
  node identity never changes
  // Essential properties are immutable
  
  // Causal integrity
  events have monotonic timestamps
  effects follow causes
  // Temporal ordering is preserved
  
  // Availability integrity
  cluster survives minority failure
  data persists through failure
  // Redundancy maintains availability
}

exegesis {
  These constraints are not arbitrary policy but derive from
  the foundational ontology. Resource integrity follows from
  conservation. Identity integrity follows from continuant
  persistence. Causal integrity follows from temporal ordering.
  Availability integrity follows from entropy management.
}
```

## Implementation Independence

The power of this ontological design:

### Same Ontology, Different Implementation

```dol
// Current: Rust implementation
system univrs.rust_impl @ 0.1.0 {
  implements univrs.container
  implements univrs.node
  implements univrs.cluster
  
  language is rust
  runtime is youki
  networking is chitchat
  consensus is openraft
}

// Future: Zig implementation
system univrs.zig_impl @ 0.1.0 {
  implements univrs.container
  implements univrs.node
  implements univrs.cluster
  
  language is zig
  runtime is alternative_oci_runtime
  networking is alternative_gossip
  consensus is alternative_raft
}

// Far future: Quantum implementation
system univrs.quantum_impl @ 0.1.0 {
  implements univrs.container
  implements univrs.node
  implements univrs.cluster
  
  substrate is quantum
  state is superposition
  consensus is quantum_byzantine_agreement
}
```

The ontology remains stable. Implementations vary.

## Mapping Summary

| Ontological Concept | Univrs Instantiation |
|---------------------|----------------------|
| Continuant | Container, Node, Cluster, Identity |
| Occurrent | Lifecycle events, Cluster events, Scheduling |
| Relation | Placement, Membership, Dependency, Ownership |
| Transformation | State transitions, Migrations, Reconciliation |
| Information Channel | Event stream, Gossip, Consensus log |
| Information Storage | State store, Logs, Metrics |
| Sensing | Metrics collection, Health checks |
| Actuation | Container operations, Resource allocation |
| Conservation | Resource accounting, Capacity limits |
| Entropy | Failure handling, Error correction, Redundancy |

## The Path Forward

With this ontological foundation:

1. **Design new features** by first modeling them ontologically
2. **Evaluate implementations** against ontological constraints
3. **Evolve the system** while preserving ontological invariants
4. **Change implementations** without changing the ontology
5. **Communicate precisely** using shared ontological vocabulary

The ontology is the stable layer. Everything else can change.

---

> "The purpose of computing is insight, not numbers."
> — Richard Hamming

The purpose of orchestration is coordination, not code. The ontology 
captures the coordination; implementations produce the code.
