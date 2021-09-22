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

use crate::{MySqlQueryBuilder, PostgresQueryBuilder, QueryBuilder, SqliteQueryBuilder};

pub trait Queryable<DB: QueryBuilder<DB> + Default> {
    /// Construct table [`SelectStatement`]
    fn select<'a>() -> SelectStatement<'a, DB> {
        SelectStatement::<'a, DB>::new()
    }

    /// Construct table [`InsertStatement`]
    fn insert<'a>() -> InsertStatement<'a, DB> {
        InsertStatement::<'a, DB>::new()
    }

    /// Construct table [`UpdateStatement`]
    fn update<'a>() -> UpdateStatement<'a, DB> {
        UpdateStatement::<'a, DB>::new()
    }

    /// Construct table [`DeleteStatement`]
    fn delete<'a>() -> DeleteStatement<'a, DB> {
        DeleteStatement::<'a, DB>::new()
    }
}

/// All available types of table query
#[derive(Debug, Clone)]
pub enum QueryStatement<'a, DB> {
    Select(SelectStatement<'a, DB>),
    Insert(InsertStatement<'a, DB>),
    Update(UpdateStatement<'a, DB>),
    Delete(DeleteStatement<'a, DB>),
}

/// Shorthand for constructing any table query
#[derive(Debug, Clone)]
pub struct Query;

impl Queryable<MySqlQueryBuilder> for Query {}
impl Queryable<PostgresQueryBuilder> for Query {}
impl Queryable<SqliteQueryBuilder> for Query {}

#[derive(Debug, Clone)]
#[cfg(feature = "backend-mysql")]
pub struct MySqlQuery;

#[cfg(feature = "backend-mysql")]
impl Queryable<MySqlQueryBuilder> for MySqlQuery {}

#[derive(Debug, Clone)]
#[cfg(feature = "backend-postgres")]
pub struct PgQuery;

#[cfg(feature = "backend-postgres")]
impl Queryable<PostgresQueryBuilder> for PgQuery {}

#[derive(Debug, Clone)]
#[cfg(feature = "backend-sqlite")]
pub struct SqliteQuery;

#[cfg(feature = "backend-sqlite")]
impl Queryable<SqliteQueryBuilder> for SqliteQuery {}
