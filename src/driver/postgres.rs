use crate::{Value, Values};
use bytes::BytesMut;
use postgres_types::{to_sql_checked, IsNull, ToSql, Type};
use std::error::Error;

pub trait PostgresDriver<'a> {
    fn as_params(&'a self) -> Vec<&'a (dyn ToSql + Sync)>;
}

impl ToSql for Value {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        macro_rules! to_sql {
            ( $v: expr, $ty: ty ) => {
                match $v {
                    Some(v) => (*v as $ty).to_sql(ty, out),
                    None => None::<$ty>.to_sql(ty, out),
                }
            };
        }
        macro_rules! box_to_sql {
            ( $v: expr, $ty: ty ) => {
                match $v {
                    Some(v) => v.as_ref().to_sql(ty, out),
                    None => None::<$ty>.to_sql(ty, out),
                }
            };
        }
        match self {
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
            Value::String(v) => box_to_sql!(v, String),
            Value::Bytes(v) => box_to_sql!(v, Vec<u8>),
            #[cfg(feature = "postgres-json")]
            Value::Json(v) => box_to_sql!(v, serde_json::Value),
            #[cfg(feature = "postgres-chrono")]
            Value::ChronoDate(v) => box_to_sql!(v, chrono::NaiveDate),
            #[cfg(feature = "postgres-chrono")]
            Value::ChronoTime(v) => box_to_sql!(v, chrono::NaiveTime),
            #[cfg(feature = "postgres-chrono")]
            Value::ChronoDateTime(v) => box_to_sql!(v, chrono::NaiveDateTime),
            #[cfg(feature = "postgres-chrono")]
            Value::ChronoDateTimeUtc(v) => box_to_sql!(v, chrono::DateTime<chrono::Utc>),
            #[cfg(feature = "postgres-chrono")]
            Value::ChronoDateTimeLocal(v) => box_to_sql!(v, chrono::DateTime<chrono::Local>),
            #[cfg(feature = "postgres-chrono")]
            Value::ChronoDateTimeWithTimeZone(v) => {
                box_to_sql!(v, chrono::DateTime<chrono::FixedOffset>)
            }
            #[cfg(feature = "postgres-time")]
            Value::TimeDate(v) => box_to_sql!(v, time::Date),
            #[cfg(feature = "postgres-time")]
            Value::TimeTime(v) => box_to_sql!(v, time::Time),
            #[cfg(feature = "postgres-time")]
            Value::TimeDateTime(v) => box_to_sql!(v, time::PrimitiveDateTime),
            #[cfg(feature = "postgres-time")]
            Value::TimeDateTimeWithTimeZone(v) => box_to_sql!(v, time::OffsetDateTime),
            #[cfg(feature = "postgres-rust_decimal")]
            Value::Decimal(v) => box_to_sql!(v, rust_decimal::Decimal),
            #[cfg(feature = "postgres-bigdecimal")]
            Value::BigDecimal(_) => unimplemented!("Not supported"),
            #[cfg(feature = "postgres-uuid")]
            Value::Uuid(v) => box_to_sql!(v, uuid::Uuid),
            #[cfg(feature = "postgres-array")]
            Value::Array(v) => box_to_sql!(v, Vec<Value>),
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        }
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }

    to_sql_checked!();
}

impl From<Vec<Value>> for Values {
    fn from(v: Vec<Value>) -> Values {
        Values(v)
    }
}

impl<'a> PostgresDriver<'a> for Values {
    fn as_params(&'a self) -> Vec<&'a (dyn ToSql + Sync)> {
        self.0
            .iter()
            .map(|x| {
                let y: &(dyn ToSql + Sync) = x;
                y
            })
            .collect()
    }
}
