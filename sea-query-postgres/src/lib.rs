use std::error::Error;

use bytes::BytesMut;
use postgres_types::{to_sql_checked, IsNull, ToSql, Type};

use sea_query::{query::*, QueryBuilder, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct PostgresValue(pub Value);
#[derive(Clone, Debug, PartialEq)]
pub struct PostgresValues(pub Vec<PostgresValue>);

impl<'a> PostgresValues {
    pub fn as_params(&'a self) -> Vec<&'a (dyn ToSql + Sync)> {
        self.0
            .iter()
            .map(|x| {
                let y: &(dyn ToSql + Sync) = x;
                y
            })
            .collect()
    }
}

pub trait PostgresBinder {
    fn build_postgres<T: QueryBuilder>(&self, query_builder: T) -> (String, PostgresValues);
}

macro_rules! impl_postgres_binder {
    ($l:ident) => {
        impl PostgresBinder for $l {
            fn build_postgres<T: QueryBuilder>(
                &self,
                query_builder: T,
            ) -> (String, PostgresValues) {
                let (query, values) = self.build(query_builder);
                (
                    query,
                    PostgresValues(values.into_iter().map(PostgresValue).collect()),
                )
            }
        }
    };
}

impl_postgres_binder!(SelectStatement);
impl_postgres_binder!(UpdateStatement);
impl_postgres_binder!(InsertStatement);
impl_postgres_binder!(DeleteStatement);

impl ToSql for PostgresValue {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        macro_rules! to_sql {
            ( $v: expr, $ty: ty ) => {
                $v.map(|v| v as $ty).as_ref().to_sql(ty, out)
            };
        }
        match &self.0 {
            Value::Bool(v) => to_sql!(v, bool),
            Value::TinyInt(v) => to_sql!(v, i8),
            Value::SmallInt(v) => to_sql!(v, i16),
            Value::Int(v) => to_sql!(v, i32),
            Value::BigInt(v) => to_sql!(v, i64),
            Value::TinyUnsigned(v) => to_sql!(v, u32),
            Value::SmallUnsigned(v) => to_sql!(v, u32),
            Value::Unsigned(v) => to_sql!(v, u32),
            Value::BigUnsigned(v) => to_sql!(v, i64),
            Value::Float(v) => to_sql!(v, f32),
            Value::Double(v) => to_sql!(v, f64),
            Value::String(v) => v.as_deref().to_sql(ty, out),
            Value::Char(v) => v.map(|v| v.to_string()).to_sql(ty, out),
            Value::Bytes(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-json")]
            Value::Json(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDate(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoTime(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-time")]
            Value::TimeDate(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-time")]
            Value::TimeTime(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-time")]
            Value::TimeDateTime(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-rust_decimal")]
            Value::Decimal(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-uuid")]
            Value::Uuid(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "postgres-array")]
            Value::Array(_, Some(v)) => v
                .iter()
                .map(|v| PostgresValue(v.clone()))
                .collect::<Vec<PostgresValue>>()
                .to_sql(ty, out),
            #[cfg(feature = "postgres-array")]
            Value::Array(_, None) => Ok(IsNull::Yes),
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        }
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }

    to_sql_checked!();
}
