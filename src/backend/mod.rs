//! Translating the SQL AST into engine-specific SQL statements.

use crate::*;

#[cfg(feature="backend-mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend-mysql")))]
mod mysql;
#[cfg(feature="backend-postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
mod postgres;
#[cfg(feature="backend-sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend-sqlite")))]
mod sqlite;

#[cfg(feature="backend-mysql")]
pub use mysql::*;
#[cfg(feature="backend-postgres")]
pub use postgres::*;
#[cfg(feature="backend-sqlite")]
pub use sqlite::*;

mod query_builder;

pub use self::query_builder::*;

pub trait GenericBuilder: QueryBuilder + SchemaBuilder {}

pub trait SchemaBuilder: TableBuilder + IndexBuilder + ForeignKeyBuilder {}

pub trait TableBuilder {

    /// Translate [`TableCreateStatement`] into SQL statement.
    fn prepare_table_create_statement(&self, insert: &TableCreateStatement, sql: &mut SqlWriter);

    /// Translate [`ColumnDef`] into SQL statement.
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut SqlWriter);

    /// Translate [`ColumnType`] into SQL statement.
    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut SqlWriter);

    /// Translate [`ColumnSpec`] into SQL statement.
    fn prepare_column_spec(&self, column_spec: &ColumnSpec, sql: &mut SqlWriter);

    /// Translate [`TableOpt`] into SQL statement.
    fn prepare_table_opt(&self, table_opt: &TableOpt, sql: &mut SqlWriter);

    /// Translate [`TablePartition`] into SQL statement.
    fn prepare_table_partition(&self, table_partition: &TablePartition, sql: &mut SqlWriter);

    /// Translate [`TableDropStatement`] into SQL statement.
    fn prepare_table_drop_statement(&self, drop: &TableDropStatement, sql: &mut SqlWriter);

    /// Translate [`TableDropOpt`] into SQL statement.
    fn prepare_table_drop_opt(&self, drop_opt: &TableDropOpt, sql: &mut SqlWriter);

    /// Translate [`TableTruncateStatement`] into SQL statement.
    fn prepare_table_truncate_statement(&self, truncate: &TableTruncateStatement, sql: &mut SqlWriter);

    /// Translate [`TableAlterStatement`] into SQL statement.
    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut SqlWriter);

    /// Translate [`TableRenameStatement`] into SQL statement.
    fn prepare_table_rename_statement(&self, rename: &TableRenameStatement, sql: &mut SqlWriter);
}

pub trait IndexBuilder {
    /// Translate [`IndexCreateStatement`] into SQL expression.
    fn prepare_table_index_expression(&self, create: &IndexCreateStatement, sql: &mut SqlWriter);

    /// Translate [`IndexCreateStatement`] into SQL statement.
    fn prepare_index_create_statement(&self, create: &IndexCreateStatement, sql: &mut SqlWriter);

    /// Translate [`IndexDropStatement`] into SQL statement.
    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut SqlWriter);
}

pub trait ForeignKeyBuilder {
    /// Translate [`ForeignKeyCreateStatement`] into SQL statement.
    fn prepare_foreign_key_create_statement(&self, create: &ForeignKeyCreateStatement, sql: &mut SqlWriter);

    /// Translate [`ForeignKeyAction`] into SQL statement.
    fn prepare_foreign_key_action(&self, foreign_key_action: &ForeignKeyAction, sql: &mut SqlWriter);

    /// Translate [`ForeignKeyDropStatement`] into SQL statement.
    fn prepare_foreign_key_drop_statement(&self, drop: &ForeignKeyDropStatement, sql: &mut SqlWriter);
}
