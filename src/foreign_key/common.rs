use std::rc::Rc;
use crate::types::*;

/// Specification of a foreign key
#[derive(Clone)]
pub struct TableForeignKey {
    pub(crate) name: Option<String>,
    pub(crate) table: Option<Rc<dyn Iden>>,
    pub(crate) ref_table: Option<Rc<dyn Iden>>,
    pub(crate) columns: Vec<Rc<dyn Iden>>,
    pub(crate) ref_columns: Vec<Rc<dyn Iden>>,
    pub(crate) on_delete: Option<ForeignKeyAction>,
    pub(crate) on_update: Option<ForeignKeyAction>,
}

/// Foreign key on update & on delete actions
#[derive(Clone)]
pub enum ForeignKeyAction {
    Restrict,
    Cascade,
    SetNull,
    NoAction,
    SetDefault,
}

impl Default for TableForeignKey {
    fn default() -> Self {
        Self::new()
    }
}

impl TableForeignKey {
    /// Construct a new foreign key
    pub fn new() -> Self {
        Self {
            name: None,
            table: None,
            ref_table: None,
            columns: Vec::new(),
            ref_columns: Vec::new(),
            on_delete: None,
            on_update: None,
        }
    }

    /// Set foreign key name
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    /// Set key table and referencing table
    pub fn table<T: 'static, R: 'static>(&mut self, table: T, ref_table: R) -> &mut Self
        where T: Iden, R: Iden {
        self.table = Some(Rc::new(table));
        self.ref_table = Some(Rc::new(ref_table));
        self
    }

    /// Set key column and referencing column
    pub fn col<T: 'static, R: 'static>(&mut self, column: T, ref_column: R) -> &mut Self
        where T: Iden, R: Iden {
        self.columns.push(Rc::new(column));
        self.ref_columns.push(Rc::new(ref_column));
        self
    }

    /// Set on delete action
    pub fn on_delete(&mut self, action: ForeignKeyAction) -> &mut Self {
        self.on_delete = Some(action);
        self
    }

    /// Set on update action
    pub fn on_update(&mut self, action: ForeignKeyAction) -> &mut Self {
        self.on_update = Some(action);
        self
    }
}