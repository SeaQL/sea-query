pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;

use super::*;

/// Sqlite query builder.
#[derive(Debug)]
pub struct SqliteQueryBuilder;

impl Default for SqliteQueryBuilder {
    fn default() -> Self {
        Self
    }
}

impl GenericBuilder for SqliteQueryBuilder {}

impl SchemaBuilder for SqliteQueryBuilder {}

impl QuotedBuilder for SqliteQueryBuilder {
    fn quote(&self) -> char {
        '"'
    }
}

impl EscapeBuilder for SqliteQueryBuilder {
    fn escape_string(&self, string: &str) -> String {
        string.replace('\'', "''")
    }

    fn unescape_string(&self, string: &str) -> String {
        string.replace("''", "'")
    }
}
