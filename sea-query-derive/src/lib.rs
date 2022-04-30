use std::convert::{TryFrom, TryInto};

use heck::ToSnakeCase;
use proc_macro::{self, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Attribute, DataEnum, DataStruct, DeriveInput, Fields};

mod error;
mod iden_attr;
mod iden_path;
mod iden_variant;

use self::{error::ErrorMsg, iden_attr::IdenAttr, iden_path::IdenPath, iden_variant::IdenVariant};

fn find_attr(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs
        .iter()
        .find(|attr| attr.path.is_ident(&IdenPath::Iden) || attr.path.is_ident(&IdenPath::Method))
}

#[proc_macro_derive(Iden, attributes(iden, method))]
pub fn derive_iden(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);

    let table_name = match find_attr(&attrs) {
        Some(att) => match att.try_into() {
            Ok(IdenAttr::Rename(lit)) => lit,
            Ok(_) => {
                return syn::Error::new_spanned(att, ErrorMsg::ContainerAttr)
                    .into_compile_error()
                    .into()
            }
            Err(e) => return e.into_compile_error().into(),
        },
        None => ident.to_string().to_snake_case(),
    };

    // Currently we only support enums and unit structs
    let variants =
        match data {
            syn::Data::Enum(DataEnum { variants, .. }) => variants,
            syn::Data::Struct(DataStruct {
                fields: Fields::Unit,
                ..
            }) => {
                return quote! {
                    impl sea_query::Iden for #ident {
                        fn unquoted(&self, s: &mut dyn sea_query::Write) {
                            write!(s, #table_name).unwrap();
                        }
                    }
                }
                .into()
            }
            _ => return quote_spanned! {
                ident.span() => compile_error!("you can only derive Iden on enums or unit structs");
            }
            .into(),
        };

    if variants.is_empty() {
        return TokenStream::new();
    }

    let match_arms = match variants
        .iter()
        .map(|v| (table_name.as_str(), v))
        .map(IdenVariant::try_from)
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let output = quote! {
        impl sea_query::Iden for #ident {
            fn unquoted(&self, s: &mut dyn sea_query::Write) {
                match self {
                    #(#match_arms),*
                };
            }
        }
    };

    output.into()
}
