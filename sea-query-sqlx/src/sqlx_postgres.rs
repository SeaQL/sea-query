#[cfg(all(feature = "postgres-array", feature = "with-bigdecimal"))]
use sea_query::prelude::BigDecimal;
#[cfg(all(feature = "postgres-array", feature = "with-rust_decimal"))]
use sea_query::prelude::Decimal;
#[cfg(all(feature = "postgres-array", feature = "with-ipnetwork"))]
use sea_query::prelude::IpNetwork;
#[cfg(all(feature = "postgres-array", feature = "with-json"))]
use sea_query::prelude::Json;
#[cfg(all(feature = "postgres-array", feature = "with-mac_address"))]
use sea_query::prelude::MacAddress;
#[cfg(all(feature = "postgres-array", feature = "with-uuid"))]
use sea_query::prelude::Uuid;
#[cfg(all(feature = "postgres-array", feature = "with-time"))]
use sea_query::prelude::time;
#[cfg(all(feature = "postgres-array", feature = "with-chrono"))]
use sea_query::prelude::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};

#[cfg(feature = "postgres-array")]
use sea_query::ArrayType;
use sea_query::{OptionEnum, Value};

use crate::{IntoArgumentsError, SqlxValues};

impl sqlx::IntoArguments<sqlx::postgres::Postgres> for SqlxValues {
    fn into_arguments(self) -> sqlx::postgres::PgArguments {
        sqlx::postgres::PgArguments::try_from(self).unwrap_or_else(|err| panic!("{err}"))
    }
}

impl TryFrom<SqlxValues> for sqlx::postgres::PgArguments {
    type Error = IntoArgumentsError;

