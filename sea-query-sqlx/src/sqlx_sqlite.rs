use crate::SqlxValues;
use sea_query::{OptionEnum, Value};

impl sqlx::IntoArguments<sqlx::sqlite::Sqlite> for SqlxValues {
    fn into_arguments(self) -> sqlx::sqlite::SqliteArguments {
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
                // There is no unsigned 64-bit type, so a value above `i64::MAX` is bound as a
                // float, the same as an inlined literal: comparisons still give the right result,
                // and storing it into an integer column fails instead of panicking here.
                Value::BigUnsigned(Some(i)) if i > i64::MAX as u64 => {
                    let _ = args.add(i as f64);
                }
                Value::BigUnsigned(i) => {
                    let _ = args.add(i.map(|i| i as i64));
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
        args
    }
}

#[cfg(test)]
mod tests {
    use sea_query::{Value, Values};
    use sqlx::{Arguments, IntoArguments};

    use crate::SqlxValues;

    #[test]
    fn bind_big_unsigned_above_i64_max() {
        let values = SqlxValues(Values(vec![
            Value::BigUnsigned(Some(u64::MAX)),
            Value::BigUnsigned(Some(1)),
            Value::BigUnsigned(None),
        ]));
        let args = IntoArguments::<sqlx::sqlite::Sqlite>::into_arguments(values);
        assert_eq!(args.len(), 3);
    }
}
