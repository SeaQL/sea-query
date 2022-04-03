pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod view;

use super::*;

/// Mysql query builder.
#[derive(Default, Debug)]
pub struct MysqlQueryBuilder;

pub type MySqlQueryBuilder = MysqlQueryBuilder;

impl GenericBuilder for MysqlQueryBuilder {}

impl SchemaBuilder for MysqlQueryBuilder {}

impl QuotedBuilder for MysqlQueryBuilder {
    fn quote(&self) -> char {
        '`'
    }
}

impl EscapeBuilder for MysqlQueryBuilder {}

impl TableRefBuilder for MysqlQueryBuilder {}
