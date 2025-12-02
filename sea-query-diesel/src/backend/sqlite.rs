use diesel::query_builder::QueryFragment;
use diesel::result::QueryResult;
use diesel::sql_types::*;
use diesel::sqlite::Sqlite;
use sea_query::{SqliteQueryBuilder, Value};

#[allow(unused_imports)]
use super::macros::{bail, build, err};
use super::{ExtractBuilder, TransformValue};

impl ExtractBuilder for Sqlite {
    type Builder = SqliteQueryBuilder;

    fn builder() -> Self::Builder {
        SqliteQueryBuilder
    }
}

impl TransformValue for Sqlite {
    fn transform_value(value: Value) -> QueryResult<Box<dyn QueryFragment<Self> + Send>> {
        let transformed = match value {
            Value::Bool(v) => build!(Bool, v),
            Value::TinyInt(v) => build!(SmallInt, v.map(i16::from)),
            Value::SmallInt(v) => build!(SmallInt, v),
            Value::Int(v) => build!(Integer, v),
            Value::BigInt(v) => build!(BigInt, v),
            Value::TinyUnsigned(v) => build!(SmallInt, v.map(i16::from)),
            Value::SmallUnsigned(v) => build!(Integer, v.map(i32::from)),
            Value::Unsigned(v) => build!(BigInt, v.map(i64::from)),
            // There is no i128 support, so hope the unsigned can be converted
            Value::BigUnsigned(v) => {
                let v = v
                    .map(|v| {
                        i64::try_from(v)
                            .map_err(|_| err!("BigUnsigned cannot be represented as i64"))
                    })
                    .transpose()?;
                build!(BigInt, v)
            }
            Value::Float(v) => build!(Float, v),
            Value::Double(v) => build!(Double, v),
            Value::String(v) => build!(Text, v),
            Value::Char(v) => build!(Text, v.map(|v| v.to_string())),
            Value::Bytes(v) => build!(Blob, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDate(v) => build!(Date, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoTime(v) => build!(Time, v),
            #[cfg(feature = "with-chrono")]
            // Prefer Timestamp because https://github.com/diesel-rs/diesel/issues/3693
            Value::ChronoDateTime(v) => build!(Timestamp, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(v) => build!(TimestamptzSqlite, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(v) => build!(TimestamptzSqlite, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(v) => build!(TimestamptzSqlite, v),
            #[cfg(feature = "with-time")]
            Value::TimeDate(v) => build!(Date, v),
            #[cfg(feature = "with-time")]
            Value::TimeTime(v) => build!(Time, v),
            #[cfg(feature = "with-time")]
            // Prefer Timestamp because https://github.com/diesel-rs/diesel/issues/3693
            Value::TimeDateTime(v) => build!(Timestamp, v),
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(v) => build!(TimestamptzSqlite, v),
            #[cfg(feature = "with-uuid")]
            Value::Uuid(v) => build!(Blob, v.map(|v| v.as_bytes().to_vec())),
            #[cfg(feature = "with-rust_decimal")]
            // Diesel recommends to use double for this
            Value::Decimal(v) => {
                use rust_decimal::prelude::ToPrimitive;
                let v = v
                    .map(|v| {
                        v.to_f64()
                            .ok_or(err!("Decimal cannot be represented as f64"))
                    })
                    .transpose()?;
                build!(Double, v)
            }
            #[cfg(feature = "with-bigdecimal")]
            // Diesel recommends to use double for this
            Value::BigDecimal(v) => {
                use bigdecimal::ToPrimitive;
                let v = v
                    .map(|v| {
                        v.to_f64()
                            .ok_or(err!("BigDecimal cannot be represented as f64"))
                    })
                    .transpose()?;
                build!(Double, v)
            }
            #[cfg(feature = "with-json")]
            Value::Json(v) => build!(Text, v.map(|v| v.to_string())),
            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(_) => bail!("Sqlite doesn't support IpNetwork arguments"),
            #[cfg(feature = "with-mac_address")]
            Value::MacAddress(_) => bail!("Sqlite doesn't support MacAddress arguments"),
            #[cfg(feature = "postgres-array")]
            Value::Array(_) => bail!("Sqlite doesn't support array arguments"),
            #[cfg(feature = "postgres-vector")]
            Value::Vector(_) => bail!("Sqlite doesn't support vector arguments"),
            #[cfg(feature = "postgres")]
            Value::Enum(_) => bail!("Sqlite doesn't support enum arguments"),
        };
        Ok(transformed)
    }
}
