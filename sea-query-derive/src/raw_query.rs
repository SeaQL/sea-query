use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
};

struct CallArgs {
    builder: Ident,
    _comma: Token![,],
    sql_string: LitStr,
}

impl Parse for CallArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(CallArgs {
            builder: input.parse()?,
            _comma: input.parse()?,
            sql_string: input.parse()?,
        })
    }
}

pub fn expand(input: proc_macro::TokenStream) -> syn::Result<TokenStream> {
    let CallArgs {
        builder,
        sql_string,
        ..
    } = syn::parse(input)?;

    let builder = match builder.to_string().as_str() {
        "PostgresQueryBuilder" => quote!(postgres),
        "MysqlQueryBuilder" => quote!(mysql),
        "SqliteQueryBuilder" => quote!(sqlite),
        _ => quote!(#builder),
    };

    Ok(quote!(
        sea_query::raw_sql!(seaql::#builder::query, #sql_string)
    ))
}
