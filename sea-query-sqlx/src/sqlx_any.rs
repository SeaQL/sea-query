use crate::{IntoArgumentsError, SqlxValues};
use sea_query::{OptionEnum, Value};

impl sqlx::IntoArguments<sqlx::any::Any> for SqlxValues {
    fn into_arguments(self) -> sqlx::any::AnyArguments {
        sqlx::any::AnyArguments::try_from(self).unwrap_or_else(|err| panic!("{err}"))
    }
}

impl TryFrom<SqlxValues> for sqlx::any::AnyArguments {
    type Error = IntoArgumentsError;

    fn try_from(values: SqlxValues) -> Result<Self, Self::Error> {
        let mut args = sqlx::any::AnyArguments::default();
        for arg in values.0.into_iter() {
            use sqlx::Arguments;
            match arg {
                Value::Bool(b) => {
                    args.add(b).map_err(IntoArgumentsError::Encode)?;
                }
                Value::TinyInt(i) => {
                    args.add(i.map(Into::<i32>::into))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                Value::SmallInt(i) => {
                    args.add(i.map(Into::<i32>::into))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                Value::Int(i) => {
                    args.add(i).map_err(IntoArgumentsError::Encode)?;
                }
                Value::BigInt(i) => {
                    args.add(i).map_err(IntoArgumentsError::Encode)?;
                }
                Value::TinyUnsigned(i) => {
                    args.add(i.map(Into::<i32>::into))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                Value::SmallUnsigned(i) => {
                    args.add(i.map(Into::<i32>::into))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                Value::Unsigned(i) => {
                    args.add(i.map(Into::<i64>::into))
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
                    args.add(s).map_err(IntoArgumentsError::Encode)?;
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
                    args.add(b).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDate(t) => {
                    args.add(Value::ChronoDate(t).chrono_as_naive_utc_in_string())
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(t) => {
                    args.add(Value::ChronoTime(t).chrono_as_naive_utc_in_string())
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(t) => {
                    args.add(Value::ChronoDateTime(t).chrono_as_naive_utc_in_string())
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(t) => {
                    args.add(Value::ChronoDateTimeUtc(t).chrono_as_naive_utc_in_string())
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(t) => {
                    args.add(Value::ChronoDateTimeLocal(t).chrono_as_naive_utc_in_string())
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(t) => {
                    args.add(Value::ChronoDateTimeWithTimeZone(t).chrono_as_naive_utc_in_string())
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeDate(t) => {
                    args.add(Value::TimeDate(t).time_as_naive_utc_in_string())
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeTime(t) => {
                    args.add(Value::TimeTime(t).time_as_naive_utc_in_string())
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(t) => {
                    args.add(Value::TimeDateTime(t).time_as_naive_utc_in_string())
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(t) => {
                    args.add(Value::TimeDateTimeWithTimeZone(t).time_as_naive_utc_in_string())
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffDate(j) => {
                    args.add(j.map(|j| j.to_string()))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffTime(j) => {
                    args.add(j.map(|j| j.to_string()))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffDateTime(j) => {
                    args.add(j.map(|j| j.to_string()))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffTimestamp(j) => {
                    args.add(j.map(|j| j.to_string()))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-uuid")]
                Value::Uuid(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "UUID support not implemented for Any",
                    ));
                }
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "Sqlite doesn't support decimal arguments",
                    ));
                }
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "Sqlite doesn't support bigdecimal arguments",
                    ));
                }
                #[cfg(feature = "with-json")]
                Value::Json(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "Json support not implemented for Any",
                    ));
                }
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "SQLx doesn't support IpNetwork arguments for Any",
                    ));
                }
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "SQLx doesn't support MacAddress arguments for Any",
                    ));
                }
                #[cfg(feature = "postgres-array")]
                Value::Array(_, _) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "SQLx doesn't support array arguments for Any",
                    ));
                }
                #[cfg(feature = "postgres-vector")]
                Value::Vector(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "SQLx doesn't support vector arguments for Any",
                    ));
                } /* #[cfg(feature = "postgres-range")]
                  Value::Range(_) => {
                  panic!("SQLx doesn't support PgRange arguments for Any");
                  } */
            }
        }
        Ok(args)
    }
}
