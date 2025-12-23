//! Rust code generation from Metal DOL declarations.
//!
//! Generates Rust structs, traits, and types from DOL declarations.
//!
//! # Type Mapping
//!
//! | DOL Type | Rust Type |
//! |----------|-----------|
//! | `Int8` | `i8` |
//! | `Int16` | `i16` |
//! | `Int32` | `i32` |
//! | `Int64` | `i64` |
//! | `UInt8` | `u8` |
//! | `UInt16` | `u16` |
//! | `UInt32` | `u32` |
//! | `UInt64` | `u64` |
//! | `Float32` | `f32` |
//! | `Float64` | `f64` |
//! | `String` | `String` |
//! | `Bool` | `bool` |
//! | `Option<T>` | `Option<T>` |
//! | `Result<T, E>` | `Result<T, E>` |
//! | `List<T>` | `Vec<T>` |
//! | `Map<K, V>` | `std::collections::HashMap<K, V>` |

use crate::ast::{Constraint, Declaration, Evolution, Gene, Statement, System, Trait, TypeExpr};
use crate::typechecker::Type;

use super::{to_pascal_case, to_snake_case, Codegen, CodegenOptions, TypeMapper, Visibility};

/// Rust code generator.
///
/// Transforms DOL declarations into Rust source code.
#[derive(Debug, Clone, Default)]
pub struct RustCodegen {
    options: CodegenOptions,
}

impl RustCodegen {
    /// Create a new Rust code generator with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new Rust code generator with custom options.
    pub fn with_options(options: CodegenOptions) -> Self {
        Self { options }
    }

    /// Generate Rust code from a declaration.
    pub fn generate(decl: &Declaration) -> String {
        Self::new().generate_declaration(decl)
    }

