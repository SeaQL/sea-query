use std::borrow::Cow;

use crate::*;

/// An arbitrary, dynamically-typed SQL expression.
///
/// It can be used in select fields, where clauses and many other places.
///
/// More concreterly, under the hood [`Expr`]s can be:
///
/// - Rust values
/// - SQL identifiers
/// - SQL function calls
/// - various operators and sub-queries
///
/// If something is not supported here, look into [`BinOper::Custom`],
/// [`Func::cust`], or [`Expr::cust*`][`Expr::cust_with_values`] as a
/// workaround, and consider reporting your issue.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Expr {
    Column(ColumnRef),
    Tuple(Vec<Expr>),
    Unary(UnOper, Box<Expr>),
    FunctionCall(FunctionCall),
    Binary(Box<Expr>, BinOper, Box<Expr>),
    SubQuery(Option<SubQueryOper>, Box<SubQueryStatement>),
    Value(Value),
    Values(Vec<Value>),
    Custom(Cow<'static, str>),
    CustomWithExpr(Cow<'static, str>, Vec<Expr>),
    Keyword(Keyword),
    AsEnum(DynIden, Box<Expr>),
    Case(Box<CaseStatement>),
    Constant(Value),
    TypeName(TypeRef),
}

impl Expr {
    #[deprecated(since = "0.29.0", note = "Please use the [`Asterisk`]")]
    pub fn asterisk() -> Self {
        Self::col(Asterisk)
    }

    /// Express the target column without table prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::SizeW).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// ```
    pub fn col<T>(n: T) -> Self
    where
        T: IntoColumnRef,
    {
        Self::Column(n.into_column_ref())
    }

    /// Express the target column without table prefix, returning a [`Expr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::column(Char::SizeW).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::column((Char::Table, Char::SizeW)).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// ```
    pub fn column<T>(n: T) -> Self
    where
        T: IntoColumnRef,
    {
        Self::Column(n.into_column_ref())
    }

    /// Wraps tuple of `Expr`, can be used for tuple comparison
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         Expr::tuple([Expr::col(Char::SizeW).into(), Expr::value(100)])
    ///             .lt(Expr::tuple([Expr::value(500), Expr::value(100)])))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (`size_w`, 100) < (500, 100)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w", 100) < (500, 100)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w", 100) < (500, 100)"#
    /// );
    /// ```
    pub fn tuple<I>(n: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        Self::Tuple(n.into_iter().collect::<Vec<Self>>())
    }

    #[deprecated(since = "0.29.0", note = "Please use the [`Asterisk`]")]
    pub fn table_asterisk<T>(t: T) -> Self
    where
        T: IntoIden,
    {
        Self::col((t.into_iden(), Asterisk))
    }

    /// Express a [`Value`], returning a [`Expr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::val(1).into())
    ///     .and_where(Expr::val(2.5).into())
    ///     .and_where(Expr::val("3").into())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// ```
    pub fn val<V>(v: V) -> Self
    where
        V: Into<Value>,
    {
        Self::from(v)
    }

    /// Wrap an expression to perform some operation on it later.
    ///
    /// Since `sea_query` 0.32.0 (`sea_orm` 1.1.1), **this is not necessary** in most cases!
    ///
    /// Some SQL operations used to be defined only as inherent methods on [`Expr`].
    /// Thus, to use them, you needed to manually convert from other types to [`Expr`].
    /// But now these operations are also defined as [`ExprTrait`] methods
    /// that can be called directly on any expression type,
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     // This is the old style, when `Expr::expr` was necessary:
    ///     .and_where(Expr::expr(Expr::col(Char::SizeW).if_null(0)).gt(2))
    ///     .to_owned();
    ///
    /// // But since 0.32.0, this compiles too:
    /// let _ = Expr::col(Char::SizeW).if_null(0).gt(2);
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE IFNULL(`size_w`, 0) > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE COALESCE("size_w", 0) > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE IFNULL("size_w", 0) > 2"#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     // This is the old style, when `Expr::expr` was necessary:
    ///     .and_where(Expr::expr(Func::lower(Expr::col(Char::Character))).is_in(["a", "b"]))
    ///     .to_owned();
    ///
    /// // But since 0.32.0, this compiles too:
    /// let _ = Func::lower(Expr::col(Char::Character)).is_in(["a", "b"]);
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE LOWER(`character`) IN ('a', 'b')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE LOWER("character") IN ('a', 'b')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE LOWER("character") IN ('a', 'b')"#
    /// );
    /// ```
    #[allow(clippy::self_named_constructors)]
    pub fn expr<T>(expr: T) -> Self
    where
        T: Into<Self>,
    {
        expr.into()
    }

    /// Express a [`Value`], returning a [`Expr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::value(1))
    ///     .and_where(Expr::value(2.5))
    ///     .and_where(Expr::value("3"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// ```
    pub fn value<V>(v: V) -> Self
    where
        V: Into<Self>,
    {
        v.into()
    }

    /// Express any custom expression in [`&str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::cust("1 = 1"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 = 1"#
    /// );
    /// ```
    pub fn cust<T>(s: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::Custom(s.into())
    }

    /// Express any custom expression with [`Value`]. Use this if your expression needs variables.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::Id).eq(1))
    ///     .and_where(Expr::cust_with_values("6 = ? * ?", [2, 3]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `id` = 1 AND (6 = 2 * 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "id" = 1 AND (6 = 2 * 3)"#
    /// );
    /// ```
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::Id).eq(1))
    ///     .and_where(Expr::cust_with_values("6 = $2 * $1", [3, 2]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "id" = 1 AND (6 = 2 * 3)"#
    /// );
    /// ```
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_values("6 = ? * ?", [2, 3]))
    ///     .to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT 6 = 2 * 3"#);
    /// assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT 6 = 2 * 3"#);
    /// ```
    /// Postgres only: use `$$` to escape `$`
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_values("$1 $$ $2", ["a", "b"]))
    ///     .to_owned();
    ///
    /// assert_eq!(query.to_string(PostgresQueryBuilder), r#"SELECT 'a' $ 'b'"#);
    /// ```
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_values("data @? ($1::JSONPATH)", ["hello"]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT data @? ('hello'::JSONPATH)"#
    /// );
    /// ```
    pub fn cust_with_values<T, V, I>(s: T, v: I) -> Self
    where
        T: Into<Cow<'static, str>>,
        V: Into<Value>,
        I: IntoIterator<Item = V>,
    {
        Self::CustomWithExpr(
            s.into(),
            v.into_iter()
                .map(|v| Into::<Value>::into(v).into())
                .collect(),
        )
    }

    /// Express any custom expression with [`Expr`]. Use this if your expression needs other expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::val(1).add(2))
    ///     .expr(Expr::cust_with_expr("data @? ($1::JSONPATH)", "hello"))
    ///     .to_owned();
    /// let (sql, values) = query.build(PostgresQueryBuilder);
    ///
    /// assert_eq!(sql, r#"SELECT $1 + $2, data @? ($3::JSONPATH)"#);
    /// assert_eq!(
    ///     values,
    ///     Values(vec![1i32.into(), 2i32.into(), "hello".into()])
    /// );
    /// ```
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_expr(
    ///         "json_agg(DISTINCT $1)",
    ///         Expr::col(Char::Character),
    ///     ))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT json_agg(DISTINCT "character")"#
    /// );
    /// ```
    pub fn cust_with_expr<T, E>(s: T, expr: E) -> Self
    where
        T: Into<Cow<'static, str>>,
        E: Into<Self>,
    {
        Self::CustomWithExpr(s.into(), vec![expr.into()])
    }

    /// Express any custom expression with [`Expr`]. Use this if your expression needs other expressions.
    pub fn cust_with_exprs<T, I>(s: T, v: I) -> Self
    where
        T: Into<Cow<'static, str>>,
        I: IntoIterator<Item = Expr>,
    {
        Self::CustomWithExpr(s.into(), v.into_iter().collect())
    }

    /// Express a `MAX` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).max())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MAX(`character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MAX("character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MAX("character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn max(self) -> Self {
        Func::max(self).into()
    }

    /// Express a `MIN` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).min())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MIN(`character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MIN("character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MIN("character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn min(self) -> Self {
        Func::min(self).into()
    }

    /// Express a `SUM` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).sum())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT SUM(`character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT SUM("character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT SUM("character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn sum(self) -> Self {
        Func::sum(self).into()
    }

    /// Express a `COUNT` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).count())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT COUNT(`character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COUNT("character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT COUNT("character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn count(self) -> Self {
        Func::count(self).into()
    }

    /// Express a `COUNT` function with the DISTINCT modifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).count_distinct())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT COUNT(DISTINCT `character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COUNT(DISTINCT "character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT COUNT(DISTINCT "character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn count_distinct(self) -> Self {
        Func::count_distinct(self).into()
    }

    /// Express a `IF NULL` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).if_null(0))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT IFNULL(`character`.`size_w`, 0) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COALESCE("character"."size_w", 0) FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT IFNULL("character"."size_w", 0) FROM "character""#
    /// );
    /// ```
    pub fn if_null<V>(self, v: V) -> Self
    where
        V: Into<Self>,
    {
        Func::if_null(self, v).into()
    }

    /// Express a `EXISTS` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr_as(Expr::exists(Query::select().column(Char::Id).from(Char::Table).take()), "character_exists")
    ///     .expr_as(Expr::exists(Query::select().column(Glyph::Id).from(Glyph::Table).take()), "glyph_exists")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT EXISTS(SELECT `id` FROM `character`) AS `character_exists`, EXISTS(SELECT `id` FROM `glyph`) AS `glyph_exists`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT EXISTS(SELECT "id" FROM "character") AS "character_exists", EXISTS(SELECT "id" FROM "glyph") AS "glyph_exists""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT EXISTS(SELECT "id" FROM "character") AS "character_exists", EXISTS(SELECT "id" FROM "glyph") AS "glyph_exists""#
    /// );
    /// ```
    pub fn exists(sel: SelectStatement) -> Self {
        Self::SubQuery(Some(SubQueryOper::Exists), Box::new(sel.into()))
    }

    /// Express a `NOT EXISTS` sub-query expression.
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr_as(Expr::not_exists(Query::select().column(Char::Id).from(Char::Table).take()), "character_exists")
    ///     .expr_as(Expr::not_exists(Query::select().column(Glyph::Id).from(Glyph::Table).take()), "glyph_exists")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT NOT EXISTS(SELECT `id` FROM `character`) AS `character_exists`, NOT EXISTS(SELECT `id` FROM `glyph`) AS `glyph_exists`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT NOT EXISTS(SELECT "id" FROM "character") AS "character_exists", NOT EXISTS(SELECT "id" FROM "glyph") AS "glyph_exists""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT NOT EXISTS(SELECT "id" FROM "character") AS "character_exists", NOT EXISTS(SELECT "id" FROM "glyph") AS "glyph_exists""#
    /// );
    /// ```
    pub fn not_exists(sel: SelectStatement) -> Self {
        Self::exists(sel).not()
    }

    /// Express a `ANY` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Id)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::Id).eq(Expr::any(
    ///         Query::select().column(Char::Id).from(Char::Table).take(),
    ///     )))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE `id` = ANY(SELECT `id` FROM `character`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "id" = ANY(SELECT "id" FROM "character")"#
    /// );
    /// ```
    pub fn any(sel: SelectStatement) -> Self {
        Self::SubQuery(Some(SubQueryOper::Any), Box::new(sel.into()))
    }

    /// Express a `SOME` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Id)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::Id).ne(Expr::some(
    ///         Query::select().column(Char::Id).from(Char::Table).take(),
    ///     )))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE `id` <> SOME(SELECT `id` FROM `character`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "id" <> SOME(SELECT "id" FROM "character")"#
    /// );
    /// ```
    pub fn some(sel: SelectStatement) -> Self {
        Self::SubQuery(Some(SubQueryOper::Some), Box::new(sel.into()))
    }

    /// Express a `ALL` sub-query expression.
    pub fn all(sel: SelectStatement) -> Self {
        Self::SubQuery(Some(SubQueryOper::All), Box::new(sel.into()))
    }

    /// Adds new `CASE WHEN` to existing case statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///         Expr::case(
    ///                 Expr::col((Glyph::Table, Glyph::Aspect)).is_in([2, 4]),
    ///                 true
    ///              )
    ///             .finally(false),
    ///          "is_even"
    ///     )
    ///     .from(Glyph::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT (CASE WHEN ("glyph"."aspect" IN (2, 4)) THEN TRUE ELSE FALSE END) AS "is_even" FROM "glyph""#
    /// );
    /// ```
    pub fn case<C, T>(cond: C, then: T) -> CaseStatement
    where
        C: IntoCondition,
        T: Into<Self>,
    {
        CaseStatement::new().case(cond, then)
    }

    /// Keyword `CURRENT_DATE`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::*;
    ///
    /// let query = Query::select().expr(Expr::current_date()).to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT CURRENT_DATE"#);
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CURRENT_DATE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CURRENT_DATE"#
    /// );
    /// ```
    pub fn current_date() -> Self {
        Self::Keyword(Keyword::CurrentDate)
    }

    /// Keyword `CURRENT_TIMESTAMP`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::*;
    ///
    /// let query = Query::select().expr(Expr::current_time()).to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT CURRENT_TIME"#);
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CURRENT_TIME"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CURRENT_TIME"#
    /// );
    /// ```
    pub fn current_time() -> Self {
        Self::Keyword(Keyword::CurrentTime)
    }

    /// Keyword `CURRENT_TIMESTAMP`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{Expr, MysqlQueryBuilder, PostgresQueryBuilder, Query, SqliteQueryBuilder};
    ///
    /// let query = Query::select().expr(Expr::current_timestamp()).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT CURRENT_TIMESTAMP"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CURRENT_TIMESTAMP"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CURRENT_TIMESTAMP"#
    /// );
    /// ```
    pub fn current_timestamp() -> Self {
        Self::Keyword(Keyword::CurrentTimestamp)
    }

    /// Keyword `DEFAULT`.
    ///
    /// SQLite does not support VALUES ​​(DEFAULT).
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{
    ///     Expr, MysqlQueryBuilder, PostgresQueryBuilder, Query, SqliteQueryBuilder, tests_cfg::*,
    /// };
    ///
    /// let query = Query::insert()
    ///     .columns([Char::Id])
    ///     .values_panic([Expr::keyword_default()])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT (`id`) VALUES (DEFAULT)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT ("id") VALUES (DEFAULT)"#
    /// );
    /// ```
    pub fn keyword_default() -> Self {
        Self::Keyword(Keyword::Default)
    }

    /// Custom keyword.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::*;
    ///
    /// let query = Query::select()
    ///     .expr(Expr::custom_keyword("test"))
    ///     .to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT test"#);
    /// assert_eq!(query.to_string(PostgresQueryBuilder), r#"SELECT test"#);
    /// assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT test"#);
    /// ```
    pub fn custom_keyword<T>(i: T) -> Self
    where
        T: IntoIden,
    {
        Self::Keyword(Keyword::Custom(i.into_iden()))
    }

    pub(crate) fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_, _, _))
    }

    pub(crate) fn get_bin_oper(&self) -> Option<&BinOper> {
        match self {
            Self::Binary(_, oper, _) => Some(oper),
            _ => None,
        }
    }
}

