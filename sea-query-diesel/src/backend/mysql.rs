use diesel::mysql::Mysql;
use diesel::mysql::sql_types::*;
use diesel::query_builder::QueryFragment;
use diesel::result::QueryResult;
use diesel::sql_types::*;
use sea_query::{MysqlQueryBuilder, Value};

#[allow(unused_imports)]
use super::macros::{bail, build};
use super::{ExtractBuilder, TransformValue};

impl ExtractBuilder for Mysql {
    type Builder = MysqlQueryBuilder;

    fn builder() -> Self::Builder {
        MysqlQueryBuilder
    }
}

impl TransformValue for Mysql {
    fn transform_value(value: Value) -> QueryResult<Box<dyn QueryFragment<Self> + Send>> {
        let transformed = match value {
            Value::Bool(v) => build!(Bool, v),
            Value::TinyInt(v) => build!(TinyInt, v),
            Value::SmallInt(v) => build!(SmallInt, v),
            Value::Int(v) => build!(Integer, v),
            Value::BigInt(v) => build!(BigInt, v),
            Value::TinyUnsigned(v) => build!(Unsigned<TinyInt>, v),
            Value::SmallUnsigned(v) => build!(Unsigned<SmallInt>, v),
            Value::Unsigned(v) => build!(Unsigned<Integer>, v),
            Value::BigUnsigned(v) => build!(Unsigned<BigInt>, v),
            Value::Float(v) => build!(Float, v),
            Value::Double(v) => build!(Double, v),
            Value::String(v) => build!(Text, v),
            Value::Enum(v) => build!(Text, v.map(|v| v.value.as_ref().to_owned())),
            Value::Char(v) => build!(Text, v.map(|v| v.to_string())),
            Value::Bytes(v) => build!(Blob, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDate(v) => build!(Date, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoTime(v) => build!(Time, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(v) => build!(Timestamp, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(v) => build!(Timestamp, v.map(|v| v.naive_utc())),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(v) => build!(Timestamp, v.map(|v| v.naive_utc())),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(v) => build!(Timestamp, v.map(|v| v.naive_utc())),
            #[cfg(feature = "with-time")]
            Value::TimeDate(v) => build!(Date, v),
            #[cfg(feature = "with-time")]
            Value::TimeTime(v) => build!(Time, v),
            #[cfg(feature = "with-time")]
            Value::TimeDateTime(v) => build!(Timestamp, v),
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(v) => build!(Timestamp, v),
            #[cfg(feature = "with-uuid")]
            Value::Uuid(v) => build!(Blob, v.map(|v| v.as_bytes().to_vec())),
            #[cfg(feature = "with-rust_decimal-mysql")]
            Value::Decimal(v) => build!(Numeric, v),
            #[cfg(all(
                feature = "with-rust_decimal",
                not(feature = "with-rust_decimal-mysql")
            ))]
            Value::Decimal(_) => bail!("Enable feature with-rust_decimal-mysql"),
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(v) => build!(Numeric, v.map(|v| *v)),
            #[cfg(feature = "with-json")]
            Value::Json(v) => build!(Json, v.map(|v| *v)),
            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(_) => bail!("Mysql doesn't support IpNetwork arguments"),
            #[cfg(feature = "with-mac_address")]
            Value::MacAddress(_) => bail!("Mysql doesn't support MacAddress arguments"),
            #[cfg(feature = "postgres-array")]
            Value::Array(_, _) => bail!("Mysql doesn't support array arguments"),
            #[cfg(feature = "postgres-vector")]
            Value::Vector(_) => bail!("Mysql doesn't support vector arguments"),
        };
        Ok(transformed)
    }
}
