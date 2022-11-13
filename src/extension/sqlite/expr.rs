use crate::{Expr, Expression, SimpleExpr};

use super::SqliteBinOper;

pub trait SqliteExpr: Expression {
    /// Express an sqlite `MATCH` operator.
    fn matches<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.bin_op(SqliteBinOper::Match, right)
    }

    /// Express an sqlite retrieves JSON field as JSON value (`->`).
    fn get_json_field<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.bin_op(SqliteBinOper::GetJsonField, right)
    }

    /// Express an sqlite retrieves JSON field and casts it to an appropriate SQL type (`->>`).
    fn cast_json_field<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.bin_op(SqliteBinOper::CastJsonField, right)
    }
}

impl SqliteExpr for Expr {}

impl SqliteExpr for SimpleExpr {}
