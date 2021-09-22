pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod types;

use super::*;

/// Postgres query builder.
#[derive(Clone, Copy, Debug, Default)]
pub struct PostgresQueryBuilder;

impl GenericBuilder<PostgresQueryBuilder> for PostgresQueryBuilder {}

impl SchemaBuilder for PostgresQueryBuilder {}

impl QuotedBuilder for PostgresQueryBuilder {
    fn quote(&self) -> char {
        '"'
    }
}
