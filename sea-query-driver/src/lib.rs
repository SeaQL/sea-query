#[allow(unused_imports)]
use proc_macro::{self, TokenStream};

#[cfg(feature = "rusqlite")]
mod rusqlite;
#[cfg(feature = "sqlx-mysql")]
mod sqlx_mysql;
#[cfg(feature = "sqlx-postgres")]
mod sqlx_postgres;
#[cfg(feature = "sqlx-sqlite")]
mod sqlx_sqlite;
#[cfg(any(
    feature = "rusqlite",
    feature = "sqlx-mysql",
    feature = "sqlx-postgres",
    feature = "sqlx-sqlite"
))]
mod utils;

/// Macro for generate `rusqlite` driver.
///
/// Examples:
/// ```
/// sea_query::sea_query_driver_rusqlite!()
/// ```
///
/// Specify a path to the `rusqlite` crate instance
/// ```
/// sea_query::sea_query_driver_rusqlite!(rusqlite = "...")
/// ```
///
/// Specify a path to the `sea-query` crate instance
/// ```
/// sea_query::sea_query_driver_rusqlite!(sea_query = "...")
/// ```
///
/// Specify paths to the `sea-query` and to the `rusqlite` crates instances
/// ```
/// sea_query::sea_query_driver_rusqlite!(rusqlite="...", sea_query="...")
/// // or
/// sea_query::sea_query_driver_rusqlite!(sea_query="...", rusqlite="...")
/// ```
#[cfg(feature = "rusqlite")]
#[proc_macro]
pub fn sea_query_driver_rusqlite(input: TokenStream) -> TokenStream {
    rusqlite::sea_query_driver_rusqlite_impl(input)
}

/// Macro for generate new mod for sqlx-mysql.
///
/// Examples:
/// ```
/// sea_query::sea_query_driver_mysql!()
/// ```
///
/// Specify a path to the `sqlx` crate instance
/// ```
/// sea_query::sea_query_driver_mysql!(sqlx = "...")
/// ```
///
/// Specify a path to the `sea-query` crate instance
/// ```
/// sea_query::sea_query_driver_mysql!(sea_query = "...")
/// ```
///
/// Specify pathes to the `sea-query` and to the `sqlx` crates instances
/// ```
/// sea_query::sea_query_driver_mysql!(sqlx="...", sea_query="...")
/// // or
/// sea_query::sea_query_driver_mysql!(sea_query="...", sqlx="...")
/// ```
#[cfg(feature = "sqlx-mysql")]
#[proc_macro]
pub fn sea_query_driver_mysql(input: TokenStream) -> TokenStream {
    sqlx_mysql::sea_query_driver_mysql_impl(input)
}

/// Macro to easily bind [`Values`] to [`sqlx::query::Query`] or to [`sqlx::query::QueryAs`] for sqlx-mysql.
#[cfg(feature = "sqlx-mysql")]
#[proc_macro]
pub fn bind_params_sqlx_mysql(input: TokenStream) -> TokenStream {
    sqlx_mysql::bind_params_sqlx_mysql_impl(input)
}

/// Macro to generate sqlx-postgres driver.
///
/// Examples:
/// ```
/// sea_query::sea_query_driver_postgres!()
/// ```
///
/// Specify a path to the `sqlx` crate instance
/// ```
/// sea_query::sea_query_driver_postgres!(sqlx = "...")
/// ```
///
/// Specify a path to the `sea-query` crate instance
/// ```
/// sea_query::sea_query_driver_postgres!(sea_query = "...")
/// ```
///
/// Specify pathes to the `sea-query` and to the `sqlx` crates instances
/// ```
/// sea_query::sea_query_driver_postgres!(sqlx="...", sea_query="...")
/// // or
/// sea_query::sea_query_driver_postgres!(sea_query="...", sqlx="...")
/// ```
#[cfg(feature = "sqlx-postgres")]
#[proc_macro]
pub fn sea_query_driver_postgres(input: TokenStream) -> TokenStream {
    sqlx_postgres::sea_query_driver_postgres_impl(input)
}

/// Macro to easily bind [`Values`] to [`sqlx::query::Query`] or to [`sqlx::query::QueryAs`] for sqlx-postgres.
#[cfg(feature = "sqlx-postgres")]
#[proc_macro]
pub fn bind_params_sqlx_postgres(input: TokenStream) -> TokenStream {
    sqlx_postgres::bind_params_sqlx_postgres_impl(input)
}

/// Macro to generate sqlx-sqlite driver.
///
/// Examples:
/// ```
/// sea_query::sea_query_driver_sqlite!()
/// ```
///
/// Specify a path to the `sqlx` crate instance
/// ```
/// sea_query::sea_query_driver_sqlite!(sqlx = "...")
/// ```
///
/// Specify a path to the `sea-query` crate instance
/// ```
/// sea_query::sea_query_driver_sqlite!(sea_query = "...")
/// ```
///
/// Specify pathes to the `sea-query` and to the `sqlx` crates instances
/// ```
/// sea_query::sea_query_driver_sqlite!(sqlx="...", sea_query="...")
/// // or
/// sea_query::sea_query_driver_sqlite!(sea_query="...", sqlx="...")
/// ```
#[cfg(feature = "sqlx-sqlite")]
#[proc_macro]
pub fn sea_query_driver_sqlite(input: TokenStream) -> TokenStream {
    sqlx_sqlite::sea_query_driver_sqlite_impl(input)
}

/// Macro to easily bind [`Values`] to [`sqlx::query::Query`] or to [`sqlx::query::QueryAs`] for sqlx-sqlite.
#[cfg(feature = "sqlx-sqlite")]
#[proc_macro]
pub fn bind_params_sqlx_sqlite(input: TokenStream) -> TokenStream {
    sqlx_sqlite::bind_params_sqlx_sqlite(input)
}
