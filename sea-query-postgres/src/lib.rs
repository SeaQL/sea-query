use std::error::Error;

use bytes::BytesMut;
use postgres_types::{to_sql_checked, IsNull, ToSql, Type};

use sea_query::BinOper::Is;
use sea_query::{query::*, QueryBuilder, Value, ValueType};

#[derive(Clone, Debug)]
pub struct PostgresValue(pub Value);
#[derive(Clone, Debug)]
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
        if self.0.is_null() {
            return Ok(IsNull::Yes);
        }

        match &self.0.ty() {
            ValueType::Bool => {
                let v = self.0.value::<bool>().unwrap();
                (*v).to_sql(ty, out)
            }
            ValueType::TinyInt => {
                let v = self.0.value::<i8>().unwrap();
                (*v).to_sql(ty, out)
            }
            ValueType::SmallInt => {
                let v = self.0.value::<i16>().unwrap();
                (*v).to_sql(ty, out)
            }
            ValueType::Int => {
                let v = self.0.value::<i32>().unwrap();
                (*v).to_sql(ty, out)
            }
            ValueType::BigInt => {
                let v = self.0.value::<i64>().unwrap();
                (*v).to_sql(ty, out)
            }
            ValueType::TinyUnsigned => {
                let v = self.0.value::<u8>().unwrap();
                (*v as i16).to_sql(ty, out)
            }
            ValueType::SmallUnsigned => {
                let v = self.0.value::<u16>().unwrap();
                (*v as i32).to_sql(ty, out)
            }
            ValueType::Unsigned => {
                let v = self.0.value::<u32>().unwrap();
                (*v as i64).to_sql(ty, out)
            }
            ValueType::BigUnsigned => {
                let v = self.0.value::<u64>().unwrap();
                (*v as i64).to_sql(ty, out)
            }
            ValueType::Float => {
                let v = self.0.value::<f32>().unwrap();
                (*v).to_sql(ty, out)
            }
            ValueType::Double => {
                let v = self.0.value::<f64>().unwrap();
                (*v).to_sql(ty, out)
            }
            ValueType::Char => {
                let v = self.0.value::<f64>().unwrap();
                v.to_string().to_sql(ty, out)
            }
            _ => panic!(),
        }
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }

    to_sql_checked!();
}
