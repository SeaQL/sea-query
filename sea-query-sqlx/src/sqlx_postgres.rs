#[cfg(feature = "with-bigdecimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "with-chrono")]
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};
#[cfg(feature = "with-ipnetwork")]
use ipnetwork::IpNetwork;
#[cfg(feature = "with-mac_address")]
use mac_address::MacAddress;
#[cfg(feature = "with-rust_decimal")]
use rust_decimal::Decimal;
#[cfg(feature = "with-json")]
use serde_json::Value as Json;
use sqlx::Arguments;
#[cfg(feature = "with-uuid")]
use uuid::Uuid;

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
        _ => {}
    }
}
