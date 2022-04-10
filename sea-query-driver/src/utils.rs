use proc_macro2::TokenStream;
use syn::{parse, token, Token};

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

mod kw {
    syn::custom_keyword!(driver);
    syn::custom_keyword!(sea_query);
}

enum DriverArgument {
    Driver(syn::Path),
    SeaQuery(syn::Path),
}

impl parse::Parse for DriverArgument {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::driver) {
            let _: kw::driver = input.parse()?;
            let _: token::Eq = input.parse()?;
            let s: syn::LitStr = input.parse()?;
            let value: syn::Path = syn::parse_str(&s.value())?;
            Ok(DriverArgument::Driver(value))
        } else if lookahead.peek(kw::sea_query) {
            let _: kw::sea_query = input.parse()?;
            let _: token::Eq = input.parse()?;
            let s: syn::LitStr = input.parse()?;
            let value: syn::Path = syn::parse_str(&s.value())?;
            Ok(DriverArgument::SeaQuery(value))
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct DriverArgs {
    pub(crate) driver: Option<syn::Path>,
    pub(crate) sea_query: Option<syn::Path>,
}

impl parse::Parse for DriverArgs {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let mut args = DriverArgs {
            driver: None,
            sea_query: None,
        };
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Ident) {
            let arg: DriverArgument = input.parse()?;
            match arg {
                DriverArgument::Driver(p) => args.driver = Some(p),
                DriverArgument::SeaQuery(p) => args.sea_query = Some(p),
            }
        }
        let comma: Option<token::Comma> = input.parse()?;
        if comma.is_some() {
            let arg: DriverArgument = input.parse()?;
            match arg {
                DriverArgument::Driver(p) => args.driver = Some(p),
                DriverArgument::SeaQuery(p) => args.sea_query = Some(p),
            }
        }
        Ok(args)
    }
}
