use heck::SnakeCase;
use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, Attribute, DataEnum, DataStruct, DeriveInput, Fields, Ident, Lit, Meta,
    Variant,
};

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

fn get_method_attr(attr: &Attribute) -> syn::Result<syn::Lit> {
    match attr.parse_meta()? {
        Meta::NameValue(nv) => Ok(nv.lit),
        a => Err(syn::Error::new_spanned(
            a,
            r#"The method attribute only supports the `#[method = "name"]` format"#,
        )),
    }
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
        } else if let Some(lit) =
            find_attr(&v.attrs, "method").and_then(|att| get_method_attr(att).ok())
        {
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
