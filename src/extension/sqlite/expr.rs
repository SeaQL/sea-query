use crate::{Expr, SimpleExpr};

use super::SqliteBinOper;

pub trait SqliteExpr {
    fn matches<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>;

    fn get_json_field<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>;

    fn cast_json_field<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>;
}

impl SqliteExpr for Expr {
    fn matches<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.binary(SqliteBinOper::Match, right)
    }

    fn get_json_field<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.binary(SqliteBinOper::GetJsonField, right)
    }

    fn cast_json_field<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.binary(SqliteBinOper::CastJsonField, right)
    }
}

impl SqliteExpr for SimpleExpr {
    fn matches<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.binary(SqliteBinOper::Match, right)
    }

    fn get_json_field<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.binary(SqliteBinOper::GetJsonField, right)
    }

    fn cast_json_field<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.binary(SqliteBinOper::CastJsonField, right)
    }
}
