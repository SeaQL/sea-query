use syn::{parse, token};

pub struct Args {
    pub(crate) query: syn::Expr,
    pub(crate) params: syn::Expr,
}

impl parse::Parse for Args {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let query: syn::Expr = input.parse()?;
        let _: token::Comma = input.parse()?;
        let params: syn::Expr = input.parse()?;

        Ok(Args { query, params })
    }
}
