//! Pretty printing for HIR nodes.
//!
//! This module provides human-readable formatting for HIR nodes,
//! useful for debugging and error messages.

use super::symbol::SymbolTable;
use super::types::*;
use std::fmt::Write;

/// Pretty printer for HIR nodes.
pub struct HirPrinter<'a> {
    /// Symbol table for resolving symbols to strings
    symbols: &'a SymbolTable,
    /// Current indentation level
    indent: usize,
    /// Output buffer
    output: String,
}

impl<'a> HirPrinter<'a> {
    /// Create a new HIR printer.
    pub fn new(symbols: &'a SymbolTable) -> Self {
        Self {
            symbols,
            indent: 0,
            output: String::new(),
        }
    }

    /// Get the output string.
    pub fn finish(self) -> String {
        self.output
    }

    /// Print a module.
    pub fn print_module(&mut self, module: &HirModule) {
        let name = self.resolve(module.name);
        writeln!(self.output, "module {} {{", name).unwrap();
        self.indent += 1;

        for decl in &module.decls {
            self.print_decl(decl);
        }

        self.indent -= 1;
        writeln!(self.output, "}}").unwrap();
    }

    /// Print a declaration.
    pub fn print_decl(&mut self, decl: &HirDecl) {
        match decl {
            HirDecl::Type(ty) => self.print_type_decl(ty),
            HirDecl::Trait(tr) => self.print_trait_decl(tr),
            HirDecl::Function(func) => self.print_function_decl(func),
            HirDecl::Module(module) => self.print_module_decl(module),
        }
    }

    /// Print a type declaration.
    pub fn print_type_decl(&mut self, decl: &HirTypeDecl) {
        self.write_indent();
        let name = self.resolve(decl.name);
        write!(self.output, "type {}", name).unwrap();
        self.print_type_params(&decl.type_params);
        writeln!(self.output, " {{ ... }}").unwrap();
    }

    /// Print a trait declaration.
    pub fn print_trait_decl(&mut self, decl: &HirTraitDecl) {
        self.write_indent();
        let name = self.resolve(decl.name);
        write!(self.output, "trait {}", name).unwrap();
        self.print_type_params(&decl.type_params);
        writeln!(self.output, " {{ ... }}").unwrap();
    }

    /// Print a function declaration.
    pub fn print_function_decl(&mut self, decl: &HirFunctionDecl) {
        self.write_indent();
        let name = self.resolve(decl.name);
        write!(self.output, "fun {}", name).unwrap();
        self.print_type_params(&decl.type_params);
        write!(self.output, "(").unwrap();
        for (i, param) in decl.params.iter().enumerate() {
            if i > 0 {
                write!(self.output, ", ").unwrap();
            }
            self.print_param(param);
        }
        write!(self.output, ") -> ").unwrap();
        self.print_type(&decl.return_type);
        if decl.body.is_some() {
            writeln!(self.output, " {{ ... }}").unwrap();
        } else {
            writeln!(self.output).unwrap();
        }
    }

    /// Print a module declaration.
    pub fn print_module_decl(&mut self, decl: &HirModuleDecl) {
        self.write_indent();
        let name = self.resolve(decl.name);
        writeln!(self.output, "module {} {{", name).unwrap();
        self.indent += 1;

        for d in &decl.decls {
            self.print_decl(d);
        }

        self.indent -= 1;
        self.write_indent();
        writeln!(self.output, "}}").unwrap();
    }

