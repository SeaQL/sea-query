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

mod kw {
    syn::custom_keyword!(rusqlite);
    syn::custom_keyword!(sqlx);
    syn::custom_keyword!(sea_query);
}

struct DriverArg(syn::Path);

impl parse::Parse for DriverArg {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let _: syn::Ident = input.parse()?;
        let _: token::Eq = input.parse()?;
        let s: syn::LitStr = input.parse()?;
        let value: syn::Path = syn::parse_str(&s.value())?;
        Ok(DriverArg(value))
    }
}

macro_rules! args_parse_impl {
    ($ident:ident, $driver_ident:path) => {
        pub struct $ident {
            pub(crate) driver: Option<syn::Path>,
            pub(crate) sea_query: Option<syn::Path>,
        }

        impl parse::Parse for $ident {
            fn parse(input: parse::ParseStream) -> parse::Result<Self> {
                let mut args = $ident {
                    driver: None,
                    sea_query: None,
                };
                let lookahead = input.lookahead1();

                if lookahead.peek($driver_ident) {
                    let arg: DriverArg = input.parse()?;
                    args.driver = Some(arg.0)
                } else if lookahead.peek(kw::sea_query) {
                    let arg: DriverArg = input.parse()?;
                    args.sea_query = Some(arg.0)
                } else {
                    return Ok(args);
                };
                let comma: Option<token::Comma> = input.parse()?;
                if comma.is_none() {
                    return Ok(args);
                }
                if lookahead.peek($driver_ident) {
                    let arg: DriverArg = input.parse()?;
                    args.driver = Some(arg.0)
                } else if lookahead.peek(kw::sea_query) {
                    let arg: DriverArg = input.parse()?;
                    args.sea_query = Some(arg.0)
                };
                Ok(args)
            }
        }
    };
}

args_parse_impl!(RusqliteDriverArgs, kw::rusqlite);
args_parse_impl!(SqlxDriverArgs, kw::sqlx);
