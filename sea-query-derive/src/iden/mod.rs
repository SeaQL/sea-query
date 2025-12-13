mod attr;
mod error;
pub mod iden_static;
mod path;
mod write_arm;

use darling::FromMeta;
use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use std::convert::{TryFrom, TryInto};
use syn::{Attribute, DataEnum, DataStruct, DeriveInput, Fields, Ident, Variant};

use attr::IdenAttr;
use error::ErrorMsg;
use path::IdenPath;
use write_arm::IdenVariant;

struct DeriveIden;
struct DeriveIdenStatic;

pub fn expand(input: DeriveInput) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = input;

    let table_name = match get_table_name(&ident, attrs) {
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
            }) => return impl_iden_for_unit_struct(&ident, &table_name).into(),
            _ => return quote_spanned! {
                ident.span() => compile_error!("you can only derive Iden on enums or unit structs");
            }
            .into(),
        };

    if variants.is_empty() {
        return TokenStream::new();
    }

    let can_be_static = variants.iter().all(|v| {
        let variant = IdenVariant::<DeriveIden>::try_from((table_name.as_str(), v));
        match variant {
            Ok(v) => v.can_be_static(),
            Err(_) => false,
        }
    });

    let output = if can_be_static {
        impl_iden_static_for_enum(&ident, &table_name, variants.iter())
    } else {
        impl_iden_for_enum(&ident, &table_name, variants.iter())
    };

    output.into()
}

fn impl_iden_for_unit_struct(
    ident: &proc_macro2::Ident,
    table_name: &str,
) -> proc_macro2::TokenStream {
    let sea_query_path = sea_query_path();
    quote! {
        impl #sea_query_path::IdenStatic for #ident {
            fn as_str(&self) -> &'static str {
                #table_name
            }
        }
    }
}

fn impl_iden_for_enum<'a, T>(
    ident: &proc_macro2::Ident,
    table_name: &str,
    variants: T,
) -> proc_macro2::TokenStream
where
    T: Iterator<Item = &'a Variant>,
{
    let sea_query_path = sea_query_path();

    let match_arms = match variants
        .map(|v| IdenVariant::<DeriveIden>::try_from((table_name, v)))
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    quote! {
        impl #sea_query_path::Iden for #ident {
            fn unquoted(&self) -> std::borrow::Cow<'static, str> {
                match self {
                    #(#match_arms),*
                }
            }
        }
    }
}

fn impl_iden_static_for_enum<'a, T>(
    ident: &proc_macro2::Ident,
    table_name: &str,
    variants: T,
) -> proc_macro2::TokenStream
where
    T: Iterator<Item = &'a Variant>,
{
    let sea_query_path = sea_query_path();

    let match_arms = match variants
        .map(|v| IdenVariant::<DeriveIdenStatic>::try_from((table_name, v)))
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    quote! {
        impl #sea_query_path::IdenStatic for #ident {
            fn as_str(&self) -> &'static str {
                match self {
                    #(#match_arms),*
                }
            }
        }
    }
}

fn sea_query_path() -> proc_macro2::TokenStream {
    if cfg!(feature = "sea-orm") {
        quote!(sea_orm::sea_query)
    } else {
        quote!(sea_query)
    }
}

pub struct NamingHolder {
    pub default: Ident,
    pub pascal: Ident,
}

#[derive(Debug, FromMeta)]
pub struct GenEnumArgs {
    #[darling(default)]
    pub prefix: Option<String>,
    #[darling(default)]
    pub suffix: Option<String>,
    #[darling(default)]
    pub crate_name: Option<String>,
    #[darling(default)]
    pub table_name: Option<String>,
}

pub const DEFAULT_PREFIX: &str = "";
pub const DEFAULT_SUFFIX: &str = "Iden";
pub const DEFAULT_CRATE_NAME: &str = "sea_query";

impl Default for GenEnumArgs {
    fn default() -> Self {
        Self {
            prefix: Some(DEFAULT_PREFIX.to_string()),
            suffix: Some(DEFAULT_SUFFIX.to_string()),
            crate_name: Some(DEFAULT_CRATE_NAME.to_string()),
            table_name: None,
        }
    }
}

fn get_table_name(ident: &proc_macro2::Ident, attrs: Vec<Attribute>) -> Result<String, syn::Error> {
    let table_name = match find_attr(&attrs) {
        Some(att) => match att.try_into()? {
            IdenAttr::Rename(lit) => lit,
            _ => return Err(syn::Error::new_spanned(att, ErrorMsg::ContainerAttr)),
        },
        None => ident.to_string().to_snake_case(),
    };
    Ok(table_name)
}

fn find_attr(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| {
        attr.path().is_ident(&IdenPath::Iden) || attr.path().is_ident(&IdenPath::Method)
    })
}
