//! For calling built-in Postgres SQL functions.

use crate::{PgDateTruncUnit, expr::*, func::*};

pub mod json_exists;
pub mod json_fn;
pub mod json_query;
pub mod json_table;
pub mod json_value;

/// Known Postgres-specific functions.
///
/// For all supported functions (including the standard ones), see [`Function`].
///
/// If something is not supported, you can use [`Function::Custom`].
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum PgFunc {
    ToTsquery,
    ToTsvector,
    PhrasetoTsquery,
    PlaintoTsquery,
    WebsearchToTsquery,
    TsRank,
    TsRankCd,
    StartsWith,
    GenRandomUUID,
    JsonBuildObject,
    JsonbBuildObject,
    JsonAgg,
    JsonbAgg,
    ArrayAgg,
    DateTrunc,
    Any,
    Some,
    All,
    AdvisoryLock,
    AdvisoryLockShared,
    TryAdvisoryLock,
    TryAdvisoryLockShared,
    AdvisoryUnlock,
    AdvisoryUnlockShared,
    AdvisoryUnlockAll,
    AdvisoryXactLock,
    AdvisoryXactLockShared,
    TryAdvisoryXactLock,
    TryAdvisoryXactLockShared,
}

impl From<PgFunc> for Func {
    fn from(func: PgFunc) -> Self {
        Self::PgFunction(func)
    }
}

/// Type alias of [`PgFunc`] for compatibility.
/// Previously, [`PgFunc`] is a namespace for building [`FunctionCall`].
#[deprecated(since = "1.0.0", note = "use `PgFunc` instead")]
pub type PgFunction = PgFunc;

/// Key accepted by PostgreSQL advisory lock functions, such as [`PgFunc::advisory_lock`].
///
/// # Examples
///
/// ```
/// use sea_query::PgAdvisoryLockKey;
///
/// assert_eq!(
///     PgAdvisoryLockKey::from(12345),
///     PgAdvisoryLockKey::BigInt(12345)
/// );
/// assert_eq!(
///     PgAdvisoryLockKey::from((1, 42)),
///     PgAdvisoryLockKey::IntPair(1, 42)
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum PgAdvisoryLockKey {
    BigInt(i64),
    IntPair(i32, i32),
    Expr(Expr),
    ExprPair(Expr, Expr),
}

impl From<i64> for PgAdvisoryLockKey {
    fn from(key: i64) -> Self {
        Self::BigInt(key)
    }
}

impl From<(i32, i32)> for PgAdvisoryLockKey {
    fn from((key1, key2): (i32, i32)) -> Self {
        Self::IntPair(key1, key2)
    }
}

impl From<Expr> for PgAdvisoryLockKey {
    fn from(expr: Expr) -> Self {
        Self::Expr(expr)
    }
}

impl From<(Expr, Expr)> for PgAdvisoryLockKey {
    fn from((key1, key2): (Expr, Expr)) -> Self {
        Self::ExprPair(key1, key2)
    }
}

