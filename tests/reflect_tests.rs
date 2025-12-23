//! Integration tests for the DOL 2.0 reflection system.
//!
//! These tests verify the reflection system's ability to:
//! - Create and query type metadata
//! - Build type registries with hierarchies
//! - Support runtime type introspection patterns

use metadol::reflect::{FieldInfo, MethodInfo, TypeInfo, TypeKind, TypeRegistry};

// ============================================
// TypeInfo Tests
// ============================================

#[test]
fn test_create_record_type() {
    let user_type = TypeInfo::record("User")
        .with_field(FieldInfo::new("id", "Int64"))
        .with_field(FieldInfo::new("name", "String"))
        .with_field(FieldInfo::new("email", "String").optional())
        .with_doc("Represents a system user");

    assert_eq!(user_type.name(), "User");
    assert_eq!(user_type.kind(), TypeKind::Record);
    assert_eq!(user_type.fields().len(), 3);
    assert!(user_type.field("email").unwrap().is_optional());
}

#[test]
fn test_create_generic_type() {
    let list_type = TypeInfo::generic("List")
        .with_type_param("T")
        .with_method(MethodInfo::new("push").with_param("item", "T"))
        .with_method(MethodInfo::new("pop").returns("Option<T>"))
        .with_method(MethodInfo::new("len").returns("Int64").pure());

    assert_eq!(list_type.kind(), TypeKind::Generic);
    assert_eq!(list_type.type_params(), &["T"]);
    assert_eq!(list_type.methods().len(), 3);
    assert!(list_type.method("len").unwrap().is_pure());
}

#[test]
fn test_type_inheritance() {
    let admin_type = TypeInfo::record("Admin")
        .with_parent("User")
        .with_field(FieldInfo::new("permissions", "Array<String>"))
        .implements("Authenticatable")
        .implements("Authorizable");

    assert_eq!(admin_type.parent(), Some("User"));
    assert!(admin_type.traits().contains(&"Authenticatable".to_string()));
    assert!(admin_type.traits().contains(&"Authorizable".to_string()));
}

#[test]
fn test_function_type() {
    let handler_type = TypeInfo::function("EventHandler").with_method(
        MethodInfo::new("handle")
            .with_param("event", "Event")
            .returns("Result<(), Error>"),
    );

    assert_eq!(handler_type.kind(), TypeKind::Function);
}

// ============================================
// FieldInfo Tests
// ============================================

#[test]
fn test_field_with_all_options() {
    let field = FieldInfo::new("status", "Status")
        .optional()
        .mutable()
        .with_doc("Current status of the entity")
        .with_default("Status::Pending");

    assert_eq!(field.name(), "status");
    assert_eq!(field.type_name(), "Status");
    assert!(field.is_optional());
    assert!(field.is_mutable());
    assert_eq!(field.doc(), Some("Current status of the entity"));
    assert_eq!(field.default(), Some("Status::Pending"));
}

#[test]
fn test_required_immutable_field() {
    let field = FieldInfo::new("id", "UUID");

    assert!(!field.is_optional());
    assert!(!field.is_mutable());
    assert!(field.doc().is_none());
    assert!(field.default().is_none());
}

// ============================================
// MethodInfo Tests
// ============================================

#[test]
fn test_method_with_multiple_params() {
    let method = MethodInfo::new("transfer")
        .with_param("from", "Account")
        .with_param("to", "Account")
        .with_param("amount", "Decimal")
        .returns("Result<Transaction, Error>")
        .with_doc("Transfers funds between accounts");

    assert_eq!(method.name(), "transfer");
    assert_eq!(method.params().len(), 3);
    assert_eq!(
        method.params()[0],
        ("from".to_string(), "Account".to_string())
    );
    assert_eq!(method.return_type(), "Result<Transaction, Error>");
}

#[test]
fn test_static_method() {
    let method = MethodInfo::new("from_json")
        .with_param("json", "String")
        .returns("User")
        .static_method()
        .pure();

    assert!(method.is_static());
    assert!(method.is_pure());
}

// ============================================
// TypeRegistry Tests
// ============================================

#[test]
fn test_registry_with_primitives() {
    let registry = TypeRegistry::with_primitives();

    // Check all primitive types are registered
    assert!(registry.lookup("Void").is_some());
    assert!(registry.lookup("Bool").is_some());
    assert!(registry.lookup("Int8").is_some());
    assert!(registry.lookup("Int16").is_some());
    assert!(registry.lookup("Int32").is_some());
    assert!(registry.lookup("Int64").is_some());
    assert!(registry.lookup("UInt8").is_some());
    assert!(registry.lookup("UInt16").is_some());
    assert!(registry.lookup("UInt32").is_some());
    assert!(registry.lookup("UInt64").is_some());
    assert!(registry.lookup("Float32").is_some());
    assert!(registry.lookup("Float64").is_some());
    assert!(registry.lookup("String").is_some());

    // Verify they're all primitives
    assert_eq!(
        registry.lookup("Int32").unwrap().kind(),
        TypeKind::Primitive
    );
}

#[test]
fn test_registry_register_and_lookup() {
    let mut registry = TypeRegistry::new();

    let product = TypeInfo::record("Product")
        .with_field(FieldInfo::new("name", "String"))
        .with_field(FieldInfo::new("price", "Decimal"));

    registry.register(product);

    let found = registry.lookup("Product");
    assert!(found.is_some());
    assert_eq!(found.unwrap().fields().len(), 2);
}

