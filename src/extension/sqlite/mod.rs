use crate::types::BinOper;

/// Sqlite-specific binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteBinOper {
    /// 'MATCH'.
    Match,
    /// '->'. Retreives JSON field as JSON value.
    GetJsonField,
    /// '->>'. Retreives JSON field and casts it to an appropriate SQL type.
    CastJsonField,
}

impl From<SqliteBinOper> for BinOper {
    fn from(o: SqliteBinOper) -> Self {
        Self::SqliteOperator(o)
    }
}
