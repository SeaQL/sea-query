use diesel::pg::sql_types::*;
use diesel::pg::Pg;
use diesel::query_builder::QueryFragment;
use diesel::result::QueryResult;
use diesel::sql_types::*;
#[allow(unused_imports)]
use sea_query::{ArrayType, PostgresQueryBuilder, Value};

#[allow(unused_imports)]
use super::macros::{bail, build, err, refine};
use super::{ExtractBuilder, TransformValue};

impl ExtractBuilder for Pg {
    type Builder = PostgresQueryBuilder;

    fn builder() -> Self::Builder {
        PostgresQueryBuilder
    }
}

impl TransformValue for Pg {
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
            Value::String(v) => build!(Text, v.map(|v| *v)),
            Value::Char(v) => build!(Text, v.map(|v| v.to_string())),
            Value::Bytes(v) => build!(Blob, v.map(|v| *v)),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDate(v) => build!(Date, v.map(|v| *v)),
            #[cfg(feature = "with-chrono")]
            Value::ChronoTime(v) => build!(Time, v.map(|v| *v)),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(v) => build!(Timestamp, v.map(|v| *v)),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(v) => build!(Timestamptz, v.map(|v| *v)),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(v) => build!(Timestamptz, v.map(|v| *v)),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(v) => build!(Timestamptz, v.map(|v| *v)),
            #[cfg(all(feature = "postgres-interval", feature = "with-chrono"))]
            Value::ChronoDuration(v) => build!(Interval, v.map(|v| *v)),
            #[cfg(feature = "with-time")]
            Value::TimeDate(v) => build!(Date, v.map(|v| *v)),
            #[cfg(feature = "with-time")]
            Value::TimeTime(v) => build!(Time, v.map(|v| *v)),
            #[cfg(feature = "with-time")]
            Value::TimeDateTime(v) => build!(Timestamp, v.map(|v| *v)),
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(v) => build!(Timestamptz, v.map(|v| *v)),
            #[cfg(all(feature = "postgres-interval", feature = "with-time"))]
            Value::TimeDuration(v) => build!(Interval, v.map(|v| *v)),
            #[cfg(feature = "with-uuid")]
            Value::Uuid(v) => build!(Uuid, v.map(|v| *v)),
            #[cfg(feature = "with-rust_decimal-postgres")]
            Value::Decimal(v) => build!(Numeric, v.map(|v| *v)),
            #[cfg(all(
                feature = "with-rust_decimal",
                not(feature = "with-rust_decimal-postgres")
            ))]
            Value::Decimal(_) => bail!("Enable feature with-rust_decimal-postgres"),
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(v) => build!(Numeric, v.map(|v| *v)),
            #[cfg(feature = "with-json")]
            Value::Json(v) => build!(Json, v.map(|v| *v)),
            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(v) => build!(Inet, v.map(|v| *v)),
            #[cfg(feature = "with-mac_address")]
            Value::MacAddress(v) => build!(MacAddr, v.map(|v| v.bytes())),
            #[cfg(feature = "postgres-array")]
            Value::Array(ty, v) => match ty {
                ArrayType::Bool => build!(Array<Bool>, refine!(bool, ty, v)),
                ArrayType::TinyInt => {
                    build!(
                        Array<SmallInt>,
                        refine!(i8, ty, v)
                            .map(|v| v.into_iter().map(i16::from).collect::<Vec<i16>>())
                    )
                }
                ArrayType::SmallInt => build!(Array<SmallInt>, refine!(i16, ty, v)),
                ArrayType::Int => build!(Array<Integer>, refine!(i32, ty, v)),
                ArrayType::BigInt => build!(Array<BigInt>, refine!(i64, ty, v)),
                ArrayType::TinyUnsigned => {
                    build!(
                        Array<SmallInt>,
                        refine!(u8, ty, v)
                            .map(|v| v.into_iter().map(i16::from).collect::<Vec<_>>())
                    )
                }
                ArrayType::SmallUnsigned => {
                    build!(
                        Array<Integer>,
                        refine!(u16, ty, v)
                            .map(|v| v.into_iter().map(i32::from).collect::<Vec<_>>())
                    )
                }
                ArrayType::Unsigned => {
                    build!(
                        Array<BigInt>,
                        refine!(u32, ty, v)
                            .map(|v| v.into_iter().map(i64::from).collect::<Vec<_>>())
                    )
                }
                ArrayType::BigUnsigned => {
                    build!(
                        Array<BigInt>,
                        refine!(u64, ty, v)
                            .map(|v| v
                                .into_iter()
                                .map(|v| i64::try_from(v)
                                    .map_err(|_| err!("BigUnsigned cannot be represented as i64")))
                                .collect::<Result<Vec<_>, _>>())
                            .transpose()?
                    )
                }
                ArrayType::Float => build!(Array<Float>, refine!(f32, ty, v)),
                ArrayType::Double => build!(Array<Double>, refine!(f64, ty, v)),
                ArrayType::String => build!(Array<Text>, refine!(String, ty, v)),
                ArrayType::Char => {
                    build!(
                        Array<Text>,
                        refine!(char, ty, v)
                            .map(|v| v.into_iter().map(|v| v.to_string()).collect::<Vec<_>>())
                    )
                }
                ArrayType::Bytes => build!(Array<Blob>, refine!(Vec<u8>, ty, v)),
                #[cfg(feature = "with-chrono")]
                ArrayType::ChronoDate => build!(Array<Date>, refine!(chrono::NaiveDate, ty, v)),
                #[cfg(feature = "with-chrono")]
                ArrayType::ChronoTime => build!(Array<Time>, refine!(chrono::NaiveTime, ty, v)),
                #[cfg(feature = "with-chrono")]
                ArrayType::ChronoDateTime => {
                    build!(Array<Timestamp>, refine!(chrono::NaiveDateTime, ty, v))
                }
                #[cfg(feature = "with-chrono")]
                ArrayType::ChronoDateTimeUtc => {
                    build!(
                        Array<Timestamptz>,
                        refine!(chrono::DateTime<chrono::Utc>, ty, v)
                    )
                }
                #[cfg(feature = "with-chrono")]
                ArrayType::ChronoDateTimeLocal => {
                    build!(
                        Array<Timestamptz>,
                        refine!(chrono::DateTime<chrono::Local>, ty, v)
                    )
                }
                #[cfg(feature = "with-chrono")]
                ArrayType::ChronoDateTimeWithTimeZone => {
                    build!(
                        Array<Timestamptz>,
                        refine!(chrono::DateTime<chrono::FixedOffset>, ty, v)
                    )
                }
                #[cfg(feature = "with-time")]
                ArrayType::TimeDate => build!(Array<Date>, refine!(time::Date, ty, v)),
                #[cfg(feature = "with-time")]
                ArrayType::TimeTime => build!(Array<Time>, refine!(time::Time, ty, v)),
                #[cfg(feature = "with-time")]
                ArrayType::TimeDateTime => {
                    build!(Array<Timestamp>, refine!(time::PrimitiveDateTime, ty, v))
                }
                #[cfg(feature = "with-time")]
                ArrayType::TimeDateTimeWithTimeZone => {
                    build!(Array<Timestamptz>, refine!(time::OffsetDateTime, ty, v))
                }
                #[cfg(feature = "with-uuid")]
                ArrayType::Uuid => build!(Array<Uuid>, refine!(uuid::Uuid, ty, v)),
                #[cfg(feature = "with-rust_decimal-postgres")]
                ArrayType::Decimal => build!(Array<Numeric>, refine!(rust_decimal::Decimal, ty, v)),
                #[cfg(all(
                    feature = "with-rust_decimal",
                    not(feature = "with-rust_decimal-postgres")
                ))]
                ArrayType::Decimal => bail!("Enable feature with-rust_decimal-postgres"),
                #[cfg(feature = "with-bigdecimal")]
                ArrayType::BigDecimal => {
                    build!(Array<Numeric>, refine!(bigdecimal::BigDecimal, ty, v))
                }
                #[cfg(feature = "with-json")]
                ArrayType::Json => build!(Array<Json>, refine!(serde_json::Value, ty, v)),
                #[cfg(feature = "with-ipnetwork")]
                ArrayType::IpNetwork => build!(Array<Inet>, refine!(ipnetwork::IpNetwork, ty, v)),
                #[cfg(feature = "with-mac_address")]
                ArrayType::MacAddress => {
                    build!(
                        Array<MacAddr>,
                        refine!(mac_address::MacAddress, ty, v)
                            .map(|v| v.into_iter().map(|v| v.bytes()).collect::<Vec<_>>())
                    )
                }
            },
        };
        Ok(transformed)
    }
}
