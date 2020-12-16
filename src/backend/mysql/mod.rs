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