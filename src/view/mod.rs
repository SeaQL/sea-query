//! View definition statements.
//!
//! # Usage
//!
//! - View Create, see [`ViewCreateStatement`]
//! - Table Drop, see [`ViewDropStatement`]
//! - Table Rename, see [`ViewRenameStatement`]

use crate::SchemaBuilder;

mod create;
mod drop;
mod rename;
mod shim;

pub use create::*;
pub use drop::*;
pub use rename::*;

/// Helper for constructing any view statement
#[derive(Debug)]
pub struct View;

/// All available types of view statement
#[derive(Debug, Clone)]
pub enum ViewStatement {
    Create(ViewCreateStatement),
    Rename(ViewRenameStatement),
    Drop(ViewDropStatement),
}

impl View {
    pub fn create() -> ViewCreateStatement {
        ViewCreateStatement::new()
    }

    pub fn rename() -> ViewRenameStatement {
        ViewRenameStatement::new()
    }

    pub fn drop() -> ViewDropStatement {
        ViewDropStatement::new()
    }
}

impl ViewStatement {
    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build<T: SchemaBuilder>(&self, view_builder: T) -> String {
        match self {
            Self::Create(stat) => stat.build(view_builder),
            Self::Drop(stat) => stat.build(view_builder),
            Self::Rename(stat) => stat.build(view_builder),
        }
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build_any(&self, view_builder: &dyn SchemaBuilder) -> String {
        match self {
            Self::Create(stat) => stat.build_any(view_builder),
            Self::Drop(stat) => stat.build_any(view_builder),
            Self::Rename(stat) => stat.build_any(view_builder),
        }
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn to_string<T: SchemaBuilder>(&self, view_builder: T) -> String {
        match self {
            Self::Create(stat) => stat.to_string(view_builder),
            Self::Drop(stat) => stat.to_string(view_builder),
            Self::Rename(stat) => stat.to_string(view_builder),
        }
    }
}
