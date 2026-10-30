use crate::{SqlxValues, TryIntoArguments};
use sea_query::{OptionEnum, Value};

impl sqlx::IntoArguments<sqlx::mysql::MySql> for SqlxValues {
    fn into_arguments(self) -> sqlx::mysql::MySqlArguments {
        TryIntoArguments::<sqlx::mysql::MySql>::try_into_arguments(self)
            .expect("value cannot be bound, use `try_into_arguments` to handle the error")
    }
}

impl TryIntoArguments<sqlx::mysql::MySql> for SqlxValues {
    fn try_into_arguments(self) -> Result<sqlx::mysql::MySqlArguments, sqlx::Error> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        for arg in self.0.into_iter() {
            use sqlx::Arguments;
            match arg {
                Value::Bool(b) => {
                    args.add(b).map_err(sqlx::Error::Encode)?;
                }
                Value::TinyInt(i) => {
                    args.add(i).map_err(sqlx::Error::Encode)?;
                }
                Value::SmallInt(i) => {
                    args.add(i).map_err(sqlx::Error::Encode)?;
                }
                Value::Int(i) => {
                    args.add(i).map_err(sqlx::Error::Encode)?;
                }
                Value::BigInt(i) => {
                    args.add(i).map_err(sqlx::Error::Encode)?;
                }
                Value::TinyUnsigned(i) => {
                    args.add(i).map_err(sqlx::Error::Encode)?;
                }
                Value::SmallUnsigned(i) => {
                    args.add(i).map_err(sqlx::Error::Encode)?;
                }
                Value::Unsigned(i) => {
                    args.add(i).map_err(sqlx::Error::Encode)?;
                }
                Value::BigUnsigned(i) => {
                    args.add(i).map_err(sqlx::Error::Encode)?;
                }
                Value::Float(f) => {
                    args.add(f).map_err(sqlx::Error::Encode)?;
                }
                Value::Double(d) => {
                    args.add(d).map_err(sqlx::Error::Encode)?;
                }
                Value::String(s) => {
                    args.add(s.as_deref()).map_err(sqlx::Error::Encode)?;
                }
                Value::Enum(e) => {
                    let value = match e {
                        OptionEnum::Some(v) => Some(v.value.into_owned()),
                        OptionEnum::None(_) => None,
                    };
                    args.add(value).map_err(sqlx::Error::Encode)?;
                }
                Value::Char(c) => {
                    args.add(c.map(|c| c.to_string()))
                        .map_err(sqlx::Error::Encode)?;
                }
                Value::Bytes(b) => {
                    args.add(b.as_deref()).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDate(d) => {
                    args.add(d).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(t) => {
                    args.add(Value::ChronoDateTimeWithTimeZone(t).chrono_as_naive_utc_in_string())
                        .map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeDate(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeTime(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffDate(_) => {
                    panic!("Mysql doesn't support JiffDate arguments");
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffTime(_) => {
                    panic!("Mysql doesn't support JiffTime arguments");
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffDateTime(_) => {
                    panic!("Mysql doesn't support JiffDateTime arguments");
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffTimestamp(_) => {
                    panic!("Mysql doesn't support JiffTimestamp arguments");
                }
                #[cfg(feature = "with-uuid")]
                Value::Uuid(uuid) => {
                    args.add(uuid).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(d) => {
                    args.add(d).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(d) => {
                    args.add(d.as_deref()).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-json")]
                Value::Json(j) => {
                    args.add(j.as_deref()).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "postgres-array")]
                Value::Array(_, _) => {
                    panic!("Mysql doesn't support array arguments");
                }
                #[cfg(feature = "postgres-vector")]
                Value::Vector(_) => {
                    panic!("Mysql doesn't support vector arguments");
                }
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(_) => {
                    panic!("Mysql doesn't support IpNetwork arguments");
                }
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(_) => {
                    panic!("Mysql doesn't support MacAddress arguments");
                } /* #[cfg(feature = "postgres-range")]
                  Value::Range(_) => {
                  panic!("Mysql doesn't support PgRange arguments");
                  } */
            }
        }
        Ok(args)
    }
}