    /// Print type parameters.
    fn print_type_params(&mut self, params: &[HirTypeParam]) {
        if params.is_empty() {
            return;
        }
        write!(self.output, "[").unwrap();
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                write!(self.output, ", ").unwrap();
            }
            let name = self.resolve(param.name);
            write!(self.output, "{}", name).unwrap();
        }
        write!(self.output, "]").unwrap();
    }

    /// Print a parameter.
    fn print_param(&mut self, param: &HirParam) {
        self.print_pat(&param.pat);
        write!(self.output, ": ").unwrap();
        self.print_type(&param.ty);
    }

    /// Print a pattern.
    fn print_pat(&mut self, pat: &HirPat) {
        match pat {
            HirPat::Wildcard => write!(self.output, "_").unwrap(),
            HirPat::Var(sym) => {
                let name = self.resolve(*sym);
                write!(self.output, "{}", name).unwrap();
            }
            HirPat::Literal(lit) => self.print_literal(lit),
            HirPat::Constructor(ctor) => {
                let name = self.resolve(ctor.name);
                write!(self.output, "{}", name).unwrap();
                if !ctor.fields.is_empty() {
                    write!(self.output, "(").unwrap();
                    for (i, field) in ctor.fields.iter().enumerate() {
                        if i > 0 {
                            write!(self.output, ", ").unwrap();
                        }
                        self.print_pat(field);
                    }
                    write!(self.output, ")").unwrap();
                }
            }
            HirPat::Tuple(pats) => {
                write!(self.output, "(").unwrap();
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(self.output, ", ").unwrap();
                    }
                    self.print_pat(p);
                }
                write!(self.output, ")").unwrap();
            }
            HirPat::Or(pats) => {
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(self.output, " | ").unwrap();
                    }
                    self.print_pat(p);
                }
            }
        }
    }

    /// Print a type.
    fn print_type(&mut self, ty: &HirType) {
        match ty {
            HirType::Named(named) => {
                let name = self.resolve(named.name);
                write!(self.output, "{}", name).unwrap();
                if !named.args.is_empty() {
                    write!(self.output, "[").unwrap();
                    for (i, arg) in named.args.iter().enumerate() {
                        if i > 0 {
                            write!(self.output, ", ").unwrap();
                        }
                        self.print_type(arg);
                    }
                    write!(self.output, "]").unwrap();
                }
            }
            HirType::Tuple(types) => {
                write!(self.output, "(").unwrap();
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        write!(self.output, ", ").unwrap();
                    }
                    self.print_type(t);
                }
                write!(self.output, ")").unwrap();
            }
            HirType::Array(arr) => {
                write!(self.output, "[").unwrap();
                self.print_type(&arr.elem);
                if let Some(size) = arr.size {
                    write!(self.output, "; {}", size).unwrap();
                }
                write!(self.output, "]").unwrap();
            }
            HirType::Function(func) => {
                write!(self.output, "(").unwrap();
                for (i, p) in func.params.iter().enumerate() {
                    if i > 0 {
                        write!(self.output, ", ").unwrap();
                    }
                    self.print_type(p);
                }
                write!(self.output, ") -> ").unwrap();
                self.print_type(&func.ret);
            }
            HirType::Ref(r) => {
                if r.mutable {
                    write!(self.output, "&mut ").unwrap();
                } else {
                    write!(self.output, "&").unwrap();
                }
                self.print_type(&r.ty);
            }
            HirType::Optional(inner) => {
                self.print_type(inner);
                write!(self.output, "?").unwrap();
            }
            HirType::Var(id) => write!(self.output, "?{}", id).unwrap(),
            HirType::Error => write!(self.output, "<error>").unwrap(),
        }
    }

    /// Print a literal.
    fn print_literal(&mut self, lit: &HirLiteral) {
        match lit {
            HirLiteral::Bool(b) => write!(self.output, "{}", b).unwrap(),
            HirLiteral::Int(n) => write!(self.output, "{}", n).unwrap(),
            HirLiteral::Float(f) => write!(self.output, "{}", f).unwrap(),
            HirLiteral::String(s) => write!(self.output, "\"{}\"", s).unwrap(),
            HirLiteral::Unit => write!(self.output, "()").unwrap(),
        }
    }

    /// Write indentation.
    fn write_indent(&mut self) {
        for _ in 0..self.indent {
            write!(self.output, "  ").unwrap();
        }
    }

    /// Resolve a symbol to a string.
    fn resolve(&self, sym: super::symbol::Symbol) -> String {
        self.symbols.resolve(sym).unwrap_or("<unknown>").to_string()
    }
}

/// Print a module to a string.
pub fn print_module(module: &HirModule, symbols: &SymbolTable) -> String {
    let mut printer = HirPrinter::new(symbols);
    printer.print_module(module);
    printer.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printer_basic() {
        let mut symbols = SymbolTable::new();
        let name = symbols.intern("test_module");
        let module = HirModule::new(name);

        let output = print_module(&module, &symbols);
        assert!(output.contains("module test_module"));
    }
}
