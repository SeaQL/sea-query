#![forbid(unsafe_code)]

use std::error::Error;

use bytes::BytesMut;
use postgres_types::{IsNull, ToSql, Type, to_sql_checked};

use sea_query::{QueryBuilder, Value, query::*};

#[derive(Clone, Debug, PartialEq)]
pub struct PostgresValue(pub Value);
#[derive(Clone, Debug, PartialEq)]
pub struct PostgresValues(pub Vec<PostgresValue>);

impl PostgresValues {
    pub fn as_params(&self) -> Vec<&(dyn ToSql + Sync)> {
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
impl_postgres_binder!(WithQuery);

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
            Value::Json(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDate(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-time")]
            Value::TimeDate(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-time")]
            Value::TimeTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-time")]
            Value::TimeDateTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-rust_decimal")]
            Value::Decimal(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(v) => {
                use bigdecimal::ToPrimitive;
                v.as_deref()
                    .map(|v| v.to_f64().expect("Fail to convert bigdecimal as f64"))
                    .to_sql(ty, out)
            }
            #[cfg(feature = "with-uuid")]
            Value::Uuid(v) => v.to_sql(ty, out),
            #[cfg(feature = "postgres-array")]
            Value::Array(_, Some(v)) => v
                .iter()
                .map(|v| PostgresValue(v.clone()))
                .collect::<Vec<PostgresValue>>()
                .to_sql(ty, out),
            #[cfg(feature = "postgres-array")]
            Value::Array(_, None) => Ok(IsNull::Yes),
            #[cfg(feature = "postgres-vector")]
            Value::Vector(Some(v)) => v.to_sql(ty, out),
            #[cfg(feature = "postgres-vector")]
            Value::Vector(None) => Ok(IsNull::Yes),
            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(v) => {
                use cidr::IpCidr;
                v.map(|v| {
                    IpCidr::new(v.network(), v.prefix())
                        .expect("Fail to convert IpNetwork to IpCidr")
                })
                .to_sql(ty, out)
            }
            #[cfg(feature = "with-mac_address")]
            Value::MacAddress(v) => {
                use eui48::MacAddress;
                v.map(|v| MacAddress::new(v.bytes())).to_sql(ty, out)
            }
        }
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }

    to_sql_checked!();
}
