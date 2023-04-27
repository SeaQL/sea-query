//! For calling built-in Postgres SQL functions.

use crate::{expr::*, func::*};

/// Functions
#[derive(Debug, Clone, PartialEq)]
pub enum PgFunction {
    ToTsquery,
    ToTsvector,
    PhrasetoTsquery,
    PlaintoTsquery,
    WebsearchToTsquery,
    TsRank,
    TsRankCd,
    StartsWith,
    GenRandomUUID,
    #[cfg(feature = "postgres-array")]
    Any,
    #[cfg(feature = "postgres-array")]
    Some,
    #[cfg(feature = "postgres-array")]
    All,
}

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
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::to_tsquery("a & b", None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT TO_TSQUERY('a & b')"#
    /// );
    /// ```
    pub fn to_tsquery<T>(expr: T, regconfig: Option<u32>) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config.into());
                FunctionCall::new(Function::PgFunction(PgFunction::ToTsquery))
                    .args([config, expr.into()])
            }
            None => FunctionCall::new(Function::PgFunction(PgFunction::ToTsquery)).arg(expr),
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
    ///     .expr(PgFunc::to_tsvector("a b", None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT TO_TSVECTOR('a b')"#
    /// );
    /// ```
    pub fn to_tsvector<T>(expr: T, regconfig: Option<u32>) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config.into());
                FunctionCall::new(Function::PgFunction(PgFunction::ToTsvector))
                    .args([config, expr.into()])
            }
            None => FunctionCall::new(Function::PgFunction(PgFunction::ToTsvector)).arg(expr),
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
    ///     .expr(PgFunc::phraseto_tsquery("a b", None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PHRASETO_TSQUERY('a b')"#
    /// );
    /// ```
    pub fn phraseto_tsquery<T>(expr: T, regconfig: Option<u32>) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config.into());
                FunctionCall::new(Function::PgFunction(PgFunction::PhrasetoTsquery))
                    .args([config, expr.into()])
            }
            None => FunctionCall::new(Function::PgFunction(PgFunction::PhrasetoTsquery)).arg(expr),
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
    ///     .expr(PgFunc::plainto_tsquery("a b", None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PLAINTO_TSQUERY('a b')"#
    /// );
    /// ```
    pub fn plainto_tsquery<T>(expr: T, regconfig: Option<u32>) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config.into());
                FunctionCall::new(Function::PgFunction(PgFunction::PlaintoTsquery))
                    .args([config, expr.into()])
            }
            None => FunctionCall::new(Function::PgFunction(PgFunction::PlaintoTsquery)).arg(expr),
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
    ///     .expr(PgFunc::websearch_to_tsquery("a b", None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT WEBSEARCH_TO_TSQUERY('a b')"#
    /// );
    /// ```
    pub fn websearch_to_tsquery<T>(expr: T, regconfig: Option<u32>) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        match regconfig {
            Some(config) => {
                let config = SimpleExpr::Value(config.into());
                FunctionCall::new(Function::PgFunction(PgFunction::WebsearchToTsquery))
                    .args([config, expr.into()])
            }
            None => {
                FunctionCall::new(Function::PgFunction(PgFunction::WebsearchToTsquery)).arg(expr)
            }
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
    ///     .expr(PgFunc::ts_rank("a b", "a&b"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT TS_RANK('a b', 'a&b')"#
    /// );
    /// ```
    pub fn ts_rank<T>(vector: T, query: T) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::PgFunction(PgFunction::TsRank))
            .args([vector.into(), query.into()])
    }

    /// Call `TS_RANK_CD` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::ts_rank_cd("a b", "a&b"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT TS_RANK_CD('a b', 'a&b')"#
    /// );
    /// ```
    pub fn ts_rank_cd<T>(vector: T, query: T) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::PgFunction(PgFunction::TsRankCd))
            .args([vector.into(), query.into()])
    }

    /// Call `ANY` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select().expr(PgFunc::any(vec![0, 1])).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT ANY(ARRAY [0,1])"#
    /// );
    /// ```
    #[cfg(feature = "postgres-array")]
    pub fn any<T>(expr: T) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::PgFunction(PgFunction::Any)).arg(expr)
    }

    /// Call `SOME` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select().expr(PgFunc::some(vec![0, 1])).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT SOME(ARRAY [0,1])"#
    /// );
    /// ```
    #[cfg(feature = "postgres-array")]
    pub fn some<T>(expr: T) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::PgFunction(PgFunction::Some)).arg(expr)
    }

    /// Call `ALL` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select().expr(PgFunc::all(vec![0, 1])).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT ALL(ARRAY [0,1])"#
    /// );
    /// ```
    #[cfg(feature = "postgres-array")]
    pub fn all<T>(expr: T) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::PgFunction(PgFunction::All)).arg(expr)
    }

    /// Call `STARTS_WITH` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::starts_with("123", "1"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT STARTS_WITH('123', '1')"#
    /// );
    /// ```
    pub fn starts_with<T, P>(text: T, prefix: P) -> FunctionCall
    where
        T: Into<SimpleExpr>,
        P: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::PgFunction(PgFunction::StartsWith))
            .args([text.into(), prefix.into()])
    }

    /// Call `GEN_RANDOM_UUID` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select().expr(PgFunc::gen_random_uuid()).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT GEN_RANDOM_UUID()"#
    /// );
    /// ```
    pub fn gen_random_uuid() -> FunctionCall {
        FunctionCall::new(Function::PgFunction(PgFunction::GenRandomUUID))
    }
}
