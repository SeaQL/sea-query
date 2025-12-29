#![forbid(unsafe_code)]

pub use rusqlite;

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
            Value::TimeDate(v) => v.to_sql(),
            #[cfg(feature = "with-time")]
            Value::TimeTime(v) => v.to_sql(),
            #[cfg(feature = "with-time")]
            Value::TimeDateTime(v) => v.to_sql(),
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(v) => v.to_sql(),
            #[cfg(feature = "with-uuid")]
            Value::Uuid(v) => v.to_sql(),
            #[cfg(feature = "with-json")]
            Value::Json(j) => {
                if cfg!(feature = "sea-orm") && j.is_some() && j.as_ref().unwrap().is_null() {
                    // rusqlite binds Json::Null as SQL NULL
                    // which is different from sqlx
                    return "null".to_sql();
                }
                match j {
                    Some(v) => v.as_ref().to_sql(),
                    None => Null.to_sql(),
                }
            }
            #[cfg(feature = "with-rust_decimal")]
            Value::Decimal(v) => opt_string_to_sql!(v.as_ref().map(|v| v.to_string())),
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(v) => opt_string_to_sql!(v.as_ref().map(|v| v.to_string())),
            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(v) => opt_string_to_sql!(v.as_ref().map(|v| v.to_string())),
            #[cfg(feature = "with-mac_address")]
            Value::MacAddress(v) => opt_string_to_sql!(v.as_ref().map(|v| v.to_string())),
            #[cfg(feature = "postgres-array")]
            Value::Array(_, _) => {
                panic!("Rusqlite doesn't support Array arguments");
            }
            #[cfg(feature = "postgres-vector")]
            Value::Vector(_) => {
                panic!("Rusqlite doesn't support Vector arguments");
            }
        }
    }
}
