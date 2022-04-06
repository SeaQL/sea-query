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

/// Macro for generate rusqlite driver.
#[cfg(feature = "rusqlite")]
#[proc_macro]
pub fn sea_query_driver_rusqlite(input: TokenStream) -> TokenStream {
    rusqlite::sea_query_driver_rusqlite_impl(input)
}

/// Macro for generate new mod for sqlx-mysql.
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
