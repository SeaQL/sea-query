use std::rc::Rc;
use crate::{backend::TableBuilder, types::*};

/// Drop a table
/// 
/// # Examples
/// 
/// ```
/// use sea_query::{*, tests_cfg::*};
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
///     r#"DROP TABLE `glyph`, `character`"#
/// );
/// ```
#[derive(Clone)]
pub struct TableDropStatement {
    pub(crate) tables: Vec<Rc<dyn Iden>>,
    pub(crate) options: Vec<TableDropOpt>,
    pub(crate) if_exist: bool,
}

/// All available table drop options
#[derive(Clone)]
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
            if_exist: false,
        }
    }

    /// Set table name
    pub fn table<T: 'static>(mut self, table: T) -> Self
        where T: Iden {
        self.tables.push(Rc::new(table));
        self
    }

    /// Drop table if exists
    pub fn if_exist(mut self) -> Self {
        self.if_exist = true;
        self
    }

    /// Drop option restrict
    pub fn restrict(mut self) -> Self {
        self.options.push(TableDropOpt::Restrict);
        self
    }

    /// Drop option cacade
    pub fn cascade(mut self) -> Self {
        self.options.push(TableDropOpt::Cascade);
        self
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build<T: TableBuilder>(&self, table_builder: T) -> String {
        let mut sql = String::new();
        table_builder.prepare_table_drop_statement(self, &mut sql);
        sql
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build_any(&self, table_builder: &dyn TableBuilder) -> String {
        let mut sql = String::new();
        table_builder.prepare_table_drop_statement(self, &mut sql);
        sql
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn to_string<T: TableBuilder>(&self, table_builder: T) -> String {
        self.build(table_builder)
    }
}