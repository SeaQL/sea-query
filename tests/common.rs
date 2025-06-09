pub use std::fmt::Write as FmtWrite;

#[cfg(feature = "with-json")]
pub use serde_json::json;

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

impl From<BinaryType> for Iden {
    fn from(value: BinaryType) -> Self {
        Self::from(match value {
            BinaryType::Table => "binary_type",
            BinaryType::BinaryLen => "binlen",
            BinaryType::Binary => "bin",
            BinaryType::BlobSize => "defb",
            BinaryType::TinyBlob => "tb",
            BinaryType::Blob => "b",
            BinaryType::MediumBlob => "mb",
            BinaryType::LongBlob => "lb",
        })
    }
}
