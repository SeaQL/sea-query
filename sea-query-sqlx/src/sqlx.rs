use crate::SqlxValues;
use sea_query::{QueryBuilder, query::*};

pub trait SqlxBinder {
    fn build_sqlx<T>(&self, query_builder: T) -> (String, SqlxValues)
    where
        T: QueryBuilder;
}

macro_rules! impl_sqlx_binder {
    ($l:ident) => {
        impl SqlxBinder for $l {
            fn build_sqlx<T>(&self, query_builder: T) -> (String, SqlxValues)
            where
                T: QueryBuilder,
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

/// Fallible counterpart of [`sqlx::IntoArguments`]: returns [`sqlx::Error::Encode`] for a value
/// that cannot be bound (e.g. a `u64` above `i64::MAX` on Postgres or SQLite, or a value
/// sqlx fails to encode) where `into_arguments` panics.
pub trait TryIntoArguments<DB: ::sqlx::Database> {
    fn try_into_arguments(self) -> Result<DB::Arguments, ::sqlx::Error>;
}
