use std::default::Default;

use crate::ColumnRef;

#[derive(Clone, Debug)]
pub enum Returning {
    All,
    Collumns(Vec<ColumnRef>),
    Nothing,
    PrimaryKey,
}

impl Default for Returning {
    fn default() -> Self {
        Self::Nothing
    }
}
