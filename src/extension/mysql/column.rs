use crate::Iden;

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MySqlType {
    TinyBlob,
    MediumBlob,
    LongBlob,
}

impl From<MySqlType> for Iden {
    fn from(value: MySqlType) -> Self {
        Self::from(match value {
            MySqlType::TinyBlob => "tinyblob",
            MySqlType::MediumBlob => "mediumblob",
            MySqlType::LongBlob => "longblob",
        })
    }
}
