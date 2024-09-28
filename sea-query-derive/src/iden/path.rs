use std::fmt::Display;

#[derive(Debug)]
pub enum IdenPath {
    Iden,
    Method,
    Rename,
    Flatten,
}

impl IdenPath {
    const fn as_str(&self) -> &'static str {
        match self {
            IdenPath::Iden => "iden",
            IdenPath::Method => "method",
            IdenPath::Rename => "rename",
            IdenPath::Flatten => "flatten",
        }
    }
}

impl Display for IdenPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl PartialEq<IdenPath> for syn::Ident {
    fn eq(&self, other: &IdenPath) -> bool {
        self.eq(other.as_str())
    }
}
