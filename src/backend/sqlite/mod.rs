pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;

use super::*;

/// Sqlite query builder.
#[derive(Clone, Copy, Debug, Default)]
pub struct SqliteQueryBuilder;

impl GenericBuilder<SqliteQueryBuilder> for SqliteQueryBuilder {}

impl SchemaBuilder for SqliteQueryBuilder {}

impl QuotedBuilder for SqliteQueryBuilder {
    fn quote(&self) -> char {
        '`'
    }
}
