//! Table definition & alternations statements.
//!
//! # Usage
//!
//! - Table Create, see [`TableCreateStatement`]
//! - Table Alter, see [`TableAlterStatement`]
//! - Table Drop, see [`TableDropStatement`]
//! - Table Rename, see [`TableRenameStatement`]
//! - Table Truncate, see [`TableTruncateStatement`]

use crate::SchemaBuilder;

mod alter;
mod column;
mod constraint;
mod create;
mod drop;
mod rename;
mod truncate;

pub use alter::*;
pub use column::*;
pub use constraint::*;
pub use create::*;
pub use drop::*;
pub use rename::*;
pub use truncate::*;

/// Helper for constructing any table statement
#[derive(Debug)]
pub struct Table;

/// All available types of table statement
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum TableStatement {
    Create(TableCreateStatement),
    Alter(TableAlterStatement),
    Drop(TableDropStatement),
    Rename(TableRenameStatement),
    Truncate(TableTruncateStatement),
}

impl Table {
    /// Construct table [`TableCreateStatement`]
    pub fn create() -> TableCreateStatement {
        TableCreateStatement::new()
    }

    /// Construct table [`TableAlterStatement`]
    pub fn alter() -> TableAlterStatement {
        TableAlterStatement::new()
    }

    /// Construct table [`TableDropStatement`]
    #[cfg(feature = "backend-sqlite")]
    pub fn drop() -> TableDropStatement<TableDropPending> {
        TableDropStatement::new()
    }

    /// Construct table [`TableDropStatement`]
    #[cfg(not(feature = "backend-sqlite"))]
    pub fn drop() -> TableDropStatement {
        TableDropStatement::new()
    }

    /// Construct table [`TableRenameStatement`]
    pub fn rename() -> TableRenameStatement {
        TableRenameStatement::new()
    }

    /// Construct table [`TableTruncateStatement`]
    pub fn truncate() -> TableTruncateStatement {
        TableTruncateStatement::new()
    }
}

impl TableStatement {
    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build<T>(&self, table_builder: T) -> String
    where
        T: SchemaBuilder,
    {
        match self {
            Self::Create(stat) => stat.build(table_builder),
            Self::Alter(stat) => stat.build(table_builder),
            Self::Drop(stat) => stat.build(table_builder),
            Self::Rename(stat) => stat.build(table_builder),
            Self::Truncate(stat) => stat.build(table_builder),
        }
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn to_string<T>(&self, table_builder: T) -> String
    where
        T: SchemaBuilder,
    {
        match self {
            Self::Create(stat) => stat.to_string(table_builder),
            Self::Alter(stat) => stat.to_string(table_builder),
            Self::Drop(stat) => stat.to_string(table_builder),
            Self::Rename(stat) => stat.to_string(table_builder),
            Self::Truncate(stat) => stat.to_string(table_builder),
        }
    }
}
