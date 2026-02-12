use inherent::inherent;

#[cfg(feature = "backend-sqlite")]
use std::marker::PhantomData;

use crate::{SchemaStatementBuilder, backend::SchemaBuilder, types::*};

/// Drop a table.
///
/// Note: SQLite only supports dropping a single table per statement.
/// When the `backend-sqlite` feature is enabled, this builder enforces that
/// constraint at compile time.
///
/// # Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let table = Table::drop().table(Glyph::Table).to_owned();
///
/// assert_eq!(table.to_string(MysqlQueryBuilder), r#"DROP TABLE `glyph`"#);
/// assert_eq!(
///     table.to_string(PostgresQueryBuilder),
///     r#"DROP TABLE "glyph""#
/// );
/// assert_eq!(table.to_string(SqliteQueryBuilder), r#"DROP TABLE "glyph""#);
/// ```
///
/// Dropping multiple tables in one statement is supported by MySQL/Postgres,
/// but not by SQLite:
///
/// ```
/// # use sea_query::{tests_cfg::*, *};
/// # #[cfg(not(feature = "backend-sqlite"))]
/// # {
/// let table = Table::drop()
///     .table(Glyph::Table)
///     .table(Char::Table)
///     .to_owned();
/// assert_eq!(
///     table.to_string(MysqlQueryBuilder),
///     r#"DROP TABLE `glyph`, `character`"#
/// );
/// assert_eq!(
///     table.to_string(PostgresQueryBuilder),
///     r#"DROP TABLE "glyph", "character""#
/// );
/// # }
/// ```

#[derive(Default, Debug, Clone)]
struct TableDropStatementInner {
    tables: Vec<TableRef>,
    options: Vec<TableDropOpt>,
    if_exists: bool,
}

impl TableDropStatementInner {
    fn push_table<T>(&mut self, table: T)
    where
        T: IntoTableRef,
    {
        self.tables.push(table.into_table_ref());
    }

    pub(crate) fn is_if_exists(&self) -> bool {
        self.if_exists
    }

    pub(crate) fn get_tables(&self) -> &[TableRef] {
        &self.tables
    }

    pub(crate) fn get_options(&self) -> &[TableDropOpt] {
        &self.options
    }

    /// Drop table if exists
    fn if_exists(&mut self) -> &mut Self {
        self.if_exists = true;
        self
    }

    /// Drop option restrict
    fn restrict(&mut self) -> &mut Self {
        self.options.push(TableDropOpt::Restrict);
        self
    }

    /// Drop option cacade
    fn cascade(&mut self) -> &mut Self {
        self.options.push(TableDropOpt::Cascade);
        self
    }

    fn take(&mut self) -> Self {
        Self {
            tables: std::mem::take(&mut self.tables),
            options: std::mem::take(&mut self.options),
            if_exists: self.if_exists,
        }
    }
}

// --- SQLite-enabled: typestate builder (only one `.table()` call allowed) ---

#[cfg(feature = "backend-sqlite")]
#[derive(Debug, Clone, Copy)]
pub struct TableDropPending;

#[cfg(feature = "backend-sqlite")]
#[derive(Debug, Clone, Copy)]
pub struct TableDropDefined;

/// When `backend-sqlite` is enabled, `TableDropStatement` becomes a typestate builder.
///
/// The default type parameter corresponds to the *defined* state (a table has been set),
/// which is the only state that can be converted into SQL.
#[cfg(feature = "backend-sqlite")]
#[derive(Default, Debug, Clone)]
pub struct TableDropStatement<S = TableDropDefined> {
    inner: TableDropStatementInner,
    _state: PhantomData<S>,
}

#[cfg(feature = "backend-sqlite")]
impl TableDropStatement<TableDropPending> {
    /// Construct drop table statement
    pub fn new() -> Self {
        Self {
            inner: TableDropStatementInner::default(),
            _state: PhantomData,
        }
    }

    /// Set table name (SQLite supports only one table per statement)
    pub fn table<T>(mut self, table: T) -> TableDropStatement<TableDropDefined>
    where
        T: IntoTableRef,
    {
        self.inner.push_table(table);
        TableDropStatement {
            inner: self.inner,
            _state: PhantomData,
        }
    }
}

