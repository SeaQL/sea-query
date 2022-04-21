pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;

use super::*;

/// Mysql query builder.
#[derive(Debug)]
pub struct MysqlQueryBuilder;

pub type MySqlQueryBuilder = MysqlQueryBuilder;

impl Default for MysqlQueryBuilder {
    fn default() -> Self {
        Self
    }
}

impl GenericBuilder for MysqlQueryBuilder {}

impl SchemaBuilder for MysqlQueryBuilder {}

impl QuotedBuilder for MysqlQueryBuilder {
    fn quote(&self) -> char {
        '`'
    }
}

impl EscapeBuilder for MysqlQueryBuilder {}
