//! Symbol table and interning for HIR.
//!
//! This module provides efficient string interning via [`Symbol`]
//! and scope management via [`SymbolTable`].

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

/// An interned string identifier.
///
/// Symbols are cheap to copy and compare (just a u32).
/// The actual string data is stored in a [`SymbolTable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Symbol(u32);

impl Symbol {
    /// Create a new symbol with the given id.
    ///
    /// This is an internal constructor; use [`SymbolTable::intern`] instead.
    #[allow(dead_code)]
    pub(crate) fn new(id: u32) -> Self {
        Symbol(id)
    }

    /// Get the raw id of this symbol.
    pub fn id(self) -> u32 {
        self.0
    }
}

/// Symbol table for string interning.
///
/// Provides O(1) symbol lookup and comparison.
#[derive(Debug, Default, Clone)]
pub struct SymbolTable {
    /// Map from string to symbol id
    strings_to_ids: HashMap<String, u32>,
    /// Map from symbol id to string
    ids_to_strings: Vec<String>,
}

impl SymbolTable {
    /// Create a new empty symbol table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Intern a string and return its symbol.
    ///
    /// If the string has already been interned, returns the existing symbol.
    pub fn intern(&mut self, s: &str) -> Symbol {
        if let Some(&id) = self.strings_to_ids.get(s) {
            return Symbol(id);
        }

        let id = self.ids_to_strings.len() as u32;
        self.strings_to_ids.insert(s.to_string(), id);
        self.ids_to_strings.push(s.to_string());
        Symbol(id)
    }

    /// Look up the string for a symbol.
    ///
    /// Returns `None` if the symbol is not in this table.
    pub fn resolve(&self, sym: Symbol) -> Option<&str> {
        self.ids_to_strings.get(sym.0 as usize).map(|s| s.as_str())
    }

    /// Get the number of interned symbols.
    pub fn len(&self) -> usize {
        self.ids_to_strings.len()
    }

    /// Check if the symbol table is empty.
    pub fn is_empty(&self) -> bool {
        self.ids_to_strings.is_empty()
    }
}

/// Global counter for generating unique HIR node IDs.
static NEXT_HIR_ID: AtomicU32 = AtomicU32::new(0);

/// Reset the HIR ID counter (for testing).
#[cfg(test)]
pub fn reset_hir_id_counter() {
    NEXT_HIR_ID.store(0, Ordering::SeqCst);
}

impl super::span::HirId {
    /// Generate a new unique HIR ID.
    pub fn new() -> Self {
        let id = NEXT_HIR_ID.fetch_add(1, Ordering::SeqCst);
        Self(id)
    }

    /// Get the raw id value.
    pub fn id(self) -> u32 {
        self.0
    }
}

impl Default for super::span::HirId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_interning() {
        let mut table = SymbolTable::new();

        let s1 = table.intern("foo");
        let s2 = table.intern("bar");
        let s3 = table.intern("foo"); // Re-intern

        assert_eq!(s1, s3);
        assert_ne!(s1, s2);

        assert_eq!(table.resolve(s1), Some("foo"));
        assert_eq!(table.resolve(s2), Some("bar"));
    }

    #[test]
    fn test_symbol_table_len() {
        let mut table = SymbolTable::new();
        assert!(table.is_empty());

        table.intern("a");
        table.intern("b");
        table.intern("a"); // Duplicate

        assert_eq!(table.len(), 2);
    }
}
