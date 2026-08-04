#![forbid(unsafe_code)]

pub use rusqlite;

#[cfg(feature = "postgres-array")]
use std::rc::Rc;

use rusqlite::{
    Result, ToSql,
    types::{Null, ToSqlOutput},
};
use sea_query::{OptionEnum, Value};
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
            Value::Enum(v) => match v {
                OptionEnum::Some(v) => v.value.as_ref().to_sql(),
                OptionEnum::None(_) => Null.to_sql(),
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
            #[cfg(feature = "with-jiff")]
            Value::JiffDate(v) => v.to_sql(),
            #[cfg(feature = "with-jiff")]
            Value::JiffTime(v) => v.to_sql(),
            #[cfg(feature = "with-jiff")]
            Value::JiffDateTime(v) => v.to_sql(),
            #[cfg(feature = "with-jiff")]
            Value::JiffTimestamp(v) => v.to_sql(),
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
            Value::Array(_, values) => match values {
                Some(values) => {
                    let values = values
                        .iter()
                        .map(rusqlite_value)
                        .collect::<Result<Vec<_>>>()?;
                    Ok(ToSqlOutput::Array(Rc::new(values)))
                }
                None => Null.to_sql(),
            },
            #[cfg(feature = "postgres-vector")]
            Value::Vector(_) => {
                panic!("Rusqlite doesn't support Vector arguments");
            }
        }
    }
}

/// Convert a single [`sea_query::Value`] element to a [`rusqlite::types::Value`],
/// reusing the existing [`ToSql`] conversion so all feature-gated value types
/// (chrono, time, uuid, json, ...) are handled consistently.
#[cfg(feature = "postgres-array")]
fn rusqlite_value(value: &sea_query::Value) -> Result<rusqlite::types::Value> {
    match RusqliteValue(value.clone()).to_sql()? {
        ToSqlOutput::Borrowed(value_ref) => Ok(value_ref.into()),
        ToSqlOutput::Owned(value) => Ok(value),
        ToSqlOutput::Array(_) => Err(rusqlite::Error::ToSqlConversionFailure(
            "Nested arrays are not supported by the rusqlite rarray".into(),
        )),
        // `ToSqlOutput` is `#[non_exhaustive]`, and the `blob`/`functions` features of
        // rusqlite are not enabled by this crate.
        _ => unreachable!("unexpected ToSqlOutput variant"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_query::{Alias, ArrayType, Expr, Func, Query, SqliteQueryBuilder};

    #[cfg(feature = "postgres-array")]
    #[test]
    fn test_array_to_sql() {
        let value = RusqliteValue(Value::Array(
            ArrayType::BigInt,
            Some(Box::new(vec![
                Value::BigInt(Some(1)),
                Value::BigInt(None),
                Value::BigInt(Some(3)),
            ])),
        ));
        match value.to_sql().unwrap() {
            ToSqlOutput::Array(array) => {
                assert_eq!(
                    array.as_ref(),
                    &[
                        rusqlite::types::Value::Integer(1),
                        rusqlite::types::Value::Null,
                        rusqlite::types::Value::Integer(3),
                    ]
                );
            }
            _ => panic!("expected ToSqlOutput::Array"),
        }
    }

    #[cfg(feature = "postgres-array")]
    #[test]
    fn test_null_array_to_sql() {
        let value = RusqliteValue(Value::Array(ArrayType::BigInt, None));
        assert!(matches!(
            value.to_sql().unwrap(),
            ToSqlOutput::Owned(rusqlite::types::Value::Null)
        ));
    }

    #[cfg(feature = "postgres-array")]
    #[test]
    fn test_rarray_query() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        rusqlite::vtab::array::load_module(&conn).unwrap();

        let (sql, values) = Query::select()
            .column(Alias::new("value"))
            .from_function(Func::cust("rarray").arg(Expr::val(vec![1i64, 2, 3])), "t")
            .build_rusqlite(SqliteQueryBuilder);

        assert_eq!(sql, r#"SELECT "value" FROM rarray(?) AS "t""#);

        let mut stmt = conn.prepare_cached(&sql).unwrap();
        let mut rows = stmt.query(&*values.as_params()).unwrap();
        let mut collected = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            collected.push(row.get_unwrap::<_, i64>(0));
        }
        assert_eq!(collected, vec![1, 2, 3]);
    }
}
