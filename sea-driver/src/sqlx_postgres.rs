use proc_macro::TokenStream;
use quote::quote;

pub fn sea_query_driver_postgres_impl() -> TokenStream {
    let output = quote! {
        mod sea_query_driver_postgres {
            use sqlx::{postgres::PgArguments, Postgres};
            use sea_query::{Value, Values};

            type SqlxQuery<'a> = sqlx::query::Query<'a, Postgres, PgArguments>;
            type SqlxQueryAs<'a, T> = sqlx::query::QueryAs<'a, Postgres, T, PgArguments>;

            pub fn bind_query<'a>(query: SqlxQuery<'a>, params: &'a Values) -> SqlxQuery<'a> {
                sea_query::bind_params_sqlx_postgres!(query, params.0)
            }

            pub fn bind_query_as<'a, T>(
                query: SqlxQueryAs<'a, T>,
                params: &'a Values,
            ) -> SqlxQueryAs<'a, T> {
                sea_query::bind_params_sqlx_postgres!(query, params.0)
            }
        }
    };
    output.into()
}
