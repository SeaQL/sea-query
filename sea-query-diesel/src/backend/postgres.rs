use diesel::pg::Pg;
use diesel::pg::sql_types::*;
use diesel::query_builder::QueryFragment;
use diesel::result::QueryResult;
use diesel::sql_types::{
    BigInt, Blob, Bool, Date, Double, Float, Integer, Json, Nullable, Numeric, SmallInt, Text,
    Time, Timestamp,
};

#[allow(unused_imports)]
use sea_query::{PostgresQueryBuilder, Value};

#[allow(unused_imports)]
use super::macros::{bail, build, err};
use super::{ExtractBuilder, TransformValue};

#[cfg(feature = "postgres-array")]
fn handle_null_array(
    arr_ty: sea_query::ArrayType,
) -> QueryResult<Box<dyn QueryFragment<Pg> + Send>> {
    use diesel::sql_types::Array as DieselArray;
    use sea_query::ArrayType;
    let fragment = match arr_ty {
        ArrayType::Bool => build!(DieselArray<Nullable<Bool>>, None::<Vec<Option<bool>>>),
        ArrayType::TinyInt => build!(DieselArray<Nullable<SmallInt>>, None::<Vec<Option<i16>>>),
        ArrayType::SmallInt => build!(DieselArray<Nullable<SmallInt>>, None::<Vec<Option<i16>>>),
        ArrayType::Int => build!(DieselArray<Nullable<Integer>>, None::<Vec<Option<i32>>>),
        ArrayType::BigInt => build!(DieselArray<Nullable<BigInt>>, None::<Vec<Option<i64>>>),
        ArrayType::TinyUnsigned => {
            build!(DieselArray<Nullable<SmallInt>>, None::<Vec<Option<i16>>>)
        }
        ArrayType::SmallUnsigned => {
            build!(DieselArray<Nullable<Integer>>, None::<Vec<Option<i32>>>)
        }
        ArrayType::Unsigned => build!(DieselArray<Nullable<BigInt>>, None::<Vec<Option<i64>>>),
        ArrayType::BigUnsigned => build!(DieselArray<Nullable<BigInt>>, None::<Vec<Option<i64>>>),
        ArrayType::Float => build!(DieselArray<Nullable<Float>>, None::<Vec<Option<f32>>>),
        ArrayType::Double => build!(DieselArray<Nullable<Double>>, None::<Vec<Option<f64>>>),
        ArrayType::String => build!(DieselArray<Nullable<Text>>, None::<Vec<Option<String>>>),
        ArrayType::Char => build!(DieselArray<Nullable<Text>>, None::<Vec<Option<String>>>),
        ArrayType::Bytes => build!(DieselArray<Nullable<Blob>>, None::<Vec<Option<Vec<u8>>>>),
        ArrayType::Enum(_) => build!(DieselArray<Nullable<Text>>, None::<Vec<Option<String>>>),
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDate => build!(
            DieselArray<Nullable<Date>>,
            None::<Vec<Option<chrono::NaiveDate>>>
        ),
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoTime => build!(
            DieselArray<Nullable<Time>>,
            None::<Vec<Option<chrono::NaiveTime>>>
        ),
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTime => build!(
            DieselArray<Nullable<Timestamp>>,
            None::<Vec<Option<chrono::NaiveDateTime>>>
        ),
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTimeUtc => build!(
            DieselArray<Nullable<Timestamptz>>,
            None::<Vec<Option<chrono::DateTime<chrono::Utc>>>>
        ),
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTimeLocal => build!(
            DieselArray<Nullable<Timestamptz>>,
            None::<Vec<Option<chrono::DateTime<chrono::Local>>>>
        ),
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTimeWithTimeZone => build!(
            DieselArray<Nullable<Timestamptz>>,
            None::<Vec<Option<chrono::DateTime<chrono::FixedOffset>>>>
        ),
        #[cfg(feature = "with-time")]
        ArrayType::TimeDate => {
            build!(DieselArray<Nullable<Date>>, None::<Vec<Option<time::Date>>>)
        }
        #[cfg(feature = "with-time")]
        ArrayType::TimeTime => {
            build!(DieselArray<Nullable<Time>>, None::<Vec<Option<time::Time>>>)
        }
        #[cfg(feature = "with-time")]
        ArrayType::TimeDateTime => build!(
            DieselArray<Nullable<Timestamp>>,
            None::<Vec<Option<time::PrimitiveDateTime>>>
        ),
        #[cfg(feature = "with-time")]
        ArrayType::TimeDateTimeWithTimeZone => build!(
            DieselArray<Nullable<Timestamptz>>,
            None::<Vec<Option<time::OffsetDateTime>>>
        ),
        #[cfg(feature = "with-uuid")]
        ArrayType::Uuid => build!(DieselArray<Nullable<Uuid>>, None::<Vec<Option<uuid::Uuid>>>),
        #[cfg(all(
            feature = "with-rust_decimal",
            not(feature = "with-rust_decimal-postgres")
        ))]
        ArrayType::Decimal => {
            bail!("Deciaml support requires enabling the `with-rust_decimal-postgres` feature")
        }
        #[cfg(feature = "with-rust_decimal-postgres")]
        ArrayType::Decimal => build!(
            DieselArray<Nullable<Numeric>>,
            None::<Vec<Option<rust_decimal::Decimal>>>
        ),
        #[cfg(feature = "with-bigdecimal")]
        ArrayType::BigDecimal => build!(
            DieselArray<Nullable<Numeric>>,
            None::<Vec<Option<bigdecimal::BigDecimal>>>
        ),
        #[cfg(feature = "with-json")]
        ArrayType::Json => build!(
            DieselArray<Nullable<Json>>,
            None::<Vec<Option<serde_json::Value>>>
        ),
        #[cfg(feature = "with-ipnetwork")]
        ArrayType::IpNetwork => build!(
            DieselArray<Nullable<Inet>>,
            None::<Vec<Option<ipnetwork::IpNetwork>>>
        ),
        #[cfg(feature = "with-mac_address")]
        ArrayType::MacAddress => {
            build!(DieselArray<Nullable<MacAddr>>, None::<Vec<Option<[u8; 6]>>>)
        }
        _ => bail!("Unsupported array type"),
    };
    Ok(fragment)
}

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
            Value::Decimal(_) => {
                bail!("Deciaml support requires enabling the `with-rust_decimal-postgres` feature")
            }
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(v) => build!(Numeric, v),
            #[cfg(feature = "with-json")]
            Value::Json(v) => build!(Json, v),
            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(v) => build!(Inet, v),
            #[cfg(feature = "with-mac_address")]
            Value::MacAddress(v) => build!(MacAddr, v.map(|v| v.bytes())),
            #[cfg(feature = "postgres-array")]
            Value::Array(arr) => {
                use diesel::sql_types::Array as DieselArray;
                use sea_query::Array as SeaArray;
                match arr {
                    SeaArray::Null(arr_ty) => handle_null_array(arr_ty)?,
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
                        let (_, arr) = slice.as_ref();
                        let converted: Vec<Option<String>> = arr
                            .iter()
                            .map(|v| v.as_ref().map(|e| e.as_str().to_string()))
                            .collect();
                        build!(DieselArray<Nullable<Text>>, Some(converted))
                    }
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
                        build!(DieselArray<Nullable<Inet>>, Some(slice.into_vec()))
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
