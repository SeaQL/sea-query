use std::convert::{TryFrom, TryInto};

use syn::{Attribute, Error, Ident, Lit, Meta, MetaNameValue, NestedMeta};

use crate::iden_path::IdenPath;

#[derive(PartialEq)]
pub enum IdenAttr {
    Rename(String),
    Method(Ident),
    Flatten,
}

impl IdenAttr {
    fn extract_method(meta: Meta) -> syn::Result<Self> {
        match meta {
            Meta::NameValue(nv) => match nv.lit {
                Lit::Str(name) => Ok(Self::Method(Ident::new(name.value().as_str(), name.span()))),
                _ => Err(Error::new_spanned(nv, "Must be a string literal")),
            },
            a => Err(Error::new_spanned(
                a,
                r#"The method attribute only supports the `#[method = "name"]` format"#,
            )),
        }
    }

    fn extract_iden(meta: Meta) -> syn::Result<Self> {
        match &meta {
            Meta::NameValue(nv) => match &nv.lit {
                Lit::Str(lit) => Ok(IdenAttr::Rename(lit.value())),
                _ => Err(Error::new_spanned(
                    &nv.lit,
                    "Only string literals are permitted in this position",
                )),
            },
            Meta::List(list) => match list.nested.first() {
                Some(NestedMeta::Meta(Meta::Path(p))) if p.is_ident(&IdenPath::Flatten) => {
                    Ok(IdenAttr::Flatten)
                }
                Some(NestedMeta::Meta(Meta::NameValue(nv))) => Self::extract_named_value_iden(nv),
                _ => Err(Error::new_spanned(
                    meta,
                    "must be of the form `#[iden(flatten)]`",
                )),
            },
            a => Err(Error::new_spanned(
                a,
                r#"The iden attribute supports only the formats `#[iden = "name"]` and #[iden(flatten)]"#,
            )),
        }
    }

    fn extract_named_value_iden(nv: &MetaNameValue) -> syn::Result<Self> {
        match &nv.lit {
            Lit::Str(name) => {
                // Don't match "iden" since that would mean `#[iden(iden = "name")]` would be accepted
                if nv.path.is_ident(&IdenPath::Rename) {
                    Ok(Self::Rename(name.value()))
                } else if nv.path.is_ident(&IdenPath::Method) {
                    Ok(Self::Method(Ident::new(name.value().as_str(), name.span())))
                } else {
                    Err(Error::new_spanned(
                        nv,
                        format! {"{} is not a supported keyword", nv.path.get_ident().unwrap()},
                    ))
                }
            }
            _ => Err(Error::new_spanned(
                &nv.lit,
                "Only string literals are permitted in this position",
            )),
        }
    }
}

impl TryFrom<&Attribute> for IdenAttr {
    type Error = Error;

    fn try_from(value: &Attribute) -> Result<Self, Self::Error> {
        value.parse_meta()?.try_into()
    }
}

impl TryFrom<Meta> for IdenAttr {
    type Error = Error;

    fn try_from(value: Meta) -> Result<Self, Self::Error> {
        let path = value.path();
        if path.is_ident(&IdenPath::Method) {
            Self::extract_method(value)
        } else if path.is_ident(&IdenPath::Iden) {
            Self::extract_iden(value)
        } else {
            todo!()
        }
    }
}
