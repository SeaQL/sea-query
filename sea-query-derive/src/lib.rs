#![forbid(unsafe_code)]

use proc_macro::TokenStream;
#[cfg(feature = "sea-query")]
use syn::{DeriveInput, parse_macro_input};

#[cfg(feature = "sea-query")]
mod enum_def;
#[cfg(feature = "sea-query")]
mod iden;
mod raw_query;
mod raw_sql;
mod sqlx;

#[cfg(feature = "sea-query")]
#[proc_macro_derive(Iden, attributes(iden, method))]
pub fn derive_iden(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    iden::expand(input)
}

#[cfg(feature = "sea-query")]
#[proc_macro_derive(IdenStatic, attributes(iden, method))]
pub fn derive_iden_static(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    iden::iden_static::expand(input)
}

#[cfg(feature = "sea-query")]
#[proc_macro_attribute]
pub fn enum_def(args: TokenStream, input: TokenStream) -> TokenStream {
    enum_def::expand(args, input)
}

#[proc_macro]
pub fn raw_sql(input: TokenStream) -> TokenStream {
    match raw_sql::expand(input) {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[cfg(feature = "sea-query")]
#[proc_macro]
pub fn raw_query(input: TokenStream) -> TokenStream {
    match raw_query::expand(input) {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn sqlx_sqlite_query(input: TokenStream) -> TokenStream {
    match sqlx::sqlite_query(input) {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn sqlx_sqlite_query_as(input: TokenStream) -> TokenStream {
    match sqlx::sqlite_query_as(input) {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn sqlx_mysql_query(input: TokenStream) -> TokenStream {
    match sqlx::mysql_query(input) {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn sqlx_mysql_query_as(input: TokenStream) -> TokenStream {
    match sqlx::mysql_query_as(input) {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn sqlx_postgres_query(input: TokenStream) -> TokenStream {
    match sqlx::postgres_query(input) {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn sqlx_postgres_query_as(input: TokenStream) -> TokenStream {
    match sqlx::postgres_query_as(input) {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
