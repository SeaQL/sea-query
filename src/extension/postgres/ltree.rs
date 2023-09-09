use std::fmt;

use crate::PgLTree;

impl fmt::Display for PgLTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<T> for PgLTree
where
    T: Into<String>,
{
    fn from(field: T) -> Self {
        PgLTree(field.into())
    }
}
