pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;

use super::*;

/// Mysql query builder.
#[derive(Default, Debug)]
pub struct MysqlQueryBuilder;

const QUOTE: Quote = Quote(b'`', b'`');

pub type MySqlQueryBuilder = MysqlQueryBuilder;

impl GenericBuilder for MysqlQueryBuilder {}

impl SchemaBuilder for MysqlQueryBuilder {}

impl QuotedBuilder for MysqlQueryBuilder {
    fn quote(&self) -> Quote {
        QUOTE
    }
}

impl EscapeBuilder for MysqlQueryBuilder {}

impl TableRefBuilder for MysqlQueryBuilder {}