impl PgAdvisoryLockKey {
    fn call(self, func: PgFunc) -> FunctionCall {
        let call = FunctionCall::new(func);
        match self {
            Self::BigInt(key) => call.arg(key),
            Self::IntPair(key1, key2) => call.arg(key1).arg(key2),
            Self::Expr(expr) => call.arg(expr),
            Self::ExprPair(key1, key2) => call.arg(key1).arg(key2),
        }
    }
}

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
        T: Into<Expr>,
    {
        match regconfig {
            Some(config) => {
                let config = Expr::Value(config.into());
                FunctionCall::new(PgFunc::ToTsquery).args([config, expr.into()])
            }
            None => FunctionCall::new(PgFunc::ToTsquery).arg(expr),
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
        T: Into<Expr>,
    {
        match regconfig {
            Some(config) => {
                let config = Expr::Value(config.into());
                FunctionCall::new(PgFunc::ToTsvector).args([config, expr.into()])
            }
            None => FunctionCall::new(PgFunc::ToTsvector).arg(expr),
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
        T: Into<Expr>,
    {
        match regconfig {
            Some(config) => {
                let config = Expr::Value(config.into());
                FunctionCall::new(PgFunc::PhrasetoTsquery).args([config, expr.into()])
            }
            None => FunctionCall::new(PgFunc::PhrasetoTsquery).arg(expr),
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
        T: Into<Expr>,
    {
        match regconfig {
            Some(config) => {
                let config = Expr::Value(config.into());
                FunctionCall::new(PgFunc::PlaintoTsquery).args([config, expr.into()])
            }
            None => FunctionCall::new(PgFunc::PlaintoTsquery).arg(expr),
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
        T: Into<Expr>,
    {
        match regconfig {
            Some(config) => {
                let config = Expr::Value(config.into());
                FunctionCall::new(PgFunc::WebsearchToTsquery).args([config, expr.into()])
            }
            None => FunctionCall::new(PgFunc::WebsearchToTsquery).arg(expr),
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
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::TsRank).args([vector.into(), query.into()])
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
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::TsRankCd).args([vector.into(), query.into()])
    }

    /// Call `ANY` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(feature = "postgres-array")]
    /// # {
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select().expr(PgFunc::any(vec![0, 1])).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT ANY(ARRAY [0,1])"#
    /// );
    /// # }
    /// ```
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column("id")
    ///     .from("post")
    ///     .and_where(Expr::col("tag").eq(PgFunc::any(Expr::cust("string_to_array('a,b,c', ',')"))))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "post" WHERE "tag" = ANY(string_to_array('a,b,c', ','))"#
    /// );
    /// ```
    pub fn any<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::Any).arg(expr)
    }

    /// Call `SOME` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(feature = "postgres-array")]
    /// # {
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select().expr(PgFunc::some(vec![0, 1])).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT SOME(ARRAY [0,1])"#
    /// );
    /// # }
    /// ```
    pub fn some<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::Some).arg(expr)
    }

    /// Call `ALL` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(feature = "postgres-array")]
    /// # {
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select().expr(PgFunc::all(vec![0, 1])).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT ALL(ARRAY [0,1])"#
    /// );
    /// # }
    /// ```
    pub fn all<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::All).arg(expr)
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
        T: Into<Expr>,
        P: Into<Expr>,
    {
        FunctionCall::new(PgFunc::StartsWith).args([text.into(), prefix.into()])
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
        FunctionCall::new(PgFunc::GenRandomUUID)
    }

    /// Call the `JSON_BUILD_OBJECT` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::json_build_object(vec![
    ///         (Expr::val("a"), Expr::val(1)),
    ///         (Expr::val("b"), Expr::val("2")),
    ///     ]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_BUILD_OBJECT('a', 1, 'b', '2')"#
    /// );
    /// ```
    pub fn json_build_object<T>(pairs: Vec<(T, T)>) -> FunctionCall
    where
        T: Into<Expr>,
    {
        let mut args = vec![];
        for (key, value) in pairs {
            args.push(key.into());
            args.push(value.into());
        }
        FunctionCall::new(PgFunc::JsonBuildObject).args(args)
    }

    /// Call the `JSONB_BUILD_OBJECT` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::jsonb_build_object(vec![
    ///         (Expr::val("a"), Expr::val(1)),
    ///         (Expr::val("b"), Expr::val("2")),
    ///     ]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSONB_BUILD_OBJECT('a', 1, 'b', '2')"#
    /// );
    /// ```
    pub fn jsonb_build_object<T>(pairs: Vec<(T, T)>) -> FunctionCall
    where
        T: Into<Expr>,
    {
        let mut args = vec![];
        for (key, value) in pairs {
            args.push(key.into());
            args.push(value.into());
        }
        FunctionCall::new(PgFunc::JsonbBuildObject).args(args)
    }

    /// Call the `DATE_TRUNC` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::date_trunc(
    ///         PgDateTruncUnit::Day,
    ///         Expr::val("2020-01-01"),
    ///     ))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT DATE_TRUNC('day', '2020-01-01')"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::date_trunc(
    ///         PgDateTruncUnit::Microseconds,
    ///         Expr::val("2020-01-01"),
    ///     ))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT DATE_TRUNC('microseconds', '2020-01-01')"#
    /// );
    /// ```
    pub fn date_trunc<T>(unit: PgDateTruncUnit, expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::DateTrunc).args([Expr::val(unit.to_string()), expr.into()])
    }

    /// Call the `JSON_AGG` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr(PgFunc::json_agg(Expr::col(Char::SizeW)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_AGG("size_w") FROM "character""#
    /// );
    /// ```
    pub fn json_agg<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::JsonAgg).arg(expr)
    }

    /// Call the `JSONB_AGG` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr(PgFunc::jsonb_agg(Expr::col(Char::SizeW)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSONB_AGG("size_w") FROM "character""#
    /// );
    /// ```
    pub fn jsonb_agg<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::JsonbAgg).arg(expr)
    }

    /// Call the `ARRAY_AGG` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr(PgFunc::array_agg(Expr::col(Char::Id)))
    ///     .group_by_col(Char::Character)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT ARRAY_AGG("id") FROM "character" GROUP BY "character""#
    /// );
    /// ```
    pub fn array_agg<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::ArrayAgg).arg(expr)
    }

    /// Call the `ARRAY_AGG` function with the `DISTINCT` modifier. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr(PgFunc::array_agg_distinct(Expr::col(Char::Id)))
    ///     .group_by_col(Char::Character)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT ARRAY_AGG(DISTINCT "id") FROM "character" GROUP BY "character""#
    /// );
    /// ```
    pub fn array_agg_distinct<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::ArrayAgg).arg_with(expr, FuncArgMod { distinct: true })
    }

    /// Call `PG_ADVISORY_LOCK` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_lock(12345))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_ADVISORY_LOCK(12345)"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_lock((1, 42)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_ADVISORY_LOCK(1, 42)"#
    /// );
    /// ```
    pub fn advisory_lock<K>(key: K) -> FunctionCall
    where
        K: Into<PgAdvisoryLockKey>,
    {
        key.into().call(PgFunc::AdvisoryLock)
    }

    /// Call `PG_ADVISORY_LOCK_SHARED` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_lock_shared(12345))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_ADVISORY_LOCK_SHARED(12345)"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_lock_shared((1, 42)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_ADVISORY_LOCK_SHARED(1, 42)"#
    /// );
    /// ```
    pub fn advisory_lock_shared<K>(key: K) -> FunctionCall
    where
        K: Into<PgAdvisoryLockKey>,
    {
        key.into().call(PgFunc::AdvisoryLockShared)
    }

    /// Call `PG_TRY_ADVISORY_LOCK` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::try_advisory_lock(12345))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_TRY_ADVISORY_LOCK(12345)"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::try_advisory_lock((1, 42)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_TRY_ADVISORY_LOCK(1, 42)"#
    /// );
    /// ```
    pub fn try_advisory_lock<K>(key: K) -> FunctionCall
    where
        K: Into<PgAdvisoryLockKey>,
    {
        key.into().call(PgFunc::TryAdvisoryLock)
    }

    /// Call `PG_TRY_ADVISORY_LOCK_SHARED` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::try_advisory_lock_shared(12345))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_TRY_ADVISORY_LOCK_SHARED(12345)"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::try_advisory_lock_shared((1, 42)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_TRY_ADVISORY_LOCK_SHARED(1, 42)"#
    /// );
    /// ```
    pub fn try_advisory_lock_shared<K>(key: K) -> FunctionCall
    where
        K: Into<PgAdvisoryLockKey>,
    {
        key.into().call(PgFunc::TryAdvisoryLockShared)
    }

    /// Call `PG_ADVISORY_UNLOCK` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_unlock(12345))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_ADVISORY_UNLOCK(12345)"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_unlock((1, 42)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_ADVISORY_UNLOCK(1, 42)"#
    /// );
    /// ```
    pub fn advisory_unlock<K>(key: K) -> FunctionCall
    where
        K: Into<PgAdvisoryLockKey>,
    {
        key.into().call(PgFunc::AdvisoryUnlock)
    }

    /// Call `PG_ADVISORY_UNLOCK_SHARED` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_unlock_shared(12345))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_ADVISORY_UNLOCK_SHARED(12345)"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_unlock_shared((1, 42)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_ADVISORY_UNLOCK_SHARED(1, 42)"#
    /// );
    /// ```
    pub fn advisory_unlock_shared<K>(key: K) -> FunctionCall
    where
        K: Into<PgAdvisoryLockKey>,
    {
        key.into().call(PgFunc::AdvisoryUnlockShared)
    }

    /// Call `PG_ADVISORY_UNLOCK_ALL` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_unlock_all())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_ADVISORY_UNLOCK_ALL()"#
    /// );
    /// ```
    pub fn advisory_unlock_all() -> FunctionCall {
        FunctionCall::new(PgFunc::AdvisoryUnlockAll)
    }

    /// Call `PG_ADVISORY_XACT_LOCK` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_xact_lock(12345))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_ADVISORY_XACT_LOCK(12345)"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_xact_lock((1, 42)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_ADVISORY_XACT_LOCK(1, 42)"#
    /// );
    /// ```
    pub fn advisory_xact_lock<K>(key: K) -> FunctionCall
    where
        K: Into<PgAdvisoryLockKey>,
    {
        key.into().call(PgFunc::AdvisoryXactLock)
    }

    /// Call `PG_ADVISORY_XACT_LOCK_SHARED` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_xact_lock_shared(12345))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_ADVISORY_XACT_LOCK_SHARED(12345)"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_xact_lock_shared((1, 42)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_ADVISORY_XACT_LOCK_SHARED(1, 42)"#
    /// );
    /// ```
    pub fn advisory_xact_lock_shared<K>(key: K) -> FunctionCall
    where
        K: Into<PgAdvisoryLockKey>,
    {
        key.into().call(PgFunc::AdvisoryXactLockShared)
    }

    /// Call `PG_TRY_ADVISORY_XACT_LOCK` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::try_advisory_xact_lock(12345))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_TRY_ADVISORY_XACT_LOCK(12345)"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::try_advisory_xact_lock((1, 42)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_TRY_ADVISORY_XACT_LOCK(1, 42)"#
    /// );
    /// ```
    pub fn try_advisory_xact_lock<K>(key: K) -> FunctionCall
    where
        K: Into<PgAdvisoryLockKey>,
    {
        key.into().call(PgFunc::TryAdvisoryXactLock)
    }

    /// Call `PG_TRY_ADVISORY_XACT_LOCK_SHARED` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::try_advisory_xact_lock_shared(12345))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_TRY_ADVISORY_XACT_LOCK_SHARED(12345)"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::try_advisory_xact_lock_shared((1, 42)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT PG_TRY_ADVISORY_XACT_LOCK_SHARED(1, 42)"#
    /// );
    /// ```
    pub fn try_advisory_xact_lock_shared<K>(key: K) -> FunctionCall
    where
        K: Into<PgAdvisoryLockKey>,
    {
        key.into().call(PgFunc::TryAdvisoryXactLockShared)
    }
}
