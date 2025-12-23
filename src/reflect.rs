//! DOL 2.0 Reflection System
//!
//! This module provides runtime type introspection capabilities for DOL.
//! The reflection system allows programs to examine type metadata at runtime,
//! enabling metaprogramming patterns like serialization, validation, and
//! dynamic dispatch.
//!
//! # Architecture
//!
//! The reflection system is built around three core concepts:
//!
//! - **TypeInfo**: Describes a type's structure including fields and methods
//! - **FieldInfo**: Describes a single field in a record type
//! - **MethodInfo**: Describes a method signature
//! - **TypeRegistry**: Central registry for looking up type information
//!
//! # Example
//!
//! ```rust
//! use metadol::reflect::{TypeInfo, TypeKind, TypeRegistry, FieldInfo};
//!
//! // Create a registry
//! let mut registry = TypeRegistry::new();
//!
//! // Register a type
//! let user_type = TypeInfo::new("User", TypeKind::Record)
//!     .with_field(FieldInfo::new("name", "String"))
//!     .with_field(FieldInfo::new("age", "Int32"));
//!
//! registry.register(user_type);
//!
//! // Look up type info
//! if let Some(info) = registry.lookup("User") {
//!     assert_eq!(info.name(), "User");
//!     assert_eq!(info.fields().len(), 2);
//! }
//! ```

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The kind of a type in the reflection system.
///
/// Determines how the type can be introspected and what operations
/// are available on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeKind {
    /// Primitive types (Int32, Bool, String, etc.)
    Primitive,
    /// Record/struct types with named fields
    Record,
    /// Enum types with variants
    Enum,
    /// Function types
    Function,
    /// Generic type (with type parameters)
    Generic,
    /// Tuple type
    Tuple,
    /// Array/list type
    Array,
    /// Optional/nullable type
    Optional,
    /// Reference type
    Reference,
    /// Unknown/opaque type
    Unknown,
}

impl std::fmt::Display for TypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeKind::Primitive => write!(f, "primitive"),
            TypeKind::Record => write!(f, "record"),
            TypeKind::Enum => write!(f, "enum"),
            TypeKind::Function => write!(f, "function"),
            TypeKind::Generic => write!(f, "generic"),
            TypeKind::Tuple => write!(f, "tuple"),
            TypeKind::Array => write!(f, "array"),
            TypeKind::Optional => write!(f, "optional"),
            TypeKind::Reference => write!(f, "reference"),
            TypeKind::Unknown => write!(f, "unknown"),
        }
    }
}

/// Information about a type's field.
///
/// Represents a single field in a record type, including its name,
/// type, and optional metadata like documentation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FieldInfo {
    /// Field name
    name: String,
    /// Type name of the field
    type_name: String,
    /// Whether the field is optional
    optional: bool,
    /// Whether the field is mutable
    mutable: bool,
    /// Documentation for the field
    doc: Option<String>,
    /// Default value expression (as string)
    default: Option<String>,
}

impl FieldInfo {
    /// Creates a new field info with the given name and type.
    ///
    /// # Arguments
    ///
    /// * `name` - The field name
    /// * `type_name` - The type name of the field
    ///
    /// # Example
    ///
    /// ```rust
    /// use metadol::reflect::FieldInfo;
    ///
    /// let field = FieldInfo::new("name", "String");
    /// assert_eq!(field.name(), "name");
    /// assert_eq!(field.type_name(), "String");
    /// ```
    pub fn new(name: impl Into<String>, type_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            type_name: type_name.into(),
            optional: false,
            mutable: false,
            doc: None,
            default: None,
        }
    }

    /// Returns the field name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the field's type name.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Returns whether the field is optional.
    pub fn is_optional(&self) -> bool {
        self.optional
    }

    /// Returns whether the field is mutable.
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }

    /// Returns the field's documentation if present.
    pub fn doc(&self) -> Option<&str> {
        self.doc.as_deref()
    }

    /// Returns the field's default value expression if present.
    pub fn default(&self) -> Option<&str> {
        self.default.as_deref()
    }

    /// Marks this field as optional.
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// Marks this field as mutable.
    pub fn mutable(mut self) -> Self {
        self.mutable = true;
        self
    }

    /// Adds documentation to this field.
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }

    /// Adds a default value expression to this field.
    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }
}

/// Information about a method.
///
/// Represents a method signature including its name, parameters,
/// return type, and metadata.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MethodInfo {
    /// Method name
    name: String,
    /// Parameter types (name, type_name pairs)
    params: Vec<(String, String)>,
    /// Return type name
    return_type: String,
    /// Whether this is a static method
    is_static: bool,
    /// Whether this method is pure (no side effects)
    is_pure: bool,
    /// Documentation for the method
    doc: Option<String>,
}

