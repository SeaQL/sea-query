use std::error::Error;

use bytes::BytesMut;
use postgres_types::{to_sql_checked, IsNull, ToSql, Type};

use sea_query::{query::*, QueryBuilder, Value, ValueTrait};

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
        self.0.to_sql(ty, out)
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }

    to_sql_checked!();
}
