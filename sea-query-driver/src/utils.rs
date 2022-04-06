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

pub struct DriverArgs {
    pub(crate) path: Option<syn::TypePath>,
}

impl parse::Parse for DriverArgs {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Ident) {
            input.parse().map(|path| Self { path: Some(path) })
        } else {
            Ok(Self { path: None })
        }
    }
}

impl quote::ToTokens for DriverArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(path) = &self.path {
            path.to_tokens(tokens)
        }
    }
}
