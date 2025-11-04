use super::PgBinOper;
use crate::{Expr, ExprTrait, IntoLikeExpr};

/// Postgres-specific operator methods for building expressions.
pub trait PgExpr: ExprTrait {
    /// Express an postgres concatenate (`||`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::postgres::PgExpr, tests_cfg::*, *};
    ///
    /// let query = Query::select().expr(Expr::val("a").concatenate("b")).take();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT 'a' || 'b'"#
    /// );
    ///
    /// #[cfg(feature = "postgres-array")]
    /// {
    ///     let query = Query::select()
    ///         .expr(Expr::val(vec!["a".to_owned()]).concatenate(vec!["b".to_owned()]))
    ///         .take();
    ///
    ///     assert_eq!(
    ///         query.to_string(PostgresQueryBuilder),
    ///         r#"SELECT ARRAY['a'] || ARRAY['b']"#
    ///     );
    /// }
    /// ```
    fn concatenate<T>(self, right: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(PgBinOper::Concatenate, right)
    }

    /// Alias of [`PgExpr::concatenate`]
    fn concat<T>(self, right: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.concatenate(right)
    }

    /// Express an postgres fulltext search matches (`@@`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*, extension::postgres::PgExpr};
    ///
    /// let query = Query::select()
    ///     .columns([Font::Name, Font::Variant, Font::Language])
    ///     .from(Font::Table)
    ///     .and_where(Expr::val("a & b").matches("a b"))
    ///     .and_where(Expr::col(Font::Name).matches("a b"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "name", "variant", "language" FROM "font" WHERE 'a & b' @@ 'a b' AND "name" @@ 'a b'"#
    /// );
    /// ```
    fn matches<T>(self, expr: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(PgBinOper::Matches, expr)
    }

    /// Express an postgres fulltext search contains (`@>`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*, extension::postgres::PgExpr};
    ///
    /// let query = Query::select()
    ///     .columns([Font::Name, Font::Variant, Font::Language])
    ///     .from(Font::Table)
    ///     .and_where(Expr::val("a & b").contains("a b"))
    ///     .and_where(Expr::col(Font::Name).contains("a b"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "name", "variant", "language" FROM "font" WHERE 'a & b' @> 'a b' AND "name" @> 'a b'"#
    /// );
    /// ```
    fn contains<T>(self, expr: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(PgBinOper::Contains, expr)
    }

    /// Express an postgres fulltext search contained (`<@`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*, extension::postgres::PgExpr};
    ///
    /// let query = Query::select()
    ///     .columns([Font::Name, Font::Variant, Font::Language])
    ///     .from(Font::Table)
    ///     .and_where(Expr::val("a & b").contained("a b"))
    ///     .and_where(Expr::col(Font::Name).contained("a b"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "name", "variant", "language" FROM "font" WHERE 'a & b' <@ 'a b' AND "name" <@ 'a b'"#
    /// );
    /// ```
    fn contained<T>(self, expr: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(PgBinOper::Contained, expr)
    }

    /// Express a `ILIKE` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*, extension::postgres::PgExpr};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::Character)).ilike("Ours'%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" ILIKE E'Ours\'%'"#
    /// );
    /// ```
    fn ilike<L>(self, like: L) -> Expr
    where
        L: IntoLikeExpr,
    {
        self.binary(PgBinOper::ILike, like.into_like_expr())
    }

    /// Express a `NOT ILIKE` expression
    fn not_ilike<L>(self, like: L) -> Expr
    where
        L: IntoLikeExpr,
    {
        self.binary(PgBinOper::NotILike, like.into_like_expr())
    }

    /// Express a postgres retrieves JSON field as JSON value (`->`).
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::postgres::PgExpr, tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Font::Variant)
    ///     .from(Font::Table)
    ///     .and_where(Expr::col(Font::Variant).get_json_field("a"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "variant" FROM "font" WHERE "variant" -> 'a'"#
    /// );
    /// ```
    fn get_json_field<T>(self, right: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(PgBinOper::GetJsonField, right)
    }

    /// Express a postgres retrieves JSON field and casts it to an appropriate SQL type (`->>`).
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::postgres::PgExpr, tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Font::Variant)
    ///     .from(Font::Table)
    ///     .and_where(Expr::col(Font::Variant).cast_json_field("a"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "variant" FROM "font" WHERE "variant" ->> 'a'"#
    /// );
    /// ```
    fn cast_json_field<T>(self, right: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(PgBinOper::CastJsonField, right)
    }
}

/// You should be able to use Postgres-specific operators with all types of expressions.
impl<T> PgExpr for T where T: ExprTrait {}
