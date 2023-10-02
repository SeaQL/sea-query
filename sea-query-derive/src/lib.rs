use std::convert::{TryFrom, TryInto};

use heck::ToSnakeCase;
use proc_macro::{self, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Attribute, DataEnum, DataStruct, DeriveInput, Fields, Variant};

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
    attrs.iter().find(|attr| {
        attr.path().is_ident(&IdenPath::Iden) || attr.path().is_ident(&IdenPath::Method)
    })
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

fn must_be_valid_iden(name: &str) -> bool {
    // can only begin with [a-z_]
    name.chars()
        .take(1)
        .all(|c| c == '_' || c.is_ascii_alphabetic())
        && name.chars().all(|c| c == '_' || c.is_ascii_alphanumeric())
}

fn impl_iden_for_unit_struct(
    ident: &proc_macro2::Ident,
    table_name: &str,
) -> proc_macro2::TokenStream {
    let sea_query_path = sea_query_path();

    let prepare = if must_be_valid_iden(table_name) {
        quote! {
            fn prepare(&self, s: &mut dyn ::std::fmt::Write, q: #sea_query_path::Quote) {
                write!(s, "{}", q.left()).unwrap();
                self.unquoted(s);
                write!(s, "{}", q.right()).unwrap();
            }
        }
    } else {
        quote! {}
    };

    quote! {
        impl #sea_query_path::Iden for #ident {
            #prepare

            fn unquoted(&self, s: &mut dyn ::std::fmt::Write) {
                write!(s, #table_name).unwrap();
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

    let mut is_all_valid = true;

    let match_arms = match variants
        .map(|v| (table_name, v))
        .map(|v| {
            let v = IdenVariant::<DeriveIden>::try_from(v)?;
            is_all_valid &= v.must_be_valid_iden();
            Ok(v)
        })
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(v) => quote! { #(#v),* },
        Err(e) => return e.to_compile_error(),
    };

    let prepare = if is_all_valid {
        quote! {
            fn prepare(&self, s: &mut dyn ::std::fmt::Write, q: #sea_query_path::Quote) {
                write!(s, "{}", q.left()).unwrap();
                self.unquoted(s);
                write!(s, "{}", q.right()).unwrap();
            }
        }
    } else {
        quote! {}
    };

    quote! {
        impl #sea_query_path::Iden for #ident {
            #prepare

            fn unquoted(&self, s: &mut dyn ::std::fmt::Write) {
                match self {
                    #match_arms
                };
            }
        }
    }
}

#[proc_macro_derive(Iden, attributes(iden, method))]
pub fn derive_iden(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);
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

    let output = impl_iden_for_enum(&ident, &table_name, variants.iter());

    output.into()
}

#[proc_macro_derive(IdenStatic, attributes(iden, method))]
pub fn derive_iden_static(input: TokenStream) -> TokenStream {
    let sea_query_path = sea_query_path();

    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);

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
            }) => {
                let impl_iden = impl_iden_for_unit_struct(&ident, &table_name);

                return quote! {
                    #impl_iden

                    impl #sea_query_path::IdenStatic for #ident {
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
                .into();
            }
            _ => return quote_spanned! {
                ident.span() => compile_error!("you can only derive Iden on enums or unit structs");
            }
            .into(),
        };

    if variants.is_empty() {
        return TokenStream::new();
    }

    let impl_iden = impl_iden_for_enum(&ident, &table_name, variants.iter());

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
        #impl_iden

        impl #sea_query_path::IdenStatic for #ident {
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

fn sea_query_path() -> proc_macro2::TokenStream {
    if cfg!(feature = "sea-orm") {
        quote!(sea_orm::sea_query)
    } else {
        quote!(sea_query)
    }
}
