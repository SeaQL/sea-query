use std::rc::Rc;
use crate::{backend::TableBuilder, types::*};

/// Rename a table
/// 
/// # Examples
/// 
/// ```
/// use sea_query::{*, tests_cfg::*};
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
///     r#"ALTER TABLE `font` RENAME TO `font_new`"#
/// );
/// ```
#[derive(Clone)]
pub struct TableRenameStatement {
    pub(crate) from_name: Option<Rc<dyn Iden>>,
    pub(crate) to_name: Option<Rc<dyn Iden>>,
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
    pub fn table<T: 'static, R: 'static>(mut self, from_name: T, to_name: R) -> Self
        where T: Iden, R: Iden {
        self.from_name = Some(Rc::new(from_name));
        self.to_name = Some(Rc::new(to_name));
        self
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build<T: TableBuilder>(&self, table_builder: T) -> String {
        let mut sql = String::new();
        table_builder.prepare_table_rename_statement(self, &mut sql);
        sql
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build_any(&self, table_builder: &dyn TableBuilder) -> String {
        let mut sql = String::new();
        table_builder.prepare_table_rename_statement(self, &mut sql);
        sql
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn to_string<T: TableBuilder>(&self, table_builder: T) -> String {
        self.build(table_builder)
    }
}