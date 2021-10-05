use std::default::Default;

use crate::{ColumnRef, IntoColumnRef};

#[derive(Clone, Debug)]
pub enum Returning {
    All,
    Columns(Vec<ColumnRef>),
    Nothing,
}

impl Returning {
    pub fn cols<T, I>(cols: I) -> Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = T>,
    {
        let cols: Vec<_> = cols.into_iter().map(|c| c.into_column_ref()).collect();
        Self::Columns(cols)
    }
}

impl Default for Returning {
    fn default() -> Self {
        Self::Nothing
    }
}
