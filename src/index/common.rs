use crate::types::*;
use std::rc::Rc;

/// Specification of a table index
#[derive(Debug, Clone)]
pub struct TableIndex {
    pub(crate) name: Option<String>,
    pub(crate) columns: Vec<IndexColumn>,
}

#[derive(Debug, Clone)]
pub struct IndexColumn {
    pub(crate) name: Rc<dyn Iden>,
    pub(crate) prefix: Option<u32>,
    pub(crate) order: Option<IndexOrder>,
}

#[derive(Debug, Clone)]
pub enum IndexOrder {
    Asc,
    Desc,
}

pub trait IntoIndexColumn {
    fn into_index_column(self) -> IndexColumn;
}

impl Default for TableIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl<I> IntoIndexColumn for I
where
    I: IntoIden,
{
    fn into_index_column(self) -> IndexColumn {
        IndexColumn {
            name: self.into_iden(),
            prefix: None,
            order: None,
        }
    }
}

impl<I> IntoIndexColumn for (I, u32)
where
    I: IntoIden,
{
    fn into_index_column(self) -> IndexColumn {
        IndexColumn {
            name: self.0.into_iden(),
            prefix: Some(self.1),
            order: None,
        }
    }
}

impl<I> IntoIndexColumn for (I, IndexOrder)
where
    I: IntoIden,
{
    fn into_index_column(self) -> IndexColumn {
        IndexColumn {
            name: self.0.into_iden(),
            prefix: None,
            order: Some(self.1),
        }
    }
}

impl<I> IntoIndexColumn for (I, u32, IndexOrder)
where
    I: IntoIden,
{
    fn into_index_column(self) -> IndexColumn {
        IndexColumn {
            name: self.0.into_iden(),
            prefix: Some(self.1),
            order: Some(self.2),
        }
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
    pub fn col(&mut self, col: IndexColumn) -> &mut Self {
        self.columns.push(col);
        self
    }
}
