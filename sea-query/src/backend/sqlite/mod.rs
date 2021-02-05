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

impl GenericBuilder for SqliteQueryBuilder {
    fn query_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(SqliteQueryBuilder)
    }

    fn table_builder(&self) -> Box<dyn TableBuilder> {
        Box::new(SqliteQueryBuilder)
    }

    fn index_builder(&self) -> Box<dyn IndexBuilder> {
        Box::new(SqliteQueryBuilder)
    }

    fn foreign_key_builder(&self) -> Box<dyn ForeignKeyBuilder> {
        Box::new(SqliteQueryBuilder)
    }
}