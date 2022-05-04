use crate::{backend::SchemaBuilder, prepare::*, types::*, SchemaStatementBuilder};

/// Rename a table
///
/// # Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let table = Table::rename()
///     .table(Font::Table, Alias::new("font_new"))
///     .to_owned();
///
/// assert_eq!(
///     table.to_string(MysqlQueryBuilder),
///     r#"RENAME TABLE `font` TO `font_new`"#
/// );
/// assert_eq!(
///     table.to_string(PostgresQueryBuilder),
///     r#"ALTER TABLE "font" RENAME TO "font_new""#
/// );
/// assert_eq!(
///     table.to_string(SqliteQueryBuilder),
///     r#"ALTER TABLE "font" RENAME TO "font_new""#
/// );
/// ```
#[derive(Debug, Clone)]
pub struct TableRenameStatement {
    pub(crate) from_name: Option<DynIden>,
    pub(crate) to_name: Option<DynIden>,
}

impl Default for TableRenameStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl TableRenameStatement {
    /// Construct rename table statement
    pub fn new() -> Self {
        Self {
            from_name: None,
            to_name: None,
        }
    }

    /// Set old and new table name
    pub fn table<T: 'static, R: 'static>(&mut self, from_name: T, to_name: R) -> &mut Self
    where
        T: Iden,
        R: Iden,
    {
        self.from_name = Some(SeaRc::new(from_name));
        self.to_name = Some(SeaRc::new(to_name));
        self
    }

    pub fn take(&mut self) -> Self {
        Self {
            from_name: self.from_name.take(),
            to_name: self.to_name.take(),
        }
    }
}

impl SchemaStatementBuilder for TableRenameStatement {
    fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_table_rename_statement(self, &mut sql);
        sql.result()
    }

    fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_table_rename_statement(self, &mut sql);
        sql.result()
    }
}
