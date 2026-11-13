use crate::{IntoArgumentsError, SqlxValues};
use sea_query::{OptionEnum, Value};

impl sqlx::IntoArguments<sqlx::sqlite::Sqlite> for SqlxValues {
    fn into_arguments(self) -> sqlx::sqlite::SqliteArguments {
        sqlx::sqlite::SqliteArguments::try_from(self).unwrap_or_else(|err| panic!("{err}"))
    }
}

impl TryFrom<SqlxValues> for sqlx::sqlite::SqliteArguments {
    type Error = IntoArgumentsError;

    fn try_from(values: SqlxValues) -> Result<Self, Self::Error> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
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
                    args.add(i).map_err(IntoArgumentsError::Encode)?;
                }
                Value::SmallUnsigned(i) => {
                    args.add(i).map_err(IntoArgumentsError::Encode)?;
                }
                Value::Unsigned(i) => {
                    args.add(i).map_err(IntoArgumentsError::Encode)?;
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
                Value::Uuid(uuid) => {
                    args.add(uuid).map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(decimal) => {
                    args.add(decimal.map(|d| d.to_string()))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(big_decimal) => {
                    args.add(big_decimal.map(|d| d.to_string()))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-json")]
                Value::Json(j) => {
                    args.add(j.map(|j| *j))
                        .map_err(IntoArgumentsError::Encode)?;
                }
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "Sqlite doesn't support IpNetwork arguments",
                    ));
                }
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "Sqlite doesn't support MacAddress arguments",
                    ));
                }
                #[cfg(feature = "postgres-array")]
                Value::Array(_, _) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "Sqlite doesn't support array arguments",
                    ));
                }
                #[cfg(feature = "postgres-vector")]
                Value::Vector(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "Sqlite doesn't support vector arguments",
                    ));
                }
                #[cfg(feature = "postgres-range")]
                Value::Range(_) => {
                    return Err(IntoArgumentsError::Unsupported(
                        "Sqlite doesn't support range arguments",
                    ));
                }
            }
        }
        Ok(args)
    }
}