#[test]
fn test_registry_type_hierarchy() {
    let mut registry = TypeRegistry::new();

    registry.register(TypeInfo::record("Entity"));
    registry.register(TypeInfo::record("User").with_parent("Entity"));
    registry.register(TypeInfo::record("Admin").with_parent("User"));
    registry.register(TypeInfo::record("Product").with_parent("Entity"));

    let entity_subtypes = registry.subtypes("Entity");
    assert_eq!(entity_subtypes.len(), 2); // User and Product

    let user_subtypes = registry.subtypes("User");
    assert_eq!(user_subtypes.len(), 1); // Admin
}

#[test]
fn test_registry_trait_implementors() {
    let mut registry = TypeRegistry::new();

    registry.register(
        TypeInfo::record("User")
            .implements("Serializable")
            .implements("Validatable"),
    );
    registry.register(TypeInfo::record("Product").implements("Serializable"));
    registry.register(TypeInfo::record("Order"));

    let serializable = registry.implementors("Serializable");
    assert_eq!(serializable.len(), 2);

    let validatable = registry.implementors("Validatable");
    assert_eq!(validatable.len(), 1);
}

#[test]
fn test_registry_iteration() {
    let mut registry = TypeRegistry::new();
    registry.register(TypeInfo::record("A"));
    registry.register(TypeInfo::record("B"));
    registry.register(TypeInfo::record("C"));

    let names: Vec<&str> = registry.type_names().collect();
    assert_eq!(names.len(), 3);
    assert!(names.contains(&"A"));
    assert!(names.contains(&"B"));
    assert!(names.contains(&"C"));
}

#[test]
fn test_registry_remove_and_clear() {
    let mut registry = TypeRegistry::new();
    registry.register(TypeInfo::record("Temp1"));
    registry.register(TypeInfo::record("Temp2"));

    assert_eq!(registry.len(), 2);

    let removed = registry.remove("Temp1");
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().name(), "Temp1");
    assert_eq!(registry.len(), 1);

    registry.clear();
    assert!(registry.is_empty());
}

// ============================================
// Integration Scenarios
// ============================================

#[test]
fn test_domain_model_registration() {
    let mut registry = TypeRegistry::with_primitives();

    // Define a domain model
    registry.register(
        TypeInfo::record("Address")
            .with_field(FieldInfo::new("street", "String"))
            .with_field(FieldInfo::new("city", "String"))
            .with_field(FieldInfo::new("zip", "String"))
            .with_doc("Physical address"),
    );

    registry.register(
        TypeInfo::record("Customer")
            .with_field(FieldInfo::new("id", "Int64"))
            .with_field(FieldInfo::new("name", "String"))
            .with_field(FieldInfo::new("billing_address", "Address"))
            .with_field(FieldInfo::new("shipping_address", "Address").optional())
            .implements("Entity")
            .implements("Auditable"),
    );

    registry.register(
        TypeInfo::record("Order")
            .with_field(FieldInfo::new("id", "Int64"))
            .with_field(FieldInfo::new("customer_id", "Int64"))
            .with_field(FieldInfo::new("total", "Decimal"))
            .with_field(FieldInfo::new("status", "OrderStatus"))
            .with_method(MethodInfo::new("place").returns("Result<(), Error>"))
            .with_method(MethodInfo::new("cancel").returns("Result<(), Error>"))
            .implements("Entity")
            .implements("Auditable"),
    );

    // Query the model
    let customer = registry.lookup("Customer").unwrap();
    assert!(customer.field("billing_address").is_some());
    assert!(customer.field("shipping_address").unwrap().is_optional());

    let auditables = registry.implementors("Auditable");
    assert_eq!(auditables.len(), 2);
}

#[test]
fn test_serialization_metadata() {
    // Simulate a reflection-based serializer checking type metadata
    let mut registry = TypeRegistry::new();

    registry.register(
        TypeInfo::record("Config")
            .with_field(FieldInfo::new("host", "String").with_default("\"localhost\""))
            .with_field(FieldInfo::new("port", "Int32").with_default("8080"))
            .with_field(FieldInfo::new("timeout_ms", "Int64").optional()),
    );

    let config = registry.lookup("Config").unwrap();

    // Simulate serializer behavior
    for field in config.fields() {
        if field.is_optional() {
            // Skip if not present in output
            assert_eq!(field.name(), "timeout_ms");
        } else if field.default().is_some() {
            // Use default if not provided
            assert!(field.name() == "host" || field.name() == "port");
        }
    }
}

#[test]
fn test_reflect_type_function() {
    use metadol::reflect::reflect_type;

    let mut registry = TypeRegistry::with_primitives();
    registry.register(TypeInfo::record("MyType"));

    // Known type
    let info = reflect_type(&registry, "MyType");
    assert_eq!(info.name(), "MyType");
    assert_eq!(info.kind(), TypeKind::Record);

    // Primitive type
    let int_info = reflect_type(&registry, "Int32");
    assert_eq!(int_info.kind(), TypeKind::Primitive);

    // Unknown type returns default
    let unknown = reflect_type(&registry, "NonExistent");
    assert_eq!(unknown.kind(), TypeKind::Unknown);
}
