use super::*;

pub fn expand(input: DeriveInput) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = input;

    let sea_query_path = sea_query_path();

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
