pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod types;

use super::*;

/// Postgres query builder.
#[derive(Debug)]
pub struct PostgresQueryBuilder;

impl Default for PostgresQueryBuilder {
    fn default() -> Self {
        Self
    }
}

impl GenericBuilder for PostgresQueryBuilder {}

impl SchemaBuilder for PostgresQueryBuilder {}

impl QuotedBuilder for PostgresQueryBuilder {
    fn quote(&self) -> char {
        '"'
    }
}
