use crate::expr::SimpleExpr;
use crate::{types::*, FunctionCall};

/// Specification of a table index
#[derive(Default, Debug, Clone)]
pub struct TableIndex {
    pub(crate) name: Option<String>,
    pub(crate) columns: Vec<IndexColumn>,
}

#[derive(Debug, Clone)]
pub struct IndexColumn {
    pub(crate) name: Option<DynIden>,
    pub(crate) prefix: Option<u32>,
    pub(crate) order: Option<IndexOrder>,
    pub(crate) expr: Option<SimpleExpr>,
}

#[derive(Debug, Clone)]
pub enum IndexOrder {
    Asc,
    Desc,
}

pub trait IntoIndexColumn {
    fn into_index_column(self) -> IndexColumn;
}

impl IntoIndexColumn for IndexColumn {
    fn into_index_column(self) -> IndexColumn {
        self
    }
}

impl<I> IntoIndexColumn for I
where
    I: IntoIden,
{
    fn into_index_column(self) -> IndexColumn {
        IndexColumn {
            name: Some(self.into_iden()),
            prefix: None,
            order: None,
            expr: None,
        }
    }
}

impl<I> IntoIndexColumn for (I, u32)
where
    I: IntoIden,
{
    fn into_index_column(self) -> IndexColumn {
        IndexColumn {
            name: Some(self.0.into_iden()),
            prefix: Some(self.1),
            order: None,
            expr: None,
        }
    }
}

impl<I> IntoIndexColumn for (I, IndexOrder)
where
    I: IntoIden,
{
    fn into_index_column(self) -> IndexColumn {
        IndexColumn {
            name: Some(self.0.into_iden()),
            prefix: None,
            order: Some(self.1),
            expr: None,
        }
    }
}

impl<I> IntoIndexColumn for (I, u32, IndexOrder)
where
    I: IntoIden,
{
    fn into_index_column(self) -> IndexColumn {
        IndexColumn {
            name: Some(self.0.into_iden()),
            prefix: Some(self.1),
            order: Some(self.2),
            expr: None,
        }
    }
}

impl IntoIndexColumn for FunctionCall {
    fn into_index_column(self) -> IndexColumn {
        IndexColumn {
            name: None,
            prefix: None,
            order: None,
            expr: Some(self.into()),
        }
    }
}

impl IntoIndexColumn for (FunctionCall, IndexOrder) {
    fn into_index_column(self) -> IndexColumn {
        IndexColumn {
            name: None,
            prefix: None,
            order: Some(self.1),
            expr: Some(self.0.into()),
        }
    }
}

impl IntoIndexColumn for SimpleExpr {
    fn into_index_column(self) -> IndexColumn {
        IndexColumn {
            name: None,
            prefix: None,
            order: None,
            expr: Some(self),
        }
    }
}

impl IntoIndexColumn for (SimpleExpr, IndexOrder) {
    fn into_index_column(self) -> IndexColumn {
        IndexColumn {
            name: None,
            prefix: None,
            order: Some(self.1),
            expr: Some(self.0),
        }
    }
}

impl TableIndex {
    /// Construct a new table index
    pub fn new() -> Self {
        Self::default()
    }

    /// Set index name
    pub fn name<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    /// Set index column
    pub fn col(&mut self, col: IndexColumn) -> &mut Self {
        self.columns.push(col);
        self
    }

    pub fn get_column_names(&self) -> Vec<String> {
        self.columns
            .iter()
            .filter_map(|col| col.name.as_ref().map(|name| name.to_string()))
            .collect()
    }

    pub fn take(&mut self) -> Self {
        Self {
            name: self.name.take(),
            columns: std::mem::take(&mut self.columns),
        }
    }
}
