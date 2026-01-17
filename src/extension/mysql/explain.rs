use crate::{DynIden, IntoTableRef, TableRef};

/// MySQL-only EXPLAIN table/column or wildcard target.
#[derive(Debug, Clone, PartialEq)]
pub enum ExplainTable {
    Table(TableRef),
    WithColumn(TableRef, DynIden),
    WithWildcard(TableRef, &'static str),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExplainTableTarget {
    Column(DynIden),
    Wildcard(&'static str),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MySqlExplainSchemaSpec {
    Schema(DynIden),
    Database(DynIden),
}

impl<T> From<T> for ExplainTable
where
    T: IntoTableRef,
{
    fn from(value: T) -> Self {
        Self::Table(value.into_table_ref())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct MySqlExplainOptions {
    pub(crate) into_variable: Option<String>,
    pub(crate) schema_spec: Option<MySqlExplainSchemaSpec>,
    pub(crate) table: Option<TableRef>,
    pub(crate) target: Option<ExplainTableTarget>,
    pub(crate) for_connection: Option<u64>,
}
