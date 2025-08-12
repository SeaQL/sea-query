use darling::FromMeta;
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro::{self, TokenStream};
use quote::quote_spanned;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, parse_macro_input, spanned::Spanned};

use crate::iden::{GenEnumArgs, NamingHolder};

pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
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
        args.prefix
            .unwrap_or_else(|| crate::iden::DEFAULT_PREFIX.to_string()),
        &input.ident,
        args.suffix
            .unwrap_or_else(|| crate::iden::DEFAULT_SUFFIX.to_string())
    );
    let pascal_def_names = field_names.iter().map(|field| &field.pascal);
    let pascal_def_names2 = pascal_def_names.clone();
    let default_names = field_names.iter().map(|field| &field.default);
    let default_names2 = default_names.clone();
    let import_name = Ident::new(
        args.crate_name
            .unwrap_or_else(|| crate::iden::DEFAULT_CRATE_NAME.to_string())
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