impl MethodInfo {
    /// Creates a new method info with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The method name
    ///
    /// # Example
    ///
    /// ```rust
    /// use metadol::reflect::MethodInfo;
    ///
    /// let method = MethodInfo::new("calculate")
    ///     .with_param("x", "Int32")
    ///     .with_param("y", "Int32")
    ///     .returns("Int32");
    ///
    /// assert_eq!(method.name(), "calculate");
    /// assert_eq!(method.params().len(), 2);
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            params: Vec::new(),
            return_type: "Void".to_string(),
            is_static: false,
            is_pure: false,
            doc: None,
        }
    }

    /// Returns the method name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the method parameters.
    pub fn params(&self) -> &[(String, String)] {
        &self.params
    }

    /// Returns the return type name.
    pub fn return_type(&self) -> &str {
        &self.return_type
    }

    /// Returns whether this is a static method.
    pub fn is_static(&self) -> bool {
        self.is_static
    }

    /// Returns whether this method is pure.
    pub fn is_pure(&self) -> bool {
        self.is_pure
    }

    /// Returns the method's documentation if present.
    pub fn doc(&self) -> Option<&str> {
        self.doc.as_deref()
    }

    /// Adds a parameter to this method.
    pub fn with_param(mut self, name: impl Into<String>, type_name: impl Into<String>) -> Self {
        self.params.push((name.into(), type_name.into()));
        self
    }

    /// Sets the return type of this method.
    pub fn returns(mut self, type_name: impl Into<String>) -> Self {
        self.return_type = type_name.into();
        self
    }

    /// Marks this method as static.
    pub fn static_method(mut self) -> Self {
        self.is_static = true;
        self
    }

    /// Marks this method as pure.
    pub fn pure(mut self) -> Self {
        self.is_pure = true;
        self
    }

    /// Adds documentation to this method.
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }
}

/// Information about a type.
///
/// Represents complete metadata about a type including its kind,
/// fields, methods, and other properties.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeInfo {
    /// Type name
    name: String,
    /// What kind of type this is
    kind: TypeKind,
    /// Fields (for record types)
    fields: Vec<FieldInfo>,
    /// Methods
    methods: Vec<MethodInfo>,
    /// Type parameters (for generic types)
    type_params: Vec<String>,
    /// Parent/base type (for inheritance)
    parent: Option<String>,
    /// Implemented traits/interfaces
    traits: Vec<String>,
    /// Documentation
    doc: Option<String>,
    /// Whether the type is public
    is_public: bool,
}

impl TypeInfo {
    /// Creates a new type info with the given name and kind.
    ///
    /// # Arguments
    ///
    /// * `name` - The type name
    /// * `kind` - The kind of type
    ///
    /// # Example
    ///
    /// ```rust
    /// use metadol::reflect::{TypeInfo, TypeKind};
    ///
    /// let info = TypeInfo::new("User", TypeKind::Record);
    /// assert_eq!(info.name(), "User");
    /// assert_eq!(info.kind(), TypeKind::Record);
    /// ```
    pub fn new(name: impl Into<String>, kind: TypeKind) -> Self {
        Self {
            name: name.into(),
            kind,
            fields: Vec::new(),
            methods: Vec::new(),
            type_params: Vec::new(),
            parent: None,
            traits: Vec::new(),
            doc: None,
            is_public: true,
        }
    }

    /// Creates a primitive type info.
    ///
    /// # Example
    ///
    /// ```rust
    /// use metadol::reflect::{TypeInfo, TypeKind};
    ///
    /// let int_type = TypeInfo::primitive("Int32");
    /// assert_eq!(int_type.kind(), TypeKind::Primitive);
    /// ```
    pub fn primitive(name: impl Into<String>) -> Self {
        Self::new(name, TypeKind::Primitive)
    }

    /// Creates a record type info.
    ///
    /// # Example
    ///
    /// ```rust
    /// use metadol::reflect::{TypeInfo, TypeKind, FieldInfo};
    ///
    /// let user_type = TypeInfo::record("User")
    ///     .with_field(FieldInfo::new("name", "String"));
    /// assert_eq!(user_type.kind(), TypeKind::Record);
    /// ```
    pub fn record(name: impl Into<String>) -> Self {
        Self::new(name, TypeKind::Record)
    }

