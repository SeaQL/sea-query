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
        Self::new()
    }
}

impl PostgresQueryBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl GenericBuilder for PostgresQueryBuilder {
    fn query_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(PostgresQueryBuilder)
    }

    fn table_builder(&self) -> Box<dyn TableBuilder> {
        Box::new(PostgresQueryBuilder)
    }

    fn index_builder(&self) -> Box<dyn IndexBuilder> {
        Box::new(PostgresQueryBuilder)
    }

    fn foreign_key_builder(&self) -> Box<dyn ForeignKeyBuilder> {
        Box::new(PostgresQueryBuilder)
    }
}