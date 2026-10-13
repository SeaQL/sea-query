mod column;
mod explain;
mod expr;
mod index;
mod select;

pub use column::*;
pub use explain::ExplainTable;
pub use expr::MysqlExpr;
pub use index::*;
pub use select::*;

pub(crate) use explain::ExplainTableTarget;
pub(crate) use explain::MySqlExplainOptions;
pub(crate) use explain::MySqlExplainSchemaSpec;

use crate::types::BinOper;

/// MySQL-specific binary operators.
///
/// For all supported operators (including the standard ones), see [`BinOper`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MysqlBinOper {
    /// `->`. Retrieves a JSON value referenced by a path as a JSON document.
    GetJsonField,
    /// `->>`. Retrieves a JSON value referenced by a path and unquotes it to a text value.
    CastJsonField,
}

impl From<MysqlBinOper> for BinOper {
    fn from(o: MysqlBinOper) -> Self {
        Self::MysqlOperator(o)
    }
}
