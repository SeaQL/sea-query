use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct CallArgs {
    sql_holder: Ident,
    _assign: Token![=],
    sql_string: LitStr,
}

impl Parse for CallArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(CallArgs {
            sql_holder: input.parse()?,
            _assign: input.parse()?,
            sql_string: input.parse()?,
        })
    }
}

pub fn sqlite_query(input: proc_macro::TokenStream) -> syn::Result<TokenStream> {
    let CallArgs {
        sql_holder,
        sql_string,
        ..
    } = syn::parse(input)?;

    Ok(quote!(
        sea_query::raw_sql!(sqlx::sqlite::query, #sql_holder = #sql_string)
    ))
}

pub fn sqlite_query_as(input: proc_macro::TokenStream) -> syn::Result<TokenStream> {
    let CallArgs {
        sql_holder,
        sql_string,
        ..
    } = syn::parse(input)?;

    Ok(quote!(
        sea_query::raw_sql!(sqlx::sqlite::query_as, #sql_holder = #sql_string)
    ))
}

pub fn mysql_query(input: proc_macro::TokenStream) -> syn::Result<TokenStream> {
    let CallArgs {
        sql_holder,
        sql_string,
        ..
    } = syn::parse(input)?;

    Ok(quote!(
        sea_query::raw_sql!(sqlx::mysql::query, #sql_holder = #sql_string)
    ))
}

pub fn mysql_query_as(input: proc_macro::TokenStream) -> syn::Result<TokenStream> {
    let CallArgs {
        sql_holder,
        sql_string,
        ..
    } = syn::parse(input)?;

    Ok(quote!(
        sea_query::raw_sql!(sqlx::mysql::query_as, #sql_holder = #sql_string)
    ))
}

pub fn postgres_query(input: proc_macro::TokenStream) -> syn::Result<TokenStream> {
    let CallArgs {
        sql_holder,
        sql_string,
        ..
    } = syn::parse(input)?;

    Ok(quote!(
        sea_query::raw_sql!(sqlx::postgres::query, #sql_holder = #sql_string)
    ))
}

pub fn postgres_query_as(input: proc_macro::TokenStream) -> syn::Result<TokenStream> {
    let CallArgs {
        sql_holder,
        sql_string,
        ..
    } = syn::parse(input)?;

    Ok(quote!(
        sea_query::raw_sql!(sqlx::postgres::query_as, #sql_holder = #sql_string)
    ))
}
