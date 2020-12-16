pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod index;
pub(crate) mod foreign_key;

use super::*;

/// Sqlite query builder.
pub struct SqliteQueryBuilder;

impl Default for SqliteQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SqliteQueryBuilder {
    pub fn new() -> Self {
        Self
    }
}