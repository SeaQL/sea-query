pub(crate) mod attr;
pub(crate) mod error;
pub(crate) mod path;
pub(crate) mod write_arm;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::sea_query_path;

use self::write_arm::WriteArm;

pub(crate) struct DeriveIden;

impl WriteArm for DeriveIden {
    fn variant(variant: TokenStream, name: TokenStream) -> TokenStream {
        quote! { Self::#variant => write!(s, "{}", #name).unwrap() }
    }

    fn flattened(variant: TokenStream, name: &Ident) -> TokenStream {
        quote! { Self::#variant => #name.unquoted(s) }
    }
}

pub(crate) struct DeriveIdenStatic;

impl WriteArm for DeriveIdenStatic {
    fn variant(variant: TokenStream, name: TokenStream) -> TokenStream {
        quote! { Self::#variant => #name }
    }

    fn flattened(variant: TokenStream, name: &Ident) -> TokenStream {
        quote! { Self::#variant => #name.as_str() }
    }
}

pub(crate) struct DeriveIdenImpl;

impl WriteArm for DeriveIdenImpl {
    fn variant(variant: TokenStream, name: TokenStream) -> TokenStream {
        let sea_query_path = sea_query_path();
        quote! { #variant => #sea_query_path::IdenImpl::from(#name) }
    }

    fn flattened(variant: TokenStream, name: &Ident) -> TokenStream {
        let sea_query_path = sea_query_path();
        quote! { #variant => #sea_query_path::IdenImpl::from(#name) }
    }
}
