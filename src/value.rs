//! Container for all SQL value types.

use std::borrow::Cow;

#[cfg(feature = "with-json")]
use serde_json::Value as Json;
#[cfg(feature = "with-json")]
use std::str::from_utf8;

#[cfg(feature = "with-chrono")]
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};

#[cfg(feature = "with-time")]
use time::{OffsetDateTime, PrimitiveDateTime};

#[cfg(feature = "with-jiff")]
use jiff::{Timestamp, Zoned};

#[cfg(feature = "with-rust_decimal")]
use rust_decimal::Decimal;

#[cfg(feature = "with-bigdecimal")]
use bigdecimal::BigDecimal;

#[cfg(feature = "with-uuid")]
use uuid::Uuid;

#[cfg(feature = "with-ipnetwork")]
use ipnetwork::IpNetwork;

#[cfg(feature = "with-ipnetwork")]
use std::net::IpAddr;

#[cfg(feature = "with-mac_address")]
use mac_address::MacAddress;

use crate::{ColumnType, CommonSqlQueryBuilder, QueryBuilder, StringLen};

#[cfg(test)]
mod tests;

#[cfg(feature = "hashable-value")]
mod hashable_value;

mod value_tuple;
pub use value_tuple::*;

#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
mod with_json;

#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
pub use with_json::sea_value_to_json_value;

#[cfg(feature = "with-chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
mod with_chrono;

#[cfg(feature = "with-time")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
pub mod time_format;

#[cfg(feature = "with-time")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
mod with_time;

#[cfg(feature = "with-jiff")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
pub(crate) mod with_jiff;

#[cfg(feature = "with-rust_decimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
mod with_rust_decimal;

#[cfg(feature = "with-bigdecimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
mod with_bigdecimal;

#[cfg(feature = "with-uuid")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
mod with_uuid;

#[cfg(feature = "with-ipnetwork")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
mod with_ipnetwork;

#[cfg(feature = "with-mac_address")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
mod with_mac_address;

#[cfg(feature = "postgres-array")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
pub mod with_array;

#[cfg(feature = "postgres-vector")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres-vector")))]
mod with_pgvector;

/// [`Value`] types variant for Postgres array
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ArrayType {
    Bool,
    TinyInt,
    SmallInt,
    Int,
    BigInt,
    TinyUnsigned,
    SmallUnsigned,
    Unsigned,
    BigUnsigned,
    Float,
    Double,
    String,
    Char,
    Bytes,

    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDate,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoTime,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTime,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeUtc,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeLocal,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeWithTimeZone,

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDate,

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeTime,

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTime,

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTimeWithTimeZone,

    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffDate,

    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffTime,

    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffDateTime,

    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffTimestamp,

    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffZoned,

    #[cfg(feature = "with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid,

    #[cfg(feature = "with-rust_decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
    Decimal,

    #[cfg(feature = "with-bigdecimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
    BigDecimal,

    #[cfg(feature = "with-ipnetwork")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
    IpNetwork,

    #[cfg(feature = "with-mac_address")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
    MacAddress,
}

/// Value variants
///
/// We want the inner Value to be exactly 1 pointer sized, so anything larger should be boxed.
///
/// If the `hashable-value` feature is enabled, NaN == NaN, which contradicts Rust's built-in
/// implementation of NaN != NaN.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "hashable-value"), derive(PartialEq))]
pub enum Value {
    Bool(Option<bool>),
    TinyInt(Option<i8>),
    SmallInt(Option<i16>),
    Int(Option<i32>),
    BigInt(Option<i64>),
    TinyUnsigned(Option<u8>),
    SmallUnsigned(Option<u16>),
    Unsigned(Option<u32>),
    BigUnsigned(Option<u64>),
    Float(Option<f32>),
    Double(Option<f64>),
    String(Option<String>),
    Char(Option<char>),

