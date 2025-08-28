use crate::expr::Expr;
use crate::{FunctionCall, types::*};

/// Specification of a table index
#[derive(Default, Debug, Clone)]
pub struct TableIndex {
    pub(crate) name: Option<String>,
    pub(crate) columns: Vec<IndexColumn>,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum IndexColumn {
    TableColumn(IndexColumnTableColumn),
    Expr(IndexColumnExpr),
}

#[derive(Debug, Clone)]
pub struct IndexColumnTableColumn {
    pub(crate) name: DynIden,
    pub(crate) prefix: Option<u32>,
    pub(crate) order: Option<IndexOrder>,
}

#[derive(Debug, Clone)]
pub struct IndexColumnExpr {
    pub(crate) expr: Expr,
    pub(crate) order: Option<IndexOrder>,
}

impl IndexColumn {
    pub(crate) fn name(&self) -> Option<&DynIden> {
        match self {
            IndexColumn::TableColumn(IndexColumnTableColumn { name, .. }) => Some(name),
            IndexColumn::Expr(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum IndexOrder {
    Asc,
    Desc,
}

pub trait IntoIndexColumn: Into<IndexColumn> {
    fn into_index_column(self) -> IndexColumn;
}

impl<T> IntoIndexColumn for T
where
    T: Into<IndexColumn>,
{
    fn into_index_column(self) -> IndexColumn {
        self.into()
    }
}

impl<I> From<I> for IndexColumn
where
    I: IntoIden,
{
    fn from(value: I) -> Self {
        IndexColumn::TableColumn(IndexColumnTableColumn {
            name: value.into_iden(),
            prefix: None,
            order: None,
        })
    }
}

impl<I> From<(I, u32)> for IndexColumn
where
    I: IntoIden,
{
    fn from(value: (I, u32)) -> Self {
        IndexColumn::TableColumn(IndexColumnTableColumn {
            name: value.0.into_iden(),
            prefix: Some(value.1),
            order: None,
        })
    }
}

impl<I> From<(I, IndexOrder)> for IndexColumn
where
    I: IntoIden,
{
    fn from(value: (I, IndexOrder)) -> Self {
        IndexColumn::TableColumn(IndexColumnTableColumn {
            name: value.0.into_iden(),
            prefix: None,
            order: Some(value.1),
        })
    }
}

impl<I> From<(I, u32, IndexOrder)> for IndexColumn
where
    I: IntoIden,
{
    fn from(value: (I, u32, IndexOrder)) -> Self {
        IndexColumn::TableColumn(IndexColumnTableColumn {
            name: value.0.into_iden(),
            prefix: Some(value.1),
            order: Some(value.2),
        })
    }
}

impl From<FunctionCall> for IndexColumn {
    fn from(value: FunctionCall) -> Self {
        IndexColumn::Expr(IndexColumnExpr {
            expr: value.into(),
            order: None,
        })
    }
}

impl From<(FunctionCall, IndexOrder)> for IndexColumn {
    fn from(value: (FunctionCall, IndexOrder)) -> Self {
        IndexColumn::Expr(IndexColumnExpr {
            expr: value.0.into(),
            order: Some(value.1),
        })
    }
}

impl From<Expr> for IndexColumn {
    fn from(value: Expr) -> Self {
        IndexColumn::Expr(IndexColumnExpr {
            expr: value,
            order: None,
        })
    }
}

impl From<(Expr, IndexOrder)> for IndexColumn {
    fn from(value: (Expr, IndexOrder)) -> Self {
        IndexColumn::Expr(IndexColumnExpr {
            expr: value.0,
            order: Some(value.1),
        })
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
            .filter_map(|col| col.name().map(|name| name.to_string()))
            .collect()
    }

    pub fn take(&mut self) -> Self {
        Self {
            name: self.name.take(),
            columns: std::mem::take(&mut self.columns),
        }
    }
}
