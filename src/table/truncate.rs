use std::rc::Rc;
use crate::{backend::TableBuilder, types::*};

/// Drop a table
/// 
/// # Examples
/// 
/// ```
/// use sea_query::{*, tests_cfg::*};
/// 
/// let table = Table::truncate()
///     .table(Font::Table)
///     .to_owned();
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
///     r#"TRUNCATE TABLE `font`"#
/// );
/// ```
#[derive(Clone)]
pub struct TableTruncateStatement {
    pub(crate) table: Option<Rc<dyn Iden>>,
}

impl Default for TableTruncateStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl TableTruncateStatement {
    /// Construct truncate table statement
    pub fn new() -> Self {
        Self {
            table: None,
        }
    }

    /// Set table name
    pub fn table<T: 'static>(mut self, table: T) -> Self
        where T: Iden {
        self.table = Some(Rc::new(table));
        self
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build<T: TableBuilder>(&self, table_builder: T) -> String {
        let mut sql = String::new();
        table_builder.prepare_table_truncate_statement(self, &mut sql);
        sql
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build_any(&self, table_builder: &dyn TableBuilder) -> String {
        let mut sql = String::new();
        table_builder.prepare_table_truncate_statement(self, &mut sql);
        sql
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn to_string<T: TableBuilder>(&self, table_builder: T) -> String {
        self.build(table_builder)
    }
}