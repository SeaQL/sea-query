use std::rc::Rc;
use crate::{ColumnDef, backend::TableBuilder, types::*, prepare::*};

/// Alter a table
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let table = Table::alter()
///     .table(Font::Table)
///     .add_column(ColumnDef::new(Alias::new("new_col")).integer().not_null().default(100))
///     .to_owned();
///
/// assert_eq!(
///     table.to_string(MysqlQueryBuilder),
///     r#"ALTER TABLE `font` ADD COLUMN `new_col` int NOT NULL DEFAULT 100"#
/// );
/// assert_eq!(
///     table.to_string(PostgresQueryBuilder),
///     r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 100"#
/// );
/// assert_eq!(
///     table.to_string(SqliteQueryBuilder),
///     r#"ALTER TABLE `font` ADD COLUMN `new_col` integer NOT NULL DEFAULT 100"#,
/// );
/// ```
#[derive(Debug, Clone)]
pub struct TableAlterStatement {
    pub(crate) table: Option<Rc<dyn Iden>>,
    pub(crate) alter_option: Option<TableAlterOption>,
}

/// All available table alter options
#[derive(Debug, Clone)]
pub enum TableAlterOption {
    AddColumn(ColumnDef),
    ModifyColumn(ColumnDef),
    RenameColumn(Rc<dyn Iden>, Rc<dyn Iden>),
    DropColumn(Rc<dyn Iden>),
}

impl Default for TableAlterStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl TableAlterStatement {
    /// Construct alter table statement
    pub fn new() -> Self {
        Self {
            table: None,
            alter_option: None,
        }
    }

    /// Set table name
    pub fn table<T: 'static>(mut self, table: T) -> Self
        where T: Iden {
        self.table = Some(Rc::new(table));
        self
    }

    /// Add a column to an existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let table = Table::alter()
    ///     .table(Font::Table)
    ///     .add_column(ColumnDef::new(Alias::new("new_col")).integer().not_null().default(100))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     r#"ALTER TABLE `font` ADD COLUMN `new_col` int NOT NULL DEFAULT 100"#
    /// );
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 100"#
    /// );
    /// assert_eq!(
    ///     table.to_string(SqliteQueryBuilder),
    ///     r#"ALTER TABLE `font` ADD COLUMN `new_col` integer NOT NULL DEFAULT 100"#,
    /// );
    /// ```
    pub fn add_column(self, column_def: ColumnDef) -> Self {
        self.alter_option(TableAlterOption::AddColumn(column_def))
    }

    /// Modify a column in an existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let table = Table::alter()
    ///     .table(Font::Table)
    ///     .modify_column(ColumnDef::new(Alias::new("new_col")).big_integer().default(999))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     r#"ALTER TABLE `font` MODIFY COLUMN `new_col` bigint DEFAULT 999"#
    /// );
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     vec![
    ///         r#"ALTER TABLE "font""#,
    ///             r#"ALTER COLUMN "new_col" TYPE bigint,"#,
    ///             r#"ALTER COLUMN "new_col" SET DEFAULT 999"#,
    ///     ].join(" ")
    /// );
    /// // Sqlite not support modifying table column
    /// ```
    pub fn modify_column(self, column_def: ColumnDef) -> Self {
        self.alter_option(TableAlterOption::ModifyColumn(column_def))
    }

    /// Rename a column in an existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let table = Table::alter()
    ///     .table(Font::Table)
    ///     .rename_column(Alias::new("new_col"), Alias::new("new_column"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     r#"ALTER TABLE `font` RENAME COLUMN `new_col` TO `new_column`"#
    /// );
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     r#"ALTER TABLE "font" RENAME COLUMN "new_col" TO "new_column""#
    /// );
    /// assert_eq!(
    ///     table.to_string(SqliteQueryBuilder),
    ///     r#"ALTER TABLE `font` RENAME COLUMN `new_col` TO `new_column`"#
    /// );
    /// ```
    pub fn rename_column<T: 'static, R: 'static>(self, from_name: T, to_name: R) -> Self
        where T: Iden, R: Iden {
        self.alter_option(TableAlterOption::RenameColumn(Rc::new(from_name), Rc::new(to_name)))
    }

    /// Add a column to existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let table = Table::alter()
    ///     .table(Font::Table)
    ///     .drop_column(Alias::new("new_column"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     r#"ALTER TABLE `font` DROP COLUMN `new_column`"#
    /// );
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     r#"ALTER TABLE "font" DROP COLUMN "new_column""#
    /// );
    /// // Sqlite not support modifying table column
    /// ```
    pub fn drop_column<T: 'static>(self, col_name: T) -> Self
        where T: Iden {
        self.alter_option(TableAlterOption::DropColumn(Rc::new(col_name)))
    }

    fn alter_option(mut self, alter_option: TableAlterOption) -> Self {
        self.alter_option = Some(alter_option);
        self
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build<T: TableBuilder>(&self, table_builder: T) -> String {
        let mut sql = SqlWriter::new();
        table_builder.prepare_table_alter_statement(self, &mut sql);
        sql.result()
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build_any(&self, table_builder: &dyn TableBuilder) -> String {
        let mut sql = SqlWriter::new();
        table_builder.prepare_table_alter_statement(self, &mut sql);
        sql.result()
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn to_string<T: TableBuilder>(&self, table_builder: T) -> String {
        self.build(table_builder)
    }
}