impl<T> From<T> for Expr
where
    T: Into<Value>,
{
    fn from(v: T) -> Self {
        Self::Value(v.into())
    }
}

impl From<Vec<Value>> for Expr {
    fn from(v: Vec<Value>) -> Self {
        Self::Values(v)
    }
}

impl From<SubQueryStatement> for Expr {
    fn from(v: SubQueryStatement) -> Self {
        Self::SubQuery(None, Box::new(v))
    }
}

macro_rules! from_into_subquery_expr {
    ($($ty:ty),+) => {
        $(
            impl From<$ty> for Expr {
                fn from(v: $ty) -> Self {
                    Self::SubQuery(None, Box::new(v.into()))
                }
            }
        )+
    };
}

from_into_subquery_expr!(
    WithQuery,
    DeleteStatement,
    UpdateStatement,
    InsertStatement,
    SelectStatement
);

impl From<FunctionCall> for Expr {
    fn from(func: FunctionCall) -> Self {
        Self::FunctionCall(func)
    }
}

impl From<ColumnRef> for Expr {
    fn from(col: ColumnRef) -> Self {
        Self::Column(col)
    }
}

impl From<Keyword> for Expr {
    fn from(k: Keyword) -> Self {
        Self::Keyword(k)
    }
}

impl From<LikeExpr> for Expr {
    fn from(like: LikeExpr) -> Self {
        match like.escape {
            Some(escape) => Self::Binary(
                Box::new(like.pattern.into()),
                BinOper::Escape,
                Box::new(Expr::Constant(escape.into())),
            ),
            None => like.pattern.into(),
        }
    }
}

impl From<TypeRef> for Expr {
    fn from(type_name: TypeRef) -> Self {
        Self::TypeName(type_name)
    }
}
