use std::rc::Rc;
use crate::types::*;

/// Specification of a table index
#[derive(Debug, Clone)]
pub struct TableIndex {
    pub(crate) name: Option<String>,
    pub(crate) columns: Vec<Rc<dyn Iden>>,
}

impl Default for TableIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl TableIndex {
    /// Construct a new table index
    pub fn new() -> Self {
        Self {
            name: None,
            columns: Vec::new(),
        }
    }

    /// Set index name
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    /// Set index column
    pub fn col<T: 'static>(&mut self, column: T) -> &mut Self
        where T: Iden {
        self.columns.push(Rc::new(column));
        self
    }
}