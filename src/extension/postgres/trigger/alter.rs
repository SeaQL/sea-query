use crate::{DynIden, IntoIden, IntoTableRef, TableRef};

/// Represents PostgreSQL trigger alteration options
#[derive(Debug, Clone, PartialEq)]
pub enum TriggerAlterOption {
    RenameTo(DynIden),
    DependsOnExtension(DynIden),
    NoDependsOnExtension(DynIden),
}

/// Creates a new "ALTER TRIGGER" statement for PostgreSQL
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TriggerAlterStatement {
    pub(crate) name: Option<DynIden>,
    pub(crate) table: Option<TableRef>,
    pub(crate) option: Option<TriggerAlterOption>,
}

impl TriggerAlterStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the trigger name to alter
    pub fn name<T: IntoIden>(&mut self, name: T) -> &mut Self {
        self.name = Some(name.into_iden());
        self
    }

    /// Set the table on which the trigger is defined
    pub fn table<T: IntoTableRef>(&mut self, table: T) -> &mut Self {
        self.table = Some(table.into_table_ref());
        self
    }

    /// Rename the trigger
    pub fn rename_to<T: IntoIden>(&mut self, new_name: T) -> &mut Self {
        self.option = Some(TriggerAlterOption::RenameTo(new_name.into_iden()));
        self
    }

    /// Mark the trigger as dependent on an extension
    pub fn depends_on_extension<T: IntoIden>(&mut self, extension_name: T) -> &mut Self {
        self.option = Some(TriggerAlterOption::DependsOnExtension(extension_name.into_iden()));
        self
    }

    /// Remove extension dependency from the trigger
    pub fn no_depends_on_extension<T: IntoIden>(&mut self, extension_name: T) -> &mut Self {
        self.option = Some(TriggerAlterOption::NoDependsOnExtension(extension_name.into_iden()));
        self
    }
}
