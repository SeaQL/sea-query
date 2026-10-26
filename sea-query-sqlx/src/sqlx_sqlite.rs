use crate::{SqlxValues, TryIntoArguments};
use sea_query::{OptionEnum, Value};

impl sqlx::IntoArguments<sqlx::sqlite::Sqlite> for SqlxValues {
    fn into_arguments(self) -> sqlx::sqlite::SqliteArguments {
        TryIntoArguments::<sqlx::sqlite::Sqlite>::try_into_arguments(self)
            .expect("value cannot be bound, use `try_into_arguments` to handle the error")
    }
}

impl TryIntoArguments<sqlx::sqlite::Sqlite> for SqlxValues {
    fn try_into_arguments(self) -> Result<sqlx::sqlite::SqliteArguments, sqlx::Error> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        for arg in self.0.into_iter() {
            use sqlx::Arguments;
            match arg {
                Value::Bool(b) => {
                    let _ = args.add(b);
                }
                Value::TinyInt(i) => {
                    let _ = args.add(i);
                }
                Value::SmallInt(i) => {
                    let _ = args.add(i);
                }
                Value::Int(i) => {
                    let _ = args.add(i);
                }
                Value::BigInt(i) => {
                    let _ = args.add(i);
                }
                Value::TinyUnsigned(i) => {
                    let _ = args.add(i);
                }
                Value::SmallUnsigned(i) => {
                    let _ = args.add(i);
                }
                Value::Unsigned(i) => {
                    let _ = args.add(i);
                }
                Value::BigUnsigned(i) => {
                    let _ = args.add(
                        i.map(i64::try_from)
                            .transpose()
                            .map_err(|e| sqlx::Error::Encode(e.into()))?,
                    );
                }
                Value::Float(f) => {
                    let _ = args.add(f);
                }
                Value::Double(d) => {
                    let _ = args.add(d);
                }
                Value::String(s) => {
                    let _ = args.add(s);
                }
                Value::Enum(e) => {
                    let value = match e {
                        OptionEnum::Some(v) => Some(v.value.into_owned()),
                        OptionEnum::None(_) => None,
                    };
                    let _ = args.add(value);
                }
                Value::Char(c) => {
                    let _ = args.add(c.map(|c| c.to_string()));
                }
                Value::Bytes(b) => {
                    let _ = args.add(b);
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDate(d) => {
                    let _ = args.add(d);
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(t) => {
                    let _ = args.add(t);
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(t) => {
                    let _ = args.add(t);
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(t) => {
                    let _ = args.add(t);
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(t) => {
                    let _ = args.add(t);
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(t) => {
                    let _ = args.add(t);
                }
                #[cfg(feature = "with-time")]
                Value::TimeDate(t) => {
                    let _ = args.add(t);
                }
                #[cfg(feature = "with-time")]
                Value::TimeTime(t) => {
                    let _ = args.add(t);
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(t) => {
                    let _ = args.add(t);
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(t) => {
                    let _ = args.add(t);
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffDate(j) => {
                    let _ = args.add(j.map(|j| j.to_string()));
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffTime(j) => {
                    let _ = args.add(j.map(|j| j.to_string()));
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffDateTime(j) => {
                    let _ = args.add(j.map(|j| j.to_string()));
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffTimestamp(j) => {
                    let _ = args.add(j.map(|j| j.to_string()));
                }
                #[cfg(feature = "with-uuid")]
                Value::Uuid(uuid) => {
                    let _ = args.add(uuid);
                }
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(decimal) => {
                    let _ = args.add(decimal.map(|d| d.to_string()));
                }
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(big_decimal) => {
                    let _ = args.add(big_decimal.map(|d| d.to_string()));
                }
                #[cfg(feature = "with-json")]
                Value::Json(j) => {
                    let _ = args.add(j.map(|j| *j));
                }
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(_) => {
                    panic!("Sqlite doesn't support IpNetwork arguments");
                }
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(_) => {
                    panic!("Sqlite doesn't support MacAddress arguments");
                }
                #[cfg(feature = "postgres-array")]
                Value::Array(_, _) => {
                    panic!("Sqlite doesn't support array arguments");
                }
                #[cfg(feature = "postgres-vector")]
                Value::Vector(_) => {
                    panic!("Sqlite doesn't support vector arguments");
                } /* #[cfg(feature = "postgres-range")]
                  Value::Range(_) => {
                  panic!("Sqlite doesn't support PgRange arguments");
                  } */
            }
        }
        Ok(args)
    }
}
