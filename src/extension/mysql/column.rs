use crate::IdenStatic;

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MySqlType {
    TinyBlob,
    MediumBlob,
    LongBlob,
}

impl IdenStatic for MySqlType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::TinyBlob => "tinyblob",
            Self::MediumBlob => "mediumblob",
            Self::LongBlob => "longblob",
        }
    }
}