    /// Creates a function type info.
    pub fn function(name: impl Into<String>) -> Self {
        Self::new(name, TypeKind::Function)
    }

    /// Creates a generic type info.
    pub fn generic(name: impl Into<String>) -> Self {
        Self::new(name, TypeKind::Generic)
    }

    /// Returns the type name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type kind.
    pub fn kind(&self) -> TypeKind {
        self.kind
    }

    /// Returns the type's fields.
    pub fn fields(&self) -> &[FieldInfo] {
        &self.fields
    }

    /// Returns the type's methods.
    pub fn methods(&self) -> &[MethodInfo] {
        &self.methods
    }

    /// Returns the type parameters.
    pub fn type_params(&self) -> &[String] {
        &self.type_params
    }

    /// Returns the parent type if any.
    pub fn parent(&self) -> Option<&str> {
        self.parent.as_deref()
    }

    /// Returns implemented traits.
    pub fn traits(&self) -> &[String] {
        &self.traits
    }

    /// Returns the type's documentation if present.
    pub fn doc(&self) -> Option<&str> {
        self.doc.as_deref()
    }

    /// Returns whether the type is public.
    pub fn is_public(&self) -> bool {
        self.is_public
    }

    /// Looks up a field by name.
    pub fn field(&self, name: &str) -> Option<&FieldInfo> {
        self.fields.iter().find(|f| f.name() == name)
    }

    /// Looks up a method by name.
    pub fn method(&self, name: &str) -> Option<&MethodInfo> {
        self.methods.iter().find(|m| m.name() == name)
    }

    /// Adds a field to this type.
    pub fn with_field(mut self, field: FieldInfo) -> Self {
        self.fields.push(field);
        self
    }

    /// Adds a method to this type.
    pub fn with_method(mut self, method: MethodInfo) -> Self {
        self.methods.push(method);
        self
    }

    /// Adds a type parameter to this type.
    pub fn with_type_param(mut self, param: impl Into<String>) -> Self {
        self.type_params.push(param.into());
        self
    }

    /// Sets the parent type.
    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }

    /// Adds an implemented trait.
    pub fn implements(mut self, trait_name: impl Into<String>) -> Self {
        self.traits.push(trait_name.into());
        self
    }

    /// Adds documentation to this type.
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }

    /// Marks this type as private.
    pub fn private(mut self) -> Self {
        self.is_public = false;
        self
    }
}

/// Registry for type information.
///
/// The type registry is the central store for all type metadata in a program.
/// It allows looking up type information by name and supports type hierarchies.
#[derive(Debug, Clone, Default)]
pub struct TypeRegistry {
    /// Registered types by name
    types: HashMap<String, TypeInfo>,
}

impl TypeRegistry {
    /// Creates a new empty type registry.
    ///
    /// # Example
    ///
    /// ```rust
    /// use metadol::reflect::TypeRegistry;
    ///
    /// let registry = TypeRegistry::new();
    /// assert!(registry.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    /// Creates a new type registry with built-in primitive types.
    ///
    /// # Example
    ///
    /// ```rust
    /// use metadol::reflect::TypeRegistry;
    ///
    /// let registry = TypeRegistry::with_primitives();
    /// assert!(registry.lookup("Int32").is_some());
    /// assert!(registry.lookup("String").is_some());
    /// ```
    pub fn with_primitives() -> Self {
        let mut registry = Self::new();

        // Register primitive types
        registry.register(TypeInfo::primitive("Void"));
        registry.register(TypeInfo::primitive("Bool"));
        registry.register(TypeInfo::primitive("Int8"));
        registry.register(TypeInfo::primitive("Int16"));
        registry.register(TypeInfo::primitive("Int32"));
        registry.register(TypeInfo::primitive("Int64"));
        registry.register(TypeInfo::primitive("UInt8"));
        registry.register(TypeInfo::primitive("UInt16"));
        registry.register(TypeInfo::primitive("UInt32"));
        registry.register(TypeInfo::primitive("UInt64"));
        registry.register(TypeInfo::primitive("Float32"));
        registry.register(TypeInfo::primitive("Float64"));
        registry.register(TypeInfo::primitive("String"));

        registry
    }

    /// Returns whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    /// Returns the number of registered types.
    pub fn len(&self) -> usize {
        self.types.len()
    }

    /// Registers a type in the registry.
    ///
    /// # Arguments
    ///
    /// * `info` - The type information to register
    ///
    /// # Example
    ///
    /// ```rust
    /// use metadol::reflect::{TypeRegistry, TypeInfo, TypeKind};
    ///
    /// let mut registry = TypeRegistry::new();
    /// registry.register(TypeInfo::new("MyType", TypeKind::Record));
    /// assert!(registry.lookup("MyType").is_some());
    /// ```
    pub fn register(&mut self, info: TypeInfo) {
        self.types.insert(info.name().to_string(), info);
    }

