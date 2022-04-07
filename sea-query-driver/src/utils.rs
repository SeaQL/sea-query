use proc_macro2::TokenStream;
use syn::{parse, token};

pub struct BindParamArgs {
    pub(crate) query: syn::Expr,
    pub(crate) params: syn::Expr,
}

impl parse::Parse for BindParamArgs {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let query: syn::Expr = input.parse()?;
        let _: token::Comma = input.parse()?;
        let params: syn::Expr = input.parse()?;

        Ok(BindParamArgs { query, params })
    }
}

fn parse_option_path(input: &parse::ParseStream) -> parse::Result<Option<syn::TypePath>> {
    let lookahead = input.lookahead1();
    if lookahead.peek(syn::Ident) {
        input.parse().map(|path| Some(path))
    } else {
        Ok(None)
    }
}

pub struct DriverArgs {
    pub(crate) driver: Option<syn::TypePath>,
    pub(crate) sea_query: Option<syn::TypePath>,
}

impl parse::Parse for DriverArgs {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let driver = parse_option_path(&input)?;
        if driver.is_none() {
            return Ok(DriverArgs {
                driver,
                sea_query: None,
            });
        }
        let _: Option<token::Comma> = input.parse()?;
        let sea_query = parse_option_path(&input)?;

        Ok(DriverArgs { driver, sea_query })
    }
}
