pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod view;

use super::*;

/// Sqlite query builder.
#[derive(Default, Debug)]
pub struct SqliteQueryBuilder;

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

impl TableRefBuilder for SqliteQueryBuilder {}
