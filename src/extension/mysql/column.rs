use crate::Iden;

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MySqlType {
    TinyBlob,
    MediumBlob,
    LongBlob,
}

impl Iden for MySqlType {
    fn unquoted(&self) -> &str {
        match self {
            Self::TinyBlob => "tinyblob",
            Self::MediumBlob => "mediumblob",
            Self::LongBlob => "longblob",
        }
    }
}
