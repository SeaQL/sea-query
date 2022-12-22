use crate::SqlxValues;
use sea_query::Value;

impl<'q> sqlx::IntoArguments<'q, sqlx::sqlite::Sqlite> for SqlxValues {
    fn into_arguments(self) -> sqlx::sqlite::SqliteArguments<'q> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        for arg in self.0.into_iter() {
            use sqlx::Arguments;
            match arg {
                Value::Bool(b) => {
                    args.add(b);
                }
                Value::TinyInt(i) => {
                    args.add(i);
                }
                Value::SmallInt(i) => {
                    args.add(i);
                }
                Value::Int(i) => {
                    args.add(i);
                }
                Value::BigInt(i) => {
                    args.add(i);
                }
                Value::TinyUnsigned(i) => {
                    args.add(i);
                }
                Value::SmallUnsigned(i) => {
                    args.add(i);
                }
                Value::Unsigned(i) => {
                    args.add(i);
                }
                Value::BigUnsigned(i) => {
                    args.add(i.map(|i| <i64 as std::convert::TryFrom<u64>>::try_from(i).unwrap()));
                }
                Value::Float(f) => {
                    args.add(f);
                }
                Value::Double(d) => {
                    args.add(d);
                }
                Value::String(s) => {
                    args.add(s.map(|s| *s));
                }
                Value::Char(c) => {
                    args.add(c.map(|c| c.to_string()));
                }
                Value::Bytes(b) => {
                    args.add(b.map(|b| *b));
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDate(d) => {
                    args.add(d.map(|d| *d));
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(t) => {
                    args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(t) => {
                    args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(t) => {
                    args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(t) => {
                    args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(t) => {
                    args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-time")]
                Value::TimeDate(t) => {
                    args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-time")]
                Value::TimeTime(t) => {
                    args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(t) => {
                    args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(t) => {
                    args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-uuid")]
                Value::Uuid(uuid) => {
                    args.add(uuid.map(|uuid| *uuid));
                }
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(decimal) => {
                    use rust_decimal::prelude::ToPrimitive;
                    args.add(
                        decimal.map(|d| d.to_f64().expect("Fail to convert rust_decimal as f64")),
                    );
                }
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(big_decimal) => {
                    use bigdecimal::ToPrimitive;
                    args.add(
                        big_decimal.map(|d| d.to_f64().expect("Fail to convert bigdecimal as f64")),
                    );
                }
                #[cfg(feature = "with-json")]
                Value::Json(j) => {
                    args.add(j.map(|j| *j));
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
            }
        }
        args
    }
}
