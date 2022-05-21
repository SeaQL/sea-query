//! For calling built-in SQL functions.

use crate::{expr::*, types::*, Value};

#[cfg(feature = "backend-postgres")]
pub use crate::extension::postgres::{PgFunc, PgFunction};

/// Functions
#[derive(Debug, Clone)]
pub enum Function {
    Max,
    Min,
    Sum,
    Avg,
    Abs,
    Count,
    IfNull,
    CharLength,
    Cast,
    Custom(DynIden),
    Coalesce,
    Lower,
    Upper,
    #[cfg(feature = "backend-postgres")]
    PgFunction(PgFunction),
}

/// Function call helper.
#[derive(Debug, Clone)]
pub struct Func;

impl Func {
    /// Call a custom function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// struct MyFunction;
    ///
    /// impl Iden for MyFunction {
    ///     fn unquoted(&self, s: &mut dyn FmtWrite) {
    ///         write!(s, "MY_FUNCTION").unwrap();
    ///     }
    /// }
    ///
    /// let query = Query::select()
    ///     .expr(Func::cust(MyFunction).args(vec![Expr::val("hello")]))
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
    pub fn cust<T>(func: T) -> Expr
    where
        T: IntoIden,
    {
        Expr::func(Function::Custom(func.into_iden()))
    }

    /// Call `MAX` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::max(Expr::tbl(Char::Table, Char::SizeW)))
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
    pub fn max<T>(expr: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        Expr::func(Function::Max).arg(expr)
    }

    /// Call `MIN` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::min(Expr::tbl(Char::Table, Char::SizeH)))
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
    pub fn min<T>(expr: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        Expr::func(Function::Min).arg(expr)
    }

    /// Call `SUM` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::sum(Expr::tbl(Char::Table, Char::SizeH)))
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
    pub fn sum<T>(expr: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        Expr::func(Function::Sum).arg(expr)
    }

    /// Call `AVG` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::avg(Expr::tbl(Char::Table, Char::SizeH)))
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
    pub fn avg<T>(expr: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        Expr::func(Function::Avg).arg(expr)
    }

    /// Call `ABS` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::abs(Expr::tbl(Char::Table, Char::SizeH)))
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
    pub fn abs<T>(expr: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        Expr::func(Function::Abs).arg(expr)
    }

    /// Call `COUNT` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::count(Expr::tbl(Char::Table, Char::Id)))
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
    pub fn count<T>(expr: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        Expr::func(Function::Count).arg(expr)
    }

    /// Call `CHAR_LENGTH` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::char_length(Expr::tbl(Char::Table, Char::Character)))
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
    pub fn char_length<T>(expr: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        Expr::func(Function::CharLength).arg(expr)
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
    pub fn if_null<A, B>(a: A, b: B) -> SimpleExpr
    where
        A: Into<SimpleExpr>,
        B: Into<SimpleExpr>,
    {
        Expr::func(Function::IfNull).args(vec![a.into(), b.into()])
    }

    /// Call `CAST` function with a custom type.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Func::cast_as("hello", Alias::new("MyType")))
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
    pub fn cast_as<V, I>(value: V, iden: I) -> SimpleExpr
    where
        V: Into<Value>,
        I: IntoIden,
    {
        Expr::func(Function::Cast).arg(Expr::val(value.into()).bin_oper(
            BinOper::As,
            Expr::cust(iden.into_iden().to_string().as_str()),
        ))
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
    ///         Expr::col(Char::SizeW),
    ///         Expr::col(Char::SizeH),
    ///         Expr::val(12),
    ///     ]))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT COALESCE(`size_w`, `size_h`, 12) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COALESCE("size_w", "size_h", 12) FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT COALESCE("size_w", "size_h", 12) FROM "character""#
    /// );
    /// ```
    pub fn coalesce<I, T>(args: I) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
        I: IntoIterator<Item = T>,
    {
        Expr::func(Function::Coalesce).args(args)
    }

    /// Call `LOWER` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::tests_cfg::Character::Character;
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
    pub fn lower<T>(expr: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        Expr::func(Function::Lower).arg(expr)
    }

    /// Call `UPPER` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::tests_cfg::Character::Character;
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
    pub fn upper<T>(expr: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        Expr::func(Function::Upper).arg(expr)
    }
}
