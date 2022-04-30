use darling::FromMeta;
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro::TokenStream;
use syn::{
    parse_macro_input, spanned::Spanned, AttributeArgs, Data, DataStruct, DeriveInput, Fields,
    Ident,
};

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
        }
    }
}

#[proc_macro_attribute]
pub fn enum_def(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as DeriveInput);
    let args = GenEnumArgs::from_list(&args).unwrap_or_default();

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("#[enum_def] can only be used on structs"),
    };

    let field_names: Vec<NamingHolder> = fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            let string = ident
                .as_ref()
                .expect("#[enum_def] can only be used on structs with named fields")
                .to_string();
            let as_pascal = string.to_pascal_case();
            NamingHolder {
                default: ident.as_ref().unwrap().clone(),
                pascal: Ident::new(as_pascal.as_str(), ident.span()),
            }
        })
        .collect();

    let table_name = Ident::new(
        input.ident.to_string().to_snake_case().as_str(),
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

    TokenStream::from(quote::quote! {
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
