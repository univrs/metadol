//! Tests for DOL biology module

use std::fs;
use std::path::Path;

const BIOLOGY_DIR: &str = "examples/stdlib/biology";

/// Helper function to check if a file exists and has content
fn file_exists_with_content(filename: &str) -> bool {
    let path = Path::new(BIOLOGY_DIR).join(filename);
    if let Ok(content) = fs::read_to_string(&path) {
        !content.is_empty()
    } else {
        false
    }
}

/// Helper function to check if file contains expected keywords
fn file_contains(filename: &str, keywords: &[&str]) -> bool {
    let path = Path::new(BIOLOGY_DIR).join(filename);
    if let Ok(content) = fs::read_to_string(&path) {
        keywords.iter().all(|kw| content.contains(kw))
    } else {
        false
    }
}

#[test]
fn test_biology_types_exists() {
    assert!(
        file_exists_with_content("types.dol"),
        "types.dol should exist in examples/stdlib/biology/"
    );
}

#[test]
fn test_biology_types_content() {
    assert!(
        file_contains(
            "types.dol",
            &["gene Vec3", "gene Nutrient", "gene Energy", "gene GeoTime"]
        ),
        "types.dol should contain Vec3, Nutrient, Energy, and GeoTime genes"
    );
}

#[test]
fn test_hyphal_exists() {
    assert!(
        file_exists_with_content("hyphal.dol"),
        "hyphal.dol should exist in examples/stdlib/biology/"
    );
}

#[test]
fn test_hyphal_content() {
    assert!(
        file_contains(
            "hyphal.dol",
            &["gene HyphalTip", "trait Hyphal", "impl Hyphal"]
        ),
        "hyphal.dol should contain HyphalTip gene, Hyphal trait, and implementation"
    );
}

#[test]
fn test_transport_exists() {
    assert!(
        file_exists_with_content("transport.dol"),
        "transport.dol should exist in examples/stdlib/biology/"
    );
}

#[test]
fn test_transport_content() {
    assert!(
        file_contains(
            "transport.dol",
            &["gene TransportNode", "gene Flow", "trait Transport"]
        ),
        "transport.dol should contain TransportNode, Flow, and Transport trait"
    );
}

#[test]
fn test_ecosystem_exists() {
    assert!(
        file_exists_with_content("ecosystem.dol"),
        "ecosystem.dol should exist in examples/stdlib/biology/"
    );
}

#[test]
fn test_ecosystem_content() {
    assert!(
        file_contains(
            "ecosystem.dol",
            &["gene Species", "gene Interaction", "system Ecosystem"]
        ),
        "ecosystem.dol should contain Species, Interaction, and Ecosystem system"
    );
}

#[test]
fn test_evolution_exists() {
    assert!(
        file_exists_with_content("evolution.dol"),
        "evolution.dol should exist in examples/stdlib/biology/"
    );
}

#[test]
fn test_evolution_content() {
    assert!(
        file_contains(
            "evolution.dol",
            &["gene Trait", "gene Genome", "trait Evolvable", "evolves"]
        ),
        "evolution.dol should contain Trait, Genome, Evolvable, and evolves declarations"
    );
}

#[test]
fn test_mycelium_exists() {
    assert!(
        file_exists_with_content("mycelium.dol"),
        "mycelium.dol should exist in examples/stdlib/biology/"
    );
}

#[test]
fn test_mycelium_content() {
    assert!(
        file_contains(
            "mycelium.dol",
            &[
                "gene MyceliumNode",
                "system MyceliumNetwork",
                "fun from_spore",
                "fun grow"
            ]
        ),
        "mycelium.dol should contain MyceliumNode, MyceliumNetwork, from_spore, and grow"
    );
}

#[test]
fn test_module_index_exists() {
    assert!(
        file_exists_with_content("mod.dol"),
        "mod.dol should exist in examples/stdlib/biology/"
    );
}

#[test]
fn test_module_index_content() {
    assert!(
        file_contains(
            "mod.dol",
            &[
                "module biology",
                "pub use types",
                "pub use hyphal",
                "pub use mycelium"
            ]
        ),
        "mod.dol should re-export all biology submodules"
    );
}

#[test]
fn test_all_biology_files_present() {
    let expected_files = [
        "types.dol",
        "hyphal.dol",
        "transport.dol",
        "ecosystem.dol",
        "evolution.dol",
        "mycelium.dol",
        "mod.dol",
    ];

    for file in &expected_files {
        assert!(
            file_exists_with_content(file),
            "Missing biology file: {}",
            file
        );
    }
}

#[test]
fn test_exegesis_present() {
    // All DOL files should have exegesis documentation
    let files_requiring_exegesis = [
        "types.dol",
        "hyphal.dol",
        "transport.dol",
        "ecosystem.dol",
        "evolution.dol",
        "mycelium.dol",
        "mod.dol",
    ];

    for file in &files_requiring_exegesis {
        assert!(
            file_contains(file, &["exegesis {"]),
            "{} should contain exegesis documentation",
            file
        );
    }
}
