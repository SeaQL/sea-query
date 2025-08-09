#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod enum_def;
mod iden;
mod raw_sql;

#[proc_macro_derive(Iden, attributes(iden, method))]
pub fn derive_iden(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    iden::expand(input)
}

#[proc_macro_derive(IdenStatic, attributes(iden, method))]
pub fn derive_iden_static(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    iden::iden_static::expand(input)
}

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