    /// Looks up a type by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The type name to look up
    ///
    /// # Returns
    ///
    /// The type information if found, or `None` if not registered.
    pub fn lookup(&self, name: &str) -> Option<&TypeInfo> {
        self.types.get(name)
    }

    /// Returns all registered type names.
    pub fn type_names(&self) -> impl Iterator<Item = &str> {
        self.types.keys().map(|s| s.as_str())
    }

    /// Returns all registered types.
    pub fn types(&self) -> impl Iterator<Item = &TypeInfo> {
        self.types.values()
    }

    /// Checks if a type is registered.
    pub fn contains(&self, name: &str) -> bool {
        self.types.contains_key(name)
    }

    /// Removes a type from the registry.
    pub fn remove(&mut self, name: &str) -> Option<TypeInfo> {
        self.types.remove(name)
    }

    /// Clears all types from the registry.
    pub fn clear(&mut self) {
        self.types.clear();
    }

    /// Returns all types that implement a given trait.
    pub fn implementors(&self, trait_name: &str) -> Vec<&TypeInfo> {
        self.types
            .values()
            .filter(|t| t.traits().contains(&trait_name.to_string()))
            .collect()
    }

    /// Returns all subtypes of a given type.
    pub fn subtypes(&self, parent_name: &str) -> Vec<&TypeInfo> {
        self.types
            .values()
            .filter(|t| t.parent() == Some(parent_name))
            .collect()
    }
}

