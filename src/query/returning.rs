use crate::{ColumnRef, IntoColumnRef};

/// RETURNING clause.
/// ## Note:
/// Works on
/// * PostgreSQL
/// * SQLite
///     - SQLite version >= 3.35.0
///     - **Note that sea-query won't try to enforce either of these constraints**
///
#[derive(Clone, Debug)]
pub enum ReturningClause {
    All,
    Columns(Vec<ColumnRef>),
}

/// Shorthand for constructing [`ReturningClause`]
#[derive(Clone, Debug, Default)]
pub struct Returning;

impl Returning {
    /// Constructs a new [`Returning`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a new [`ReturningClause::All`].
    pub fn all(&self) -> ReturningClause {
        ReturningClause::All
    }

    /// Constructs a new [`ReturningClause::Columns`].
    pub fn column<C>(&self, col: C) -> ReturningClause
    where
        C: IntoColumnRef,
    {
        ReturningClause::Columns(vec![col.into_column_ref()])
    }

    /// Constructs a new [`ReturningClause::Columns`].
    pub fn columns<T, I>(self, cols: I) -> ReturningClause
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = T>,
    {
        let cols: Vec<_> = cols.into_iter().map(|c| c.into_column_ref()).collect();
        ReturningClause::Columns(cols)
    }
}
