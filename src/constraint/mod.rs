//! Constraint definition
//!
//! # Usage
//!
//! - Table Constraint Create, see [`ConstraintCreateStatement`]
//! - Table Constraint Drop, see [`ConstraintDropStatement`]

mod common;
mod create;
mod drop;

pub use common::*;
pub use create::*;
pub use drop::*;

/// Shorthand for constructing constraint statement
#[derive(Debug, Clone)]
pub struct Constraint;

/// All available types of index statement
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ConstraintStatement {
    Create(ConstraintCreateStatement),
    /* Drop(ConstraintDropStatement), */
}

impl Constraint {
    /// Construct constraint [`ConstraintCreateStatement`]
    pub fn create() -> ConstraintCreateStatement {
        ConstraintCreateStatement::new()
    }

    /* /// Construct constraint [`ConstraintDropStatement`]
    pub fn drop() -> ConstraintDropStatement {
        ConstraintDropStatement::new()
    } */
}
