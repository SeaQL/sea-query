#![forbid(unsafe_code)]

use std::error::Error;

use bytes::BytesMut;
use postgres_types::{IsNull, ToSql, Type, to_sql_checked};

#[cfg(feature = "postgres-array")]
use sea_query::Array;
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
            Value::Enum(v) => v.map(|v| v.as_str().to_sql(ty, out)),
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
            #[cfg(feature = "with-jiff")]
            Value::JiffDate(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-jiff")]
            Value::JiffTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-jiff")]
            Value::JiffDateTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-jiff")]
            Value::JiffTimestamp(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-jiff")]
            Value::JiffZoned(v) => v.as_ref().map(|z| z.timestamp()).to_sql(ty, out),
            #[cfg(feature = "with-rust_decimal")]
            Value::Decimal(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(v) => {
                use bigdecimal::ToPrimitive;
                v.as_ref()
                    .map(|x| {
                        x.to_f64().ok_or(PostgresBindError::new(
                            "Fail to convert bigdecimal as f64 for sea-query-postgres binder",
                        ))
                    })
                    .transpose()?
                    .to_sql(ty, out)
            }
            #[cfg(feature = "with-uuid")]
            Value::Uuid(v) => v.to_sql(ty, out),
            #[cfg(feature = "postgres-array")]
            Value::Array(Some(arr)) => match arr {
                Array::Bool(inner) => inner.to_sql(ty, out),
                Array::TinyInt(inner) => inner.to_sql(ty, out),
                Array::SmallInt(inner) => inner.to_sql(ty, out),
                Array::Int(inner) => inner.to_sql(ty, out),
                Array::BigInt(inner) => inner.to_sql(ty, out),
                Array::TinyUnsigned(inner) => inner
                    .iter()
                    .map(|v| v.map(|x| x as u32))
                    .collect::<Vec<Option<_>>>()
                    .to_sql(ty, out),
                Array::SmallUnsigned(inner) => inner
                    .iter()
                    .map(|v| v.map(|x| x as u32))
                    .collect::<Vec<Option<_>>>()
                    .to_sql(ty, out),
                Array::Unsigned(inner) => inner.to_sql(ty, out),
                Array::BigUnsigned(inner) => inner
                    .into_iter()
                    .map(|v| v.map(i64::try_from).transpose())
                    .collect::<Result<Vec<Option<_>>, _>>()?
                    .to_sql(ty, out),
                Array::Float(inner) => inner.to_sql(ty, out),
                Array::Double(inner) => inner.to_sql(ty, out),
                Array::String(inner) => inner.to_sql(ty, out),
                Array::Char(inner) => inner
                    .into_iter()
                    .map(|v| v.map(|c| c.to_string()))
                    .collect::<Vec<Option<String>>>()
                    .to_sql(ty, out),
                Array::Bytes(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-json")]
                Array::Json(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-chrono")]
                Array::ChronoDate(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-chrono")]
                Array::ChronoTime(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTime(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTimeUtc(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTimeLocal(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTimeWithTimeZone(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-time")]
                Array::TimeDate(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-time")]
                Array::TimeTime(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-time")]
                Array::TimeDateTime(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-time")]
                Array::TimeDateTimeWithTimeZone(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-uuid")]
                Array::Uuid(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-rust_decimal")]
                Array::Decimal(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-bigdecimal")]
                Array::BigDecimal(inner) => {
                    use bigdecimal::ToPrimitive;
                    inner
                        .iter()
                        .map(|v| {
                            v.as_ref()
                                .map(|bd| {
                                    bd.to_f64().ok_or(PostgresBindError::new(
                                    "Fail to convert bigdecimal as f64 for sea-query-postgres binder",
                                    ))
                                })
                                .transpose()
                        })
                        .collect::<Result<Vec<Option<f64>>, _>>()?
                        .to_sql(ty, out)
                }
                #[cfg(feature = "with-ipnetwork")]
                Array::IpNetwork(inner) => inner
                    .iter()
                    .cloned()
                    .map(|v| v.map(conv_ip_network))
                    .collect::<Vec<_>>()
                    .to_sql(ty, out),
                #[cfg(feature = "with-mac_address")]
                Array::MacAddress(inner) => inner
                    .into_iter()
                    .map(|v| v.map(conv_mac_address))
                    .collect::<Vec<_>>()
                    .to_sql(ty, out),
                #[cfg(feature = "with-jiff")]
                Array::JiffDate(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-jiff")]
                Array::JiffTime(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-jiff")]
                Array::JiffDateTime(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-jiff")]
                Array::JiffTimestamp(inner) => inner.to_sql(ty, out),
                #[cfg(feature = "with-jiff")]
                Array::JiffZoned(inner) => inner
                    .iter()
                    .map(|v| v.as_ref().map(|z| z.timestamp()))
                    .collect::<Vec<_>>()
                    .to_sql(ty, out),
                Array::Array(_) => Err(PostgresBindError::new(
                    "Nested arrays (Array::Array) are not supported by sea-query-postgres binder",
                )
                .into()),
                Array::Enum(v) => v
                    .as_ref()
                    .1
                    .iter()
                    .map(|v| v.as_ref())
                    .collect::<Vec<_>>()
                    .to_sql(ty, out),
                _ => Err(PostgresBindError::new(
                    "Unsupported array variant for sea-query-postgres binder",
                )
                .into()),
            },
            #[cfg(feature = "postgres-array")]
            Value::Array(None) => Ok(IsNull::Yes),
            #[cfg(feature = "postgres-vector")]
            Value::Vector(Some(v)) => v.to_sql(ty, out),
            #[cfg(feature = "postgres-vector")]
            Value::Vector(None) => Ok(IsNull::Yes),
            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(v) => v.map(conv_ip_network).to_sql(ty, out),
            #[cfg(feature = "with-mac_address")]
            Value::MacAddress(v) => v.map(conv_mac_address).to_sql(ty, out),
            #[cfg(feature = "postgres-range")]
            Value::Range(None) => Ok(IsNull::Yes),
            #[cfg(feature = "postgres-range")]
            Value::Range(Some(v)) => v.to_sql(ty, out),
        }
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }

    to_sql_checked!();
}

#[derive(Debug, Clone)]
struct PostgresBindError(&'static str);

impl PostgresBindError {
    fn new(msg: &'static str) -> Self {
        Self(msg)
    }
}

impl std::fmt::Display for PostgresBindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl Error for PostgresBindError {}

#[cfg(feature = "with-mac_address")]
fn conv_mac_address(input: mac_address::MacAddress) -> eui48::MacAddress {
    use eui48::MacAddress;
    MacAddress::new(input.bytes())
}

#[cfg(feature = "with-ipnetwork")]
fn conv_ip_network(input: ipnetwork::IpNetwork) -> cidr::IpCidr {
    use cidr::IpCidr;
    IpCidr::new(input.network(), input.prefix()).expect("Fail to convert IpNetwork to IpCidr")
}
