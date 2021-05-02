//! Integration with different database drivers.

#[cfg(feature="postgres")]
mod postgres;
#[cfg(feature="postgres")]
pub use postgres::*;

#[cfg(feature="rusqlite")]
mod rusqlite;
#[cfg(feature="rusqlite")]
pub use rusqlite::*;

#[cfg(feature="sqlx-mysql")]
mod sqlx_mysql;
#[cfg(feature="sqlx-mysql")]
pub use sqlx_mysql::*;

#[cfg(feature="sqlx-postgres")]
mod sqlx_postgres;
#[cfg(feature="sqlx-postgres")]
pub use sqlx_postgres::*;