    /// Generate Rust code from multiple declarations.
    pub fn generate_all(decls: &[Declaration]) -> String {
        let generator = Self::new();
        decls
            .iter()
            .map(|d| generator.generate_declaration(d))
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// Generate code for a single declaration.
    fn generate_declaration(&self, decl: &Declaration) -> String {
        match decl {
            Declaration::Gene(gene) => self.generate_gene(gene),
            Declaration::Trait(trait_decl) => self.generate_trait(trait_decl),
            Declaration::Constraint(constraint) => self.generate_constraint(constraint),
            Declaration::System(system) => self.generate_system(system),
            Declaration::Evolution(evolution) => self.generate_evolution(evolution),
        }
    }

    /// Generate a Rust struct from a gene declaration.
    fn generate_gene(&self, gene: &Gene) -> String {
        let struct_name = to_pascal_case(&gene.name);
        let visibility = self.visibility_str();

        // Collect properties from "has" statements
        let fields = self.extract_fields(&gene.statements);

        let mut output = String::new();

        // Doc comment from exegesis (always include by default)
        output.push_str(&self.format_doc_comment(&gene.exegesis));

        // Derive macros
        let derives = self.derive_clause();
        if !derives.is_empty() {
            output.push_str(&format!("#[derive({})]\n", derives));
        }

        // Struct definition
        output.push_str(&format!("{visibility}struct {struct_name} {{\n"));

        for (field_name, field_type) in &fields {
            let rust_field = to_snake_case(field_name);
            output.push_str(&format!("    {visibility}{rust_field}: {field_type},\n"));
        }

        output.push_str("}\n");

        output
    }

    /// Generate a Rust trait from a trait declaration.
    fn generate_trait(&self, trait_decl: &Trait) -> String {
        let trait_name = to_pascal_case(&trait_decl.name);
        let visibility = self.visibility_str();

        // Collect supertraits from "uses" statements
        let supertraits = self.extract_supertraits(&trait_decl.statements);

        // Collect methods from "is" statements (state transitions become getters)
        let methods = self.extract_methods(&trait_decl.statements);

        let mut output = String::new();

        // Doc comment
        output.push_str(&self.format_doc_comment(&trait_decl.exegesis));

        // Trait definition with supertraits
        let supertrait_clause = if supertraits.is_empty() {
            String::new()
        } else {
            format!(": {}", supertraits.join(" + "))
        };

        output.push_str(&format!(
            "{visibility}trait {trait_name}{supertrait_clause} {{\n"
        ));

        for (method_name, return_type) in &methods {
            let rust_method = to_snake_case(method_name);
            output.push_str(&format!(
                "    /// Get the {} state.\n",
                method_name.replace('_', " ")
            ));
            output.push_str(&format!(
                "    fn {rust_method}(&self) -> {return_type};\n\n"
            ));
        }

        // Remove trailing newline if we have methods
        if !methods.is_empty() {
            output.pop();
        }

        output.push_str("}\n");

        output
    }

    /// Generate Rust assertions/invariants from a constraint declaration.
    fn generate_constraint(&self, constraint: &Constraint) -> String {
        let fn_name = to_snake_case(&constraint.name);
        let visibility = self.visibility_str();

        let mut output = String::new();

        // Doc comment
        output.push_str(&self.format_doc_comment(&constraint.exegesis));

        output.push_str(&format!(
            "/// Validates the {} constraint.\n",
            constraint.name
        ));
        output.push_str(&format!(
            "{visibility}fn validate_{fn_name}<T>(value: &T) -> bool {{\n"
        ));

        // Generate constraint checks from statements
        for stmt in &constraint.statements {
            match stmt {
                Statement::Matches {
                    subject, target, ..
                } => {
                    output.push_str(&format!("    // {subject} matches {target}\n"));
                }
                Statement::Never {
                    subject, action, ..
                } => {
                    output.push_str(&format!("    // {subject} never {action}\n"));
                }
                _ => {}
            }
        }

        output.push_str("    // Implement validation logic based on the constraint rules above\n");
        output.push_str("    true\n");
        output.push_str("}\n");

        output
    }

    /// Generate a Rust module from a system declaration.
    fn generate_system(&self, system: &System) -> String {
        let mod_name = to_snake_case(&system.name);
        let visibility = self.visibility_str();

        let mut output = String::new();

        // Doc comment
        output.push_str(&self.format_doc_comment(&system.exegesis));
        output.push_str(&format!("/// System version: {}\n", system.version));

        // Module with requirements as comments
        output.push_str(&format!("{visibility}mod {mod_name} {{\n"));

        if !system.requirements.is_empty() {
            output.push_str("    //! # Requirements\n    //!\n");
            for req in &system.requirements {
                output.push_str(&format!(
                    "    //! - `{}` {} {}\n",
                    req.name, req.constraint, req.version
                ));
            }
            output.push('\n');
        }

        output.push_str("    // Add system components and implementation here\n");
        output.push_str("}\n");

        output
    }

    /// Generate documentation for an evolution declaration.
    fn generate_evolution(&self, evolution: &Evolution) -> String {
        let mut output = String::new();

        // Evolution is primarily documentation
        output.push_str(&format!(
            "// Evolution: {} @ {} (from {})\n",
            evolution.name, evolution.version, evolution.parent_version
        ));

        if let Some(rationale) = &evolution.rationale {
            output.push_str(&format!("// Rationale: {}\n", rationale));
        }

        output.push_str("//\n");
        output.push_str(&self.format_doc_comment(&evolution.exegesis));

        if !evolution.additions.is_empty() {
            output.push_str("// Additions:\n");
            for stmt in &evolution.additions {
                output.push_str(&format!("//   - {:?}\n", stmt));
            }
        }

        if !evolution.deprecations.is_empty() {
            output.push_str("// Deprecations:\n");
            for stmt in &evolution.deprecations {
                output.push_str(&format!("//   - {:?}\n", stmt));
            }
        }

        if !evolution.removals.is_empty() {
            output.push_str("// Removals:\n");
            for item in &evolution.removals {
                output.push_str(&format!("//   - {}\n", item));
            }
        }

        output
    }

    // === Helper Methods ===

    /// Extract fields from statements (for structs).
    fn extract_fields(&self, statements: &[Statement]) -> Vec<(String, String)> {
        statements
            .iter()
            .filter_map(|stmt| {
                if let Statement::Has { property, .. } = stmt {
                    // Default to String type for properties without type annotations
                    Some((property.clone(), "String".to_string()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Extract supertraits from uses statements.
    fn extract_supertraits(&self, statements: &[Statement]) -> Vec<String> {
        statements
            .iter()
            .filter_map(|stmt| {
                if let Statement::Uses { reference, .. } = stmt {
                    Some(to_pascal_case(reference))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Extract methods from is statements.
    fn extract_methods(&self, statements: &[Statement]) -> Vec<(String, String)> {
        statements
            .iter()
            .filter_map(|stmt| {
                if let Statement::Is { state, .. } = stmt {
                    // State becomes a getter method returning bool or state type
                    Some((format!("is_{}", to_snake_case(state)), "bool".to_string()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get visibility string.
    fn visibility_str(&self) -> &'static str {
        match self.options.visibility {
            Visibility::Private => "",
            Visibility::Public => "pub ",
            Visibility::Crate => "pub(crate) ",
        }
    }

    /// Format exegesis as a doc comment.
    fn format_doc_comment(&self, exegesis: &str) -> String {
        let trimmed = exegesis.trim();
        if trimmed.is_empty() {
            return String::new();
        }

        trimmed
            .lines()
            .map(|line| format!("/// {}\n", line.trim()))
            .collect()
    }

    /// Generate derive clause.
    fn derive_clause(&self) -> String {
        let mut derives = vec!["Debug", "Clone"];

        if !self.options.derive_macros.is_empty() {
            derives.extend(self.options.derive_macros.iter().map(|s| s.as_str()));
        }

        derives.join(", ")
    }
}

impl Codegen for RustCodegen {
    fn generate(decl: &Declaration) -> String {
        RustCodegen::generate(decl)
    }
}

impl TypeMapper for RustCodegen {
    fn map_type(ty: &Type) -> String {
        match ty {
            Type::Void => "()".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Int8 => "i8".to_string(),
            Type::Int16 => "i16".to_string(),
            Type::Int32 => "i32".to_string(),
            Type::Int64 => "i64".to_string(),
            Type::UInt8 => "u8".to_string(),
            Type::UInt16 => "u16".to_string(),
            Type::UInt32 => "u32".to_string(),
            Type::UInt64 => "u64".to_string(),
            Type::Float32 => "f32".to_string(),
            Type::Float64 => "f64".to_string(),
            Type::String => "String".to_string(),
            Type::Function {
                params,
                return_type,
            } => {
                let param_types: Vec<_> = params.iter().map(Self::map_type).collect();
                let ret = Self::map_type(return_type);
                format!("fn({}) -> {}", param_types.join(", "), ret)
            }
            Type::Tuple(types) => {
                let mapped: Vec<_> = types.iter().map(Self::map_type).collect();
                format!("({})", mapped.join(", "))
            }
            Type::Generic { name, args } => {
                let mapped_args: Vec<_> = args.iter().map(Self::map_type).collect();
                // Map DOL generic types to Rust equivalents
                let rust_name = match name.as_str() {
                    "List" => "Vec",
                    "Map" => "std::collections::HashMap",
                    "Option" => "Option",
                    "Result" => "Result",
                    _ => name.as_str(),
                };
                if args.is_empty() {
                    to_pascal_case(name)
                } else {
                    format!("{}<{}>", rust_name, mapped_args.join(", "))
                }
            }
            Type::Var(id) => format!("T{}", id),
            Type::Any => "Box<dyn std::any::Any>".to_string(),
            Type::Unknown => "/* unknown */".to_string(),
            Type::Error => "/* error */".to_string(),
        }
    }

    fn map_type_expr(ty: &TypeExpr) -> String {
        match ty {
            TypeExpr::Named(name) => match name.as_str() {
                "Int8" => "i8".to_string(),
                "Int16" => "i16".to_string(),
                "Int32" => "i32".to_string(),
                "Int64" => "i64".to_string(),
                "UInt8" => "u8".to_string(),
                "UInt16" => "u16".to_string(),
                "UInt32" => "u32".to_string(),
                "UInt64" => "u64".to_string(),
                "Float32" => "f32".to_string(),
                "Float64" => "f64".to_string(),
                "String" => "String".to_string(),
                "Bool" => "bool".to_string(),
                "Void" => "()".to_string(),
                _ => to_pascal_case(name),
            },
            TypeExpr::Generic { name, args } => {
                let mapped_args: Vec<_> = args.iter().map(Self::map_type_expr).collect();
                let rust_name = match name.as_str() {
                    "List" => "Vec",
                    "Map" => "std::collections::HashMap",
                    "Option" => "Option",
                    "Result" => "Result",
                    _ => name,
                };
                format!("{}<{}>", rust_name, mapped_args.join(", "))
            }
            TypeExpr::Function {
                params,
                return_type,
            } => {
                let param_types: Vec<_> = params.iter().map(Self::map_type_expr).collect();
                let ret = Self::map_type_expr(return_type);
                format!("fn({}) -> {}", param_types.join(", "), ret)
            }
            TypeExpr::Tuple(types) => {
                let mapped: Vec<_> = types.iter().map(Self::map_type_expr).collect();
                format!("({})", mapped.join(", "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    #[test]
    fn test_generate_gene_struct() {
        let gene = Gene {
            name: "container.exists".to_string(),
            statements: vec![
                Statement::Has {
                    subject: "container".to_string(),
                    property: "id".to_string(),
                    span: Span::default(),
                },
                Statement::Has {
                    subject: "container".to_string(),
                    property: "image".to_string(),
                    span: Span::default(),
                },
            ],
            exegesis: "A container is the fundamental unit.".to_string(),
            span: Span::default(),
        };

        let code = RustCodegen::generate(&Declaration::Gene(gene));

        assert!(code.contains("pub struct ContainerExists"));
        assert!(code.contains("pub id: String"));
        assert!(code.contains("pub image: String"));
        assert!(code.contains("/// A container is the fundamental unit."));
    }

    #[test]
    fn test_generate_trait() {
        let trait_decl = Trait {
            name: "container.lifecycle".to_string(),
            statements: vec![
                Statement::Uses {
                    reference: "container.exists".to_string(),
                    span: Span::default(),
                },
                Statement::Is {
                    subject: "container".to_string(),
                    state: "created".to_string(),
                    span: Span::default(),
                },
                Statement::Is {
                    subject: "container".to_string(),
                    state: "started".to_string(),
                    span: Span::default(),
                },
            ],
            exegesis: "Container lifecycle management.".to_string(),
            span: Span::default(),
        };

        let code = RustCodegen::generate(&Declaration::Trait(trait_decl));

        assert!(code.contains("pub trait ContainerLifecycle: ContainerExists"));
        assert!(code.contains("fn is_created(&self) -> bool"));
        assert!(code.contains("fn is_started(&self) -> bool"));
    }

    #[test]
    fn test_map_type() {
        assert_eq!(RustCodegen::map_type(&Type::Int32), "i32");
        assert_eq!(RustCodegen::map_type(&Type::String), "String");
        assert_eq!(RustCodegen::map_type(&Type::Bool), "bool");
        assert_eq!(
            RustCodegen::map_type(&Type::Generic {
                name: "Option".to_string(),
                args: vec![Type::String]
            }),
            "Option<String>"
        );
        assert_eq!(
            RustCodegen::map_type(&Type::Generic {
                name: "List".to_string(),
                args: vec![Type::Int32]
            }),
            "Vec<i32>"
        );
        assert_eq!(
            RustCodegen::map_type(&Type::Generic {
                name: "Map".to_string(),
                args: vec![Type::String, Type::Int64]
            }),
            "std::collections::HashMap<String, i64>"
        );
    }

    #[test]
    fn test_map_type_expr() {
        assert_eq!(
            RustCodegen::map_type_expr(&TypeExpr::Named("Int32".to_string())),
            "i32"
        );
        assert_eq!(
            RustCodegen::map_type_expr(&TypeExpr::Generic {
                name: "List".to_string(),
                args: vec![TypeExpr::Named("String".to_string())]
            }),
            "Vec<String>"
        );
        assert_eq!(
            RustCodegen::map_type_expr(&TypeExpr::Generic {
                name: "Map".to_string(),
                args: vec![
                    TypeExpr::Named("String".to_string()),
                    TypeExpr::Named("Int32".to_string())
                ]
            }),
            "std::collections::HashMap<String, i32>"
        );
    }

    #[test]
    fn test_generate_constraint() {
        let constraint = Constraint {
            name: "container.integrity".to_string(),
            statements: vec![
                Statement::Matches {
                    subject: "state".to_string(),
                    target: "declared_state".to_string(),
                    span: Span::default(),
                },
                Statement::Never {
                    subject: "identity".to_string(),
                    action: "changes".to_string(),
                    span: Span::default(),
                },
            ],
            exegesis: "Container integrity constraints.".to_string(),
            span: Span::default(),
        };

        let code = RustCodegen::generate(&Declaration::Constraint(constraint));

        assert!(code.contains("fn validate_container_integrity"));
        assert!(code.contains("// state matches declared_state"));
        assert!(code.contains("// identity never changes"));
    }

    #[test]
    fn test_generate_system() {
        let system = System {
            name: "univrs.orchestrator".to_string(),
            version: "0.1.0".to_string(),
            requirements: vec![crate::ast::Requirement {
                name: "container.lifecycle".to_string(),
                constraint: ">=".to_string(),
                version: "0.0.2".to_string(),
                span: Span::default(),
            }],
            statements: vec![],
            exegesis: "The Univrs orchestrator.".to_string(),
            span: Span::default(),
        };

        let code = RustCodegen::generate(&Declaration::System(system));

        assert!(code.contains("pub mod univrs_orchestrator"));
        assert!(code.contains("/// System version: 0.1.0"));
        assert!(code.contains("`container.lifecycle` >= 0.0.2"));
    }
}
