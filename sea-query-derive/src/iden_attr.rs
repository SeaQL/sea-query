use std::convert::{TryFrom, TryInto};

use syn::{Attribute, Error, Ident, Lit, Meta, MetaNameValue, NestedMeta};

use crate::{error::ErrorMsg, iden_path::IdenPath};

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
                _ => Err(Error::new_spanned(nv, ErrorMsg::WrongLiteral)),
            },
            a => Err(Error::new_spanned(
                a,
                ErrorMsg::WrongNamedValueFormat(IdenPath::Method, IdenPath::Method),
            )),
        }
    }

    fn extract_iden(meta: Meta) -> syn::Result<Self> {
        match &meta {
            Meta::NameValue(nv) => match &nv.lit {
                Lit::Str(lit) => Ok(IdenAttr::Rename(lit.value())),
                _ => Err(Error::new_spanned(&nv.lit, ErrorMsg::WrongLiteral)),
            },
            Meta::List(list) => match list.nested.first() {
                Some(NestedMeta::Meta(Meta::Path(p))) if p.is_ident(&IdenPath::Flatten) => {
                    Ok(IdenAttr::Flatten)
                }
                Some(NestedMeta::Meta(Meta::NameValue(nv))) => Self::extract_named_value_iden(nv),
                _ => Err(Error::new_spanned(meta, ErrorMsg::WrongListFormat)),
            },
            a => Err(Error::new_spanned(a, ErrorMsg::WrongAttributeFormat)),
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
                        ErrorMsg::UnsupportedKeyword(nv.path.get_ident().unwrap().clone()),
                    ))
                }
            }
            _ => Err(Error::new_spanned(&nv.lit, ErrorMsg::WrongLiteral)),
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
