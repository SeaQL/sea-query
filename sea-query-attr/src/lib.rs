use heck::{ToPascalCase, ToSnakeCase};
use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Fields, Ident};

extern crate proc_macro;

#[macro_use]
extern crate quote;
extern crate syn;

struct NamingHolder {
    pub default: Ident,
    pub pascal: Ident,
}

#[proc_macro_attribute]
pub fn gen_type_def(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
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
    
    let struct_name = quote::format_ident!("{}TypeDef", &input.ident);
    let pascal_def_names = field_names.iter().map(|field| &field.pascal);
    let pascal_def_names2 = pascal_def_names.clone(); // we can't repeat the same ident twice in a quote!, so we need to clone the first one
    let default_names = field_names.iter().map(|field| &field.default);

    TokenStream::from(quote! {
        #input

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum #struct_name {
            Table,
            #(#pascal_def_names,)*
        }

        impl sea_query::Iden for #struct_name {
            fn unquoted(&self, s: &mut dyn sea_query::Write) {
                write!(s, "{}", match self {
                    #struct_name::Table => stringify!(#table_name),
                    #(#struct_name::#pascal_def_names2 => stringify!(#default_names)),*
                }).unwrap();
            }
        }
    })
}
