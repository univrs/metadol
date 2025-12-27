//! Span mapping for HIR nodes.
//!
//! This module provides source location tracking for HIR nodes.
//! Unlike the AST which embeds spans directly, HIR uses a side table
//! mapping [`HirId`] to [`Span`] for cleaner node definitions.

use std::collections::HashMap;

/// Unique identifier for HIR nodes.
///
/// Every HIR node has a unique HirId that can be used to look up
/// additional information like source spans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirId(pub(crate) u32);

/// Source span (byte offsets).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Start byte offset (inclusive)
    pub start: u32,
    /// End byte offset (exclusive)
    pub end: u32,
}

impl Span {
    /// Create a new span.
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    /// Create an empty/dummy span.
    pub fn dummy() -> Self {
        Self { start: 0, end: 0 }
    }

    /// Check if this is a dummy span.
    pub fn is_dummy(&self) -> bool {
        self.start == 0 && self.end == 0
    }

    /// Get the length of this span.
    pub fn len(&self) -> u32 {
        self.end - self.start
    }

    /// Check if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Merge two spans to create a span covering both.
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

/// Map from HIR node IDs to source spans.
///
/// This is a side table that stores source location information
/// separately from the HIR nodes themselves.
#[derive(Debug, Default)]
pub struct SpanMap {
    spans: HashMap<HirId, Span>,
}

impl SpanMap {
    /// Create a new empty span map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a span for an HIR node.
    pub fn insert(&mut self, id: HirId, span: Span) {
        self.spans.insert(id, span);
    }

    /// Look up the span for an HIR node.
    pub fn get(&self, id: HirId) -> Option<Span> {
        self.spans.get(&id).copied()
    }

    /// Get the span for an HIR node, or a dummy span if not found.
    pub fn get_or_dummy(&self, id: HirId) -> Span {
        self.get(id).unwrap_or_else(Span::dummy)
    }

    /// Get the number of recorded spans.
    pub fn len(&self) -> usize {
        self.spans.len()
    }

    /// Check if the span map is empty.
    pub fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_basic() {
        let span = Span::new(10, 20);
        assert_eq!(span.len(), 10);
        assert!(!span.is_dummy());
        assert!(!span.is_empty());
    }

    #[test]
    fn test_span_dummy() {
        let span = Span::dummy();
        assert!(span.is_dummy());
        assert!(span.is_empty());
    }

    #[test]
    fn test_span_merge() {
        let s1 = Span::new(10, 20);
        let s2 = Span::new(15, 30);
        let merged = s1.merge(s2);
        assert_eq!(merged.start, 10);
        assert_eq!(merged.end, 30);
    }

    #[test]
    fn test_span_map() {
        let mut map = SpanMap::new();
        let id = HirId(42);
        let span = Span::new(100, 200);

        map.insert(id, span);
        assert_eq!(map.get(id), Some(span));
        assert_eq!(map.len(), 1);

        let missing = HirId(999);
        assert_eq!(map.get(missing), None);
        assert!(map.get_or_dummy(missing).is_dummy());
    }
}
