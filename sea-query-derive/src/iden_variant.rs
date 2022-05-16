use std::convert::TryFrom;

use heck::ToSnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Error, Fields, FieldsNamed, Ident, Variant};

use crate::{error::ErrorMsg, find_attr, iden_attr::IdenAttr};

pub struct IdenVariant<'a> {
    ident: &'a Ident,
    fields: &'a Fields,
    table_name: &'a str,
    attr: Option<IdenAttr>,
}

impl<'a> TryFrom<(&'a str, &'a Variant)> for IdenVariant<'a> {
    type Error = Error;

    fn try_from((table_name, value): (&'a str, &'a Variant)) -> Result<Self, Self::Error> {
        let Variant {
            ident,
            fields,
            attrs,
            ..
        } = value;
        let attr = find_attr(attrs).map(IdenAttr::try_from).transpose()?;

        Self::new(ident, fields, table_name, attr)
    }
}

impl ToTokens for IdenVariant<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.fields {
            Fields::Named(named) => self.to_tokens_from_named(named, tokens),
            Fields::Unnamed(_) => self.to_tokens_from_unnamed(tokens),
            Fields::Unit => self.to_tokens_from_unit(tokens),
        }
    }
}

impl<'a> IdenVariant<'a> {
    fn new(
        ident: &'a Ident,
        fields: &'a Fields,
        table_name: &'a str,
        attr: Option<IdenAttr>,
    ) -> syn::Result<Self> {
        let unsupported_error = Err(Error::new_spanned(
            fields,
            ErrorMsg::UnsupportedFlattenTarget,
        ));
        // sanity check to not have flatten on a unit variant, or variants with more than 1 field
        if attr == Some(IdenAttr::Flatten) {
            match fields {
                Fields::Named(n) => {
                    if n.named.len() != 1 {
                        return unsupported_error;
                    }
                }
                Fields::Unnamed(u) => {
                    if u.unnamed.len() != 1 {
                        return unsupported_error;
                    }
                }
                Fields::Unit => return unsupported_error,
            }
        }

        Ok(Self {
            ident,
            fields,
            table_name,
            attr,
        })
    }

    fn to_tokens_from_named(&self, named: &FieldsNamed, tokens: &mut TokenStream) {
        let ident = self.ident;

        let match_arm = if self.attr == Some(IdenAttr::Flatten) {
            // indexing is safe because len is guaranteed to be 1 from the constructor.
            let field = &named.named[0];
            // Unwrapping the ident is also safe because a named field always has an ident.
            let capture = field.ident.as_ref().unwrap();
            let variant = quote! { #ident{#capture} };
            write_flattened(variant, capture)
        } else {
            let variant = quote! { #ident{..} };
            self.write_variant_name(variant)
        };

        tokens.append_all(match_arm)
    }

    fn to_tokens_from_unnamed(&self, tokens: &mut TokenStream) {
        let ident = self.ident;

        let match_arm = if self.attr == Some(IdenAttr::Flatten) {
            // The case where unnamed fields length is not 1 is handled by new
            let capture = Delegated.into();
            let variant = quote! { #ident(#capture) };
            write_flattened(variant, &capture)
        } else {
            let variant = quote! { #ident(..) };
            self.write_variant_name(variant)
        };

        tokens.append_all(match_arm)
    }

    fn to_tokens_from_unit(&self, tokens: &mut TokenStream) {
        let ident = self.ident;
        let variant = quote! { #ident };

        tokens.append_all(self.write_variant_name(variant))
    }

    fn table_or_snake_case(&self) -> TokenStream {
        if self.ident == "Table" {
            let table_name = self.table_name;
            quote! { #table_name }
        } else {
            let name = self.ident.to_string().to_snake_case();
            quote! { #name }
        }
    }

    fn write_variant_name(&self, variant: TokenStream) -> TokenStream {
        let name = self
            .attr
            .as_ref()
            .map(|a| match a {
                IdenAttr::Rename(name) => quote! { #name },
                IdenAttr::Method(method) => quote! { self.#method() },
                IdenAttr::Flatten => unreachable!(),
            })
            .unwrap_or_else(|| self.table_or_snake_case());

        write_variant(variant, name)
    }
}

struct Delegated;

impl From<Delegated> for Ident {
    fn from(_: Delegated) -> Self {
        Ident::new("delegated", Span::call_site())
    }
}

fn write_variant(variant: TokenStream, name: TokenStream) -> TokenStream {
    quote! { Self::#variant => write!(s, "{}", #name).unwrap() }
}

fn write_flattened(variant: TokenStream, name: &Ident) -> TokenStream {
    quote! { Self::#variant => #name.unquoted(s) }
}
