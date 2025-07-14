use std::convert::{TryFrom, TryInto};

use darling::FromMeta;
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro::{self, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, Ident, Variant, parse_macro_input,
    spanned::Spanned,
};

mod iden;

use self::iden::{
    DeriveIden, DeriveIdenStatic, attr::IdenAttr, error::ErrorMsg, path::IdenPath,
    write_arm::IdenVariant,
};

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
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let output = quote! {
        #impl_iden

        impl #sea_query_path::IdenStatic for #ident {
            fn as_str(&self) -> &'static str {
                match self {
                    #(#match_arms),*
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

fn is_static_iden(name: &str) -> bool {
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

    if is_static_iden(table_name) {
        quote! {
            impl #sea_query_path::Iden for #ident {
                fn quoted(&self) -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::Borrowed(#table_name)
                }

                fn unquoted(&self) -> &str {
                    #table_name
                }
            }
        }
    } else {
        quote! {
            impl #sea_query_path::Iden for #ident {
                fn unquoted(&self) -> &str {
                    #table_name
                }
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

    let mut is_all_static_iden = true;

    let match_arms = match variants
        .map(|v| {
            let v = IdenVariant::<DeriveIden>::try_from((table_name, v))?;
            is_all_static_iden &= v.is_static_iden();
            Ok(v)
        })
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    if is_all_static_iden {
        quote! {
            impl #sea_query_path::Iden for #ident {
                fn quoted(&self) -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::Borrowed(match self {
                        #(#match_arms),*
                    })
                }

                fn unquoted(&self) -> &str {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        }
    } else {
        quote! {
            impl #sea_query_path::Iden for #ident {
                fn unquoted(&self) -> &str {
                    match self {
                        #(#match_arms),*
                    }
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

struct NamingHolder {
    pub default: Ident,
    pub pascal: Ident,
}

#[derive(Debug, FromMeta)]
struct GenEnumArgs {
    #[darling(default)]
    pub prefix: Option<String>,
    #[darling(default)]
    pub suffix: Option<String>,
    #[darling(default)]
    pub crate_name: Option<String>,
    #[darling(default)]
    pub table_name: Option<String>,
}

const DEFAULT_PREFIX: &str = "";
const DEFAULT_SUFFIX: &str = "Iden";
const DEFAULT_CRATE_NAME: &str = "sea_query";

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

#[proc_macro_attribute]
pub fn enum_def(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match darling::ast::NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let input = parse_macro_input!(input as DeriveInput);

    let args = match GenEnumArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let fields =
        match &input.data {
            Data::Struct(DataStruct {
                fields: Fields::Named(fields),
                ..
            }) => &fields.named,
            _ => return quote_spanned! {
                input.span() => compile_error!("#[enum_def] can only be used on non-tuple structs");
            }
            .into(),
        };

    let field_names: Vec<NamingHolder> = fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            NamingHolder {
                default: ident.clone(),
                pascal: Ident::new(ident.to_string().to_pascal_case().as_str(), ident.span()),
            }
        })
        .collect();

    let table_name = Ident::new(
        args.table_name
            .unwrap_or_else(|| input.ident.to_string().to_snake_case())
            .as_str(),
        input.ident.span(),
    );

    let enum_name = quote::format_ident!(
        "{}{}{}",
        args.prefix.unwrap_or_else(|| DEFAULT_PREFIX.to_string()),
        &input.ident,
        args.suffix.unwrap_or_else(|| DEFAULT_SUFFIX.to_string())
    );
    let pascal_def_names = field_names.iter().map(|field| &field.pascal);
    let pascal_def_names2 = pascal_def_names.clone();
    let default_names = field_names.iter().map(|field| &field.default);
    let default_names2 = default_names.clone();
    let import_name = Ident::new(
        args.crate_name
            .unwrap_or_else(|| DEFAULT_CRATE_NAME.to_string())
            .as_str(),
        input.span(),
    );

    TokenStream::from(quote::quote! {
        #input

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum #enum_name {
            Table,
            #(#pascal_def_names,)*
        }

        impl #import_name::IdenStatic for #enum_name {
            fn as_str(&self) -> &'static str {
                match self {
                    #enum_name::Table => stringify!(#table_name),
                    #(#enum_name::#pascal_def_names2 => stringify!(#default_names2)),*
                }
            }
        }

        impl #import_name::Iden for #enum_name {
            fn unquoted(&self) -> &str {
                <Self as #import_name::IdenStatic>::as_str(&self)
            }
        }

        impl ::std::convert::AsRef<str> for #enum_name {
            fn as_ref(&self) -> &str {
                <Self as #import_name::IdenStatic>::as_str(&self)
            }
        }
    })
}
