use crate::SqlxValues;
use sea_query::Value;

impl<'q> sqlx::IntoArguments<'q, sqlx::any::Any> for SqlxValues {
    fn into_arguments(self) -> sqlx::any::AnyArguments<'q> {
        let mut args = sqlx::any::AnyArguments::default();
        for arg in self.0.into_iter() {
            use sqlx::Arguments;
            match arg {
                Value::Bool(b) => {
                    args.add(b);
                }
                Value::TinyInt(i) => {
                    args.add(i.map(Into::<i32>::into));
                }
                Value::SmallInt(i) => {
                    args.add(i.map(Into::<i32>::into));
                }
                Value::Int(i) => {
                    args.add(i);
                }
                Value::BigInt(i) => {
                    args.add(i);
                }
                Value::TinyUnsigned(i) => {
                    args.add(i.map(Into::<i32>::into));
                }
                Value::SmallUnsigned(i) => {
                    args.add(i.map(Into::<i32>::into));
                }
                Value::Unsigned(i) => {
                    args.add(i.map(Into::<i64>::into));
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
                #[cfg(feature = "postgres-array")]
                Value::BoolArray(_)
                | Value::TinyIntArray(_)
                | Value::SmallIntArray(_)
                | Value::IntArray(_)
                | Value::BigIntArray(_)
                | Value::SmallUnsignedArray(_)
                | Value::UnsignedArray(_)
                | Value::BigUnsignedArray(_)
                | Value::FloatArray(_)
                | Value::DoubleArray(_)
                | Value::StringArray(_)
                | Value::CharArray(_) => panic!("Array support not implemented for Any"),

                #[cfg(feature = "with-chrono")]
                Value::ChronoDate(d) => {
                    args.add(d.map(|d| *d));
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::ChronoDateArray(_) => panic!("Array support not implemented for Any"),
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(t) => {
                    args.add(t.map(|t| *t));
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::ChronoTimeArray(_) => panic!("Array support not implemented for Any"),
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(t) => {
                    args.add(t.map(|t| *t));
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::ChronoDateTimeArray(_) => panic!("Array support not implemented for Any"),
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(t) => {
                    args.add(t.map(|t| *t));
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::ChronoDateTimeUtcArray(_) => panic!("Array support not implemented for Any"),
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(t) => {
                    args.add(t.map(|t| *t));
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::ChronoDateTimeLocalArray(_) => {
                    panic!("Array support not implemented for Any")
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(t) => {
                    args.add(Value::ChronoDateTimeWithTimeZone(t).chrono_as_naive_utc_in_string());
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::ChronoDateTimeWithTimeZoneArray(_) => {
                    panic!("Array support not implemented for Any")
                }
                #[cfg(feature = "with-time")]
                Value::TimeDate(t) => {
                    args.add(Value::TimeDate(t).time_as_naive_utc_in_string());
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::TimeDateArray(_) => panic!("Array support not implemented for Any"),
                #[cfg(feature = "with-time")]
                Value::TimeTime(t) => {
                    args.add(Value::TimeTime(t).time_as_naive_utc_in_string());
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::TimeTimeArray(_) => panic!("Array support not implemented for Any"),
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(t) => {
                    args.add(Value::TimeDateTime(t).time_as_naive_utc_in_string());
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::TimeDateTimeArray(_) => panic!("Array support not implemented for Any"),
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(t) => {
                    args.add(Value::TimeDateTimeWithTimeZone(t).time_as_naive_utc_in_string());
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::TimeDateTimeWithTimeZoneArray(_) => {
                    panic!("Array support not implemented for Any")
                }
                #[cfg(feature = "with-uuid")]
                Value::Uuid(_) => {
                    panic!("UUID support not implemented for Any");
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::UuidArray(_) => panic!("Array support not implemented for Any"),
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(_) => {
                    panic!("Sqlite doesn't support decimal arguments");
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::DecimalArray(_) => panic!("Array support not implemented for Any"),
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(_) => {
                    panic!("Sqlite doesn't support bigdecimal arguments");
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::BigDecimalArray(_) => panic!("Array support not implemented for Any"),
                #[cfg(feature = "with-json")]
                Value::Json(_) => {
                    panic!("Json support not implemented for Any");
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::JsonArray(_) => panic!("Array support not implemented for Any"),
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(_) => {
                    panic!("SeaQuery doesn't support IpNetwork arguments for Any");
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::IpNetworkArray(_) => panic!("Array support not implemented for Any"),
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(_) => {
                    panic!("SeaQuery doesn't support MacAddress arguments for Any");
                }
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::MacAddressArray(_) => panic!("Array support not implemented for Any"),
            }
        }
        args
    }
}
