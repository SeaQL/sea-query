pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod index;
pub(crate) mod foreign_key;

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