    fn try_from(values: SqlxValues) -> Result<Self, Self::Error> {
        let mut args = sqlx::postgres::PgArguments::default();
        for arg in values.0.into_iter() {
            use sqlx::Arguments;
            match arg {
                Value::Bool(b) => {
                    args.add(b).map_err(IntoArgumentsError::Encode)?;
                }
                Value::TinyInt(i) => {
                    args.add(i).map_err(IntoArgumentsError::Encode)?;
                }
                Value::SmallInt(i) => {
                    args.add(i).map_err(IntoArgumentsError::Encode)?;
                }
                Value::Int(i) => {
                    args.add(i).map_err(IntoArgumentsError::Encode)?;
                }
                Value::BigInt(i) => {
                    args.add(i).map_err(IntoArgumentsError::Encode)?;
                }
                Value::TinyUnsigned(i) => {
                    args.add(i.map(|i| i as i16))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                Value::SmallUnsigned(i) => {
                    args.add(i.map(|i| i as i32))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                Value::Unsigned(i) => {
                    args.add(i.map(|i| i as i64))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                Value::BigUnsigned(i) => {
                    args.add(
                        i.map(i64::try_from)
                            .transpose()
                            .map_err(|e| IntoArgumentsError::Encode(e.into()))?,
                    )
                    .map_err(IntoArgumentsError::Encode)?;
                }
                Value::Float(f) => {
                    args.add(f).map_err(IntoArgumentsError::Encode)?;
                }
                Value::Double(d) => {
                    args.add(d).map_err(IntoArgumentsError::Encode)?;
                }
                Value::String(s) => {
                    args.add(s.as_deref()).map_err(IntoArgumentsError::Encode)?;
                }
                Value::Enum(e) => {
                    let value = match e {
                        OptionEnum::Some(v) => Some(v.value.into_owned()),
                        OptionEnum::None(_) => None,
                    };
                    args.add(value).map_err(IntoArgumentsError::Encode)?;
                }
                Value::Char(c) => {
                    args.add(c.map(|c| c.to_string()))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                Value::Bytes(b) => {
                    args.add(b.as_deref()).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDate(d) => {
                    args.add(d).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(t) => {
                    args.add(t).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(t) => {
                    args.add(t).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(t) => {
                    args.add(t).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(t) => {
                    args.add(t).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(t) => {
                    args.add(t).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeDate(t) => {
                    args.add(t).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeTime(t) => {
                    args.add(t).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(t) => {
                    args.add(t).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(t) => {
                    args.add(t).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffDate(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "SQLx 0.9 does not support Jiff arguments for Postgres yet",
                    ));
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffTime(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "SQLx 0.9 does not support Jiff arguments for Postgres yet",
                    ));
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffDateTime(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "SQLx 0.9 does not support Jiff arguments for Postgres yet",
                    ));
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffTimestamp(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "SQLx 0.9 does not support Jiff arguments for Postgres yet",
                    ));
                }
                #[cfg(feature = "with-uuid")]
                Value::Uuid(uuid) => {
                    args.add(uuid).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(d) => {
                    args.add(d).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(d) => {
                    args.add(d.as_deref()).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-json")]
                Value::Json(j) => {
                    args.add(j.as_deref()).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(ip) => {
                    args.add(ip).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(mac) => {
                    args.add(mac).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "postgres-array")]
                Value::Array(ty, v) => {
                    let v = v.map(|v| *v);
                    match ty {
                        ArrayType::Bool => {
                            let value = array_values::<bool>(v, "Bool")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::TinyInt => {
                            let value = array_values::<i8>(v, "TinyInt")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::SmallInt => {
                            let value = array_values::<i16>(v, "SmallInt")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::Int => {
                            let value = array_values::<i32>(v, "Int")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::BigInt => {
                            let value = array_values::<i64>(v, "BigInt")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::TinyUnsigned => {
                            let value = array_values::<u8>(v, "TinyUnsigned")?;
                            let value: Option<Vec<Option<i16>>> = value.map(|vec| {
                                vec.into_iter()
                                    .map(|value| value.map(|i| i as i16))
                                    .collect()
                            });
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::SmallUnsigned => {
                            let value = array_values::<u16>(v, "SmallUnsigned")?;
                            let value: Option<Vec<Option<i32>>> = value.map(|vec| {
                                vec.into_iter()
                                    .map(|value| value.map(|i| i as i32))
                                    .collect()
                            });
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::Unsigned => {
                            let value = array_values::<u32>(v, "Unsigned")?;
                            let value: Option<Vec<Option<i64>>> = value.map(|vec| {
                                vec.into_iter()
                                    .map(|value| value.map(|i| i as i64))
                                    .collect()
                            });
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::BigUnsigned => {
                            let value = array_values::<u64>(v, "BigUnsigned")?;
                            let value: Option<Vec<Option<i64>>> = value
                                .map(|vec| {
                                    vec.into_iter()
                                        .map(|value| value.map(i64::try_from).transpose())
                                        .collect::<Result<_, _>>()
                                })
                                .transpose()
                                .map_err(|e| IntoArgumentsError::Encode(e.into()))?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::Float => {
                            let value = array_values::<f32>(v, "Float")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::Double => {
                            let value = array_values::<f64>(v, "Double")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::String => {
                            let value = array_values::<String>(v, "String")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::Enum(_) => {
                            let value = v
                                .map(|values| {
                                    values
                                        .into_iter()
                                        .map(|value| match value {
                                            Value::Enum(OptionEnum::Some(value)) => {
                                                Ok(Some(value.value.into_owned()))
                                            }
                                            Value::Enum(OptionEnum::None(_)) => Ok(None),
                                            _ => Err(IntoArgumentsError::InvalidArray {
                                                element_type: "Enum",
                                                source: sea_query::ValueTypeErr,
                                            }),
                                        })
                                        .collect::<Result<Vec<_>, _>>()
                                })
                                .transpose()?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::Char => {
                            let value = array_values::<char>(v, "Char")?;
                            let value: Option<Vec<Option<String>>> = value.map(|vec| {
                                vec.into_iter()
                                    .map(|value| value.map(|c| c.to_string()))
                                    .collect()
                            });
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        ArrayType::Bytes => {
                            let value = array_values::<Vec<u8>>(v, "Bytes")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-chrono")]
                        ArrayType::ChronoDate => {
                            let value = array_values::<NaiveDate>(v, "ChronoDate")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-chrono")]
                        ArrayType::ChronoTime => {
                            let value = array_values::<NaiveTime>(v, "ChronoTime")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-chrono")]
                        ArrayType::ChronoDateTime => {
                            let value = array_values::<NaiveDateTime>(v, "ChronoDateTime")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-chrono")]
                        ArrayType::ChronoDateTimeUtc => {
                            let value = array_values::<DateTime<Utc>>(v, "ChronoDateTimeUtc")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-chrono")]
                        ArrayType::ChronoDateTimeLocal => {
                            let value = array_values::<DateTime<Local>>(v, "ChronoDateTimeLocal")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-chrono")]
                        ArrayType::ChronoDateTimeWithTimeZone => {
                            let value = array_values::<DateTime<FixedOffset>>(
                                v,
                                "ChronoDateTimeWithTimeZone",
                            )?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-time")]
                        ArrayType::TimeDate => {
                            let value = array_values::<time::Date>(v, "TimeDate")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-time")]
                        ArrayType::TimeTime => {
                            let value = array_values::<time::Time>(v, "TimeTime")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-time")]
                        ArrayType::TimeDateTime => {
                            let value = array_values::<time::PrimitiveDateTime>(v, "TimeDateTime")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-time")]
                        ArrayType::TimeDateTimeWithTimeZone => {
                            let value = array_values::<time::OffsetDateTime>(
                                v,
                                "TimeDateTimeWithTimeZone",
                            )?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-jiff")]
                        ArrayType::JiffDate => {
                            return Err(IntoArgumentsError::Unsupported(
                                "SQLx 0.9 does not support Jiff array arguments for Postgres yet",
                            ));
                        }
                        #[cfg(feature = "with-jiff")]
                        ArrayType::JiffTime => {
                            return Err(IntoArgumentsError::Unsupported(
                                "SQLx 0.9 does not support Jiff array arguments for Postgres yet",
                            ));
                        }
                        #[cfg(feature = "with-jiff")]
                        ArrayType::JiffDateTime => {
                            return Err(IntoArgumentsError::Unsupported(
                                "SQLx 0.9 does not support Jiff array arguments for Postgres yet",
                            ));
                        }
                        #[cfg(feature = "with-jiff")]
                        ArrayType::JiffTimestamp => {
                            return Err(IntoArgumentsError::Unsupported(
                                "SQLx 0.9 does not support Jiff array arguments for Postgres yet",
                            ));
                        }
                        #[cfg(feature = "with-uuid")]
                        ArrayType::Uuid => {
                            let value = array_values::<Uuid>(v, "Uuid")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-rust_decimal")]
                        ArrayType::Decimal => {
                            let value = array_values::<Decimal>(v, "Decimal")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-bigdecimal")]
                        ArrayType::BigDecimal => {
                            let value = array_values::<BigDecimal>(v, "BigDecimal")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-json")]
                        ArrayType::Json => {
                            let value = array_values::<Json>(v, "Json")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-ipnetwork")]
                        ArrayType::IpNetwork => {
                            let value = array_values::<IpNetwork>(v, "IpNetwork")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "with-mac_address")]
                        ArrayType::MacAddress => {
                            let value = array_values::<MacAddress>(v, "MacAddress")?;
                            args.add(value).map_err(IntoArgumentsError::Encode)?;
                        }
                        #[cfg(feature = "postgres-range")]
                        ArrayType::Range => {
                            let value = array_values::<sea_query::prelude::RangeType>(v, "Range")?;
                            range::add_range_array(&mut args, value)?;
                        }
                    }
                }
                #[cfg(feature = "postgres-vector")]
                Value::Vector(v) => {
                    args.add(v).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "postgres-range")]
                Value::Range(v) => {
                    range::add_range(&mut args, v.map(|v| *v))?;
                }
            }
        }
        Ok(args)
    }
}

#[cfg(feature = "postgres-array")]
fn array_values<T: sea_query::ValueType + sea_query::Nullable>(
    values: Option<Vec<Value>>,
    element_type: &'static str,
) -> Result<Option<Vec<Option<T>>>, IntoArgumentsError> {
    values
        .map(|values| {
            values
                .into_iter()
                .map(<Option<T> as sea_query::ValueType>::try_from)
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()
        .map_err(|source| IntoArgumentsError::InvalidArray {
            element_type,
            source,
        })
}

#[cfg(feature = "postgres-range")]
mod range {
    use std::ops::Bound;

    use sea_query::prelude::{RangeBound, RangeType};
    use sqlx::encode::IsNull;
    use sqlx::postgres::types::{Oid, PgRange};
    use sqlx::postgres::{PgArgumentBuffer, PgArguments, PgTypeInfo, Postgres};
    use sqlx::{Arguments, Encode, Type};

    use crate::IntoArgumentsError;

    /// A `NULL` sent with the `unknown` type, so that Postgres infers the
    /// concrete range type (`int4range`, `int8range`, ...) from the query.
    struct UntypedNull;

    impl Type<Postgres> for UntypedNull {
        fn type_info() -> PgTypeInfo {
            PgTypeInfo::with_oid(Oid(705))
        }
    }

    impl Encode<'_, Postgres> for UntypedNull {
        fn encode_by_ref(
            &self,
            _buf: &mut PgArgumentBuffer,
        ) -> Result<IsNull, sqlx::error::BoxDynError> {
            Ok(IsNull::Yes)
        }
    }

    fn to_bound<T: Clone + std::fmt::Display, U>(
        bound: RangeBound<T>,
        convert: impl Fn(T) -> Result<U, IntoArgumentsError>,
    ) -> Result<Bound<U>, IntoArgumentsError> {
        Ok(match bound {
            RangeBound::Inclusive(v) => Bound::Included(convert(v)?),
            RangeBound::Exclusive(v) => Bound::Excluded(convert(v)?),
            RangeBound::Unbounded => Bound::Unbounded,
        })
    }

    fn to_range<T: Clone + std::fmt::Display, U>(
        start: RangeBound<T>,
        end: RangeBound<T>,
        convert: impl Fn(T) -> Result<U, IntoArgumentsError>,
    ) -> Result<PgRange<U>, IntoArgumentsError> {
        Ok(PgRange {
            start: to_bound(start, &convert)?,
            end: to_bound(end, &convert)?,
        })
    }

    #[cfg(feature = "with-bigdecimal")]
    type Numeric = sea_query::prelude::BigDecimal;

    #[cfg(all(feature = "with-rust_decimal", not(feature = "with-bigdecimal")))]
    type Numeric = sea_query::prelude::Decimal;

    #[cfg(any(feature = "with-bigdecimal", feature = "with-rust_decimal"))]
    fn to_numeric(v: f64) -> Result<Numeric, IntoArgumentsError> {
        // go through the shortest decimal representation, so 0.1 binds as 0.1
        v.to_string().parse::<Numeric>().map_err(|e| {
            IntoArgumentsError::Encode(format!("invalid numrange bound {v}: {e}").into())
        })
    }

    #[cfg(any(feature = "with-bigdecimal", feature = "with-rust_decimal"))]
    fn add_num_range(
        args: &mut PgArguments,
        start: RangeBound<f64>,
        end: RangeBound<f64>,
    ) -> Result<(), IntoArgumentsError> {
        args.add(to_range(start, end, to_numeric)?)
            .map_err(IntoArgumentsError::Encode)
    }

    #[cfg(not(any(feature = "with-bigdecimal", feature = "with-rust_decimal")))]
    fn add_num_range(
        _args: &mut PgArguments,
        _start: RangeBound<f64>,
        _end: RangeBound<f64>,
    ) -> Result<(), IntoArgumentsError> {
        Err(IntoArgumentsError::Unsupported(
            "numrange arguments require the with-bigdecimal or with-rust_decimal feature",
        ))
    }

    pub(super) fn add_range(
        args: &mut PgArguments,
        range: Option<RangeType>,
    ) -> Result<(), IntoArgumentsError> {
        match range {
            None => args.add(UntypedNull).map_err(IntoArgumentsError::Encode),
            Some(RangeType::Int4Range(start, end)) => args
                .add(to_range(start, end, Ok)?)
                .map_err(IntoArgumentsError::Encode),
            Some(RangeType::Int8Range(start, end)) => args
                .add(to_range(start, end, Ok)?)
                .map_err(IntoArgumentsError::Encode),
            Some(RangeType::NumRange(start, end)) => add_num_range(args, start, end),
        }
    }

    #[cfg(feature = "postgres-array")]
    pub(super) fn add_range_array(
        args: &mut PgArguments,
        values: Option<Vec<Option<RangeType>>>,
    ) -> Result<(), IntoArgumentsError> {
        let Some(values) = values else {
            return args.add(UntypedNull).map_err(IntoArgumentsError::Encode);
        };

        let mismatch = || {
            IntoArgumentsError::Unsupported(
                "Value::Array of ranges must contain ranges of a single type",
            )
        };

        // the element type is taken from the first non-null range
        match values.iter().flatten().next() {
            Some(RangeType::Int8Range(..)) => {
                let value = values
                    .into_iter()
                    .map(|v| match v {
                        Some(RangeType::Int8Range(start, end)) => {
                            to_range(start, end, Ok).map(Some)
                        }
                        Some(_) => Err(mismatch()),
                        None => Ok(None),
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                args.add(value).map_err(IntoArgumentsError::Encode)
            }
            Some(RangeType::NumRange(..)) => Err(IntoArgumentsError::Unsupported(
                "Value::Array of numrange is not supported",
            )),
            _ => {
                let value = values
                    .into_iter()
                    .map(|v| match v {
                        Some(RangeType::Int4Range(start, end)) => {
                            to_range(start, end, Ok).map(Some)
                        }
                        Some(_) => Err(mismatch()),
                        None => Ok(None),
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                args.add(value).map_err(IntoArgumentsError::Encode)
            }
        }
    }
}
