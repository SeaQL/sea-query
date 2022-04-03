use crate::{backend::SchemaBuilder, prepare::*, types::*, SchemaStatementBuilder};

/// Rename a view
///
/// # Examples
///
/// ```
/// ```
#[derive(Debug, Clone)]
pub struct ViewRenameStatement {
    pub(crate) from_name: Option<DynIden>,
    pub(crate) to_name: Option<DynIden>,
}

impl Default for ViewRenameStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewRenameStatement {
    /// Construct rename view statement.
    pub fn new() -> Self {
        Self {
            from_name: None,
            to_name: None,
        }
    }

    /// Set old and new view name.
    pub fn view<T: 'static, R: 'static>(&mut self, from_name: T, to_name: R) -> &mut Self
    where
        T: Iden,
        R: Iden,
    {
        self.from_name = Some(SeaRc::new(from_name));
        self.to_name = Some(SeaRc::new(to_name));
        self
    }

    pub fn take(&mut self) -> Self {
        Self {
            from_name: self.from_name.take(),
            to_name: self.to_name.take(),
        }
    }
}

impl SchemaStatementBuilder for ViewRenameStatement {
    fn build<T: SchemaBuilder>(&self, view_builder: T) -> String {
        let mut sql = SqlWriter::new();
        view_builder.prepare_view_rename_statement(self, &mut sql);
        sql.result()
    }

    fn build_any(&self, view_builder: &dyn SchemaBuilder) -> String {
        let mut sql = SqlWriter::new();
        view_builder.prepare_view_rename_statement(self, &mut sql);
        sql.result()
    }
}
