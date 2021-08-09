pub enum IdenPath {
    Iden,
    Method,
    Rename,
    Flatten,
}

impl PartialEq<IdenPath> for syn::Ident {
    fn eq(&self, other: &IdenPath) -> bool {
        match other {
            IdenPath::Iden => self.eq("iden"),
            IdenPath::Method => self.eq("method"),
            IdenPath::Rename => self.eq("rename"),
            IdenPath::Flatten => self.eq("flatten"),
        }
    }
}
