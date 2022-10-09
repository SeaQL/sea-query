use crate::SqlxValues;
use sea_query::Value;

impl<'q> sqlx::IntoArguments<'q, sqlx::postgres::Postgres> for SqlxValues {
    fn into_arguments(self) -> sqlx::postgres::PgArguments {
        let mut args = sqlx::postgres::PgArguments::default();
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
                    args.add(i.map(|i| i as i16));
                }
                Value::SmallUnsigned(i) => {
                    args.add(i.map(|i| i as i32));
                }
                Value::Unsigned(i) => {
                    args.add(i.map(|i| i as i64));
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
                    args.add(s.as_deref());
                }
                Value::Char(c) => {
                    args.add(c.map(|c| c.to_string()));
                }
                Value::Bytes(b) => {
                    args.add(b.as_deref());
                }
                #[cfg(feature = "postgres-array")]
                Value::BoolArray(v) => args.add(v.as_deref()),
                #[cfg(feature = "postgres-array")]
                Value::TinyIntArray(v) => args.add(v.as_deref()),
                #[cfg(feature = "postgres-array")]
                Value::SmallIntArray(v) => args.add(v.as_deref()),
                #[cfg(feature = "postgres-array")]
                Value::IntArray(v) => args.add(v.as_deref()),
                #[cfg(feature = "postgres-array")]
                Value::BigIntArray(v) => args.add(v.as_deref()),
                #[cfg(feature = "postgres-array")]
                Value::SmallUnsignedArray(v) => {
                    args.add(v.map(|v| v.iter().map(|&i| i as i32).collect::<Vec<i32>>()))
                }
                #[cfg(feature = "postgres-array")]
                Value::UnsignedArray(v) => {
                    args.add(v.map(|v| v.iter().map(|&i| i as i64).collect::<Vec<i64>>()))
                }
                #[cfg(feature = "postgres-array")]
                Value::BigUnsignedArray(v) => args.add(v.map(|v| {
                    v.iter()
                        .map(|&i| <i64 as std::convert::TryFrom<u64>>::try_from(i).unwrap())
                        .collect::<Vec<i64>>()
                })),
                #[cfg(feature = "postgres-array")]
                Value::FloatArray(v) => args.add(v.as_deref()),
                #[cfg(feature = "postgres-array")]
                Value::DoubleArray(v) => args.add(v.as_deref()),
                #[cfg(feature = "postgres-array")]
                Value::StringArray(v) => args.add(v.as_deref()),
                #[cfg(feature = "postgres-array")]
                Value::CharArray(v) => {
                    args.add(v.map(|v| v.iter().map(|c| c.to_string()).collect::<Vec<String>>()))
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDate(d) => args.add(d.as_deref()),
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::ChronoDateArray(d) => args.add(d.as_deref()),
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(d) => args.add(d.as_deref()),
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::ChronoTimeArray(d) => args.add(d.as_deref()),
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(d) => args.add(d.as_deref()),
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::ChronoDateTimeArray(d) => args.add(d.as_deref()),
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(d) => args.add(d.as_deref()),
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::ChronoDateTimeUtcArray(d) => args.add(d.as_deref()),
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(d) => args.add(d.as_deref()),
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::ChronoDateTimeLocalArray(d) => args.add(d.as_deref()),
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(d) => args.add(d.as_deref()),
                #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                Value::ChronoDateTimeWithTimeZoneArray(d) => args.add(d.as_deref()),
                #[cfg(feature = "with-time")]
                Value::TimeDate(t) => args.add(t.as_deref()),
                #[cfg(all(feature = "with-time", feature = "postgres-array"))]
                Value::TimeDateArray(t) => args.add(t.as_deref()),
                #[cfg(feature = "with-time")]
                Value::TimeTime(t) => args.add(t.as_deref()),
                #[cfg(all(feature = "with-time", feature = "postgres-array"))]
                Value::TimeTimeArray(t) => args.add(t.as_deref()),
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(t) => args.add(t.as_deref()),
                #[cfg(all(feature = "with-time", feature = "postgres-array"))]
                Value::TimeDateTimeArray(t) => args.add(t.as_deref()),
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(t) => args.add(t.as_deref()),
                #[cfg(all(feature = "with-time", feature = "postgres-array"))]
                Value::TimeDateTimeWithTimeZoneArray(t) => args.add(t.as_deref()),
                #[cfg(feature = "with-uuid")]
                Value::Uuid(uuid) => {
                    args.add(uuid.as_deref());
                }
                #[cfg(all(feature = "with-uuid", feature = "postgres-array"))]
                Value::UuidArray(uuid) => args.add(uuid.as_deref()),
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(d) => {
                    args.add(d.as_deref());
                }
                #[cfg(all(feature = "with-rust_decimal", feature = "postgres-array"))]
                Value::DecimalArray(d) => args.add(d.as_deref()),
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(d) => {
                    args.add(d.as_deref());
                }
                #[cfg(all(feature = "with-bigdecimal", feature = "postgres-array"))]
                Value::BigDecimalArray(d) => args.add(d.as_deref()),
                #[cfg(feature = "with-json")]
                Value::Json(j) => {
                    args.add(j.as_deref());
                }
                #[cfg(all(feature = "with-json", feature = "postgres-array"))]
                Value::JsonArray(d) => args.add(d.as_deref()),
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(ip) => {
                    args.add(ip.as_deref());
                }
                #[cfg(all(feature = "with-ipnetwork", feature = "postgres-array"))]
                Value::IpNetworkArray(d) => args.add(d.as_deref()),
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(mac) => {
                    args.add(mac.as_deref());
                }
                #[cfg(all(feature = "with-mac_address", feature = "postgres-array"))]
                Value::MacAddressArray(d) => args.add(d.as_deref()),
            }
        }
        args
    }
}
