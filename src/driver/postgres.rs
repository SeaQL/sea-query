use std::error::Error;
use bytes::BytesMut;
use postgres_types::{IsNull, ToSql, Type, to_sql_checked};
use crate::{Value, Values};

pub trait PostgresDriver<'a> {
    fn as_params(&'a self) -> Vec<&'a (dyn ToSql + Sync)>;
}

impl ToSql for Value {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match self {
            Value::Null => None::<bool>.to_sql(ty, out),
            Value::Bool(v) => v.to_sql(ty, out),
            Value::TinyInt(v) => v.to_sql(ty, out),
            Value::SmallInt(v) => v.to_sql(ty, out),
            Value::Int(v) => v.to_sql(ty, out),
            Value::BigInt(v) => v.to_sql(ty, out),
            Value::TinyUnsigned(v) => (*v as u32).to_sql(ty, out),
            Value::SmallUnsigned(v) => (*v as u32).to_sql(ty, out),
            Value::Unsigned(v) => v.to_sql(ty, out),
            Value::BigUnsigned(v) => (*v as i64).to_sql(ty, out),
            Value::Float(v) => v.to_sql(ty, out),
            Value::Double(v) => v.to_sql(ty, out),
            Value::String(v) => v.as_str().to_sql(ty, out),
            Value::Bytes(v) => v.as_ref().to_sql(ty, out),
            #[cfg(feature="postgres-json")]
            Value::Json(v) => v.as_ref().to_sql(ty, out),
            #[cfg(feature="postgres-chrono")]
            Value::DateTime(v) => v.as_ref().to_sql(ty, out),
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
        self.0.iter().map(|x| {
            let y: &(dyn ToSql + Sync) = x;
            y
        }).collect()
    }
}