//! "Compound" identifiers built on top of `DynIden`.

use crate::{FunctionCall, SelectStatement, ValueTuple};
use std::{fmt::Debug, iter::Flatten};

use super::*;

pub trait IdenList {
    type IntoIter: Iterator<Item = DynIden>;

    fn into_iter(self) -> Self::IntoIter;
}

impl<I> IdenList for I
where
    I: IntoIden,
{
    type IntoIter = std::iter::Once<DynIden>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.into_iden())
    }
}

impl<A, B> IdenList for (A, B)
where
    A: IntoIden,
    B: IntoIden,
{
    type IntoIter = std::array::IntoIter<DynIden, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0.into_iden(), self.1.into_iden()].into_iter()
    }
}

impl<A, B, C> IdenList for (A, B, C)
where
    A: IntoIden,
    B: IntoIden,
    C: IntoIden,
{
    type IntoIter = std::array::IntoIter<DynIden, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0.into_iden(), self.1.into_iden(), self.2.into_iden()].into_iter()
    }
}

/// An identifier that represents a database name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DatabaseName(pub DynIden);

impl<T> From<T> for DatabaseName
where
    T: IntoIden,
{
    fn from(iden: T) -> Self {
        DatabaseName(iden.into_iden())
    }
}

/// A schema name, potentially qualified as `(database.)schema`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemaName(pub Option<DatabaseName>, pub DynIden);

/// An SQL type name, potentially qualified as `(database.)(schema.)type`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeRef(pub Option<SchemaName>, pub DynIden);

pub trait IntoTypeRef: Into<TypeRef> {
    fn into_type_ref(self) -> TypeRef;
}

impl<T> IntoTypeRef for T
where
    T: Into<TypeRef>,
{
    fn into_type_ref(self) -> TypeRef {
        self.into()
    }
}

/// A table name, potentially qualified as `(database.)(schema.)table`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableName(pub Option<SchemaName>, pub DynIden);

impl TableName {
    /// A flat `(db?, schema?, table)` tuple view, for quick pattern matching.
    ///
    /// Don't use this if you need exhaustiveness.
    /// The return type is too lax and allows invalid shapes like `(Some(_), None, _)`.
    pub(crate) fn as_iden_tuple(&self) -> (Option<&DynIden>, Option<&DynIden>, &DynIden) {
        let TableName(schema_name, table) = self;
        match schema_name {
            None => (None, None, table),
            Some(SchemaName(db_name, schema)) => match db_name {
                None => (None, Some(schema), table),
                Some(DatabaseName(db)) => (Some(db), Some(schema), table),
            },
        }
    }
}

/// A column name, potentially qualified as `(database.)(schema.)(table.)column`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnName(pub Option<TableName>, pub DynIden);

/// Iteration over `[db?, schema?, table?, column]` identifiers.
impl IdenList for ColumnName {
    type IntoIter = Flatten<std::array::IntoIter<Option<DynIden>, 4>>;

    /// Iteration over `[db?, schema?, table?, column]` identifiers.
    fn into_iter(self) -> Self::IntoIter {
        let ColumnName(table_name, column) = self;
        let arr = match table_name {
            None => [None, None, None, Some(column)],
            Some(TableName(schema_name, table)) => match schema_name {
                None => [None, None, Some(table), Some(column)],
                Some(SchemaName(db_name, schema)) => {
                    let db = db_name.map(|db| db.0);
                    [db, Some(schema), Some(table), Some(column)]
                }
            },
        };
        arr.into_iter().flatten()
    }
}

/// Column references.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ColumnRef {
    /// A column name, potentially qualified as `(database.)(schema.)(table.)column`.
    Column(ColumnName),
    /// An `*` expression, potentially qualified as `(database.)(schema.)(table.)*`.
    Asterisk(Option<TableName>),
    /// NEW.*
    New(DynIden),
    /// Old.*
    Old(DynIden),
    #[cfg(feature = "backend-postgres")]
    /// excluded.*
    Excluded(DynIden),
}

impl ColumnRef {
    #[doc(hidden)]
    /// Returns the unqualified column name if it's not an asterisk.
    pub fn column(&self) -> Option<&DynIden> {
        match self {
            ColumnRef::Column(ColumnName(_table_ref, column_itself)) => Some(column_itself),
            ColumnRef::Asterisk(..) => None,
            ColumnRef::New(column_itself) => Some(column_itself),
            ColumnRef::Old(column_itself) => Some(column_itself),
            #[cfg(feature = "backend-postgres")]
            ColumnRef::Excluded(column_itself) => Some(column_itself),
        }
    }
}

impl From<Asterisk> for ColumnRef {
    fn from(_: Asterisk) -> Self {
        ColumnRef::Asterisk(None)
    }
}

impl<T> From<T> for ColumnRef
where
    T: Into<ColumnName>,
{
    fn from(value: T) -> Self {
        ColumnRef::Column(value.into())
    }
}

impl<T> From<(T, Asterisk)> for ColumnRef
where
    T: IntoIden,
{
    fn from(value: (T, Asterisk)) -> Self {
        ColumnRef::Asterisk(Some(value.0.into_iden().into()))
    }
}

pub trait IntoColumnRef: Into<ColumnRef> {
    fn into_column_ref(self) -> ColumnRef;
}

impl<T> IntoColumnRef for T
where
    T: Into<ColumnRef>,
{
    fn into_column_ref(self) -> ColumnRef {
        self.into()
    }
}

/// Table references
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum TableRef {
    /// A table identifier with optional Alias. Potentially qualified.
    Table(TableName, Option<DynIden>),
    /// Subquery with alias
    SubQuery(Box<SelectStatement>, DynIden),
    /// Values list with alias
    ValuesList(Vec<ValueTuple>, DynIden),
    /// Function call with alias
    FunctionCall(FunctionCall, DynIden),
}

impl TableRef {
    /// Add or replace the current alias
    pub fn alias<A>(self, alias: A) -> Self
    where
        A: IntoIden,
    {
        match self {
            Self::Table(table, _) => Self::Table(table, Some(alias.into_iden())),
            Self::SubQuery(statement, _) => Self::SubQuery(statement, alias.into_iden()),
            Self::ValuesList(values, _) => Self::ValuesList(values, alias.into_iden()),
            Self::FunctionCall(func, _) => Self::FunctionCall(func, alias.into_iden()),
        }
    }

    #[doc(hidden)]
    pub fn sea_orm_table(&self) -> &DynIden {
        match self {
            TableRef::Table(TableName(_, tbl), _)
            | TableRef::SubQuery(_, tbl)
            | TableRef::ValuesList(_, tbl)
            | TableRef::FunctionCall(_, tbl) => tbl,
        }
    }

    #[doc(hidden)]
    pub fn sea_orm_table_alias(&self) -> Option<&DynIden> {
        match self {
            TableRef::Table(_, None) | TableRef::SubQuery(_, _) | TableRef::ValuesList(_, _) => {
                None
            }
            TableRef::Table(_, Some(alias)) | TableRef::FunctionCall(_, alias) => Some(alias),
        }
    }
}

impl<T> From<T> for TableRef
where
    T: Into<TableName>,
{
    fn from(value: T) -> Self {
        TableRef::Table(value.into(), None)
    }
}

pub trait IntoTableRef: Into<TableRef> {
    fn into_table_ref(self) -> TableRef {
        self.into()
    }
}

impl<T> IntoTableRef for T where T: Into<TableRef> {}
