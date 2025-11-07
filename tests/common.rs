use sea_query::Iden;

/// Representation of a database table named `BloB`.
///
/// A `Enum` implemented [`Iden`] used in rustdoc and test to demonstrate the library usage.
///
/// [`Iden`]: crate::types::Iden
#[derive(Debug)]
#[allow(dead_code)]
pub enum BinaryType {
    Table,
    BinaryLen,
    Binary,
    BlobSize,
    TinyBlob,
    Blob,
    MediumBlob,
    LongBlob,
}

impl Iden for BinaryType {
    fn unquoted(&self) -> &str {
        match self {
            Self::Table => "binary_type",
            Self::BinaryLen => "binlen",
            Self::Binary => "bin",
            Self::BlobSize => "defb",
            Self::TinyBlob => "tb",
            Self::Blob => "b",
            Self::MediumBlob => "mb",
            Self::LongBlob => "lb",
        }
    }
}
