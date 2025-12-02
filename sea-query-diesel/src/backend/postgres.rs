use diesel::pg::Pg;
use diesel::pg::sql_types::*;
use diesel::query_builder::QueryFragment;
use diesel::result::QueryResult;
use diesel::sql_types::{
    BigInt, Bool, Date, Double, Float, Integer, Nullable, SmallInt, Text, Time, Timestamp,
};

#[allow(unused_imports)]
use sea_query::{PostgresQueryBuilder, Value};

#[allow(unused_imports)]
use super::macros::{bail, build, err};
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
            Value::String(v) => build!(Text, v),
            Value::Char(v) => build!(Text, v.map(|v| v.to_string())),
            Value::Bytes(v) => build!(Blob, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDate(v) => build!(Date, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoTime(v) => build!(Time, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(v) => build!(Timestamp, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(v) => build!(Timestamptz, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(v) => build!(Timestamptz, v),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(v) => build!(Timestamptz, v),
            #[cfg(feature = "with-time")]
            Value::TimeDate(v) => build!(Date, v),
            #[cfg(feature = "with-time")]
            Value::TimeTime(v) => build!(Time, v),
            #[cfg(feature = "with-time")]
            Value::TimeDateTime(v) => build!(Timestamp, v),
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(v) => build!(Timestamptz, v),
            #[cfg(feature = "with-uuid")]
            Value::Uuid(v) => build!(Uuid, v),
            #[cfg(feature = "with-rust_decimal-postgres")]
            Value::Decimal(v) => build!(Numeric, v),
            #[cfg(all(
                feature = "with-rust_decimal",
                not(feature = "with-rust_decimal-postgres")
            ))]
            Value::Decimal(_) => bail!("Enable feature with-rust_decimal-postgres"),
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(v) => {
                build!(Numeric, v.map(|v| bigdecimal::BigDecimal::clone(&v)))
            }
            #[cfg(feature = "with-json")]
            Value::Json(v) => build!(Json, v),
            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(v) => build!(Cidr, v),
            #[cfg(feature = "with-mac_address")]
            Value::MacAddress(v) => build!(MacAddr, v.map(|v| v.bytes())),
            #[cfg(feature = "postgres-array")]
            Value::Array(v) => {
                use diesel::sql_types::Array as DieselArray;
                use sea_query::Array as SeaArray;
                match v {
                    // Use dummy value to represent NULL array
                    None => build!(DieselArray<Nullable<Bool>>, None::<Vec<Option<bool>>>),
                    Some(arr) => match arr {
                        SeaArray::Bool(slice) => {
                            build!(DieselArray<Nullable<Bool>>, Some(slice.into_vec()))
                        }
                        SeaArray::TinyInt(slice) => {
                            build!(
                                DieselArray<Nullable<SmallInt>>,
                                Some(
                                    slice
                                        .into_vec()
                                        .into_iter()
                                        .map(|v| v.map(i16::from))
                                        .collect::<Vec<_>>()
                                )
                            )
                        }
                        SeaArray::SmallInt(slice) => {
                            build!(DieselArray<Nullable<SmallInt>>, Some(slice.into_vec()))
                        }
                        SeaArray::Int(slice) => {
                            build!(DieselArray<Nullable<Integer>>, Some(slice.into_vec()))
                        }
                        SeaArray::BigInt(slice) => {
                            build!(DieselArray<Nullable<BigInt>>, Some(slice.into_vec()))
                        }
                        SeaArray::TinyUnsigned(slice) => {
                            build!(
                                DieselArray<Nullable<SmallInt>>,
                                Some(
                                    slice
                                        .into_vec()
                                        .into_iter()
                                        .map(|v| v.map(i16::from))
                                        .collect::<Vec<_>>()
                                )
                            )
                        }
                        SeaArray::SmallUnsigned(slice) => {
                            build!(
                                DieselArray<Nullable<Integer>>,
                                Some(
                                    slice
                                        .into_vec()
                                        .into_iter()
                                        .map(|v| v.map(i32::from))
                                        .collect::<Vec<_>>()
                                )
                            )
                        }
                        SeaArray::Unsigned(slice) => {
                            build!(
                                DieselArray<Nullable<BigInt>>,
                                Some(
                                    slice
                                        .into_vec()
                                        .into_iter()
                                        .map(|v| v.map(i64::from))
                                        .collect::<Vec<_>>()
                                )
                            )
                        }
                        SeaArray::BigUnsigned(slice) => {
                            let converted = slice
                                .into_vec()
                                .into_iter()
                                .map(|v| {
                                    v.map(|v| {
                                        i64::try_from(v).map_err(|_| {
                                            err!("BigUnsigned cannot be represented as i64")
                                        })
                                    })
                                    .transpose()
                                })
                                .collect::<Result<Vec<_>, _>>()?;
                            build!(DieselArray<Nullable<BigInt>>, Some(converted))
                        }
                        SeaArray::Float(slice) => {
                            build!(DieselArray<Nullable<Float>>, Some(slice.into_vec()))
                        }
                        SeaArray::Double(slice) => {
                            build!(DieselArray<Nullable<Double>>, Some(slice.into_vec()))
                        }
                        SeaArray::String(slice) => {
                            build!(DieselArray<Nullable<Text>>, Some(slice.into_vec()))
                        }
                        SeaArray::Char(slice) => {
                            build!(
                                DieselArray<Nullable<Text>>,
                                Some(
                                    slice
                                        .into_vec()
                                        .into_iter()
                                        .map(|v| v.map(|c| c.to_string()))
                                        .collect::<Vec<_>>()
                                )
                            )
                        }
                        SeaArray::Bytes(slice) => {
                            build!(DieselArray<Nullable<Blob>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "postgres")]
                        SeaArray::Enum(slice) => {
                            build!(DieselArray<Nullable<Text>>, Some(slice.into_vec()))
                        }
                        SeaArray::Array(_) => bail!("Nested arrays are not supported"),
                        #[cfg(feature = "with-chrono")]
                        SeaArray::ChronoDate(slice) => {
                            build!(DieselArray<Nullable<Date>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-chrono")]
                        SeaArray::ChronoTime(slice) => {
                            build!(DieselArray<Nullable<Time>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-chrono")]
                        SeaArray::ChronoDateTime(slice) => {
                            build!(DieselArray<Nullable<Timestamp>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-chrono")]
                        SeaArray::ChronoDateTimeUtc(slice) => {
                            build!(DieselArray<Nullable<Timestamptz>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-chrono")]
                        SeaArray::ChronoDateTimeLocal(slice) => {
                            build!(DieselArray<Nullable<Timestamptz>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-chrono")]
                        SeaArray::ChronoDateTimeWithTimeZone(slice) => {
                            build!(DieselArray<Nullable<Timestamptz>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-time")]
                        SeaArray::TimeDate(slice) => {
                            build!(DieselArray<Nullable<Date>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-time")]
                        SeaArray::TimeTime(slice) => {
                            build!(DieselArray<Nullable<Time>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-time")]
                        SeaArray::TimeDateTime(slice) => {
                            build!(DieselArray<Nullable<Timestamp>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-time")]
                        SeaArray::TimeDateTimeWithTimeZone(slice) => {
                            build!(DieselArray<Nullable<Timestamptz>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-uuid")]
                        SeaArray::Uuid(slice) => {
                            build!(DieselArray<Nullable<Uuid>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-rust_decimal-postgres")]
                        SeaArray::Decimal(slice) => {
                            build!(DieselArray<Nullable<Numeric>>, Some(slice.into_vec()))
                        }
                        #[cfg(all(
                            feature = "with-rust_decimal",
                            not(feature = "with-rust_decimal-postgres")
                        ))]
                        SeaArray::Decimal(_) => bail!("Enable feature with-rust_decimal-postgres"),
                        #[cfg(feature = "with-bigdecimal")]
                        SeaArray::BigDecimal(slice) => {
                            build!(DieselArray<Nullable<Numeric>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-json")]
                        SeaArray::Json(slice) => {
                            build!(DieselArray<Nullable<Json>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-ipnetwork")]
                        SeaArray::IpNetwork(slice) => {
                            build!(DieselArray<Nullable<Cidr>>, Some(slice.into_vec()))
                        }
                        #[cfg(feature = "with-mac_address")]
                        SeaArray::MacAddress(slice) => {
                            build!(
                                DieselArray<Nullable<MacAddr>>,
                                Some(
                                    slice
                                        .into_vec()
                                        .into_iter()
                                        .map(|v| v.map(|m| m.bytes()))
                                        .collect::<Vec<_>>()
                                )
                            )
                        }
                        _ => bail!("Unsupported array type"),
                    },
                }
            }
            #[cfg(feature = "postgres")]
            Value::Enum(v) => build!(Text, v.map(|e| e.as_str().to_string())),
            #[cfg(feature = "postgres-vector")]
            Value::Vector(v) => build!(pgvector::sql_types::Vector, v),
        };
        Ok(transformed)
    }
}
