use crate::{Expr, ExprTrait};

use super::MysqlBinOper;

/// MySQL-specific operator methods for building expressions.
pub trait MysqlExpr: ExprTrait {
    /// Express a MySQL retrieves JSON value referenced by a path as a JSON document (`->`).
    ///
    /// `col -> path` is a shorthand for `JSON_EXTRACT(col, path)`, where `path`
    /// is a JSON path expression such as `"$.a"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::mysql::MysqlExpr, tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Font::Variant)
    ///     .from(Font::Table)
    ///     .and_where(Expr::col(Font::Variant).get_json_field("$.a"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `variant` FROM `font` WHERE `variant` -> '$.a'"#
    /// );
    /// ```
    fn get_json_field<T>(self, right: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(MysqlBinOper::GetJsonField, right)
    }

    /// Express a MySQL retrieves JSON value referenced by a path and unquotes it to a text value (`->>`).
    ///
    /// `col ->> path` is a shorthand for `JSON_UNQUOTE(JSON_EXTRACT(col, path))`,
    /// where `path` is a JSON path expression such as `"$.a"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::mysql::MysqlExpr, tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Font::Variant)
    ///     .from(Font::Table)
    ///     .and_where(Expr::col(Font::Variant).cast_json_field("$.a"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `variant` FROM `font` WHERE `variant` ->> '$.a'"#
    /// );
    /// ```
    fn cast_json_field<T>(self, right: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(MysqlBinOper::CastJsonField, right)
    }
}

/// You should be able to use MySQL-specific operators with all types of expressions.
impl<T> MysqlExpr for T where T: ExprTrait {}
