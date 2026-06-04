use crate::{ColumnType, DynIden, IntoIden};

/// Creates a new "DROP FUNCTION" statement for PostgreSQL
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FunctionDropStatement {
    pub(crate) name: Option<DynIden>,
    pub(crate) if_exists: bool,
    pub(crate) arg_types: Option<Vec<ColumnType>>,
    pub(crate) cascade: bool,
    pub(crate) restrict: bool,
}

impl FunctionDropStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the function name to drop
    pub fn name(&mut self, name: impl IntoIden) -> &mut Self {
        self.name = Some(name.into_iden());
        self
    }

    /// Use "IF EXISTS" on the DROP FUNCTION statement
    pub fn if_exists(&mut self) -> &mut Self {
        self.if_exists = true;
        self
    }

    /// Specify the argument types to uniquely identify the function overload to drop
    pub fn arg_types(&mut self, types: impl IntoIterator<Item = ColumnType>) -> &mut Self {
        self.arg_types = Some(types.into_iter().collect());
        self
    }

    /// Use "CASCADE" on the DROP FUNCTION statement
    pub fn cascade(&mut self) -> &mut Self {
        self.cascade = true;
        self
    }

    /// Use "RESTRICT" on the DROP FUNCTION statement
    pub fn restrict(&mut self) -> &mut Self {
        self.restrict = true;
        self
    }
}
