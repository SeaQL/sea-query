#[allow(deprecated)]
use crate::primitive_value::{PrimitiveValue, Values};
use bytes::BytesMut;
use postgres_types::{to_sql_checked, IsNull, ToSql, Type};
use std::error::Error;

pub trait PostgresDriver<'a> {
    fn as_params(&'a self) -> Vec<&'a (dyn ToSql + Sync)>;
}

#[allow(deprecated)]
impl ToSql for PrimitiveValue {
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
            PrimitiveValue::Bool(v) => to_sql!(v, bool),
            PrimitiveValue::TinyInt(v) => to_sql!(v, i8),
            PrimitiveValue::SmallInt(v) => to_sql!(v, i16),
            PrimitiveValue::Int(v) => to_sql!(v, i32),
            PrimitiveValue::BigInt(v) => to_sql!(v, i64),
            PrimitiveValue::TinyUnsigned(v) => to_sql!(v, u32),
            PrimitiveValue::SmallUnsigned(v) => to_sql!(v, u32),
            PrimitiveValue::Unsigned(v) => to_sql!(v, u32),
            PrimitiveValue::BigUnsigned(v) => to_sql!(v, i64),
            PrimitiveValue::Float(v) => to_sql!(v, f32),
            PrimitiveValue::Double(v) => to_sql!(v, f64),
            PrimitiveValue::String(v) => box_to_sql!(v, String),
            PrimitiveValue::Bytes(v) => box_to_sql!(v, Vec<u8>),
            #[cfg(feature = "postgres-json")]
            PrimitiveValue::Json(v) => box_to_sql!(v, serde_json::Value),
            #[cfg(feature = "postgres-chrono")]
            PrimitiveValue::Date(v) => box_to_sql!(v, chrono::NaiveDate),
            #[cfg(feature = "postgres-chrono")]
            PrimitiveValue::Time(v) => box_to_sql!(v, chrono::NaiveTime),
            #[cfg(feature = "postgres-chrono")]
            PrimitiveValue::DateTime(v) => box_to_sql!(v, chrono::NaiveDateTime),
            #[cfg(feature = "postgres-chrono")]
            PrimitiveValue::DateTimeWithTimeZone(v) => {
                box_to_sql!(v, chrono::DateTime<chrono::FixedOffset>)
            }
            #[cfg(feature = "postgres-rust_decimal")]
            PrimitiveValue::Decimal(v) => box_to_sql!(v, rust_decimal::Decimal),
            #[cfg(feature = "postgres-bigdecimal")]
            PrimitiveValue::BigDecimal(_) => unimplemented!("Not supported"),
            #[cfg(feature = "postgres-uuid")]
            PrimitiveValue::Uuid(v) => box_to_sql!(v, uuid::Uuid),
        }
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }

    to_sql_checked!();
}

#[allow(deprecated)]
impl From<Vec<PrimitiveValue>> for Values {
    fn from(v: Vec<PrimitiveValue>) -> Values {
        Values(v)
    }
}

#[allow(deprecated)]
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