#[cfg(feature = "backend-sqlite")]
impl TableDropStatement<TableDropDefined> {
    #[deprecated(
        since = "1.0.0",
        note = "SQLite strictly forbids dropping multiple tables in a single statement. Please split this into multiple Table::drop() calls."
    )]
    pub fn table<T>(self, _table: T) -> Self
    where
        T: IntoTableRef,
    {
        panic!(
            "Attempted to drop multiple tables in SQLite mode. This is not supported. See compiler warnings."
        );
    }
}

#[cfg(feature = "backend-sqlite")]
impl<S> TableDropStatement<S> {
    pub(crate) fn is_if_exists(&self) -> bool {
        self.inner.is_if_exists()
    }

    pub(crate) fn get_tables(&self) -> &[TableRef] {
        self.inner.get_tables()
    }

    pub(crate) fn get_options(&self) -> &[TableDropOpt] {
        self.inner.get_options()
    }

    /// Drop table if exists
    pub fn if_exists(&mut self) -> &mut Self {
        self.inner.if_exists();
        self
    }

    /// Drop option restrict
    pub fn restrict(&mut self) -> &mut Self {
        self.inner.restrict();
        self
    }

    /// Drop option cacade
    pub fn cascade(&mut self) -> &mut Self {
        self.inner.cascade();
        self
    }

    pub fn take(&mut self) -> Self {
        Self {
            inner: self.inner.take(),
            _state: PhantomData,
        }
    }
}

#[cfg(feature = "backend-sqlite")]
#[inherent]
impl SchemaStatementBuilder for TableDropStatement<TableDropDefined> {
    pub fn build<T>(&self, schema_builder: T) -> String
    where
        T: SchemaBuilder,
    {
        let mut sql = String::with_capacity(256);
        schema_builder.prepare_table_drop_statement(self, &mut sql);
        sql
    }

    pub fn to_string<T>(&self, schema_builder: T) -> String
    where
        T: SchemaBuilder;
}

// --- SQLite-disabled: legacy builder (multiple `.table()` calls allowed) ---

#[cfg(not(feature = "backend-sqlite"))]
#[derive(Default, Debug, Clone)]
pub struct TableDropStatement {
    inner: TableDropStatementInner,
}

/// All available table drop options
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum TableDropOpt {
    Restrict,
    Cascade,
}

#[cfg(not(feature = "backend-sqlite"))]
impl TableDropStatement {
    /// Construct drop table statement
    pub fn new() -> Self {
        Self::default()
    }

    /// Set table name
    pub fn table<T>(&mut self, table: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.inner.push_table(table);
        self
    }

    pub fn take(&mut self) -> Self {
        Self {
            inner: self.inner.take(),
        }
    }

    pub(crate) fn is_if_exists(&self) -> bool {
        self.inner.is_if_exists()
    }

    pub(crate) fn get_tables(&self) -> &[TableRef] {
        self.inner.get_tables()
    }

    pub(crate) fn get_options(&self) -> &[TableDropOpt] {
        self.inner.get_options()
    }

    /// Drop table if exists
    pub fn if_exists(&mut self) -> &mut Self {
        self.inner.if_exists();
        self
    }

    /// Drop option restrict
    pub fn restrict(&mut self) -> &mut Self {
        self.inner.restrict();
        self
    }

    /// Drop option cacade
    pub fn cascade(&mut self) -> &mut Self {
        self.inner.cascade();
        self
    }
}

#[cfg(not(feature = "backend-sqlite"))]
#[inherent]
impl SchemaStatementBuilder for TableDropStatement {
    pub fn build<T>(&self, schema_builder: T) -> String
    where
        T: SchemaBuilder,
    {
        let mut sql = String::with_capacity(256);
        schema_builder.prepare_table_drop_statement(self, &mut sql);
        sql
    }

    pub fn to_string<T>(&self, schema_builder: T) -> String
    where
        T: SchemaBuilder;
}
