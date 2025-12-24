//! Stress tests for parser edge cases

use metadol::parse_file;

#[test]
fn test_nested_generics() {
    let source = include_str!("corpus/genes/nested_generics.dol");
    let result = parse_file(source);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_complex_constraints() {
    let source = include_str!("corpus/genes/complex_constraints.dol");
    let result = parse_file(source);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_nested_sex_blocks() {
    let source = include_str!("corpus/sex/nested_sex.dol");
    let result = parse_file(source);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_evolution_chain() {
    let source = include_str!("corpus/genes/evolution_chain.dol");
    let result = parse_file(source);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_trait_relationships() {
    let source = include_str!("corpus/traits/trait_relationships.dol");
    let result = parse_file(source);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}
