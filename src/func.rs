//! For calling built-in SQL functions.

use crate::{expr::*, types::*};

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
    Random,
    #[cfg(feature = "backend-postgres")]
    PgFunction(PgFunction),
}

/// Function call.
#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub(crate) func: Function,
    pub(crate) args: Vec<SimpleExpr>,
}

impl FunctionCall {
    pub(crate) fn new(func: Function) -> Self {
        Self {
            func,
            args: Vec::new(),
        }
    }

    pub fn arg<T>(mut self, arg: T) -> Self
    where
        T: Into<SimpleExpr>,
    {
        self.args = vec![arg.into()];
        self
    }

    pub fn args<I>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = SimpleExpr>,
    {
        self.args = args.into_iter().collect();
        self
    }
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
    ///     fn unquoted(&self, s: &mut dyn Write) {
    ///         write!(s, "MY_FUNCTION").unwrap();
    ///     }
    /// }
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
        FunctionCall::new(Function::Custom(func.into_iden()))
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
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::Max).arg(expr)
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
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::Min).arg(expr)
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
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::Sum).arg(expr)
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
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::Avg).arg(expr)
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
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::Abs).arg(expr)
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
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::Count).arg(expr)
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
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::CharLength).arg(expr)
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
        A: Into<SimpleExpr>,
        B: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::IfNull).args([a.into(), b.into()])
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
    pub fn cast_as<V, I>(expr: V, iden: I) -> FunctionCall
    where
        V: Into<SimpleExpr>,
        I: IntoIden,
    {
        let expr: SimpleExpr = expr.into();
        FunctionCall::new(Function::Cast).arg(expr.binary(
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
    ///         Query::select()
    ///             .from(Char::Table)
    ///             .expr(Func::max(Expr::col(Char::SizeW)))
    ///             .take()
    ///             .into(),
    ///         Expr::col(Char::SizeH).into(),
    ///         Expr::val(12).into(),
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
    pub fn coalesce<I>(args: I) -> FunctionCall
    where
        I: IntoIterator<Item = SimpleExpr>,
    {
        FunctionCall::new(Function::Coalesce).args(args)
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
    pub fn lower<T>(expr: T) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::Lower).arg(expr)
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
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::Upper).arg(expr)
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
        FunctionCall::new(Function::Random)
    }
}
