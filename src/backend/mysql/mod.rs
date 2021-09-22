pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;

use super::*;

/// Mysql query builder.
#[derive(Clone, Copy, Debug, Default)]
pub struct MysqlQueryBuilder;

pub type MySqlQueryBuilder = MysqlQueryBuilder;

impl GenericBuilder<MysqlQueryBuilder> for MysqlQueryBuilder {}

impl SchemaBuilder for MysqlQueryBuilder {}

impl QuotedBuilder for MysqlQueryBuilder {
    fn quote(&self) -> char {
        '`'
    }
}
