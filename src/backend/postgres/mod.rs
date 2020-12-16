pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod index;
pub(crate) mod foreign_key;

use super::*;

/// Postgres query builder.
pub struct PostgresQueryBuilder;

impl Default for PostgresQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PostgresQueryBuilder {
    pub fn new() -> Self {
        Self
    }
}