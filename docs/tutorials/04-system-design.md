# Tutorial 4: System Design

## What Are Systems?

**Systems** are the top-level compositions in Metal DOL. They represent complete, deployable units that combine multiple traits into cohesive architectures.

The ontological hierarchy:
- **Genes** = Atomic properties (container.exists)
- **Traits** = Behaviors (container.lifecycle)
- **Systems** = Complete architectures (container.orchestration.platform)

### System Philosophy

Systems are where ontology meets architecture:
- Systems compose traits and genes
- Systems define version requirements and compatibility
- Systems track evolution over time
- Systems represent deployable units

## System Syntax

The complete system syntax:

```dol
system qualified.identifier @X.Y.Z {
    uses trait.identifier @version
    uses trait.identifier >=version
    uses trait.identifier >version
    uses trait.identifier =version

    has property: type
    is category
    requires dependency @version

    evolves from previous.system @version
}

exegesis {
    Mandatory exegesis explaining the system architecture.
}
```

## Version Requirements

Systems introduce flexible version requirements using operators.

### Exact Version (`@`)

Require an exact version:

```dol
system user.management @1.0.0 {
    uses user.authentication @1.0.0    // Exactly 1.0.0
}

exegesis {
    User management system.
}
```

Use exact versions when:
- You need a specific implementation
- Breaking changes between versions matter
- You're in early development

### Minimum Version (`>=`)

Require a minimum version (any version >= specified):

```dol
system user.management @1.0.0 {
    uses user.authentication >=1.0.0   // 1.0.0, 1.1.0, 2.0.0, etc.
}

exegesis {
    User management system.
}
```

Use minimum versions when:
- You need features introduced in a version
- You can handle backward-compatible changes
- You want automatic updates

### Greater Than (`>`)

Require a version strictly greater than specified:

```dol
system user.management @2.0.0 {
    uses user.authentication >1.0.0    // 1.0.1, 1.1.0, 2.0.0, but NOT 1.0.0
}

exegesis {
    User management system v2.
}
```

Use greater than when:
- A specific version has known issues
- You require a bug fix or feature from newer versions

### Compatible Version (`=`)

Require a compatible version (same major version):

```dol
system user.management @1.0.0 {
    uses user.authentication =1.0.0    // 1.0.0, 1.0.1, 1.1.0, but NOT 2.0.0
}

exegesis {
    User management system.
}
```

Use compatible versions when:
- You want patch and minor updates
- You need to avoid breaking changes
- You're following semantic versioning strictly

## Composing Traits into Systems

Let's build a complete container orchestration system.

### Step 1: Define Required Traits

First, identify the traits you need:

```dol
trait container.lifecycle @1.0.0 { ... }
trait container.networking @1.0.0 { ... }
trait container.storage @1.0.0 { ... }
trait container.monitoring @1.0.0 { ... }
trait container.security @1.0.0 { ... }
```

### Step 2: Compose the System

```dol
system container.orchestration.basic @1.0.0 {
    // Core capabilities - exact versions for stability
    uses container.lifecycle @1.0.0
    uses container.networking @1.0.0
    uses container.storage @1.0.0

    // Monitoring - allow compatible updates
    uses container.monitoring =1.0.0

    // Security - require minimum version (security fixes)
    uses container.security >=1.0.0

    // System-level properties
    has max_containers: integer
    has default_network: string
    has storage_driver: string

    // System-level events
    emits system.started
    emits system.stopped
    emits system.error
    emits system.capacity_warning

    // Categories
    is orchestrator
    is distributed
    is resilient
}

exegesis {
    Basic container orchestration system.

    This system provides fundamental container management capabilities:
    - Lifecycle management (create, start, stop, destroy)
    - Network configuration and connectivity
    - Volume mounting and storage management
    - Basic health monitoring
    - Security isolation

    This is the minimal viable orchestration system. For production
    deployments, consider container.orchestration.production.

    Architecture:
    - Single-host deployment
    - Local storage only
    - Basic networking (bridge mode)
    - Essential monitoring
}
```

### Step 3: Add Advanced Capabilities

Build a production system:

