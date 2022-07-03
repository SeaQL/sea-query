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
                $v.map(|v| v as $ty).as_ref().to_sql(ty, out)
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
            #[cfg(feature = "postgres-rust_decimal")]
            Value::Decimal(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-uuid")]
            Value::Uuid(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "postgres-array")]
            Value::Array(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(v) => v.as_deref().map(|v| v.network()).to_sql(ty, out),
            #[cfg(feature = "postgres-cidr")]
            Value::IpInet(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "postgres-cidr")]
            Value::IpCidr(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "postgres-eui48")]
            Value::Eui48MacAddress(v) => v.as_deref().to_sql(ty, out),
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
