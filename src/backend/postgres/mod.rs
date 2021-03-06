pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod types;

use super::*;

/// Postgres query builder.
pub struct PostgresQueryBuilder;

impl Default for PostgresQueryBuilder {
    fn default() -> Self {
        Self
    }
}

impl GenericBuilder for PostgresQueryBuilder {
    type QueryBuilder = Self;
    type TableBuilder = Self;
    type IndexBuilder = Self;
    type ForeignKeyBuilder = Self;
}