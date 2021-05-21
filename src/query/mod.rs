//! Query statements (select, insert, update & delete).
//!
//! # Usage
//!
//! - Query Select, see [`SelectStatement`]
//! - Query Insert, see [`InsertStatement`]
//! - Query Update, see [`UpdateStatement`]
//! - Query Delete, see [`DeleteStatement`]

mod condition;
mod delete;
mod insert;
mod ordered;
mod select;
mod shim;
mod traits;
mod update;

pub use condition::*;
pub use delete::*;
pub use insert::*;
pub use ordered::*;
pub use select::*;
pub use traits::*;
pub use update::*;

/// Shorthand for constructing any table query
#[derive(Debug, Clone)]
pub struct Query;

/// All available types of table query
#[derive(Debug, Clone)]
pub enum QueryStatement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
}

impl Query {
    /// Construct table [`SelectStatement`]
    pub fn select() -> SelectStatement {
        SelectStatement::new()
    }

    /// Construct table [`InsertStatement`]
    pub fn insert() -> InsertStatement {
        InsertStatement::new()
    }

    /// Construct table [`UpdateStatement`]
    pub fn update() -> UpdateStatement {
        UpdateStatement::new()
    }

    /// Construct table [`DeleteStatement`]
    pub fn delete() -> DeleteStatement {
        DeleteStatement::new()
    }
}
