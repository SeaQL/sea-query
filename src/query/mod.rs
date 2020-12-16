//! Table query (select, insert, update & delete).
//! 
//! # Usage
//! 
//! - Query Select, see [`SelectStatement`]
//! - Query Insert, see [`InsertStatement`]
//! - Query Update, see [`UpdateStatement`]
//! - Query Delete, see [`DeleteStatement`]

mod select;
mod insert;
mod update;
mod delete;

pub use select::*;
pub use insert::*;
pub use update::*;
pub use delete::*;

/// Shorthand for constructing any table query
#[derive(Clone)]
pub struct Query;

/// All available types of table query
#[derive(Clone)]
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
