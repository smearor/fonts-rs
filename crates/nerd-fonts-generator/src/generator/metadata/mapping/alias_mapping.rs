//! Alias mapping type: alias name to canonical icon name.

use std::collections::HashMap;

use super::alias::RawAlias;

/// Maps alternative names to canonical icon names for keyword lookup.
#[derive(Debug, Clone, Default)]
pub struct AliasMapping(HashMap<RawAlias, RawAlias>);

impl AliasMapping {
    /// Creates an empty alias mapping.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns the canonical icon name for the given alias, if any.
    pub fn get(&self, alias: &str) -> Option<&str> {
        self.0.get(alias).map(RawAlias::as_str)
    }

    /// Inserts an alias mapping.
    pub fn insert(&mut self, alias: RawAlias, canonical_name: RawAlias) {
        self.0.insert(alias, canonical_name);
    }

    /// Returns the number of aliases.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no aliases.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over all (alias, canonical) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&RawAlias, &RawAlias)> {
        self.0.iter()
    }
}
