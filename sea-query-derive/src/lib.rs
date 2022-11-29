use std::convert::{TryFrom, TryInto};

use heck::ToSnakeCase;
use proc_macro::{self, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Attribute, DataEnum, DataStruct, DeriveInput, Fields};

mod error;
mod iden_attr;
mod iden_path;
mod iden_variant;

use self::{
    error::ErrorMsg,
    iden_attr::IdenAttr,
    iden_path::IdenPath,
    iden_variant::{DeriveIden, DeriveIdenStatic, IdenVariant},
};

fn find_attr(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs
        .iter()
        .find(|attr| attr.path.is_ident(&IdenPath::Iden) || attr.path.is_ident(&IdenPath::Method))
}

fn find_table_name(
    ident: &proc_macro2::Ident,
    attrs: Vec<Attribute>,
) -> Result<String, syn::Error> {
    let table_name = match find_attr(&attrs) {
        Some(att) => match att.try_into()? {
            IdenAttr::Rename(lit) => lit,
            _ => return Err(syn::Error::new_spanned(att, ErrorMsg::ContainerAttr)),
        },
        None => ident.to_string().to_snake_case(),
    };
    Ok(table_name)
}

#[proc_macro_derive(Iden, attributes(iden, method))]
pub fn derive_iden(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);
    let table_name = match find_table_name(&ident, attrs) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
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
        .map(IdenVariant::<DeriveIden>::try_from)
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(v) => quote! { #(#v),* },
        Err(e) => return e.to_compile_error().into(),
    };

    let output = quote! {
        impl sea_query::Iden for #ident {
            fn unquoted(&self, s: &mut dyn sea_query::Write) {
                match self {
                    #match_arms
                };
            }
        }
    };

    output.into()
}

#[proc_macro_derive(IdenStatic, attributes(iden, method))]
pub fn derive_iden_static(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);

    let table_name = match find_table_name(&ident, attrs) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
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
                    impl sea_query::IdenStatic for #ident {
                        fn as_str(&self) -> &'static str {
                            #table_name
                        }
                    }

                    impl std::convert::AsRef<str> for #ident {
                        fn as_ref(&self) -> &str {
                            self.as_str()
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
        .map(IdenVariant::<DeriveIdenStatic>::try_from)
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(v) => quote! { #(#v),* },
        Err(e) => return e.to_compile_error().into(),
    };

    let output = quote! {
        impl sea_query::IdenStatic for #ident {
            fn as_str(&self) -> &'static str {
                match self {
                    #match_arms
                }
            }
        }

        impl std::convert::AsRef<str> for #ident {
            fn as_ref(&self) -> &'static str {
                self.as_str()
            }
        }
    };

    output.into()
}
