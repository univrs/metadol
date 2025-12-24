# Biology Module Examples

The biology module demonstrates DOL 2.0 capabilities through
biologically-inspired modeling.

## Overview

```
biology/
├── types.dol      # Vec3, Gradient, Nutrient, Energy
├── hyphal.dol     # Hyphal growth and branching
├── transport.dol  # Source-sink dynamics
├── ecosystem.dol  # Lotka-Volterra population dynamics
├── evolution.dol  # Speciation via evolves
├── mycelium.dol   # Complete network simulation
└── mod.dol        # Module index
```

## Core Types

### Vec3 - 3D Spatial Vector

```dol
pub gene Vec3 {
    has x: Float64 = 0.0
    has y: Float64 = 0.0
    has z: Float64 = 0.0

    fun magnitude() -> Float64 {
        return sqrt(this.x*this.x + this.y*this.y + this.z*this.z)
    }

    fun normalize() -> Vec3 {
        mag = this.magnitude()
        if mag == 0.0 {
            return this
        }
        return Vec3 {
            x: this.x / mag,
            y: this.y / mag,
            z: this.z / mag
        }
    }

    fun dot(other: Vec3) -> Float64 {
        return this.x*other.x + this.y*other.y + this.z*other.z
    }

    exegesis {
        3D vector for spatial calculations in biological simulations.
    }
}
```

### Nutrient - Elemental Composition

```dol
/// Follows Redfield ratio: C:N:P = 106:16:1
pub gene Nutrient {
    has carbon: Float64 = 0.0
    has nitrogen: Float64 = 0.0
    has phosphorus: Float64 = 0.0
    has water: Float64 = 0.0

    fun total_mass() -> Float64 {
        return this.carbon + this.nitrogen + this.phosphorus + this.water
    }

    constraint non_negative {
        this.carbon >= 0.0 &&
        this.nitrogen >= 0.0 &&
        this.phosphorus >= 0.0 &&
        this.water >= 0.0
    }

    constraint stoichiometry {
        // Approximate Redfield ratio constraints
        (this.carbon / this.nitrogen) > 5.0 implies
            (this.nitrogen / this.phosphorus) > 10.0
    }

    exegesis {
        Nutrient composition following the Redfield ratio found in
        marine phytoplankton and many other organisms.
    }
}
```

### Energy - ATP/ADP System

```dol
pub gene Energy {
    has atp: Float64 = 0.0
    has adp: Float64 = 0.0

    fun total() -> Float64 {
        return this.atp + this.adp
    }

    fun charge_ratio() -> Float64 {
        total = this.total()
        if total == 0.0 {
            return 0.0
        }
        return this.atp / total
    }

    fun consume(amount: Float64) -> Energy {
        actual = min(amount, this.atp)
        return Energy {
            atp: this.atp - actual,
            adp: this.adp + actual
        }
    }

    constraint conservation {
        this.total() >= 0.0
    }

    exegesis {
        Energy currency modeled on cellular ATP/ADP system.
    }
}
```

## Ecosystem Dynamics

### Species with Lotka-Volterra

```dol
pub gene Species {
    has name: String
    has population: Float64
    has carrying_capacity: Float64
    has growth_rate: Float64

    fun logistic_growth(dt: Float64) -> Float64 {
        // dN/dt = rN(1 - N/K)
        return this.growth_rate * this.population *
               (1.0 - this.population / this.carrying_capacity) * dt
    }

    fun step(dt: Float64) -> Species {
        delta = this.logistic_growth(dt)
        return Species {
            ...this,
            population: max(0.0, this.population + delta)
        }
    }

    constraint viable {
        this.population >= 0.0 &&
        this.carrying_capacity > 0.0
    }

    exegesis {
        Species population dynamics using logistic growth model.
    }
}
```

### Predator-Prey System

