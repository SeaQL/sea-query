//! For calling built-in SQL functions.

use crate::{expr::*, types::*};

/// Functions
#[derive(Debug, Clone)]
pub enum Function {
    Max,
    Min,
    Sum,
    Avg,
    Count,
    IfNull,
    CharLength,
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    ToTsquery,
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    ToTsvector,
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    PhrasetoTsquery,
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    PlaintoTsquery,
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    WebsearchToTsquery,
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    TsRank,
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    TsRankCd,
    Custom(DynIden),
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
    where
        T: Into<SimpleExpr>,
    {
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
    /// use sea_query::{*, tests_cfg::*};
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
    ///     r#"SELECT LENGTH(`character`.`character`) FROM `character`"#
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
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr(Func::if_null(Expr::col(Char::SizeW), Expr::col(Char::SizeH)))
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
    ///     r#"SELECT IFNULL(`size_w`, `size_h`) FROM `character`"#
    /// );
    /// ```
    pub fn if_null<A, B>(a: A, b: B) -> SimpleExpr
    where
        A: Into<SimpleExpr>,
        B: Into<SimpleExpr>,
    {
        Expr::func(Function::IfNull).args(vec![a.into(), b.into()])
    }

    /// Call `TO_TSQUERY` function. Postgres only.
    ///
    /// The parameter `regconfig` represents the OID of the text search configuration.
    /// If the value is `None` the argument is omitted from the query, and hence the database default used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr(Func::to_tsquery(Expr::val("a & b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT TO_TSQUERY('a & b')"#
    /// );
    /// ```
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    pub fn to_tsquery<T>(expr: T, regconfig: Option<u32>) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config.into());
                Expr::func(Function::ToTsquery).args(vec![config, expr.into()])
            }
            None => Expr::func(Function::ToTsquery).arg(expr),
        }
    }

    /// Call `TO_TSVECTOR` function. Postgres only.
    ///
    /// The parameter `regconfig` represents the OID of the text search configuration.
    /// If the value is `None` the argument is omitted from the query, and hence the database default used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr(Func::to_tsvector(Expr::val("a b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT TO_TSVECTOR('a b')"#
    /// );
    /// ```
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    pub fn to_tsvector<T>(expr: T, regconfig: Option<u32>) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config.into());
                Expr::func(Function::ToTsvector).args(vec![config, expr.into()])
            }
            None => Expr::func(Function::ToTsvector).arg(expr),
        }
    }

    /// Call `PHRASE_TO_TSQUERY` function. Postgres only.
    ///
    /// The parameter `regconfig` represents the OID of the text search configuration.
    /// If the value is `None` the argument is omitted from the query, and hence the database default used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr(Func::phraseto_tsquery(Expr::val("a b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PHRASETO_TSQUERY('a b')"#
    /// );
    /// ```
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    pub fn phraseto_tsquery<T>(expr: T, regconfig: Option<u32>) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config.into());
                Expr::func(Function::PhrasetoTsquery).args(vec![config, expr.into()])
            }
            None => Expr::func(Function::PhrasetoTsquery).arg(expr),
        }
    }

    /// Call `PLAIN_TO_TSQUERY` function. Postgres only.
    ///
    /// The parameter `regconfig` represents the OID of the text search configuration.
    /// If the value is `None` the argument is omitted from the query, and hence the database default used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr(Func::plainto_tsquery(Expr::val("a b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PLAINTO_TSQUERY('a b')"#
    /// );
    /// ```
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    pub fn plainto_tsquery<T>(expr: T, regconfig: Option<u32>) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config.into());
                Expr::func(Function::PlaintoTsquery).args(vec![config, expr.into()])
            }
            None => Expr::func(Function::PlaintoTsquery).arg(expr),
        }
    }

    /// Call `WEBSEARCH_TO_TSQUERY` function. Postgres only.
    ///
    /// The parameter `regconfig` represents the OID of the text search configuration.
    /// If the value is `None` the argument is omitted from the query, and hence the database default used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr(Func::websearch_to_tsquery(Expr::val("a b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT WEBSEARCH_TO_TSQUERY('a b')"#
    /// );
    /// ```
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    pub fn websearch_to_tsquery<T>(expr: T, regconfig: Option<u32>) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config.into());
                Expr::func(Function::WebsearchToTsquery).args(vec![config, expr.into()])
            }
            None => Expr::func(Function::WebsearchToTsquery).arg(expr),
        }
    }

    /// Call `TS_RANK` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr(Func::ts_rank(Expr::val("a b"), Expr::val("a&b")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT TS_RANK('a b', 'a&b')"#
    /// );
    /// ```
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    pub fn ts_rank<T>(vector: T, query: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        Expr::func(Function::TsRank).args(vec![vector, query])
    }

    /// Call `TS_RANK_CD` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr(Func::ts_rank_cd(Expr::val("a b"), Expr::val("a&b")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT TS_RANK_CD('a b', 'a&b')"#
    /// );
    /// ```
    #[cfg(feature = "backend-postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
    pub fn ts_rank_cd<T>(vector: T, query: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        Expr::func(Function::TsRankCd).args(vec![vector, query])
    }
}