/// Reflects on a type expression and returns type information.
///
/// This function is the runtime entry point for the `reflect` operator.
/// It examines a type and returns metadata about its structure.
///
/// # Arguments
///
/// * `registry` - The type registry to look up type information
/// * `type_name` - The name of the type to reflect on
///
/// # Returns
///
/// The type information if the type is registered, or a default Unknown type info.
///
/// # Example
///
/// ```rust
/// use metadol::reflect::{reflect_type, TypeRegistry, TypeInfo, TypeKind};
///
/// let mut registry = TypeRegistry::with_primitives();
/// registry.register(TypeInfo::record("User"));
///
/// let info = reflect_type(&registry, "User");
/// assert_eq!(info.name(), "User");
/// ```
pub fn reflect_type<'a>(registry: &'a TypeRegistry, type_name: &str) -> &'a TypeInfo {
    static UNKNOWN: std::sync::LazyLock<TypeInfo> =
        std::sync::LazyLock::new(|| TypeInfo::new("Unknown", TypeKind::Unknown));

    registry.lookup(type_name).unwrap_or(&UNKNOWN)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_info_creation() {
        let field = FieldInfo::new("name", "String")
            .optional()
            .mutable()
            .with_doc("User's name")
            .with_default("\"unnamed\"");

        assert_eq!(field.name(), "name");
        assert_eq!(field.type_name(), "String");
        assert!(field.is_optional());
        assert!(field.is_mutable());
        assert_eq!(field.doc(), Some("User's name"));
        assert_eq!(field.default(), Some("\"unnamed\""));
    }

    #[test]
    fn test_method_info_creation() {
        let method = MethodInfo::new("calculate")
            .with_param("x", "Int32")
            .with_param("y", "Int32")
            .returns("Int32")
            .pure()
            .with_doc("Calculates sum");

        assert_eq!(method.name(), "calculate");
        assert_eq!(method.params().len(), 2);
        assert_eq!(method.return_type(), "Int32");
        assert!(method.is_pure());
        assert!(!method.is_static());
        assert_eq!(method.doc(), Some("Calculates sum"));
    }

    #[test]
    fn test_method_static() {
        let method = MethodInfo::new("create").returns("User").static_method();

        assert!(method.is_static());
    }

    #[test]
    fn test_type_info_record() {
        let info = TypeInfo::record("User")
            .with_field(FieldInfo::new("name", "String"))
            .with_field(FieldInfo::new("age", "Int32"))
            .with_method(MethodInfo::new("greet").returns("String"))
            .with_doc("A user entity");

        assert_eq!(info.name(), "User");
        assert_eq!(info.kind(), TypeKind::Record);
        assert_eq!(info.fields().len(), 2);
        assert_eq!(info.methods().len(), 1);
        assert!(info.is_public());
        assert_eq!(info.doc(), Some("A user entity"));
    }

    #[test]
    fn test_type_info_field_lookup() {
        let info = TypeInfo::record("Person")
            .with_field(FieldInfo::new("name", "String"))
            .with_field(FieldInfo::new("age", "Int32"));

        assert!(info.field("name").is_some());
        assert_eq!(info.field("name").unwrap().type_name(), "String");
        assert!(info.field("nonexistent").is_none());
    }

    #[test]
    fn test_type_info_method_lookup() {
        let info = TypeInfo::record("Calculator")
            .with_method(MethodInfo::new("add").returns("Int32"))
            .with_method(MethodInfo::new("subtract").returns("Int32"));

        assert!(info.method("add").is_some());
        assert!(info.method("multiply").is_none());
    }

    #[test]
    fn test_type_info_generic() {
        let info = TypeInfo::generic("List")
            .with_type_param("T")
            .with_method(MethodInfo::new("push").with_param("item", "T"));

        assert_eq!(info.kind(), TypeKind::Generic);
        assert_eq!(info.type_params(), &["T"]);
    }

    #[test]
    fn test_type_info_inheritance() {
        let info = TypeInfo::record("Admin")
            .with_parent("User")
            .implements("Authenticatable")
            .implements("Authorizable");

        assert_eq!(info.parent(), Some("User"));
        assert_eq!(info.traits().len(), 2);
        assert!(info.traits().contains(&"Authenticatable".to_string()));
    }

    #[test]
    fn test_type_registry_basic() {
        let mut registry = TypeRegistry::new();
        assert!(registry.is_empty());

        registry.register(TypeInfo::record("User"));
        assert!(!registry.is_empty());
        assert_eq!(registry.len(), 1);
        assert!(registry.contains("User"));
        assert!(registry.lookup("User").is_some());
    }

    #[test]
    fn test_type_registry_with_primitives() {
        let registry = TypeRegistry::with_primitives();

        assert!(registry.lookup("Int32").is_some());
        assert!(registry.lookup("String").is_some());
        assert!(registry.lookup("Bool").is_some());
        assert!(registry.lookup("Float64").is_some());
        assert_eq!(
            registry.lookup("Int32").unwrap().kind(),
            TypeKind::Primitive
        );
    }

    #[test]
    fn test_type_registry_implementors() {
        let mut registry = TypeRegistry::new();

        registry.register(TypeInfo::record("User").implements("Serializable"));
        registry.register(TypeInfo::record("Product").implements("Serializable"));
        registry.register(TypeInfo::record("Order"));

        let implementors = registry.implementors("Serializable");
        assert_eq!(implementors.len(), 2);
    }

    #[test]
    fn test_type_registry_subtypes() {
        let mut registry = TypeRegistry::new();

        registry.register(TypeInfo::record("Entity"));
        registry.register(TypeInfo::record("User").with_parent("Entity"));
        registry.register(TypeInfo::record("Product").with_parent("Entity"));
        registry.register(TypeInfo::record("Order"));

        let subtypes = registry.subtypes("Entity");
        assert_eq!(subtypes.len(), 2);
    }

    #[test]
    fn test_type_registry_remove() {
        let mut registry = TypeRegistry::new();
        registry.register(TypeInfo::record("User"));

        assert!(registry.contains("User"));
        let removed = registry.remove("User");
        assert!(removed.is_some());
        assert!(!registry.contains("User"));
    }

    #[test]
    fn test_type_registry_clear() {
        let mut registry = TypeRegistry::new();
        registry.register(TypeInfo::record("User"));
        registry.register(TypeInfo::record("Product"));

        assert_eq!(registry.len(), 2);
        registry.clear();
        assert!(registry.is_empty());
    }

    #[test]
    fn test_type_kind_display() {
        assert_eq!(format!("{}", TypeKind::Primitive), "primitive");
        assert_eq!(format!("{}", TypeKind::Record), "record");
        assert_eq!(format!("{}", TypeKind::Enum), "enum");
        assert_eq!(format!("{}", TypeKind::Function), "function");
    }

    #[test]
    fn test_reflect_type() {
        let mut registry = TypeRegistry::with_primitives();
        registry.register(TypeInfo::record("User"));

        let info = reflect_type(&registry, "User");
        assert_eq!(info.name(), "User");

        let unknown = reflect_type(&registry, "NonExistent");
        assert_eq!(unknown.kind(), TypeKind::Unknown);
    }

    #[test]
    fn test_type_info_private() {
        let info = TypeInfo::record("InternalType").private();
        assert!(!info.is_public());
    }
}
