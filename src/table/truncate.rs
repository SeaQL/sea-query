use crate::{backend::SchemaBuilder, prepare::*, types::*, SchemaStatementBuilder};

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
///     table.to_string(),
///     r#"TRUNCATE TABLE "font""#
/// );
/// assert_eq!(
///     table.to_string(SqliteQueryBuilder),
///     r#"TRUNCATE TABLE `font`"#
/// );
/// ```
#[derive(Debug, Clone)]
pub struct TableTruncateStatement {
    pub(crate) table: Option<DynIden>,
}

impl Default for TableTruncateStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl TableTruncateStatement {
    /// Construct truncate table statement
    pub fn new() -> Self {
        Self { table: None }
    }

    /// Set table name
    pub fn table<T: 'static>(&mut self, table: T) -> &mut Self
    where
        T: Iden,
    {
        self.table = Some(SeaRc::new(table));
        self
    }

    pub fn take(&mut self) -> Self {
        Self {
            table: self.table.take(),
        }
    }
}

impl SchemaStatementBuilder for TableTruncateStatement {
    fn build<T: SchemaBuilder>(&self) -> String {
        let schema_builder = T::default();
        let mut sql = SqlWriter::new();
        schema_builder.prepare_table_truncate_statement(self, &mut sql);
        sql.result()
    }
}
