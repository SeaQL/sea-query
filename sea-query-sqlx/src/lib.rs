#![forbid(unsafe_code)]

//! Driver library for using SeaQuery with SQLx
//!
//! This library introduces various traits that add methods to the query types from `sea-query`.
//! For instance, using the [`SqlxBinder`] trait adds a [`SqlxBinder::build_sqlx`] method that
//! returns the query and a [`SqlxValues`] object, which can be directly passed to `sqlx`'s
//! [`sqlx::query_with`][1] method.
//!
//! MySQL does not support Jiff values. Binding them causes a runtime error.
//!
//! [1]: ../sqlx/fn.query_with.html

#[cfg(feature = "sqlx-any")]
mod sqlx_any;
#[cfg(feature = "sqlx-mysql")]
mod sqlx_mysql;
#[cfg(feature = "sqlx-postgres")]
mod sqlx_postgres;
#[cfg(feature = "sqlx-sqlite")]
mod sqlx_sqlite;

mod values;
pub use crate::values::*;

mod sqlx;
pub use crate::sqlx::SqlxBinder;
