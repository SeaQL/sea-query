pub use expr::SqliteExpr;

use crate::types::BinOper;

mod expr;

/// Sqlite-specific binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteBinOper {
    /// `MATCH`.
    Match,
    /// `->`. Retrieves JSON field as JSON value.
    GetJsonField,
    /// `->>`. Retrieves JSON field and casts it to an appropriate SQL type.
    CastJsonField,
}

impl From<SqliteBinOper> for BinOper {
    fn from(o: SqliteBinOper) -> Self {
        Self::SqliteOperator(o)
    }
}
