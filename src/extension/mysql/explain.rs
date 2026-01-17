use crate::{
    DynIden, ExplainFormat, ExplainableStatement, IntoIden, IntoTableRef, QueryBuilder, SqlWriter,
    TableRef, write_int,
};

/// MySQL-only EXPLAIN table/column or wildcard target.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplainTable {
    pub(crate) table: TableRef,
    pub(crate) target: Option<ExplainTableTarget>,
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

impl ExplainTable {
    /// Start building a table target for `EXPLAIN tbl_name [col_name | wild]`.
    pub fn new(table: impl IntoTableRef) -> Self {
        Self {
            table: table.into_table_ref(),
            target: None,
        }
    }

    /// Use a column target; replaces any previously column/wildcard.
    pub fn column(mut self, column: impl IntoIden) -> Self {
        self.target = Some(ExplainTableTarget::Column(column.into_iden()));
        self
    }

    /// Use a wildcard target (string literal); replaces any previously column/wildcard.
    pub fn wildcard(mut self, wildcard: &'static str) -> Self {
        self.target = Some(ExplainTableTarget::Wildcard(wildcard));
        self
    }
}

impl<T> From<T> for ExplainTable
where
    T: IntoTableRef,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct MySqlExplainOptions {
    pub(crate) analyze: Option<bool>,
    pub(crate) format: Option<ExplainFormat>,
    pub(crate) into_variable: Option<String>,
    pub(crate) schema_spec: Option<MySqlExplainSchemaSpec>,
    pub(crate) table: Option<TableRef>,
    pub(crate) target: Option<ExplainTableTarget>,
    pub(crate) for_connection: Option<u64>,
}

impl MySqlExplainOptions {
    pub(crate) fn write_options(
        &self,
        query_builder: &impl QueryBuilder,
        sql: &mut impl SqlWriter,
        statement: Option<&ExplainableStatement>,
    ) {
        if let Some(table) = &self.table {
            sql.write_str(" ").unwrap();
            query_builder.prepare_table_ref(table, sql);
            if let Some(target) = &self.target {
                match target {
                    ExplainTableTarget::Column(column) => {
                        sql.write_str(" ").unwrap();
                        query_builder.prepare_iden(column, sql);
                    }
                    ExplainTableTarget::Wildcard(wildcard) => {
                        sql.write_str(" '").unwrap();
                        if query_builder.needs_escape(wildcard) {
                            sql.write_str(&query_builder.escape_string(wildcard))
                                .unwrap();
                        } else {
                            sql.write_str(wildcard).unwrap();
                        }
                        sql.write_str("'").unwrap();
                    }
                }
            }
            return;
        }

        if let Some(analyze) = self.analyze {
            if analyze {
                sql.write_str(" ANALYZE").unwrap();
            } else {
                sql.write_str(" ANALYZE FALSE").unwrap();
            }
        }

        if let Some(format) = self.format {
            sql.write_str(" FORMAT = ").unwrap();
            sql.write_str(format.as_str()).unwrap();
        }

        if let Some(variable) = &self.into_variable {
            sql.write_str(" INTO ").unwrap();
            sql.write_str(variable).unwrap();
        }

        if let Some(schema) = &self.schema_spec {
            match schema {
                MySqlExplainSchemaSpec::Schema(schema) => {
                    sql.write_str(" FOR SCHEMA ").unwrap();
                    query_builder.prepare_iden(schema, sql);
                }
                MySqlExplainSchemaSpec::Database(schema) => {
                    sql.write_str(" FOR DATABASE ").unwrap();
                    query_builder.prepare_iden(schema, sql);
                }
            }
        }

        if let Some(connection_id) = self.for_connection {
            sql.write_str(" FOR CONNECTION ").unwrap();
            write_int(sql, connection_id);
            return;
        }

        if let Some(statement) = statement {
            sql.write_str(" ").unwrap();
            statement.write_to(query_builder, sql);
        }
    }
}