    #[allow(clippy::box_collection)]
    Bytes(Option<Vec<u8>>),

    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json(Option<Json>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDate(Option<NaiveDate>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoTime(Option<NaiveTime>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTime(Option<NaiveDateTime>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeUtc(Option<DateTime<Utc>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeLocal(Option<DateTime<Local>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeWithTimeZone(Option<DateTime<FixedOffset>>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDate(Option<time::Date>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeTime(Option<time::Time>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTime(Option<PrimitiveDateTime>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTimeWithTimeZone(Option<OffsetDateTime>),

    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffDate(Option<jiff::civil::Date>),

    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffTime(Option<jiff::civil::Time>),

    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffDateTime(Option<Box<jiff::civil::DateTime>>),

    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffTimestamp(Option<Box<Timestamp>>),

    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffZoned(Option<Box<Zoned>>),

    #[cfg(feature = "with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid(Option<Uuid>),

    #[cfg(feature = "with-rust_decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
    Decimal(Option<Decimal>),

    #[cfg(feature = "with-bigdecimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
    BigDecimal(Option<Box<BigDecimal>>),

    #[cfg(feature = "postgres-array")]
    #[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
    Array(ArrayType, Option<Box<Vec<Value>>>),

    #[cfg(feature = "postgres-vector")]
    #[cfg_attr(docsrs, doc(cfg(feature = "postgres-vector")))]
    Vector(Option<pgvector::Vector>),

    #[cfg(feature = "with-ipnetwork")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
    IpNetwork(Option<IpNetwork>),

    #[cfg(feature = "with-mac_address")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
    MacAddress(Option<MacAddress>),
}

/// This test is to check if the size of [`Value`] exceeds the limit.
///
/// If the size exceeds the limit, you should box the variant.
/// Previously, the size was 24. We bumped it to 32 such that `String`
/// can be unboxed.
pub const VALUE_SIZE: usize = check_value_size();

const fn check_value_size() -> usize {
    if std::mem::size_of::<Value>() > 32 {
        panic!("the size of Value shouldn't be greater than 32 bytes")
    }
    std::mem::size_of::<Value>()
}

impl Value {
    pub fn unwrap<T>(self) -> T
    where
        T: ValueType,
    {
        T::unwrap(self)
    }

    pub fn expect<T>(self, msg: &str) -> T
    where
        T: ValueType,
    {
        T::expect(self, msg)
    }

    /// Get the null variant of self
    ///
    /// ```
    /// use sea_query::Value;
    ///
    /// let v = Value::Int(Some(2));
    /// let n = v.as_null();
    ///
    /// assert_eq!(n, Value::Int(None));
    ///
    /// // one liner:
    /// assert_eq!(Into::<Value>::into(2.2).as_null(), Value::Double(None));
    /// ```
    pub fn as_null(&self) -> Self {
        match self {
            Self::Bool(_) => Self::Bool(None),
            Self::TinyInt(_) => Self::TinyInt(None),
            Self::SmallInt(_) => Self::SmallInt(None),
            Self::Int(_) => Self::Int(None),
            Self::BigInt(_) => Self::BigInt(None),
            Self::TinyUnsigned(_) => Self::TinyUnsigned(None),
            Self::SmallUnsigned(_) => Self::SmallUnsigned(None),
            Self::Unsigned(_) => Self::Unsigned(None),
            Self::BigUnsigned(_) => Self::BigUnsigned(None),
            Self::Float(_) => Self::Float(None),
            Self::Double(_) => Self::Double(None),
            Self::String(_) => Self::String(None),
            Self::Char(_) => Self::Char(None),
            Self::Bytes(_) => Self::Bytes(None),

            #[cfg(feature = "with-json")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
            Self::Json(_) => Self::Json(None),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDate(_) => Self::ChronoDate(None),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoTime(_) => Self::ChronoTime(None),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTime(_) => Self::ChronoDateTime(None),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTimeUtc(_) => Self::ChronoDateTimeUtc(None),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTimeLocal(_) => Self::ChronoDateTimeLocal(None),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTimeWithTimeZone(_) => Self::ChronoDateTimeWithTimeZone(None),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeDate(_) => Self::TimeDate(None),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeTime(_) => Self::TimeTime(None),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeDateTime(_) => Self::TimeDateTime(None),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeDateTimeWithTimeZone(_) => Self::TimeDateTimeWithTimeZone(None),

            #[cfg(feature = "with-jiff")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
            Self::JiffDate(_) => Self::JiffDate(None),

            #[cfg(feature = "with-jiff")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
            Self::JiffTime(_) => Self::JiffTime(None),

            #[cfg(feature = "with-jiff")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
            Self::JiffDateTime(_) => Self::JiffDateTime(None),

            #[cfg(feature = "with-jiff")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
            Self::JiffTimestamp(_) => Self::JiffTimestamp(None),

            #[cfg(feature = "with-jiff")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
            Self::JiffZoned(_) => Self::JiffZoned(None),

            #[cfg(feature = "with-uuid")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
            Self::Uuid(_) => Self::Uuid(None),

            #[cfg(feature = "with-rust_decimal")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
            Self::Decimal(_) => Self::Decimal(None),

            #[cfg(feature = "with-bigdecimal")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
            Self::BigDecimal(_) => Self::BigDecimal(None),

            #[cfg(feature = "postgres-array")]
            #[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
            Self::Array(ty, _) => Self::Array(ty.clone(), None),

            #[cfg(feature = "postgres-vector")]
            #[cfg_attr(docsrs, doc(cfg(feature = "postgres-vector")))]
            Self::Vector(_) => Self::Vector(None),

            #[cfg(feature = "with-ipnetwork")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
            Self::IpNetwork(_) => Self::IpNetwork(None),

            #[cfg(feature = "with-mac_address")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
            Self::MacAddress(_) => Self::MacAddress(None),
        }
    }

    /// Get a default value of self's type
    ///
    /// ```
    /// use sea_query::Value;
    ///
    /// let v = Value::Int(None);
    /// let n = v.dummy_value();
    /// assert_eq!(n, Value::Int(Some(0)));
    /// ```
    #[expect(clippy::default_trait_access)]
    pub fn dummy_value(&self) -> Self {
        match self {
            Self::Bool(_) => Self::Bool(Some(Default::default())),
            Self::TinyInt(_) => Self::TinyInt(Some(Default::default())),
            Self::SmallInt(_) => Self::SmallInt(Some(Default::default())),
            Self::Int(_) => Self::Int(Some(Default::default())),
            Self::BigInt(_) => Self::BigInt(Some(Default::default())),
            Self::TinyUnsigned(_) => Self::TinyUnsigned(Some(Default::default())),
            Self::SmallUnsigned(_) => Self::SmallUnsigned(Some(Default::default())),
            Self::Unsigned(_) => Self::Unsigned(Some(Default::default())),
            Self::BigUnsigned(_) => Self::BigUnsigned(Some(Default::default())),
            Self::Float(_) => Self::Float(Some(Default::default())),
            Self::Double(_) => Self::Double(Some(Default::default())),
            Self::String(_) => Self::String(Some(Default::default())),
            Self::Char(_) => Self::Char(Some(Default::default())),
            Self::Bytes(_) => Self::Bytes(Some(Default::default())),

            #[cfg(feature = "with-json")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
            Self::Json(_) => Self::Json(Some(Default::default())),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDate(_) => Self::ChronoDate(Some(Default::default())),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoTime(_) => Self::ChronoTime(Some(Default::default())),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTime(_) => Self::ChronoDateTime(Some(Default::default())),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTimeUtc(_) => Self::ChronoDateTimeUtc(Some(Default::default())),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTimeLocal(_) => Self::ChronoDateTimeLocal(Some(Default::default())),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTimeWithTimeZone(_) => {
                Self::ChronoDateTimeWithTimeZone(Some(Default::default()))
            }

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeDate(_) => Self::TimeDate(Some(time::Date::MIN)),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeTime(_) => Self::TimeTime(Some(time::Time::MIDNIGHT)),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeDateTime(_) => Self::TimeDateTime(Some(PrimitiveDateTime::MIN)),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeDateTimeWithTimeZone(_) => {
                Self::TimeDateTimeWithTimeZone(Some(OffsetDateTime::UNIX_EPOCH))
            }

            #[cfg(feature = "with-jiff")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
            Self::JiffDate(_) => Self::JiffDate(Some(jiff::civil::date(1970, 1, 1))),

            #[cfg(feature = "with-jiff")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
            Self::JiffTime(_) => Self::JiffTime(Some(jiff::civil::time(0, 0, 0, 0))),

            #[cfg(feature = "with-jiff")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
            Self::JiffDateTime(_) => {
                Self::JiffDateTime(Some(jiff::civil::date(1970, 1, 1).at(0, 0, 0, 0).into()))
            }

            #[cfg(feature = "with-jiff")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
            Self::JiffTimestamp(_) => Self::JiffTimestamp(Some(Timestamp::UNIX_EPOCH.into())),

            #[cfg(feature = "with-jiff")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
            Self::JiffZoned(_) => Self::JiffZoned(Some(
                Timestamp::UNIX_EPOCH
                    .to_zoned(jiff::tz::TimeZone::UTC)
                    .into(),
            )),

            #[cfg(feature = "with-uuid")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
            Self::Uuid(_) => Self::Uuid(Some(Default::default())),

            #[cfg(feature = "with-rust_decimal")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
            Self::Decimal(_) => Self::Decimal(Some(Default::default())),

            #[cfg(feature = "with-bigdecimal")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
            Self::BigDecimal(_) => Self::BigDecimal(Some(Default::default())),

            #[cfg(feature = "postgres-array")]
            #[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
            Self::Array(ty, _) => Self::Array(ty.clone(), Some(Default::default())),

            #[cfg(feature = "postgres-vector")]
            #[cfg_attr(docsrs, doc(cfg(feature = "postgres-vector")))]
            Self::Vector(_) => Self::Vector(Some(vec![].into())),

            #[cfg(feature = "with-ipnetwork")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
            Self::IpNetwork(_) => Self::IpNetwork(Some("0.0.0.0".parse().unwrap())),

            #[cfg(feature = "with-mac_address")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
            Self::MacAddress(_) => Self::MacAddress(Some(Default::default())),
        }
    }
}

impl From<&[u8]> for Value {
    fn from(x: &[u8]) -> Value {
        Value::Bytes(Some(x.into()))
    }
}

impl From<&str> for Value {
    fn from(x: &str) -> Value {
        Value::String(Some(x.to_owned()))
    }
}

impl From<&String> for Value {
    fn from(x: &String) -> Value {
        Value::String(Some(x.clone()))
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value> + Nullable,
{
    fn from(x: Option<T>) -> Value {
        match x {
            Some(v) => v.into(),
            None => T::null(),
        }
    }
}

impl From<Cow<'_, str>> for Value {
    fn from(x: Cow<'_, str>) -> Value {
        x.into_owned().into()
    }
}

impl IntoIterator for Values {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", CommonSqlQueryBuilder.value_to_string(self))
    }
}

pub trait ValueType: Sized {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr>;

    fn unwrap(v: Value) -> Self {
        Self::try_from(v).unwrap()
    }

    fn expect(v: Value, msg: &str) -> Self {
        Self::try_from(v).expect(msg)
    }

    fn is_option() -> bool {
        false
    }

    fn type_name() -> String;

    fn array_type() -> ArrayType;

    fn column_type() -> ColumnType;

    fn enum_type_name() -> Option<&'static str> {
        None
    }
}

impl<T> ValueType for Option<T>
where
    T: ValueType + Nullable,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        if v == T::null() {
            Ok(None)
        } else {
            Ok(Some(T::try_from(v)?))
        }
    }

    fn is_option() -> bool {
        true
    }

    fn type_name() -> String {
        format!("Option<{}>", T::type_name())
    }

    fn array_type() -> ArrayType {
        T::array_type()
    }

    fn column_type() -> ColumnType {
        T::column_type()
    }
}

impl ValueType for Cow<'_, str> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(x)) => Ok((x).into()),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "Cow<str>".into()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::None)
    }
}

#[derive(Debug)]
pub struct ValueTypeErr;

impl std::error::Error for ValueTypeErr {}

impl std::fmt::Display for ValueTypeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Value type mismatch")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Values(pub Vec<Value>);

impl Values {
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }
}

pub trait Nullable {
    fn null() -> Value;
}

impl Nullable for &str {
    fn null() -> Value {
        Value::String(None)
    }
}

macro_rules! type_to_value {
    ( $type: ty, $name: ident, $col_type: expr ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::$name(Some(x))
            }
        }

        impl Nullable for $type {
            fn null() -> Value {
                Value::$name(None)
            }
        }

        impl ValueType for $type {
            fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                match v {
                    Value::$name(Some(x)) => Ok(x),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($type).to_owned()
            }

            fn array_type() -> ArrayType {
                ArrayType::$name
            }

            fn column_type() -> ColumnType {
                use ColumnType::*;
                $col_type
            }
        }
    };
}
use type_to_value;

type_to_value!(bool, Bool, Boolean);
type_to_value!(i8, TinyInt, TinyInteger);
type_to_value!(i16, SmallInt, SmallInteger);
type_to_value!(i32, Int, Integer);
type_to_value!(i64, BigInt, BigInteger);
type_to_value!(u8, TinyUnsigned, TinyUnsigned);
type_to_value!(u16, SmallUnsigned, SmallUnsigned);
type_to_value!(u32, Unsigned, Unsigned);
type_to_value!(u64, BigUnsigned, BigUnsigned);
type_to_value!(f32, Float, Float);
type_to_value!(f64, Double, Double);
type_to_value!(char, Char, Char(None));
type_to_value!(Vec<u8>, Bytes, VarBinary(StringLen::None));
type_to_value!(String, String, String(StringLen::None));

#[cfg(any(feature = "with-bigdecimal", feature = "with-jiff"))]
macro_rules! type_to_box_value {
    ( $type: ty, $name: ident, $col_type: expr ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::$name(Some(Box::new(x)))
            }
        }

        impl Nullable for $type {
            fn null() -> Value {
                Value::$name(None)
            }
        }

        impl ValueType for $type {
            fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                match v {
                    Value::$name(Some(x)) => Ok(*x),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($type).to_owned()
            }

            fn array_type() -> ArrayType {
                ArrayType::$name
            }

            fn column_type() -> ColumnType {
                use ColumnType::*;
                $col_type
            }
        }
    };
}

#[cfg(any(feature = "with-bigdecimal", feature = "with-jiff"))]
use type_to_box_value;
