pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod index;
pub(crate) mod foreign_key;

use super::*;

/// Mysql query builder.
pub struct MysqlQueryBuilder;

impl Default for MysqlQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MysqlQueryBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl GenericBuilder for MysqlQueryBuilder {
    fn query_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(MysqlQueryBuilder)
    }

    fn table_builder(&self) -> Box<dyn TableBuilder> {
        Box::new(MysqlQueryBuilder)
    }

    fn index_builder(&self) -> Box<dyn IndexBuilder> {
        Box::new(MysqlQueryBuilder)
    }

    fn foreign_key_builder(&self) -> Box<dyn ForeignKeyBuilder> {
        Box::new(MysqlQueryBuilder)
    }
}