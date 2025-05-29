use crate::{Iden, IdenImpl, value};

#[derive(Debug, Copy, Clone)]
pub enum MySqlType {
    TinyBlob,
    MediumBlob,
    LongBlob,
}

impl Iden for MySqlType {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        let ty = match self {
            Self::TinyBlob => "tinyblob",
            Self::MediumBlob => "mediumblob",
            Self::LongBlob => "longblob",
        };
        write!(s, "{ty}").unwrap();
    }
}

impl From<MySqlType> for IdenImpl {
    fn from(value: MySqlType) -> Self {
        Self::new(match value {
            MySqlType::TinyBlob => "tinyblob",
            MySqlType::MediumBlob => "mediumblob",
            MySqlType::LongBlob => "longblob",
        })
    }
}
