#![forbid(unsafe_code)]

use proc_macro::{self, TokenStream};
use syn::{DeriveInput, parse_macro_input};

mod enum_def;
mod iden;

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
