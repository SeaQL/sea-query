use std::convert::{TryFrom, TryInto};

use syn::spanned::Spanned;
use syn::{Attribute, Error, Expr, ExprLit, Ident, Lit, LitStr, Meta};

use crate::{error::ErrorMsg, iden_path::IdenPath};

#[derive(PartialEq, Eq)]
pub(crate) enum IdenAttr {
    Rename(String),
    Method(Ident),
    Flatten,
}

impl IdenAttr {
    fn extract_method(meta: Meta) -> syn::Result<Self> {
        match meta {
            Meta::NameValue(nv) => match nv.value {
                Expr::Lit(ExprLit { lit, .. }) => match lit {
                    Lit::Str(name) => {
                        Ok(Self::Method(Ident::new(name.value().as_str(), name.span())))
                    }
                    _ => Err(Error::new_spanned(nv.eq_token, ErrorMsg::WrongLiteral)),
                },
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
            Meta::NameValue(nv) => match &nv.value {
                Expr::Lit(ExprLit { lit, .. }) => match lit {
                    Lit::Str(lit) => Ok(IdenAttr::Rename(lit.value())),
                    _ => Err(Error::new_spanned(&nv.value, ErrorMsg::WrongLiteral)),
                },
                _ => Err(Error::new_spanned(nv, ErrorMsg::WrongLiteral)),
            },
            Meta::List(list) if list.path.is_ident("iden") => {
                let mut iden_attr: Option<Self> = None;
                list.parse_nested_meta(|nested| {
                    if nested.path.is_ident(&IdenPath::Flatten) {
                        iden_attr = Some(IdenAttr::Flatten);
                        Ok(())
                    } else if nested.path.is_ident(&IdenPath::Rename) {
                        let value = nested.value()?;
                        let value: LitStr = value.parse()?;
                        iden_attr = Some(IdenAttr::Rename(value.value()));
                        Ok(())
                    } else if nested.path.is_ident(&IdenPath::Method) {
                        let value = nested.value()?;
                        let value: LitStr = value.parse()?;
                        iden_attr = Some(IdenAttr::Method(Ident::new(&value.value(), meta.span())));
                        Ok(())
                    } else {
                        Err(Error::new_spanned(
                            &meta,
                            ErrorMsg::UnsupportedKeyword(nested.path.get_ident().unwrap().clone()),
                        ))
                    }
                })?;
                iden_attr.ok_or(Error::new_spanned(meta, ErrorMsg::WrongListFormat))
            }
            a => Err(Error::new_spanned(a, ErrorMsg::WrongAttributeFormat)),
        }
    }
}

impl TryFrom<&Attribute> for IdenAttr {
    type Error = Error;

    fn try_from(value: &Attribute) -> Result<Self, Self::Error> {
        value.meta.clone().try_into()
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
