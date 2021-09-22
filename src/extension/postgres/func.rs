//! For calling built-in Postgres SQL functions.

use crate::{expr::*, func::Function, PostgresQueryBuilder, QueryValue};

/// Functions
#[derive(Debug, Clone)]
pub enum PgFunction {
    ToTsquery,
    ToTsvector,
    PhrasetoTsquery,
    PlaintoTsquery,
    WebsearchToTsquery,
    TsRank,
    TsRankCd,
}

/// Function call helper.
#[derive(Debug, Clone)]
pub struct PgFunc;

impl<'a> PgFunc {
    /// Call `TO_TSQUERY` function. Postgres only.
    ///
    /// The parameter `regconfig` represents the OID of the text search configuration.
    /// If the value is `None` the argument is omitted from the query, and hence the database default used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::to_tsquery(Expr::val("a & b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(),
    ///     r#"SELECT TO_TSQUERY('a & b')"#
    /// );
    /// ```
    pub fn to_tsquery<T>(
        expr: T,
        regconfig: Option<&'a dyn QueryValue<PostgresQueryBuilder>>,
    ) -> SimpleExpr<'a, PostgresQueryBuilder>
    where
        T: Into<SimpleExpr<'a, PostgresQueryBuilder>>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config);
                Expr::func(Function::PgFunction(PgFunction::ToTsquery))
                    .args(vec![config, expr.into()])
            }
            None => Expr::func(Function::PgFunction(PgFunction::ToTsquery)).arg(expr),
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
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::to_tsvector(Expr::val("a b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(),
    ///     r#"SELECT TO_TSVECTOR('a b')"#
    /// );
    /// ```
    pub fn to_tsvector<T>(
        expr: T,
        regconfig: Option<&'a dyn QueryValue<PostgresQueryBuilder>>,
    ) -> SimpleExpr<'a, PostgresQueryBuilder>
    where
        T: Into<SimpleExpr<'a, PostgresQueryBuilder>>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config);
                Expr::func(Function::PgFunction(PgFunction::ToTsvector))
                    .args(vec![config, expr.into()])
            }
            None => Expr::func(Function::PgFunction(PgFunction::ToTsvector)).arg(expr),
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
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::phraseto_tsquery(Expr::val("a b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(),
    ///     r#"SELECT PHRASETO_TSQUERY('a b')"#
    /// );
    /// ```
    pub fn phraseto_tsquery<T>(
        expr: T,
        regconfig: Option<&'a dyn QueryValue<PostgresQueryBuilder>>,
    ) -> SimpleExpr<'a, PostgresQueryBuilder>
    where
        T: Into<SimpleExpr<'a, PostgresQueryBuilder>>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config);
                Expr::func(Function::PgFunction(PgFunction::PhrasetoTsquery))
                    .args(vec![config, expr.into()])
            }
            None => Expr::func(Function::PgFunction(PgFunction::PhrasetoTsquery)).arg(expr),
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
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::plainto_tsquery(Expr::val("a b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(),
    ///     r#"SELECT PLAINTO_TSQUERY('a b')"#
    /// );
    /// ```
    pub fn plainto_tsquery<T>(
        expr: T,
        regconfig: Option<&'a dyn QueryValue<PostgresQueryBuilder>>,
    ) -> SimpleExpr<'a, PostgresQueryBuilder>
    where
        T: Into<SimpleExpr<'a, PostgresQueryBuilder>>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config);
                Expr::func(Function::PgFunction(PgFunction::PlaintoTsquery))
                    .args(vec![config, expr.into()])
            }
            None => Expr::func(Function::PgFunction(PgFunction::PlaintoTsquery)).arg(expr),
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
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::websearch_to_tsquery(Expr::val("a b"), None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(),
    ///     r#"SELECT WEBSEARCH_TO_TSQUERY('a b')"#
    /// );
    /// ```
    pub fn websearch_to_tsquery<T>(
        expr: T,
        regconfig: Option<&'a dyn QueryValue<PostgresQueryBuilder>>,
    ) -> SimpleExpr<'a, PostgresQueryBuilder>
    where
        T: Into<SimpleExpr<'a, PostgresQueryBuilder>>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config);
                Expr::func(Function::PgFunction(PgFunction::WebsearchToTsquery))
                    .args(vec![config, expr.into()])
            }
            None => Expr::func(Function::PgFunction(PgFunction::WebsearchToTsquery)).arg(expr),
        }
    }

    /// Call `TS_RANK` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::ts_rank(Expr::val("a b"), Expr::val("a&b")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(),
    ///     r#"SELECT TS_RANK('a b', 'a&b')"#
    /// );
    /// ```
    pub fn ts_rank<T>(vector: T, query: T) -> SimpleExpr<'a, PostgresQueryBuilder>
    where
        T: Into<SimpleExpr<'a, PostgresQueryBuilder>>,
    {
        Expr::func(Function::PgFunction(PgFunction::TsRank)).args(vec![vector, query])
    }

    /// Call `TS_RANK_CD` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::ts_rank_cd(Expr::val("a b"), Expr::val("a&b")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(),
    ///     r#"SELECT TS_RANK_CD('a b', 'a&b')"#
    /// );
    /// ```
    pub fn ts_rank_cd<T>(vector: T, query: T) -> SimpleExpr<'a, PostgresQueryBuilder>
    where
        T: Into<SimpleExpr<'a, PostgresQueryBuilder>>,
    {
        Expr::func(Function::PgFunction(PgFunction::TsRankCd)).args(vec![vector, query])
    }
}
