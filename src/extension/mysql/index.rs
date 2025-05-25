use crate::{Alias, DynIden};

pub type IndexName = Alias;

#[derive(Debug, Clone, PartialEq)]
pub struct IndexHint {
    pub index: DynIden,
    pub r#type: IndexHintType,
    pub scope: IndexHintScope,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum IndexHintType {
    Use,
    Ignore,
    Force,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum IndexHintScope {
    Join,
    OrderBy,
    GroupBy,
    All,
}
