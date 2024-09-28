use darling::{ast::NestedMeta, FromMeta};
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro::TokenStream;
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, Data, DataStruct, DeriveInput,
    Fields, Ident,
};

type AttributeArgs = Punctuated<NestedMeta, syn::Token![,]>;

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

#[deprecated(since = "0.1.2", note = "use #[enum_def] attr defined in `sea-query-derive` crate")]
#[proc_macro_attribute]
pub fn enum_def(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args with AttributeArgs::parse_terminated);
    let input = parse_macro_input!(input as DeriveInput);

    expand(args, input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand(attr_args: AttributeArgs, input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let attr_args = attr_args.into_iter().collect::<Vec<_>>();
    let args = GenEnumArgs::from_list(&attr_args)?;
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => {
            return Err(syn::Error::new(
                input.span(),
                "#[enum_def] can only be used on structs",
            ))
        }
    };

    let field_names = fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().ok_or(syn::Error::new(
                field.span(),
                "#[enum_def] can only be used on structs with named fields",
            ))?;
            let string = ident.to_string();
            let as_pascal = string.to_pascal_case();
            syn::Result::Ok(NamingHolder {
                default: ident.clone(),
                pascal: Ident::new(as_pascal.as_str(), ident.span()),
            })
        })
        .collect::<syn::Result<Vec<NamingHolder>>>()?;

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
    let import_name = Ident::new(
        args.crate_name
            .unwrap_or_else(|| DEFAULT_CRATE_NAME.to_string())
            .as_str(),
        input.span(),
    );

    Ok(quote::quote! {
        #input

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum #enum_name {
            Table,
            #(#pascal_def_names,)*
        }

        impl #import_name::Iden for #enum_name {
            fn unquoted(&self, s: &mut dyn sea_query::Write) {
                write!(s, "{}", match self {
                    #enum_name::Table => stringify!(#table_name),
                    #(#enum_name::#pascal_def_names2 => stringify!(#default_names)),*
                }).unwrap();
            }
        }
    })
}
