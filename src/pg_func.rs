//! For calling built-in Postgres SQL functions.

use crate::{expr::*, func::Function};

/// Function call helper.
#[derive(Debug, Clone)]
pub struct PgFunc;

impl PgFunc {
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
    ///     .expr(PgFunc::to_tsquery(Expr::val("a & b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT TO_TSQUERY('a & b')"#
    /// );
    /// ```
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
    ///     .expr(PgFunc::to_tsvector(Expr::val("a b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT TO_TSVECTOR('a b')"#
    /// );
    /// ```
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
    ///     .expr(PgFunc::phraseto_tsquery(Expr::val("a b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PHRASETO_TSQUERY('a b')"#
    /// );
    /// ```
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
    ///     .expr(PgFunc::plainto_tsquery(Expr::val("a b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PLAINTO_TSQUERY('a b')"#
    /// );
    /// ```
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
    ///     .expr(PgFunc::websearch_to_tsquery(Expr::val("a b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT WEBSEARCH_TO_TSQUERY('a b')"#
    /// );
    /// ```
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
    ///     .expr(PgFunc::ts_rank(Expr::val("a b"), Expr::val("a&b")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT TS_RANK('a b', 'a&b')"#
    /// );
    /// ```
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
    ///     .expr(PgFunc::ts_rank_cd(Expr::val("a b"), Expr::val("a&b")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT TS_RANK_CD('a b', 'a&b')"#
    /// );
    /// ```
    pub fn ts_rank_cd<T>(vector: T, query: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        Expr::func(Function::TsRankCd).args(vec![vector, query])
    }
}