```dol
pub system PredatorPrey {
    has predator: Species
    has prey: Species
    has predation_rate: Float64 = 0.01
    has conversion_efficiency: Float64 = 0.1

    fun step(dt: Float64) -> PredatorPrey {
        // Lotka-Volterra equations
        prey_growth = this.prey.growth_rate * this.prey.population
        predation = this.predation_rate * this.predator.population * this.prey.population
        pred_growth = this.conversion_efficiency * predation
        pred_death = 0.1 * this.predator.population

        return PredatorPrey {
            predator: Species {
                ...this.predator,
                population: this.predator.population + (pred_growth - pred_death) * dt
            },
            prey: Species {
                ...this.prey,
                population: this.prey.population + (prey_growth - predation) * dt
            },
            ...this
        }
    }

    exegesis {
        Classic predator-prey dynamics using Lotka-Volterra equations.
    }
}
```

## Hyphal Growth

### HyphalTip - Growing tip of mycelium

```dol
pub gene HyphalTip {
    has position: Vec3
    has direction: Vec3
    has growth_rate: Float64 = 1.0
    has age: Float64 = 0.0
    has branch_probability: Float64 = 0.01

    fun grow(dt: Float64, nutrients: Nutrient) -> HyphalTip {
        // Growth rate depends on nutrient availability
        nutrient_factor = min(1.0, nutrients.total_mass() / 100.0)
        effective_rate = this.growth_rate * nutrient_factor

        new_pos = Vec3 {
            x: this.position.x + this.direction.x * effective_rate * dt,
            y: this.position.y + this.direction.y * effective_rate * dt,
            z: this.position.z + this.direction.z * effective_rate * dt
        }

        return HyphalTip {
            ...this,
            position: new_pos,
            age: this.age + dt
        }
    }

    constraint valid_direction {
        this.direction.magnitude() > 0.9 &&
        this.direction.magnitude() < 1.1
    }

    exegesis {
        Represents the growing tip of a fungal hypha.
        Growth is tropism-responsive and nutrient-dependent.
    }
}
```

## Evolution via `evolves`

### Speciation Example

```dol
/// Prokaryote: simple cell without nucleus
pub gene Prokaryote {
    has genome_size: UInt64
    has cell_wall: Bool = true
    has flagella: UInt64 = 0

    exegesis {
        Prokaryotic cell - the original form of life.
        No membrane-bound organelles.
    }
}

/// Eukaryote evolves from Prokaryote via endosymbiosis
evolves Prokaryote > Eukaryote @ 2.0Gya {
    added nucleus: Bool = true
    added mitochondria: UInt64 = 1
    added cell_size: Float64 = 10.0
    added organelles: List<String> = []

    migrate from Prokaryote {
        return Eukaryote {
            genome_size: old.genome_size * 10,
            cell_wall: old.cell_wall,
            flagella: old.flagella,
            nucleus: true,
            mitochondria: 1,
            cell_size: 10.0,
            organelles: ["mitochondria"]
        }
    }

    exegesis {
        Major evolutionary transition ~2 billion years ago.
        Endosymbiotic origin of mitochondria.
    }
}

/// Multicellular evolves from Eukaryote
evolves Eukaryote > Multicellular @ 600Mya {
    added cell_count: UInt64 = 2
    added differentiation: Bool = false
    added cell_types: List<String> = []

    migrate from Eukaryote {
        return Multicellular {
            ...old,
            cell_count: 2,
            differentiation: false,
            cell_types: []
        }
    }

    exegesis {
        Transition to multicellularity ~600 million years ago.
        Enabled specialization and complex body plans.
    }
}
```

## Running the Examples

```bash
# Validate all biology files
dol-check examples/stdlib/biology/

# Generate Rust code
dol-codegen examples/stdlib/biology/ -o generated/biology/

# Run biology tests
cargo test biology

# View generated code
cat generated/biology/species.rs
```

## Integration with Rust

The generated Rust code integrates seamlessly:

```rust
use dol_biology::{Species, PredatorPrey, Vec3};

fn main() {
    let rabbit = Species::new("Rabbit", 100.0, 1000.0, 0.5);
    let fox = Species::new("Fox", 10.0, 100.0, 0.1);

    let mut ecosystem = PredatorPrey::new(fox, rabbit);

    for _ in 0..1000 {
        ecosystem = ecosystem.step(0.1);
        println!("Prey: {:.1}, Predator: {:.1}",
                 ecosystem.prey.population,
                 ecosystem.predator.population);
    }
}
```