```dol
system container.orchestration.production @2.0.0 {
    // Inherit from basic system
    uses container.orchestration.basic @1.0.0

    // Additional traits - allow flexible versioning
    uses container.scheduling >=2.0.0
    uses container.clustering =2.0.0
    uses container.service_discovery >=1.0.0
    uses container.load_balancing >=1.5.0
    uses container.auto_scaling >1.0.0

    // Advanced monitoring
    uses monitoring.metrics =2.0.0
    uses monitoring.alerting >=1.0.0
    uses monitoring.tracing >1.0.0

    // HA and resilience
    uses cluster.consensus @2.0.0
    uses cluster.failover >=2.0.0

    // System properties
    has cluster_size: integer
    has replication_factor: integer
    has consensus_algorithm: string
    has election_timeout: integer

    // Advanced events
    emits cluster.node_joined
    emits cluster.node_left
    emits cluster.leader_elected
    emits cluster.quorum_lost
    emits service.deployed
    emits service.scaled
    emits service.failed

    // Quantifications
    quantified by cluster_health
    quantified by node_utilization
    quantified by service_availability
    quantified by request_latency

    is production_grade
    is highly_available
    is scalable
    is self_healing
}

exegesis {
    Production-grade container orchestration platform.

    Extends basic orchestration with:
    - Multi-host clustering
    - Advanced scheduling and placement
    - Service discovery and load balancing
    - Distributed storage
    - Advanced monitoring and alerting
    - High availability and failover
    - Rolling updates and rollbacks

    Architecture:
    - Multi-node cluster
    - Distributed consensus (Raft)
    - Overlay networking
    - Distributed storage (Ceph, GlusterFS)
    - Prometheus + Grafana monitoring
    - Auto-scaling

    Requirements:
    - Minimum 3 nodes for HA
    - 8GB RAM per node minimum
    - SSD storage recommended
}
```

## Evolution Tracking with `evolves`

The `evolves from` predicate tracks system evolution:

```dol
system user.management @1.0.0 {
    uses user.authentication @1.0.0
    uses user.authorization @1.0.0
}

exegesis {
    Initial user management system.
}

system user.management @2.0.0 {
    evolves from user.management @1.0.0

    uses user.authentication @2.0.0      // Updated
    uses user.authorization @2.0.0       // Updated
    uses user.sso >=1.0.0                // New
    uses user.mfa >=1.0.0                // New
}

exegesis {
    Enhanced user management with SSO and MFA.
}

system user.management @3.0.0 {
    evolves from user.management @2.0.0

    uses user.authentication @3.0.0
    uses user.authorization @3.0.0
    uses user.sso >=2.0.0
    uses user.mfa >=2.0.0
    uses user.rbac >=1.0.0               // New
    uses user.audit >=1.0.0              // New
    uses user.compliance >=1.0.0         // New
}

exegesis {
    Enterprise user management with advanced security.
}
```

Evolution tracking enables:
- Migration path documentation
- Backward compatibility analysis
- Upgrade planning
- Deprecation warnings

## Complete Example: E-Commerce System

Let's design a complete e-commerce system.

### Define the System

