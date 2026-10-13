#![forbid(unsafe_code)]

//! PostgreSQL-only SQLx bindings for SeaQuery.
//!
//! This crate mirrors the Postgres parts of `sea-query-sqlx`, but its manifest does not mention
//! the top-level `sqlx` facade or the MySQL/SQLite SQLx driver crates. That gives Postgres-only
//! users a dependency that does not record unrelated SQLx optional packages in their lockfile.

mod postgres;
mod sqlx;
mod values;

pub use crate::sqlx::SqlxBinder;
pub use crate::values::SqlxValues;
