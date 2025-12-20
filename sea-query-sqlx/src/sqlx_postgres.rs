use sqlx::Arguments;

use sea_query::Value;
#[cfg(all(feature = "with-json", feature = "postgres-array"))]
use sea_query::prelude::Json;
#[cfg(feature = "postgres-array")]
use sea_query::{Array, ArrayType};

use crate::SqlxValues;

impl sqlx::IntoArguments<'_, sqlx::postgres::Postgres> for SqlxValues {
    fn into_arguments(self) -> sqlx::postgres::PgArguments {
        let mut args = sqlx::postgres::PgArguments::default();
        for arg in self.0.into_iter() {
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
                    let _ = args.add(i.map(|i| i as i16));
                }
                Value::SmallUnsigned(i) => {
                    let _ = args.add(i.map(|i| i as i32));
                }
                Value::Unsigned(i) => {
                    let _ = args.add(i.map(|i| i as i64));
                }
                Value::BigUnsigned(i) => {
                    let _ = args.add(i.map(|i| <i64 as TryFrom<u64>>::try_from(i).unwrap()));
                }
                Value::Float(f) => {
                    let _ = args.add(f);
                }
                Value::Double(d) => {
                    let _ = args.add(d);
                }
                Value::String(s) => {
                    let _ = args.add(s.as_deref());
                }
                Value::Char(c) => {
                    let _ = args.add(c.map(|c| c.to_string()));
                }
                Value::Bytes(b) => {
                    let _ = args.add(b.as_deref());
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
                #[cfg(feature = "with-uuid")]
                Value::Uuid(uuid) => {
                    let _ = args.add(uuid);
                }
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(d) => {
                    let _ = args.add(d);
                }
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(d) => {
                    let _ = args.add(d.as_ref());
                }
                #[cfg(feature = "with-json")]
                Value::Json(j) => {
                    let _ = args.add(j);
                }
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(ip) => {
                    let _ = args.add(ip);
                }
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(mac) => {
                    let _ = args.add(mac);
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffDate(d) => {
                    let _ = args.add(d.map(jiff_sqlx::Date::from));
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffTime(t) => {
                    let _ = args.add(t.map(jiff_sqlx::Time::from));
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffDateTime(dt) => {
                    let _ = args.add(dt.map(jiff_sqlx::DateTime::from));
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffTimestamp(ts) => {
                    let _ = args.add(ts.map(jiff_sqlx::Timestamp::from));
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffZoned(z) => {
                    let _ = args.add(z.map(|z| jiff_sqlx::Timestamp::from(z.timestamp())));
                }
                Value::Enum(e) => {
                    // Bind as TEXT; We will explicit cast it in SQL (e.g., $1::my_enum_type)
                    let _ = args.add(e.map(|e| e.as_str().to_owned()));
                }
                #[cfg(feature = "postgres-array")]
                Value::Array(arr) => {
                    match_array(arr, &mut args);
                }
                #[cfg(feature = "postgres-vector")]
                Value::Vector(v) => {
                    let _ = args.add(v);
                }
                #[cfg(feature = "postgres-range")]
                Value::Range(r) => {
                    let _ = args.add(r);
                }
            }
        }
        args
    }
}

#[cfg(feature = "postgres-array")]
fn match_array(arr: Array, args: &mut sqlx::postgres::PgArguments) {
    match arr {
        Array::Null(ty) => match_null_array(ty, args),
        Array::Bool(inner) => {
            let _ = args.add(inner.into_vec());
        }
        Array::TinyInt(inner) => {
            let _ = args.add(inner.into_vec());
        }
        Array::SmallInt(inner) => {
            let _ = args.add(inner.into_vec());
        }
        Array::Int(inner) => {
            let _ = args.add(inner.into_vec());
        }
        Array::BigInt(inner) => {
            let _ = args.add(inner.into_vec());
        }
        Array::TinyUnsigned(inner) => {
            let v: Vec<Option<i16>> = inner
                .into_vec()
                .into_iter()
                .map(|x| x.map(|y| y as i16))
                .collect();
            let _ = args.add(v);
        }
        Array::SmallUnsigned(inner) => {
            let v: Vec<Option<i32>> = inner
                .into_vec()
                .into_iter()
                .map(|x| x.map(|y| y as i32))
                .collect();
            let _ = args.add(v);
        }
        Array::Unsigned(inner) => {
            let v: Vec<Option<i64>> = inner
                .into_vec()
                .into_iter()
                .map(|x| x.map(|y| y as i64))
                .collect();
            let _ = args.add(v);
        }
        Array::BigUnsigned(inner) => {
            let v: Vec<Option<i64>> = inner
                .into_vec()
                .into_iter()
                .map(|x| x.map(|y| <i64 as TryFrom<u64>>::try_from(y).unwrap()))
                .collect();
            let _ = args.add(v);
        }
        Array::Float(inner) => {
            let _ = args.add(inner.into_vec());
        }
        Array::Double(inner) => {
            let _ = args.add(inner.into_vec());
        }
        Array::String(inner) => {
            let _ = args.add(inner.into_vec());
        }
        Array::Char(inner) => {
            let v: Vec<Option<String>> = inner
                .into_vec()
                .into_iter()
                .map(|c| c.map(|x| x.to_string()))
                .collect();
            let _ = args.add(v);
        }
        Array::Bytes(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-json")]
        Array::Json(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-chrono")]
        Array::ChronoDate(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-chrono")]
        Array::ChronoTime(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-chrono")]
        Array::ChronoDateTime(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-chrono")]
        Array::ChronoDateTimeUtc(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-chrono")]
        Array::ChronoDateTimeLocal(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-chrono")]
        Array::ChronoDateTimeWithTimeZone(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-time")]
        Array::TimeDate(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-time")]
        Array::TimeTime(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-time")]
        Array::TimeDateTime(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-time")]
        Array::TimeDateTimeWithTimeZone(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-uuid")]
        Array::Uuid(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-rust_decimal")]
        Array::Decimal(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-bigdecimal")]
        Array::BigDecimal(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-ipnetwork")]
        Array::IpNetwork(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-mac_address")]
        Array::MacAddress(inner) => {
            let _ = args.add(inner.into_vec());
        }
        #[cfg(feature = "with-jiff")]
        Array::JiffDate(inner) => {
            let v: Vec<Option<jiff_sqlx::Date>> = inner
                .into_vec()
                .into_iter()
                .map(|x| x.map(Into::into))
                .collect();
            let _ = args.add(v);
        }
        #[cfg(feature = "with-jiff")]
        Array::JiffTime(inner) => {
            let v: Vec<Option<jiff_sqlx::Time>> = inner
                .into_vec()
                .into_iter()
                .map(|x| x.map(Into::into))
                .collect();
            let _ = args.add(v);
        }
        #[cfg(feature = "with-jiff")]
        Array::JiffDateTime(inner) => {
            let v: Vec<Option<jiff_sqlx::DateTime>> = inner
                .into_vec()
                .into_iter()
                .map(|x| x.map(Into::into))
                .collect();
            let _ = args.add(v);
        }
        #[cfg(feature = "with-jiff")]
        Array::JiffTimestamp(inner) => {
            let v: Vec<Option<jiff_sqlx::Timestamp>> = inner
                .into_vec()
                .into_iter()
                .map(|x| x.map(Into::into))
                .collect();
            let _ = args.add(v);
        }
        #[cfg(feature = "with-jiff")]
        Array::JiffZoned(inner) => {
            let v: Vec<Option<jiff_sqlx::Timestamp>> = inner
                .into_vec()
                .into_iter()
                .map(|x| x.map(|z| z.timestamp().into()))
                .collect();
            let _ = args.add(v);
        }
        Array::Enum(inner) => {
            // Bind as TEXT[]; use explicit cast in SQL (e.g., $1::my_enum_type[])
            let (_, arr) = inner.as_ref();
            let v: Vec<Option<String>> = arr
                .iter()
                .map(|e| e.as_ref().map(|e| e.as_str().to_owned()))
                .collect();
            let _ = args.add(v);
        }
        _ => {
            panic!("Unsupported array variant for sea-query-sqlx");
        }
    }
}

#[cfg(feature = "postgres-array")]
fn match_null_array(ty: ArrayType, args: &mut sqlx::postgres::PgArguments) {
    match ty {
        ArrayType::Bool => {
            let _ = args.add(Option::<Vec<Option<bool>>>::None);
        }
        ArrayType::TinyInt => {
            let _ = args.add(Option::<Vec<Option<i8>>>::None);
        }
        ArrayType::SmallInt => {
            let _ = args.add(Option::<Vec<Option<i16>>>::None);
        }
        ArrayType::Int => {
            let _ = args.add(Option::<Vec<Option<i32>>>::None);
        }
        ArrayType::BigInt => {
            let _ = args.add(Option::<Vec<Option<i64>>>::None);
        }
        ArrayType::TinyUnsigned => {
            let _ = args.add(Option::<Vec<Option<i16>>>::None);
        }
        ArrayType::SmallUnsigned => {
            let _ = args.add(Option::<Vec<Option<i32>>>::None);
        }
        ArrayType::Unsigned => {
            let _ = args.add(Option::<Vec<Option<i64>>>::None);
        }
        ArrayType::BigUnsigned => {
            let _ = args.add(Option::<Vec<Option<i64>>>::None);
        }
        ArrayType::Float => {
            let _ = args.add(Option::<Vec<Option<f32>>>::None);
        }
        ArrayType::Double => {
            let _ = args.add(Option::<Vec<Option<f64>>>::None);
        }
        ArrayType::String => {
            let _ = args.add(Option::<Vec<Option<String>>>::None);
        }
        ArrayType::Char => {
            let _ = args.add(Option::<Vec<Option<String>>>::None);
        }
        ArrayType::Bytes => {
            let _ = args.add(Option::<Vec<Option<Vec<u8>>>>::None);
        }
        ArrayType::Enum(_) => {
            let _ = args.add(Option::<Vec<Option<String>>>::None);
        }
        #[cfg(feature = "with-json")]
        ArrayType::Json => {
            let _ = args.add(Option::<Vec<Option<Json>>>::None);
        }
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDate => {
            let _ = args.add(Option::<Vec<Option<chrono::NaiveDate>>>::None);
        }
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoTime => {
            let _ = args.add(Option::<Vec<Option<chrono::NaiveTime>>>::None);
        }
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTime => {
            let _ = args.add(Option::<Vec<Option<chrono::NaiveDateTime>>>::None);
        }
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTimeUtc => {
            let _ = args.add(Option::<Vec<Option<chrono::DateTime<chrono::Utc>>>>::None);
        }
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTimeLocal => {
            let _ = args.add(Option::<Vec<Option<chrono::DateTime<chrono::Local>>>>::None);
        }
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTimeWithTimeZone => {
            let _ = args.add(Option::<Vec<Option<chrono::DateTime<chrono::FixedOffset>>>>::None);
        }
        #[cfg(feature = "with-time")]
        ArrayType::TimeDate => {
            let _ = args.add(Option::<Vec<Option<time::Date>>>::None);
        }
        #[cfg(feature = "with-time")]
        ArrayType::TimeTime => {
            let _ = args.add(Option::<Vec<Option<time::Time>>>::None);
        }
        #[cfg(feature = "with-time")]
        ArrayType::TimeDateTime => {
            let _ = args.add(Option::<Vec<Option<time::PrimitiveDateTime>>>::None);
        }
        #[cfg(feature = "with-time")]
        ArrayType::TimeDateTimeWithTimeZone => {
            let _ = args.add(Option::<Vec<Option<time::OffsetDateTime>>>::None);
        }
        #[cfg(feature = "with-jiff")]
        ArrayType::JiffDate => {
            let _ = args.add(Option::<Vec<Option<jiff_sqlx::Date>>>::None);
        }
        #[cfg(feature = "with-jiff")]
        ArrayType::JiffTime => {
            let _ = args.add(Option::<Vec<Option<jiff_sqlx::Time>>>::None);
        }
        #[cfg(feature = "with-jiff")]
        ArrayType::JiffDateTime => {
            let _ = args.add(Option::<Vec<Option<jiff_sqlx::DateTime>>>::None);
        }
        #[cfg(feature = "with-jiff")]
        ArrayType::JiffTimestamp => {
            let _ = args.add(Option::<Vec<Option<jiff_sqlx::Timestamp>>>::None);
        }
        #[cfg(feature = "with-jiff")]
        ArrayType::JiffZoned => {
            let _ = args.add(Option::<Vec<Option<jiff_sqlx::Timestamp>>>::None);
        }
        #[cfg(feature = "with-uuid")]
        ArrayType::Uuid => {
            let _ = args.add(Option::<Vec<Option<uuid::Uuid>>>::None);
        }
        #[cfg(feature = "with-rust_decimal")]
        ArrayType::Decimal => {
            let _ = args.add(Option::<Vec<Option<rust_decimal::Decimal>>>::None);
        }
        #[cfg(feature = "with-bigdecimal")]
        ArrayType::BigDecimal => {
            let _ = args.add(Option::<Vec<Option<bigdecimal::BigDecimal>>>::None);
        }
        #[cfg(feature = "with-ipnetwork")]
        ArrayType::IpNetwork => {
            let _ = args.add(Option::<Vec<Option<ipnetwork::IpNetwork>>>::None);
        }
        #[cfg(feature = "with-mac_address")]
        ArrayType::MacAddress => {
            let _ = args.add(Option::<Vec<Option<mac_address::MacAddress>>>::None);
        }
    }
}

#[cfg(all(test, feature = "postgres-array", feature = "with-json"))]
mod tests {
    use super::*;
    use sqlx::Arguments;

    #[test]
    fn bind_null_json_array_adds_argument() {
        let mut args = sqlx::postgres::PgArguments::default();
        match_array(Array::Null(ArrayType::Json), &mut args);
        assert_eq!(args.len(), 1);
    }
}
