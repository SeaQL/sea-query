use crate::{DynIden, IntoIden, IntoTableRef, TableRef};

/// Creates a new "DROP TRIGGER" statement for PostgreSQL
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TriggerDropStatement {
    pub(crate) name: Option<DynIden>,
    pub(crate) if_exists: bool,
    pub(crate) table: Option<TableRef>,
    pub(crate) cascade: bool,
    pub(crate) restrict: bool,
}

impl TriggerDropStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the trigger name to drop
    pub fn name(&mut self, name: impl IntoIden) -> &mut Self {
        self.name = Some(name.into_iden());
        self
    }

    /// Use "IF EXISTS" on the DROP TRIGGER statement
    pub fn if_exists(&mut self) -> &mut Self {
        self.if_exists = true;
        self
    }

    /// Set the table on which the trigger is defined
    pub fn table(&mut self, table: impl IntoTableRef) -> &mut Self {
        self.table = Some(table.into_table_ref());
        self
    }

    /// Use "CASCADE" on the DROP TRIGGER statement
    pub fn cascade(&mut self) -> &mut Self {
        self.cascade = true;
        self
    }

    /// Use "RESTRICT" on the DROP TRIGGER statement
    pub fn restrict(&mut self) -> &mut Self {
        self.restrict = true;
        self
    }
}
