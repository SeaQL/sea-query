use crate::{DynIden, IntoIden};

#[derive(Debug, Clone)]
pub struct OnConflict {
    pub(crate) target: Option<OnConflictTarget>,
    pub(crate) action: Option<OnConflictAction>,
}

/// Represents ON CONFLICT (upsert) targets
#[derive(Debug, Clone)]
pub enum OnConflictTarget {
    /// A list of columns with unique constraint
    ConflictColumns(Vec<DynIden>),
}

/// Represents ON CONFLICT (upsert) actions
#[derive(Debug, Clone)]
pub enum OnConflictAction {
    /// Update the column values of existing row instead of insert
    UpdateColumns(Vec<DynIden>),
}

impl OnConflict {
    pub fn new() -> Self {
        Default::default()
    }

    // Set ON CONFLICT target column
    pub fn column<C>(column: C) -> Self
    where
        C: IntoIden,
    {
        Self::columns(vec![column])
    }

    // Set ON CONFLICT target columns
    pub fn columns<I, C>(columns: I) -> Self
    where
        C: IntoIden,
        I: IntoIterator<Item = C>,
    {
        Self {
            target: Some(OnConflictTarget::ConflictColumns(
                columns.into_iter().map(IntoIden::into_iden).collect(),
            )),
            action: None,
        }
    }

    // Set ON CONFLICT update column
    pub fn update_column<C>(&mut self, column: C) -> &mut Self
    where
        C: IntoIden,
    {
        self.update_columns(vec![column])
    }

    // Set ON CONFLICT update columns
    pub fn update_columns<C, I>(&mut self, columns: I) -> &mut Self
    where
        C: IntoIden,
        I: IntoIterator<Item = C>,
    {
        self.action = Some(OnConflictAction::UpdateColumns(
            columns.into_iter().map(IntoIden::into_iden).collect(),
        ));
        self
    }
}

impl Default for OnConflict {
    fn default() -> Self {
        Self {
            target: None,
            action: None,
        }
    }
}
