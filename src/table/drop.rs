use crate::{backend::SchemaBuilder, prepare::*, types::*, SchemaStatementBuilder};

/// Drop a table
///
/// # Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let table = Table::drop()
///     .table(Glyph::Table)
///     .table(Char::Table)
///     .to_owned();
///
/// assert_eq!(
///     table.to_string(MysqlQueryBuilder),
///     r#"DROP TABLE `glyph`, `character`"#
/// );
/// assert_eq!(
///     table.to_string(PostgresQueryBuilder),
///     r#"DROP TABLE "glyph", "character""#
/// );
/// assert_eq!(
///     table.to_string(SqliteQueryBuilder),
///     r#"DROP TABLE "glyph", "character""#
/// );
/// ```
#[derive(Debug, Clone)]
pub struct TableDropStatement {
    pub(crate) tables: Vec<TableRef>,
    pub(crate) options: Vec<TableDropOpt>,
    pub(crate) if_exists: bool,
}

/// All available table drop options
#[derive(Debug, Clone)]
pub enum TableDropOpt {
    Restrict,
    Cascade,
}

impl Default for TableDropStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl TableDropStatement {
    /// Construct drop table statement
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            options: Vec::new(),
            if_exists: false,
        }
    }

    /// Set table name
    pub fn table<T: 'static>(&mut self, table: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.tables.push(table.into_table_ref());
        self
    }

    /// Drop table if exists
    pub fn if_exists(&mut self) -> &mut Self {
        self.if_exists = true;
        self
    }

    /// Drop option restrict
    pub fn restrict(&mut self) -> &mut Self {
        self.options.push(TableDropOpt::Restrict);
        self
    }

    /// Drop option cacade
    pub fn cascade(&mut self) -> &mut Self {
        self.options.push(TableDropOpt::Cascade);
        self
    }

    pub fn take(&mut self) -> Self {
        Self {
            tables: std::mem::take(&mut self.tables),
            options: std::mem::take(&mut self.options),
            if_exists: self.if_exists,
        }
    }
}

impl SchemaStatementBuilder for TableDropStatement {
    fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_table_drop_statement(self, &mut sql);
        sql.result()
    }

    fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_table_drop_statement(self, &mut sql);
        sql.result()
    }
}