```dol
system ecommerce.platform @1.0.0 {
    // User subsystem
    uses user.authentication =2.0.0
    uses user.authorization >=1.0.0
    uses user.profile.management >=1.5.0
    uses user.session.management @2.1.0

    // Product subsystem
    uses product.catalog >=3.0.0
    uses product.search >2.0.0
    uses product.inventory =1.0.0
    uses product.recommendations >=1.0.0

    // Shopping subsystem
    uses shopping.cart =1.0.0
    uses shopping.wishlist >=1.0.0
    uses shopping.checkout >=2.0.0

    // Payment subsystem
    uses payment.processing >=3.0.0
    uses payment.gateway.stripe @2.5.0
    uses payment.gateway.paypal >=2.0.0
    uses payment.fraud_detection >1.0.0

    // Order subsystem
    uses order.management >=2.0.0
    uses order.fulfillment =1.0.0
    uses order.tracking >=1.0.0

    // Shipping subsystem
    uses shipping.calculation >=1.0.0
    uses shipping.carrier.integration >2.0.0

    // Analytics subsystem
    uses analytics.tracking >=1.0.0
    uses analytics.reporting >1.5.0
    uses analytics.forecasting >=1.0.0

    // Infrastructure traits
    uses infrastructure.caching =1.0.0
    uses infrastructure.cdn >=1.0.0
    uses infrastructure.messaging >1.0.0
    uses infrastructure.monitoring >=2.0.0

    // Platform properties
    has platform_name: string
    has supported_currencies: list<string>
    has supported_languages: list<string>
    has default_timezone: string
    has tax_calculation_mode: string

    // Configuration
    has max_cart_items: integer
    has session_timeout: integer
    has max_concurrent_users: integer
    has cache_ttl: integer

    // Platform events
    emits platform.started
    emits platform.stopped
    emits platform.maintenance_mode
    emits user.registered
    emits order.placed
    emits payment.completed
    emits shipment.dispatched

    // Platform metrics
    quantified by active_users
    quantified by orders_per_second
    quantified by revenue
    quantified by conversion_rate
    quantified by cart_abandonment_rate
    quantified by average_order_value
    quantified by customer_lifetime_value

    // Categories
    is ecommerce
    is microservices
    is event_driven
    is scalable
    is production_grade
    is pci_compliant
}

exegesis {
    Complete e-commerce platform architecture.

    This system implements a full-featured e-commerce platform with:

    User Management:
    - User registration and authentication
    - Profile management
    - Session handling

    Product Catalog:
    - Product browsing and search
    - Inventory management
    - Category organization

    Shopping Experience:
    - Shopping cart
    - Wishlist
    - Product recommendations

    Order Processing:
    - Checkout workflow
    - Payment processing
    - Order fulfillment
    - Shipping integration

    Business Intelligence:
    - Analytics and reporting
    - Customer insights
    - Sales forecasting

    Architecture:
    - Microservices-based
    - Event-driven
    - CQRS pattern
    - Distributed caching
    - CDN for static assets

    Scalability:
    - Horizontal scaling
    - Database sharding
    - Read replicas
    - Message queues for async processing
}
```

### Version 2.0 with Enhanced Features

```dol
system ecommerce.platform @2.0.0 {
    evolves from ecommerce.platform @1.0.0

    // All v1.0 dependencies (updated versions)
    uses user.authentication =3.0.0
    uses user.authorization >=2.0.0
    // ... other updated dependencies

    // New AI features
    uses ai.recommendations >=1.0.0
    uses ai.visual_search >1.0.0
    uses ai.chatbot =1.0.0
    uses ai.personalization >=1.0.0

    // Omnichannel
    uses omnichannel.inventory >=1.0.0
    uses omnichannel.fulfillment >1.0.0
    uses pos.integration =1.0.0

    // Mobile
    uses mobile.api >=2.0.0
    uses mobile.push_notifications >1.0.0

    // Social commerce
    uses social.facebook_shop >=1.0.0
    uses social.instagram_shop >=1.0.0
    uses social.pinterest_integration >1.0.0

    // Subscription management
    uses subscription.management >=1.0.0
    uses subscription.billing >1.0.0

    // Loyalty
    uses loyalty.program >=1.0.0
    uses loyalty.rewards =1.0.0

    // Enhanced metrics
    quantified by ai_recommendation_accuracy
    quantified by chatbot_resolution_rate
    quantified by omnichannel_conversion
    quantified by mobile_app_engagement
    quantified by social_commerce_revenue
    quantified by subscription_retention

    is ai_powered
    is omnichannel
    is mobile_first
}

exegesis {
    Enhanced e-commerce platform with AI and omnichannel support.

    New in v2.0:
    - AI-powered product recommendations
    - Visual search
    - Chatbot customer service
    - Omnichannel inventory (online + physical stores)
    - Mobile app support
    - Social commerce integration
    - Subscription management
    - Loyalty programs
}
```

## Best Practices

### 1. Document System Architecture

Use exegesis to document architecture decisions:

```dol
system payment.gateway @1.0.0 {
    // ... payment gateway implementation ...
}

exegesis {
    Payment gateway system.

    Architecture Decision Records:

    ADR-001: Event Sourcing
    We use event sourcing for payment transactions to ensure
    complete audit trails and enable time-travel debugging.

    ADR-002: Saga Pattern
    Distributed transactions use the saga pattern to handle
    failures gracefully across multiple payment providers.

    ADR-003: Circuit Breaker
    All external payment provider calls are protected by
    circuit breakers to prevent cascade failures.
}
```

### 2. Use Version Ranges Wisely

Choose the right version operator:

