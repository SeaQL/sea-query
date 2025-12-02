use sqlx::Arguments;

#[cfg(feature = "postgres-array")]
use sea_query::Array;
use sea_query::Value;

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
                    match arr {
                        Some(a) => match_some_array(a, &mut args),
                        None => {
                            // TODO: Add Array::Null?
                            panic!("Postgres does not support binding null to arrays");
                        }
                    };
                }
                #[cfg(feature = "postgres-vector")]
                Value::Vector(v) => {
                    let _ = args.add(v);
                }
                #[cfg(feature = "postgres-range")]
                Value::Range(r) => {
                    use sea_query::value::prelude::RangeType;
                    use sqlx::postgres::types::PgRange;

                    match r.as_deref() {
                        Some(RangeType::Int4Range(lo, hi)) => {
                            let _ = args.add(PgRange::from((lo.into(), hi.into())));
                        }
                        Some(RangeType::Int8Range(lo, hi)) => {
                            let _ = args.add(PgRange::from((lo.into(), hi.into())));
                        }
                        // sqlx doesn't support PgRange<f64>, so we convert to Decimal/BigDecimal
                        #[cfg(feature = "with-rust_decimal")]
                        Some(RangeType::NumRange(lo, hi)) => {
                            use rust_decimal::Decimal;
                            use std::ops::Bound;
                            let lo: Bound<Decimal> = lo.into();
                            let hi: Bound<Decimal> = hi.into();
                            let _ = args.add(PgRange::from((lo, hi)));
                        }
                        #[cfg(all(
                            feature = "with-bigdecimal",
                            not(feature = "with-rust_decimal")
                        ))]
                        Some(RangeType::NumRange(lo, hi)) => {
                            use bigdecimal::BigDecimal;
                            use std::ops::Bound;
                            let lo: Bound<BigDecimal> = lo.into();
                            let hi: Bound<BigDecimal> = hi.into();
                            let _ = args.add(PgRange::from((lo, hi)));
                        }
                        #[cfg(not(any(
                            feature = "with-rust_decimal",
                            feature = "with-bigdecimal"
                        )))]
                        Some(RangeType::NumRange(_, _)) => {
                            panic!(
                                "NumRange requires with-rust_decimal or with-bigdecimal feature"
                            );
                        }
                        None => {
                            // use a dummy type to represent NULL range
                            let _ = args.add(None::<PgRange<i32>>);
                        }
                    }
                }
            }
        }
        args
    }
}

#[cfg(feature = "postgres-array")]
fn match_some_array(arr: Array, args: &mut sqlx::postgres::PgArguments) {
    match arr {
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
        Array::Array(_) => {
            panic!("Nested arrays (Array::Array) are not supported by sea-query-sqlx");
        }
        _ => {
            panic!("Unsupported array variant for sea-query-sqlx");
        }
    }
}
