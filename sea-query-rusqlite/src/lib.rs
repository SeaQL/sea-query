#![forbid(unsafe_code)]

use rusqlite::{
    Result, ToSql,
    types::{Null, ToSqlOutput},
};
use sea_query::Value;
use sea_query::{QueryBuilder, query::*};

#[derive(Clone, Debug, PartialEq)]
pub struct RusqliteValue(pub sea_query::Value);
#[derive(Clone, Debug, PartialEq)]
pub struct RusqliteValues(pub Vec<RusqliteValue>);

impl RusqliteValues {
    pub fn as_params(&self) -> Vec<&dyn ToSql> {
        self.0
            .iter()
            .map(|x| {
                let y: &dyn ToSql = x;
                y
            })
            .collect()
    }
}

pub trait RusqliteBinder {
    fn build_rusqlite<T: QueryBuilder>(&self, query_builder: T) -> (String, RusqliteValues);
}

macro_rules! impl_rusqlite_binder {
    ($l:ident) => {
        impl RusqliteBinder for $l {
            fn build_rusqlite<T: QueryBuilder>(
                &self,
                query_builder: T,
            ) -> (String, RusqliteValues) {
                let (query, values) = self.build(query_builder);
                (
                    query,
                    RusqliteValues(values.into_iter().map(RusqliteValue).collect()),
                )
            }
        }
    };
}

impl_rusqlite_binder!(SelectStatement);
impl_rusqlite_binder!(UpdateStatement);
impl_rusqlite_binder!(InsertStatement);
impl_rusqlite_binder!(DeleteStatement);
impl_rusqlite_binder!(WithQuery);

impl ToSql for RusqliteValue {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        macro_rules! box_to_sql {
            ( $v: expr ) => {
                match $v {
                    Some(v) => v.to_sql(),
                    None => Null.to_sql(),
                }
            };
        }

        macro_rules! opt_string_to_sql {
            ( $v: expr ) => {
                match $v {
                    Some(v) => Ok(ToSqlOutput::from(v)),
                    None => Null.to_sql(),
                }
            };
        }

        match &self.0 {
            Value::Bool(v) => v.to_sql(),
            Value::TinyInt(v) => v.to_sql(),
            Value::SmallInt(v) => v.to_sql(),
            Value::Int(v) => v.to_sql(),
            Value::BigInt(v) => v.to_sql(),
            Value::TinyUnsigned(v) => v.to_sql(),
            Value::SmallUnsigned(v) => v.to_sql(),
            Value::Unsigned(v) => v.to_sql(),
            Value::BigUnsigned(v) => v.to_sql(),
            Value::Float(v) => v.to_sql(),
            Value::Double(v) => v.to_sql(),
            Value::String(v) => match v {
                Some(v) => v.as_str().to_sql(),
                None => Null.to_sql(),
            },
            Value::Char(v) => opt_string_to_sql!(v.map(|v| v.to_string())),
            Value::Bytes(v) => match v {
                Some(v) => v.as_slice().to_sql(),
                None => Null.to_sql(),
            },
            #[cfg(feature = "with-chrono")]
            Value::ChronoDate(v) => v.to_sql(),
            #[cfg(feature = "with-chrono")]
            Value::ChronoTime(v) => v.to_sql(),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(v) => v.to_sql(),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(v) => v.to_sql(),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(v) => v.to_sql(),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(v) => v.to_sql(),
            #[cfg(feature = "with-time")]
            v @ Value::TimeDate(_) => opt_string_to_sql!(v.time_as_naive_utc_in_string()),
            #[cfg(feature = "with-time")]
            v @ Value::TimeTime(_) => opt_string_to_sql!(v.time_as_naive_utc_in_string()),
            #[cfg(feature = "with-time")]
            v @ Value::TimeDateTime(_) => opt_string_to_sql!(v.time_as_naive_utc_in_string()),
            #[cfg(feature = "with-time")]
            v @ Value::TimeDateTimeWithTimeZone(_) => {
                opt_string_to_sql!(v.time_as_naive_utc_in_string())
            }
            #[cfg(feature = "with-uuid")]
            Value::Uuid(v) => v.to_sql(),
            #[cfg(feature = "with-json")]
            Value::Json(j) => box_to_sql!(j),
            #[cfg(feature = "with-rust_decimal")]
            Value::Decimal(_) => {
                panic!("Rusqlite doesn't support rust_decimal arguments");
            }
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(_) => {
                panic!("Rusqlite doesn't support bigdecimal arguments");
            }
            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(_) => {
                panic!("Rusqlite doesn't support IpNetwork arguments");
            }
            #[cfg(feature = "with-mac_address")]
            Value::MacAddress(_) => {
                panic!("Rusqlite doesn't support MacAddress arguments");
            }
            #[cfg(feature = "postgres-array")]
            Value::Array(_) => {
                panic!("Rusqlite doesn't support Array arguments");
            }
            #[cfg(feature = "postgres-vector")]
            Value::Vector(_) => {
                panic!("Rusqlite doesn't support Vector arguments");
            }
        }
    }
}
