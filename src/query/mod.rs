//! Query statements (select, insert, update & delete).
//!
//! # Usage
//!
//! - Query Select, see [`SelectStatement`]
//! - Query Insert, see [`InsertStatement`]
//! - Query Update, see [`UpdateStatement`]
//! - Query Delete, see [`DeleteStatement`]

mod case;
mod condition;
mod delete;
mod insert;
mod on_conflict;
mod ordered;
mod returning;
mod select;
mod traits;
mod update;
mod window;
mod with;

pub use case::*;
pub use condition::*;
pub use delete::*;
pub use insert::*;
pub use on_conflict::*;
pub use ordered::*;
pub use returning::*;
pub use select::*;
pub use traits::*;
pub use update::*;
pub use window::*;
pub use with::*;

/// Shorthand for constructing any table query
#[derive(Debug, Clone)]
pub struct Query;

/// All available types of table query
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum QueryStatement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum SubQueryStatement {
    SelectStatement(SelectStatement),
    InsertStatement(InsertStatement),
    UpdateStatement(UpdateStatement),
    DeleteStatement(DeleteStatement),
    WithStatement(WithQuery),
}

impl Query {
    /// Construct table [`SelectStatement`]
    #[must_use]
    pub fn select() -> SelectStatement {
        SelectStatement::new()
    }

    /// Construct table [`InsertStatement`]
    #[must_use]
    pub fn insert() -> InsertStatement {
        InsertStatement::new()
    }

    /// Construct table [`UpdateStatement`]
    #[must_use]
    pub fn update() -> UpdateStatement {
        UpdateStatement::new()
    }

    /// Construct table [`DeleteStatement`]
    #[must_use]
    pub fn delete() -> DeleteStatement {
        DeleteStatement::new()
    }

    /// Construct [`WithClause`]
    #[must_use]
    pub fn with() -> WithClause {
        WithClause::new()
    }

    /// Construct [`Returning`]
    #[must_use]
    pub fn returning() -> Returning {
        Returning::new()
    }
}
