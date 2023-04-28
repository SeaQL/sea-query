use syn::Ident;

use crate::iden_path::IdenPath;

#[derive(Debug, thiserror::Error)]
pub enum ErrorMsg {
    #[error("Only the attributes `#[iden = \"name\"]` or `#[iden(rename = \"name\") are supported in this position")]
    ContainerAttr,
    #[error("Must be a string literal")]
    WrongLiteral,
    #[error("The method attribute only supports the `#[{0} = \"name\"]` or `#[iden({0} = \"name\")]` formats")]
    WrongNamedValueFormat(IdenPath, IdenPath),
    #[error("Must one of the following attributes: `flatten`, `rename` or `method`")]
    WrongListFormat,
    #[error("The iden attribute supports only the formats `#[iden = \"name\"]` or `#[iden(<ATTRIBUTE>)]` where ATTRIBUTE is either `flatten`, `rename` or `method`")]
    WrongAttributeFormat,
    #[error("{0} is not a supported keyword")]
    UnsupportedKeyword(Ident),
    #[error("Must have a single field is supported for flattenning")]
    UnsupportedFlattenTarget,
}
