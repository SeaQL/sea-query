//! For calling built-in SQL functions.

use crate::{expr::*, types::*};

#[cfg(feature = "backend-postgres")]
pub use crate::extension::postgres::PgFunc;

/// Known SQL functions.
///
/// If something is not supported here, you can use [`Function::Custom`].
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Func {
    Max,
    Min,
    Sum,
    Avg,
    Abs,
    Count,
    IfNull,
    Greatest,
    Least,
    CharLength,
    Cast,
    Custom(DynIden),
    Coalesce,
    Lower,
    Upper,
    BitAnd,
    BitOr,
    Random,
    Round,
    Md5,
    #[cfg(feature = "backend-postgres")]
    PgFunction(PgFunc),
}

/// Type alias of [`Func`] for compatibility.
/// Previously, [`Func`] is a namespace for building [`FunctionCall`].
#[deprecated(since = "1.0.0", note = "use `Func` instead")]
pub type Function = Func;

impl Func {
    /// Call a custom function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// #[derive(Iden)]
    /// #[iden = "MY_FUNCTION"]
    /// struct MyFunction;
    ///
    /// let query = Query::select()
    ///     .expr(Func::cust(MyFunction).arg("hello"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MY_FUNCTION('hello')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MY_FUNCTION('hello')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MY_FUNCTION('hello')"#
    /// );
    /// ```
    pub fn cust<T>(func: T) -> FunctionCall
    where
        T: IntoIden,
    {
        FunctionCall::new(Func::Custom(func.into_iden()))
    }

