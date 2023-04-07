use std::convert::TryFrom;
use std::marker::PhantomData;

use heck::ToSnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Error, Fields, FieldsNamed, Ident, Variant};

use crate::{error::ErrorMsg, find_attr, iden_attr::IdenAttr, must_be_valid_iden};

pub(crate) trait WriteArm {
    fn variant(variant: TokenStream, name: TokenStream) -> TokenStream;
    fn flattened(variant: TokenStream, name: &Ident) -> TokenStream;
}

pub(crate) struct DeriveIden;

impl WriteArm for DeriveIden {
    fn variant(variant: TokenStream, name: TokenStream) -> TokenStream {
        quote! { Self::#variant => write!(s, "{}", #name).unwrap() }
    }

    fn flattened(variant: TokenStream, name: &Ident) -> TokenStream {
        quote! { Self::#variant => #name.unquoted(s) }
    }
}

pub(crate) struct DeriveIdenStatic;

impl WriteArm for DeriveIdenStatic {
    fn variant(variant: TokenStream, name: TokenStream) -> TokenStream {
        quote! { Self::#variant => #name }
    }

    fn flattened(variant: TokenStream, name: &Ident) -> TokenStream {
        quote! { Self::#variant => #name.as_str() }
    }
}

pub(crate) struct IdenVariant<'a, T> {
    ident: &'a Ident,
    fields: &'a Fields,
    table_name: &'a str,
    attr: Option<IdenAttr>,
    _p: PhantomData<T>,
}

impl<'a, T> TryFrom<(&'a str, &'a Variant)> for IdenVariant<'a, T>
where
    T: WriteArm,
{
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

impl<T> ToTokens for IdenVariant<'_, T>
where
    T: WriteArm,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.fields {
            Fields::Named(named) => self.to_tokens_from_named(named, tokens),
            Fields::Unnamed(_) => self.to_tokens_from_unnamed(tokens),
            Fields::Unit => self.to_tokens_from_unit(tokens),
        }
    }
}

impl<'a, T> IdenVariant<'a, T>
where
    T: WriteArm,
{
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
            _p: PhantomData::<T>,
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
            T::flattened(variant, capture)
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
            T::flattened(variant, &capture)
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

    fn table_or_snake_case(&self) -> String {
        if self.ident == "Table" {
            self.table_name.to_owned()
        } else {
            self.ident.to_string().to_snake_case()
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
            .unwrap_or_else(|| {
                let name = self.table_or_snake_case();
                quote! { #name }
            });

        T::variant(variant, name)
    }

    pub(super) fn must_be_valid_iden(&self) -> bool {
        let name: String = match &self.attr {
            Some(a) => match a {
                IdenAttr::Rename(name) => name.to_owned(),
                IdenAttr::Method(_) => return false,
                IdenAttr::Flatten => return false,
            },
            None => self.table_or_snake_case(),
        };

        must_be_valid_iden(&name)
    }
}

struct Delegated;

impl From<Delegated> for Ident {
    fn from(_: Delegated) -> Self {
        Ident::new("delegated", Span::call_site())
    }
}
