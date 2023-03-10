use crate::{backend::SchemaBuilder, query::SelectStatement, types::*, SchemaStatementBuilder};

/// Create a view
///
/// # Examples
///
/// ```
/// ```
#[derive(Default, Debug, Clone)]
pub struct ViewCreateStatement {
    pub(crate) view: Option<TableRef>,
    pub(crate) columns: Vec<DynIden>,
    pub(crate) query: SelectStatement,
    pub(crate) or_replace: bool,
    pub(crate) if_not_exists: bool,
    pub(crate) recursive: bool,
    pub(crate) temporary: bool,
    pub(crate) opt: Option<ViewCreateOpt>,
}

#[derive(Debug, Clone)]
pub enum ViewCreateOpt {
    Cascade,
    Local,
}

impl ViewCreateStatement {
    /// Construct create view statement.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create view if view not exists.
    pub fn if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = true;
        self
    }

    /// Create or replace view.
    pub fn or_replace(&mut self) -> &mut Self {
        self.or_replace = true;
        self
    }

    /// Create recursive view .
    pub fn recursive(&mut self) -> &mut Self {
        self.recursive = true;
        self
    }

    /// Create temporary view.
    pub fn temporary(&mut self) -> &mut Self {
        self.temporary = true;
        self
    }

    /// Set view name.
    pub fn view<T>(&mut self, view: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.view = Some(view.into_table_ref());
        self
    }

    /// Add a new view column.
    pub fn column<C>(&mut self, column: C) -> &mut Self
    where
        C: IntoIden,
    {
        let column = column.into_iden();
        self.columns.push(column);
        self
    }

    /// Adds a columns to the view definition.
    pub fn columns<T, I>(&mut self, cols: I) -> &mut Self
    where
        T: IntoIden,
        I: IntoIterator<Item = T>,
    {
        self.columns
            .extend(cols.into_iter().map(|col| col.into_iden()));
        self
    }

    /// Adds AS select query to the view.
    pub fn query(&mut self, select: SelectStatement) -> &mut Self {
        self.query = select;
        self
    }

    pub fn create_opt(&mut self, opt: ViewCreateOpt) -> &mut Self {
        self.opt = Some(opt);
        self
    }

    pub fn get_view_name(&self) -> Option<&TableRef> {
        self.view.as_ref()
    }

    pub fn get_columns(&self) -> &Vec<DynIden> {
        self.columns.as_ref()
    }

    pub fn get_query(&self) -> &SelectStatement {
        &self.query
    }

    pub fn take(&mut self) -> Self {
        Self {
            view: self.view.take(),
            columns: std::mem::take(&mut self.columns),
            query: self.query.take(),
            or_replace: self.or_replace,
            if_not_exists: self.if_not_exists,
            recursive: self.recursive,
            temporary: self.temporary,
            opt: self.opt.take(),
        }
    }
}

impl SchemaStatementBuilder for ViewCreateStatement {
    fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        let mut sql = String::with_capacity(256);
        schema_builder.prepare_view_create_statement(self, &mut sql);
        sql
    }

    fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
        let mut sql = String::with_capacity(256);
        schema_builder.prepare_view_create_statement(self, &mut sql);
        sql
    }
}