    /// Call `MAX` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::max(Expr::col((Char::Table, Char::SizeW))))
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
    pub fn max<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Max).arg(expr)
    }

    /// Call `MIN` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::min(Expr::col((Char::Table, Char::SizeH))))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MIN(`character`.`size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MIN("character"."size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MIN("character"."size_h") FROM "character""#
    /// );
    /// ```
    pub fn min<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Min).arg(expr)
    }

    /// Call `SUM` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::sum(Expr::col((Char::Table, Char::SizeH))))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT SUM(`character`.`size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT SUM("character"."size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT SUM("character"."size_h") FROM "character""#
    /// );
    /// ```
    pub fn sum<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Sum).arg(expr)
    }

    /// Call `AVG` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::avg(Expr::col((Char::Table, Char::SizeH))))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT AVG(`character`.`size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT AVG("character"."size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT AVG("character"."size_h") FROM "character""#
    /// );
    /// ```
    pub fn avg<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Avg).arg(expr)
    }

    /// Call `ABS` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::abs(Expr::col((Char::Table, Char::SizeH))))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT ABS(`character`.`size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT ABS("character"."size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT ABS("character"."size_h") FROM "character""#
    /// );
    /// ```
    pub fn abs<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Abs).arg(expr)
    }

    /// Call `COUNT` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::count(Expr::col((Char::Table, Char::Id))))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT COUNT(`character`.`id`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COUNT("character"."id") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT COUNT("character"."id") FROM "character""#
    /// );
    /// ```
    pub fn count<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Count).arg(expr)
    }

    /// Call `COUNT` function with the `DISTINCT` modifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::count_distinct(Expr::col((Char::Table, Char::Id))))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT COUNT(DISTINCT `character`.`id`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COUNT(DISTINCT "character"."id") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT COUNT(DISTINCT "character"."id") FROM "character""#
    /// );
    /// ```
    pub fn count_distinct<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Count).arg_with(expr, FuncArgMod { distinct: true })
    }

    /// Call `CHAR_LENGTH` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::char_length(Expr::col((Char::Table, Char::Character))))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT CHAR_LENGTH(`character`.`character`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CHAR_LENGTH("character"."character") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT LENGTH("character"."character") FROM "character""#
    /// );
    /// ```
    pub fn char_length<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::CharLength).arg(expr)
    }

    /// Call `GREATEST` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::greatest([
    ///         Expr::col(Char::SizeW).into(),
    ///         Expr::col(Char::SizeH).into(),
    ///     ]))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT GREATEST(`size_w`, `size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT GREATEST("size_w", "size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MAX("size_w", "size_h") FROM "character""#
    /// );
    /// ```
    pub fn greatest<I>(args: I) -> FunctionCall
    where
        I: IntoIterator<Item = Expr>,
    {
        FunctionCall::new(Func::Greatest).args(args)
    }

    /// Call `LEAST` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::least([
    ///         Expr::col(Char::SizeW).into(),
    ///         Expr::col(Char::SizeH).into(),
    ///     ]))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT LEAST(`size_w`, `size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT LEAST("size_w", "size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MIN("size_w", "size_h") FROM "character""#
    /// );
    /// ```
    pub fn least<I>(args: I) -> FunctionCall
    where
        I: IntoIterator<Item = Expr>,
    {
        FunctionCall::new(Func::Least).args(args)
    }

    /// Call `IF NULL` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::if_null(
    ///         Expr::col(Char::SizeW),
    ///         Expr::col(Char::SizeH),
    ///     ))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT IFNULL(`size_w`, `size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COALESCE("size_w", "size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT IFNULL("size_w", "size_h") FROM "character""#
    /// );
    /// ```
    pub fn if_null<A, B>(a: A, b: B) -> FunctionCall
    where
        A: Into<Expr>,
        B: Into<Expr>,
    {
        FunctionCall::new(Func::IfNull).args([a.into(), b.into()])
    }

    /// Call `CAST` function with a custom type.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::cast_as("hello", "MyType"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT CAST('hello' AS MyType)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CAST('hello' AS MyType)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CAST('hello' AS MyType)"#
    /// );
    /// ```
    pub fn cast_as<V, I>(expr: V, iden: I) -> FunctionCall
    where
        V: Into<Expr>,
        I: IntoIden,
    {
        let expr: Expr = expr.into();
        FunctionCall::new(Func::Cast).arg(expr.binary(BinOper::As, Expr::cust(iden.into_iden().0)))
    }

    /// Call `CAST` function with a case-sensitive custom type.
    ///
    /// Type can be qualified with a schema name.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::cast_as_quoted("hello", "MyType"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT CAST('hello' AS `MyType`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CAST('hello' AS "MyType")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CAST('hello' AS "MyType")"#
    /// );
    ///
    /// // Also works with a schema-qualified type name:
    ///
    /// let query = Query::select()
    ///     .expr(Func::cast_as_quoted("hello", ("MySchema", "MyType")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT CAST('hello' AS `MySchema`.`MyType`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CAST('hello' AS "MySchema"."MyType")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CAST('hello' AS "MySchema"."MyType")"#
    /// );
    /// ```
    pub fn cast_as_quoted<V, I>(expr: V, r#type: I) -> FunctionCall
    where
        V: Into<Expr>,
        I: Into<TypeRef>,
    {
        let expr: Expr = expr.into();
        FunctionCall::new(Func::Cast).arg(expr.binary(BinOper::As, Expr::TypeName(r#type.into())))
    }

    /// Call `COALESCE` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::coalesce([
    ///         Query::select()
    ///             .from(Char::Table)
    ///             .expr(Func::max(Expr::col(Char::SizeW)))
    ///             .take()
    ///             .into(),
    ///         Expr::col(Char::SizeH),
    ///         Expr::val(12),
    ///     ]))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT COALESCE((SELECT MAX(`size_w`) FROM `character`), `size_h`, 12) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COALESCE((SELECT MAX("size_w") FROM "character"), "size_h", 12) FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT COALESCE((SELECT MAX("size_w") FROM "character"), "size_h", 12) FROM "character""#
    /// );
    /// ```
    pub fn coalesce<I, T>(args: I) -> FunctionCall
    where
        I: IntoIterator<Item = T>,
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Coalesce).args(args.into_iter().map(Into::into))
    }

    /// Call `LOWER` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::lower(Expr::col(Char::Character)))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT LOWER(`character`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT LOWER("character") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT LOWER("character") FROM "character""#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Font::Id)
    ///     .from(Font::Table)
    ///     .and_where(Func::lower(Expr::col(Font::Name)).eq("abc".trim().to_lowercase()))
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     "SELECT `id` FROM `font` WHERE LOWER(`name`) = 'abc'"
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "font" WHERE LOWER("name") = 'abc'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "id" FROM "font" WHERE LOWER("name") = 'abc'"#
    /// );
    /// ```
    pub fn lower<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Lower).arg(expr)
    }

    /// Call `UPPER` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::upper(Expr::col(Char::Character)))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT UPPER(`character`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT UPPER("character") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT UPPER("character") FROM "character""#
    /// );
    /// ```
    pub fn upper<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Upper).arg(expr)
    }

    /// Call `BIT_AND` function, this is not supported on SQLite.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::bit_and(Expr::col(Char::FontSize)))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT BIT_AND(`font_size`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT BIT_AND("font_size") FROM "character""#
    /// );
    /// ```
    pub fn bit_and<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::BitAnd).arg(expr)
    }

    /// Call `BIT_OR` function, this is not supported on SQLite.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::bit_or(Expr::col(Char::FontSize)))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT BIT_OR(`font_size`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT BIT_OR("font_size") FROM "character""#
    /// );
    /// ```
    pub fn bit_or<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::BitOr).arg(expr)
    }

    /// Call `ROUND` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::tests_cfg::Character::Character;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select().expr(Func::round(5.654)).to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT ROUND(5.654)"#);
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT ROUND(5.654)"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT ROUND(5.654)"#
    /// );
    /// ```
    pub fn round<A>(expr: A) -> FunctionCall
    where
        A: Into<Expr>,
    {
        FunctionCall::new(Func::Round).arg(expr)
    }

    /// Call `ROUND` function with the precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::tests_cfg::Character::Character;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::round_with_precision(5.654, 2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT ROUND(5.654, 2)"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT ROUND(5.654, 2)"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT ROUND(5.654, 2)"#
    /// );
    /// ```
    pub fn round_with_precision<A, B>(a: A, b: B) -> FunctionCall
    where
        A: Into<Expr>,
        B: Into<Expr>,
    {
        FunctionCall::new(Func::Round).args([a.into(), b.into()])
    }

    /// Call `RANDOM` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::tests_cfg::Character::Character;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select().expr(Func::random()).to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT RAND()"#);
    ///
    /// assert_eq!(query.to_string(PostgresQueryBuilder), r#"SELECT RANDOM()"#);
    ///
    /// assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT RANDOM()"#);
    /// ```
    pub fn random() -> FunctionCall {
        FunctionCall::new(Func::Random)
    }

    /// Call `MD5` function, this is only available in Postgres and MySQL.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::md5(Expr::col((Char::Table, Char::Character))))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MD5(`character`.`character`) FROM `character`"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MD5("character"."character") FROM "character""#
    /// );
    /// ```
    pub fn md5<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Md5).arg(expr)
    }
}

/// Function call.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub(crate) func: Func,
    pub(crate) args: Vec<Expr>,
    pub(crate) mods: Vec<FuncArgMod>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct FuncArgMod {
    pub distinct: bool,
}

impl FunctionCall {
    pub(crate) fn new(func: Func) -> Self {
        Self {
            func,
            args: Vec::new(),
            mods: Vec::new(),
        }
    }

    /// Append an argument to the function call
    pub fn arg<T>(self, arg: T) -> Self
    where
        T: Into<Expr>,
    {
        self.arg_with(arg, Default::default())
    }

    pub(crate) fn arg_with<T>(mut self, arg: T, mod_: FuncArgMod) -> Self
    where
        T: Into<Expr>,
    {
        self.args.push(arg.into());
        self.mods.push(mod_);
        self
    }

    /// Replace the arguments of the function call
    pub fn args<I>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = Expr>,
    {
        self.args = args.into_iter().collect();
        self.mods = vec![Default::default(); self.args.len()];
        self
    }

    pub fn get_func(&self) -> &Func {
        &self.func
    }

    pub fn get_args(&self) -> &[Expr] {
        &self.args
    }

    pub fn get_mods(&self) -> &[FuncArgMod] {
        &self.mods
    }
}