```dol
system production.app @1.0.0 {
    // Critical security component - allow updates
    uses security.authentication >=2.0.0

    // Stable API - lock major version
    uses api.gateway =3.0.0

    // Experimental feature - exact version
    uses experimental.feature @0.1.0

    // Bug fix required - greater than
    uses database.driver >1.2.3
}
```

### 3. Group Related Dependencies

Organize uses statements logically:

```dol
system web.application @1.0.0 {
    // Authentication & Authorization
    uses auth.authentication >=2.0.0
    uses auth.authorization >=1.0.0
    uses auth.session =1.0.0

    // Data Layer
    uses database.connection >=3.0.0
    uses database.migration =2.0.0
    uses cache.redis >5.0.0

    // API Layer
    uses api.rest >=1.0.0
    uses api.graphql >1.0.0
    uses api.websocket =1.0.0

    // Frontend
    uses frontend.spa >=4.0.0
    uses frontend.ssr >2.0.0
}

exegesis { Web application system. }
```

### 4. Track Evolution Carefully

Document what changed between versions:

```dol
system api.platform @3.0.0 {
    evolves from api.platform @2.0.0

    uses auth.oauth2 >=3.0.0              // Was auth.apikey
    uses ratelimit.token_bucket >=1.0.0   // Was ratelimit.fixed
    uses response.jsonapi =1.0.0          // Was response.custom
}

exegesis {
    API platform v3.0.

    Breaking changes from v2.0:
    - Authentication now requires OAuth 2.0 (was API keys)
    - Rate limiting changed to token bucket (was fixed window)
    - Response format is now JSON:API (was custom format)

    Migration guide: docs/migration/v2-to-v3.md
}
```

### 5. Quantify System Health

Define meaningful system metrics:

```dol
system distributed.system @1.0.0 {
    quantified by availability_slo       // 99.99% target
    quantified by latency_p99            // <100ms target
    quantified by error_rate             // <0.1% target
    quantified by throughput             // requests/second
    quantified by data_freshness         // replication lag

    has availability_target: float       // 0.9999
    has latency_target_ms: integer       // 100
    has error_rate_target: float         // 0.001
}

exegesis { Distributed system with SLO tracking. }
```

## Common Patterns

### The Subsystem Pattern

Break large systems into subsystems:

```dol
system platform.auth @1.0.0 {
    uses user.authentication >=1.0.0
    uses session.management >=1.0.0
}

exegesis { Authentication subsystem. }

system platform.data @1.0.0 {
    uses database.connection >=1.0.0
    uses cache.management >=1.0.0
}

exegesis { Data subsystem. }

system platform.complete @1.0.0 {
    uses platform.auth @1.0.0
    uses platform.data @1.0.0
}

exegesis { Complete platform. }
```

### The Environment Pattern

Model different deployment environments:

```dol
system app.development @1.0.0 {
    uses database.sqlite @1.0.0
    uses cache.memory @1.0.0
    uses monitoring.console @1.0.0
}

exegesis { Development environment. }

system app.production @1.0.0 {
    evolves from app.development @1.0.0
    uses database.postgresql >=12.0.0
    uses cache.redis >=6.0.0
    uses monitoring.datadog >=1.0.0
}

exegesis { Production environment. }
```

## Testing Your Systems

Validate system definitions:

```bash
cargo run --bin dol-parse -- my-system.dol
```

Check all dependencies are satisfied:

```bash
cargo run --bin dol-check -- my-system.dol --check-dependencies
```

Generate system documentation:

```bash
cargo run --bin dol-parse -- my-system.dol --format=markdown > system-docs.md
```

## Next Steps

Congratulations! You now understand the complete Metal DOL ontology hierarchy:

- **Genes** - Atomic units
- **Traits** - Composed behaviors
- **Systems** - Complete architectures

Explore advanced topics:
- Constraint definitions
- Evolution strategies
- Code generation
- CI/CD integration

## Key Takeaways

1. **Systems are top-level compositions** - complete architectures
2. **Version operators** - @, >=, >, = for flexible dependencies
3. **Evolution tracking** - use `evolves from` to document changes
4. **Document architecture** - exegesis explains design decisions
5. **Group dependencies** - organize uses statements logically
6. **Quantify health** - define SLOs and metrics
7. **Think in subsystems** - break complexity into manageable pieces
8. **Model environments** - development vs production systems
