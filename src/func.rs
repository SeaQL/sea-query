use std::rc::Rc;
use crate::{expr::*, types::*};

/// Functions
#[derive(Clone)]
pub enum Function {
    Max,
    Min,
    Sum,
    Avg,
    Count,
    IfNull,
    Custom(Rc<dyn Iden>),
}

/// Function call helper.
#[derive(Clone)]
pub struct Func;

impl Func {

    /// Call a custom function.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
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
    pub fn cust<T: 'static>(func: T) -> Expr
        where T: Iden {
        Self::cust_dyn(Rc::new(func))
    }

    pub fn cust_dyn(func: Rc<dyn Iden>) -> Expr {
        Expr::func(Function::Custom(func))
    }

    /// Call `MAX` function.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
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
    ///     r#"SELECT MAX(`character`.`size_w`) FROM `character`"#
    /// );
    /// ```
    pub fn max<T>(expr: T) -> SimpleExpr
        where T: Into<SimpleExpr> {
        Expr::func(Function::Max).arg(expr)
    }

    /// Call `MIN` function.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
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
    ///     r#"SELECT MIN(`character`.`size_h`) FROM `character`"#
    /// );
    /// ```
    pub fn min<T>(expr: T) -> SimpleExpr
        where T: Into<SimpleExpr> {
        Expr::func(Function::Min).arg(expr)
    }

    /// Call `SUM` function.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
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
    ///     r#"SELECT SUM(`character`.`size_h`) FROM `character`"#
    /// );
    /// ```
    pub fn sum<T>(expr: T) -> SimpleExpr
        where T: Into<SimpleExpr> {
        Expr::func(Function::Sum).arg(expr)
    }

    /// Call `AVG` function.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
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
    ///     r#"SELECT AVG(`character`.`size_h`) FROM `character`"#
    /// );
    /// ```
    pub fn avg<T>(expr: T) -> SimpleExpr
        where T: Into<SimpleExpr> {
        Expr::func(Function::Avg).arg(expr)
    }

    /// Call `COUNT` function.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
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
    ///     r#"SELECT COUNT(`character`.`id`) FROM `character`"#
    /// );
    /// ```
    pub fn count<T>(expr: T) -> SimpleExpr
        where T: Into<SimpleExpr> {
        Expr::func(Function::Count).arg(expr)
    }
}