use heck::{SnakeCase, CamelCase};
use proc_macro::{TokenStream};
use proc_macro2::Span;
use quote::{format_ident, quote, quote_spanned};
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Ident, Lit, Meta, Type, Variant, parse_macro_input};

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

fn get_method_attr(attrs: &[Attribute]) -> Option<syn::Lit> {
    for attr in attrs {
        let name_value = match attr.parse_meta() {
            Ok(Meta::NameValue(nv)) => nv,
            _ => continue,
        };
        if name_value.path.is_ident("method") {
            return Some(name_value.lit);
        }
    }
    None
}

#[proc_macro_derive(Iden, attributes(iden, method))]
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
            }
            _ => return quote_spanned! {
                ident.span() => compile_error!("you can only derive Iden on enums or unit structs");
            }
            .into(),
        };

    if variants.is_empty() {
        return TokenStream::new();
    }

    let variant = variants
        .iter()
        .map(|Variant { ident, fields, .. }| match fields {
            Fields::Named(_) => quote! { #ident{..} },
            Fields::Unnamed(_) => quote! { #ident(..) },
            Fields::Unit => quote! { #ident },
        });

    let name = variants.iter().map(|v| {
        if let Some(lit) = get_iden_attr(&v.attrs) {
            // If the user supplied a name, just use it
            quote! { #lit }
        } else if let Some(lit) = get_method_attr(&v.attrs) {
            // If the user supplied a method, call it
            let name: String = match lit {
                Lit::Str(name) => name.value(),
                _ => panic!("expected string for `method`"),
            };
            let ident = Ident::new(name.as_str(), Span::call_site());
            quote! { self.#ident() }
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
                match self {
                    #(Self::#variant => write!(s, "{}", #name).unwrap()),*
                };
            }
        }
    };

    output.into()
}

#[proc_macro_derive(ReadOnly)]
pub fn derive_read_only(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, ..
    } = parse_macro_input!(input);

    let read_only_ident = format_ident!("ReadOnly{}", ident.to_string().to_camel_case());

    let fields = match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(named),
            ..
        }) => named.named,
        _ => return quote_spanned! {
            ident.span() => compile_error!("you can only derive ReadOnly on structs");
        }.into(),
    };

    let field: Vec<Ident> = fields
        .clone()
        .into_iter()
        .map(|Field { ident, .. }| format_ident!("{}", ident.unwrap().to_string()))
        .collect();

    let ty: Vec<Type> = fields
        .into_iter()
        .map(|Field { ty, .. }| ty)
        .collect();

    quote!(
        impl #ident {
            pub fn into_read_only(self) -> #read_only_ident {
                #read_only_ident {
                    #(#field: self.#field),*
                }
            }
        }

        #[derive(Debug, Clone)]
        pub struct #read_only_ident {
            #(pub #field: #ty),*
        }
    ).into()
}
