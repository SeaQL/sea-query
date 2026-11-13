#![forbid(unsafe_code)]

use std::error::Error;

use bytes::BytesMut;
use postgres_types::{IsNull, ToSql, Type, to_sql_checked};

use sea_query::{ArrayType, OptionEnum, QueryBuilder, Value, query::*};

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
            Value::Enum(v) => match v {
                OptionEnum::Some(v) => Some(v.value.as_ref()).to_sql(ty, out),
                OptionEnum::None(_) => Option::<&str>::None.to_sql(ty, out),
            },
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
            #[cfg(feature = "with-jiff")]
            Value::JiffDate(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-jiff")]
            Value::JiffTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-jiff")]
            Value::JiffDateTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-jiff")]
            Value::JiffTimestamp(v) => v.to_sql(ty, out),
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
        Value::Enum(_) => Type::TEXT,
        #[cfg(feature = "postgres-range")]
        Value::Range(v) => match v.as_deref() {
            Some(sea_query::prelude::RangeType::Int4Range(..)) => Type::INT4_RANGE,
            Some(sea_query::prelude::RangeType::Int8Range(..)) => Type::INT8_RANGE,
            Some(sea_query::prelude::RangeType::NumRange(..)) => Type::NUM_RANGE,
            // a NULL range has no range type, let PostgreSQL infer it (as SQLx does)
            None => Type::UNKNOWN,
        },
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
        #[cfg(feature = "with-jiff")]
        Value::JiffDate(_) => Type::DATE,
        #[cfg(feature = "with-jiff")]
        Value::JiffTime(_) => Type::TIME,
        #[cfg(feature = "with-jiff")]
        Value::JiffDateTime(_) => Type::TIMESTAMP,
        #[cfg(feature = "with-jiff")]
        Value::JiffTimestamp(_) => Type::TIMESTAMPTZ,
        #[cfg(feature = "with-uuid")]
        Value::Uuid(_) => Type::UUID,
        #[cfg(feature = "with-rust_decimal")]
        Value::Decimal(_) => Type::NUMERIC,
        #[cfg(feature = "with-bigdecimal")]
        Value::BigDecimal(_) => Type::NUMERIC,
        #[cfg(all(feature = "postgres-array", feature = "postgres-range"))]
        Value::Array(ArrayType::Range, v) => range_array_type(v.as_deref()),
        #[cfg(feature = "postgres-array")]
        Value::Array(ty, _) => array_type_to_pg_type(ty),
        #[cfg(feature = "postgres-vector")]
        Value::Vector(_) => Type::FLOAT4_ARRAY,
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
        #[cfg(feature = "with-jiff")]
        ArrayType::JiffDate => Type::DATE_ARRAY,
        #[cfg(feature = "with-jiff")]
        ArrayType::JiffTime => Type::TIME_ARRAY,
        #[cfg(feature = "with-jiff")]
        ArrayType::JiffDateTime => Type::TIMESTAMP_ARRAY,
        #[cfg(feature = "with-jiff")]
        ArrayType::JiffTimestamp => Type::TIMESTAMPTZ_ARRAY,
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
        ArrayType::Enum(_) => Type::TEXT_ARRAY,
        // handled by `range_array_type`
        #[cfg(feature = "postgres-range")]
        ArrayType::Range => Type::UNKNOWN,
    }
}

/// `ArrayType::Range` doesn't say which range type the array holds, so it is taken from the
/// first non-NULL element, the same way `sea-query-sqlx` does. Mixing range types in one array
/// is an encoding error. A NULL, empty or all-NULL array has no range type, so it is left for
/// PostgreSQL to infer from the query.
#[cfg(all(feature = "postgres-array", feature = "postgres-range"))]
fn range_array_type(values: Option<&Vec<Value>>) -> Type {
    use sea_query::prelude::RangeType;

    let first = values.into_iter().flatten().find_map(|v| match v {
        Value::Range(Some(range)) => Some(range.as_ref()),
        _ => None,
    });

    match first {
        Some(RangeType::Int4Range(..)) => Type::INT4_RANGE_ARRAY,
        Some(RangeType::Int8Range(..)) => Type::INT8_RANGE_ARRAY,
        Some(RangeType::NumRange(..)) => Type::NUM_RANGE_ARRAY,
        None => Type::UNKNOWN,
    }
}

#[cfg(all(test, feature = "postgres-range"))]
mod tests {
    use super::*;
    use sea_query::prelude::{RangeBound, RangeType};

    #[test]
    fn range_types_match_range_variant() {
        let values = PostgresValues(vec![
            PostgresValue(
                RangeType::Int4Range(RangeBound::Inclusive(1), RangeBound::Exclusive(2)).into(),
            ),
            PostgresValue(
                RangeType::Int8Range(RangeBound::Inclusive(1), RangeBound::Exclusive(2)).into(),
            ),
            PostgresValue(
                RangeType::NumRange(RangeBound::Inclusive(1.0), RangeBound::Unbounded).into(),
            ),
        ]);

        assert_eq!(
            values.as_types(),
            vec![Type::INT4_RANGE, Type::INT8_RANGE, Type::NUM_RANGE]
        );

        assert_eq!(
            PostgresValues(vec![PostgresValue(Value::Range(None))]).as_types(),
            vec![Type::UNKNOWN]
        );
    }

    #[cfg(feature = "postgres-array")]
    #[test]
    fn range_array_types_follow_first_non_null_element() {
        let int8: Value = vec![RangeType::Int8Range(
            RangeBound::Inclusive(1),
            RangeBound::Exclusive(2),
        )]
        .into();
        let num: Value = vec![RangeType::NumRange(
            RangeBound::Inclusive(0.5),
            RangeBound::Unbounded,
        )]
        .into();
        let int4_after_null = Value::Array(
            ArrayType::Range,
            Some(Box::new(vec![
                Value::Range(None),
                RangeType::Int4Range(RangeBound::Inclusive(1), RangeBound::Exclusive(2)).into(),
            ])),
        );
        let empty: Value = Vec::<RangeType>::new().into();
        let all_null = Value::Array(ArrayType::Range, Some(Box::new(vec![Value::Range(None)])));
        let null = Value::Array(ArrayType::Range, None);

        let values = PostgresValues(
            [int8, num, int4_after_null, empty, all_null, null]
                .into_iter()
                .map(PostgresValue)
                .collect(),
        );

        assert_eq!(
            values.as_types(),
            vec![
                Type::INT8_RANGE_ARRAY,
                Type::NUM_RANGE_ARRAY,
                Type::INT4_RANGE_ARRAY,
                Type::UNKNOWN,
                Type::UNKNOWN,
                Type::UNKNOWN,
            ]
        );
    }
}
