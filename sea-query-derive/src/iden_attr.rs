use std::convert::{TryFrom, TryInto};

use syn::{Attribute, Error, Ident, Lit, Meta};

#[derive(PartialEq)]
pub enum IdenAttr {
    Rename(syn::LitStr),
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
        match meta {
            Meta::NameValue(nv) => match nv.lit {
                Lit::Str(lit) => Ok(IdenAttr::Rename(lit)),
                _ => Err(Error::new_spanned(
                    nv.lit,
                    "Only string literals are permitted in this position",
                )),
            },
            a => Err(Error::new_spanned(
                a,
                r#"The iden attribute supports only the formats `#[iden = "name"]` and #[iden(flatten)]"#,
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
        if path.is_ident("method") {
            Self::extract_method(value)
        } else if path.is_ident("iden") {
            Self::extract_iden(value)
        } else {
            todo!()
        }
    }
}
