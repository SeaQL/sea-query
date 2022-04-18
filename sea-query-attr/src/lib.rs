use darling::FromMeta;
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro::TokenStream;
use syn::{
    parse_macro_input, spanned::Spanned, AttributeArgs, Data, DataStruct, DeriveInput, Fields,
    Ident,
};

#[macro_use]
extern crate quote;

struct NamingHolder {
    pub default: Ident,
    pub pascal: Ident,
}

#[derive(Debug, FromMeta)]
struct GenTypeDefArgs {
    #[darling(default)]
    pub prefix: Option<String>,
    #[darling(default)]
    pub suffix: Option<String>,
}

const DEFAULT_PREFIX: &'static str = "";
const DEFAULT_SUFFIX: &'static str = "TypeDef";

impl Default for GenTypeDefArgs {
    fn default() -> Self {
        Self {
            prefix: Some(DEFAULT_PREFIX.to_string()),
            suffix: Some(DEFAULT_SUFFIX.to_string()),
        }
    }
}

#[proc_macro_attribute]
pub fn gen_type_def(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as DeriveInput);
    let args = GenTypeDefArgs::from_list(&args).unwrap_or_default();

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("#[gen_type_def] can only be used on structs"),
    };

    let field_names: Vec<NamingHolder> = fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            let string = ident
                .as_ref()
                .expect("#[gen_type_def] can only be used on structs with named fields")
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
        args.prefix.unwrap_or(DEFAULT_PREFIX.to_string()),
        &input.ident,
        args.suffix.unwrap_or(DEFAULT_SUFFIX.to_string())
    );
    let pascal_def_names = field_names.iter().map(|field| &field.pascal);
    let pascal_def_names2 = pascal_def_names.clone();
    let default_names = field_names.iter().map(|field| &field.default);

    TokenStream::from(quote! {
        #input

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum #enum_name {
            Table,
            #(#pascal_def_names,)*
        }

        impl sea_query::Iden for #enum_name {
            fn unquoted(&self, s: &mut dyn sea_query::Write) {
                write!(s, "{}", match self {
                    #enum_name::Table => stringify!(#table_name),
                    #(#enum_name::#pascal_def_names2 => stringify!(#default_names)),*
                }).unwrap();
            }
        }
    })
}
