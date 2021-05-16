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

pub trait GenericBuilder: QueryBuilder + SchemaBuilder {}

pub trait SchemaBuilder: TableBuilder + IndexBuilder + ForeignKeyBuilder {}

pub trait QueryBuilder {

    /// Translate [`InsertStatement`] into SQL statement.
    fn prepare_insert_statement(&self, insert: &InsertStatement, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`SelectStatement`] into SQL statement.
    fn prepare_select_statement(&self, select: &SelectStatement, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`UpdateStatement`] into SQL statement.
    fn prepare_update_statement(&self, update: &UpdateStatement, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`DeleteStatement`] into SQL statement.
    fn prepare_delete_statement(&self, delete: &DeleteStatement, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`SimpleExpr`] into SQL statement.
    fn prepare_simple_expr(&self, simple_expr: &SimpleExpr, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`SelectDistinct`] into SQL statement.
    fn prepare_select_distinct(&self, select_distinct: &SelectDistinct, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`SelectExpr`] into SQL statement.
    fn prepare_select_expr(&self, select_expr: &SelectExpr, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`JoinExpr`] into SQL statement.
    fn prepare_join_expr(&self, join_expr: &JoinExpr, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`TableRef`] into SQL statement.
    fn prepare_table_ref(&self, table_ref: &TableRef, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`UnOper`] into SQL statement.
    fn prepare_un_oper(&self, un_oper: &UnOper, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`BinOper`] into SQL statement.
    fn prepare_bin_oper(&self, bin_oper: &BinOper, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`LogicalChainOper`] into SQL statement.
    fn prepare_logical_chain_oper(&self, log_chain_oper: &LogicalChainOper, i: usize, length: usize, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`Function`] into SQL statement.
    fn prepare_function(&self, function: &Function, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`JoinType`] into SQL statement.
    fn prepare_join_type(&self, join_type: &JoinType, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`OrderExpr`] into SQL statement.
    fn prepare_order_expr(&self, order_expr: &OrderExpr, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`JoinOn`] into SQL statement.
    fn prepare_join_on(&self, join_on: &JoinOn, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`Order`] into SQL statement.
    fn prepare_order(&self, order: &Order, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`Value`] into SQL statement.
    fn prepare_value(&self, value: &Value, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Translate [`Keyword`] into SQL statement.
    fn prepare_keyword(&self, keyword: &Keyword, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value));

    /// Convert a SQL value into syntax-specific string
    fn value_to_string(&self, v: &Value) -> String;
}

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
