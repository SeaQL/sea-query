//! Database table (create, alter, drop, rename & truncate).
//! 
//! # Usage
//! 
//! - Table Create, see [`TableCreateStatement`]
//! - Table Alter, see [`TableAlterStatement`]
//! - Table Drop, see [`TableDropStatement`]
//! - Table Rename, see [`TableRenameStatement`]
//! - Table Truncate, see [`TableTruncateStatement`]

mod alter;
mod common;
mod create;
mod drop;
mod rename;
mod truncate;

pub use alter::*;
pub use common::*;
pub use create::*;
pub use drop::*;
pub use rename::*;
pub use truncate::*;

/// Shorthand for constructing any table statement
#[derive(Clone)]
pub struct Table;

/// All available types of table statement
#[derive(Clone)]
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