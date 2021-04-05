use heck::SnakeCase;
use proc_macro::{self, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Attribute, DataEnum, DataStruct, DeriveInput, Fields, Meta, Variant};

fn get_iden_attr(attrs: &[Attribute]) -> Option<syn::Lit> {
    for attr in attrs {
        let name_value = match attr.parse_meta() {
            Ok(Meta::NameValue(nv)) => nv,
            _ => continue,
        };
        if name_value.path.is_ident("iden") {
            return Some(name_value.lit);
        }
    }
    None
}

#[proc_macro_derive(Iden, attributes(iden))]
pub fn derive_iden(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);

    let table_name = match get_iden_attr(&attrs) {
        Some(lit) => quote! { #lit },
        None => {
            let normalized = ident.to_string().to_snake_case();
            quote! { #normalized }
        }
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
            },
            _ => return quote_spanned! {
                ident.span() => compile_error!("you can only derive Iden on enums or unit structs");
            }.into(),
        };

    if variants.is_empty() {
        return TokenStream::new();
    }

    let variant = variants
        .iter()
        .map(|Variant { ident, fields, .. }| {
            match fields {
                Fields::Named(_) => quote! { #ident{..} },
                Fields::Unnamed(_) => quote! { #ident(..) },
                Fields::Unit => quote! { #ident },
            }
        });

    let name = variants.iter().map(|v| {
        if let Some(lit) = get_iden_attr(&v.attrs) {
            // If the user supplied a name, just adapt it.
            quote! { #lit }
        } else if v.ident == "Table" {
            table_name.clone()
        } else {
            let ident = v.ident.to_string().to_snake_case();
            quote! { #ident }
        }
    });

    let output = quote! {
        impl sea_query::Iden for #ident {
            fn unquoted(&self, s: &mut dyn sea_query::Write) {
                write!(s, "{}", match self {
                    #(Self::#variant => #name),*
                }).unwrap();
            }
        }
    };

    output.into()
}
