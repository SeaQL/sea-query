#![forbid(unsafe_code)]

use std::error::Error;

use bytes::BytesMut;
use postgres_types::{IsNull, ToSql, Type, to_sql_checked};

use sea_query::{ArrayType, QueryBuilder, Value, query::*};

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

    pub fn as_types(&self) -> Vec<Type> {
        self.0
            .iter()
            .map(|x| &x.0)
            .map(value_to_postgres_type)
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
            Value::Json(v) => v.as_deref().to_sql(ty, out),
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

fn value_to_postgres_type(value: &Value) -> Type {
    match value {
        Value::Bool(_) => Type::BOOL,
        Value::TinyInt(_) => Type::INT2,
        Value::TinyUnsigned(_) => Type::INT2,
        Value::SmallInt(_) => Type::INT2,
        Value::SmallUnsigned(_) => Type::INT4,
        Value::Int(_) => Type::INT4,
        Value::BigInt(_) => Type::INT8,
        Value::Unsigned(_) => Type::INT8,
        Value::BigUnsigned(_) => Type::NUMERIC,
        Value::Float(_) => Type::FLOAT4,
        Value::Double(_) => Type::FLOAT8,
        Value::String(_) => Type::TEXT,
        Value::Char(_) => Type::CHAR,
        Value::Bytes(_) => Type::BYTEA,
        #[cfg(feature = "with-json")]
        Value::Json(_) => Type::JSON,
        #[cfg(feature = "with-chrono")]
        Value::ChronoDate(_) => Type::DATE,
        #[cfg(feature = "with-chrono")]
        Value::ChronoTime(_) => Type::TIME,
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTime(_) => Type::TIMESTAMP,
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeUtc(_) => Type::TIMESTAMP,
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeLocal(_) => Type::TIMESTAMP,
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeWithTimeZone(_) => Type::TIMESTAMPTZ,
        #[cfg(feature = "with-time")]
        Value::TimeDate(_) => Type::DATE,
        #[cfg(feature = "with-time")]
        Value::TimeTime(_) => Type::TIME,
        #[cfg(feature = "with-time")]
        Value::TimeDateTime(_) => Type::TIMESTAMP,
        #[cfg(feature = "with-time")]
        Value::TimeDateTimeWithTimeZone(_) => Type::TIMESTAMPTZ,
        #[cfg(feature = "with-uuid")]
        Value::Uuid(_) => Type::UUID,
        #[cfg(feature = "with-rust_decimal")]
        Value::Decimal(_) => Type::NUMERIC,
        #[cfg(feature = "with-bigdecimal")]
        Value::BigDecimal(_) => Type::NUMERIC,
        #[cfg(feature = "postgres-array")]
        Value::Array(ty, _) => array_type_to_pg_type(ty),
        #[cfg(feature = "postgres-vector")]
        Value::Vector(_) => todo!(),
        #[cfg(feature = "with-ipnetwork")]
        Value::IpNetwork(_) => Type::INET,
        #[cfg(feature = "with-mac_address")]
        Value::MacAddress(_) => Type::MACADDR,
    }
}

fn array_type_to_pg_type(ty: &ArrayType) -> Type {
    match ty {
        ArrayType::Bool => Type::BOOL_ARRAY,
        ArrayType::TinyInt => Type::INT2_ARRAY,
        ArrayType::TinyUnsigned => Type::INT2_ARRAY,
        ArrayType::SmallInt => Type::INT2_ARRAY,
        ArrayType::SmallUnsigned => Type::INT4_ARRAY,
        ArrayType::Int => Type::INT4_ARRAY,
        ArrayType::Unsigned => Type::INT8_ARRAY,
        ArrayType::BigInt => Type::INT8_ARRAY,
        ArrayType::BigUnsigned => Type::NUMERIC_ARRAY,
        ArrayType::Float => Type::FLOAT4_ARRAY,
        ArrayType::Double => Type::FLOAT8_ARRAY,
        ArrayType::String => Type::TEXT_ARRAY,
        ArrayType::Char => Type::CHAR_ARRAY,
        ArrayType::Bytes => Type::BYTEA_ARRAY,
        #[cfg(feature = "with-json")]
        ArrayType::Json => Type::JSON_ARRAY,
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDate => Type::DATE_ARRAY,
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoTime => Type::TIME_ARRAY,
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTime => Type::TIMESTAMP_ARRAY,
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTimeUtc => Type::TIMESTAMP_ARRAY,
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTimeLocal => Type::TIMESTAMP_ARRAY,
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTimeWithTimeZone => Type::TIMESTAMPTZ_ARRAY,
        #[cfg(feature = "with-time")]
        ArrayType::TimeDate => Type::DATE_ARRAY,
        #[cfg(feature = "with-time")]
        ArrayType::TimeTime => Type::TIME_ARRAY,
        #[cfg(feature = "with-time")]
        ArrayType::TimeDateTime => Type::TIMESTAMP_ARRAY,
        #[cfg(feature = "with-time")]
        ArrayType::TimeDateTimeWithTimeZone => Type::TIMESTAMPTZ_ARRAY,
        #[cfg(feature = "with-uuid")]
        ArrayType::Uuid => Type::UUID_ARRAY,
        #[cfg(feature = "with-rust_decimal")]
        ArrayType::Decimal => Type::NUMERIC_ARRAY,
        #[cfg(feature = "with-bigdecimal")]
        ArrayType::BigDecimal => Type::NUMERIC_ARRAY,
        #[cfg(feature = "with-ipnetwork")]
        ArrayType::IpNetwork => Type::INET_ARRAY,
        #[cfg(feature = "with-mac_address")]
        ArrayType::MacAddress => Type::MACADDR_ARRAY,
    }
}
