use crate::{backend::SchemaBuilder, types::*, SchemaStatementBuilder};

/// Rename a view
///
/// # Examples
///
/// ```
/// ```
#[derive(Default, Debug, Clone)]
pub struct ViewRenameStatement {
    pub(crate) from_name: Option<DynIden>,
    pub(crate) to_name: Option<DynIden>,
}

impl ViewRenameStatement {
    /// Construct rename view statement.
    pub fn new() -> Self {
        Self::default()
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
        let mut sql = String::with_capacity(256);
        view_builder.prepare_view_rename_statement(self, &mut sql);
        sql
    }

    fn build_any(&self, view_builder: &dyn SchemaBuilder) -> String {
        let mut sql = String::with_capacity(256);
        view_builder.prepare_view_rename_statement(self, &mut sql);
        sql
    }
}
