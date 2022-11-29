use crate::{backend::SchemaBuilder, types::*, SchemaStatementBuilder};

/// Drop a table
///
/// # Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let table = Table::truncate().table(Font::Table).to_owned();
///
/// assert_eq!(
///     table.to_string(MysqlQueryBuilder),
///     r#"TRUNCATE TABLE `font`"#
/// );
/// assert_eq!(
///     table.to_string(PostgresQueryBuilder),
///     r#"TRUNCATE TABLE "font""#
/// );
/// assert_eq!(
///     table.to_string(SqliteQueryBuilder),
///     r#"TRUNCATE TABLE "font""#
/// );
/// ```
#[derive(Default, Debug, Clone)]
pub struct TableTruncateStatement {
    pub(crate) table: Option<TableRef>,
}

impl TableTruncateStatement {
    /// Construct truncate table statement
    pub fn new() -> Self {
        Self::default()
    }

    /// Set table name
    pub fn table<T>(&mut self, table: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.table = Some(table.into_table_ref());
        self
    }

    pub fn take(&mut self) -> Self {
        Self {
            table: self.table.take(),
        }
    }
}

impl SchemaStatementBuilder for TableTruncateStatement {
    fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        let mut sql = String::with_capacity(256);
        schema_builder.prepare_table_truncate_statement(self, &mut sql);
        sql
    }

    fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
        let mut sql = String::with_capacity(256);
        schema_builder.prepare_table_truncate_statement(self, &mut sql);
        sql
    }
}
