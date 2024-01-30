use crate::Iden;

#[derive(Debug, Copy, Clone)]
pub enum MySqlType {
    Blob,
    TinyBlob,
    MediumBlob,
    LongBlob,
}

impl Iden for MySqlType {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        let ty = match self {
            Self::Blob => "blob",
            Self::TinyBlob => "tinyblob",
            Self::MediumBlob => "mediumblob",
            Self::LongBlob => "longblob",
        };
        write!(s, "{ty}").unwrap();
    }
}
