use crate::SqlxValues;
use sea_query::{QueryBuilder, query::*};

pub trait SqlxBinder {
    fn build_sqlx<T>(&self, query_builder: T) -> (String, SqlxValues)
    where
        T: QueryBuilder + ?Sized;
}

macro_rules! impl_sqlx_binder {
    ($l:ident) => {
        impl SqlxBinder for $l {
            fn build_sqlx<T>(&self, query_builder: T) -> (String, SqlxValues)
            where
                T: QueryBuilder + ?Sized,
            {
                let (query, values) = self.build(query_builder);
                (query, SqlxValues(values))
            }
        }
    };
}

impl_sqlx_binder!(SelectStatement);
impl_sqlx_binder!(UpdateStatement);
impl_sqlx_binder!(InsertStatement);
impl_sqlx_binder!(DeleteStatement);
impl_sqlx_binder!(WithQuery);
