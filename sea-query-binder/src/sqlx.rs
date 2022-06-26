use crate::SqlxValues;
use sea_query::{query::*, QueryBuilder};

pub trait SqlxBinder {
    fn build_sqlx<T: QueryBuilder>(&self, query_builder: T) -> (String, SqlxValues);
    fn build_any_sqlx(&self, query_builder: &dyn QueryBuilder) -> (String, SqlxValues);
}

macro_rules! impl_sqlx_binder {
    ($l:ident) => {
        impl SqlxBinder for $l {
            fn build_sqlx<T: QueryBuilder>(&self, query_builder: T) -> (String, SqlxValues) {
                let (query, values) = self.build(query_builder);
                (query, SqlxValues(values))
            }
            fn build_any_sqlx(&self, query_builder: &dyn QueryBuilder) -> (String, SqlxValues) {
                let (query, values) = self.build_any(query_builder);
                (query, SqlxValues(values))
            }
        }
    };
}

pub fn query_builder_from_kind(kind: sqlx::any::AnyKind) -> Box<dyn sea_query::QueryBuilder> {
    match kind {
        #[cfg(feature = "sqlx-postgres")]
        sqlx::any::AnyKind::Postgres => Box::new(sea_query::PostgresQueryBuilder),
        #[cfg(feature = "sqlx-mysql")]
        sqlx::any::AnyKind::MySql => Box::new(sea_query::MysqlQueryBuilder),
        #[cfg(feature = "sqlx-sqlite")]
        sqlx::any::AnyKind::Sqlite => Box::new(sea_query::SqliteQueryBuilder),
    }
}

pub fn schema_builder_from_kind(kind: sqlx::any::AnyKind) -> Box<dyn sea_query::SchemaBuilder> {
    match kind {
        #[cfg(feature = "sqlx-postgres")]
        sqlx::any::AnyKind::Postgres => Box::new(sea_query::PostgresQueryBuilder),
        #[cfg(feature = "sqlx-mysql")]
        sqlx::any::AnyKind::MySql => Box::new(sea_query::MysqlQueryBuilder),
        #[cfg(feature = "sqlx-sqlite")]
        sqlx::any::AnyKind::Sqlite => Box::new(sea_query::SqliteQueryBuilder),
    }
}

impl_sqlx_binder!(SelectStatement);
impl_sqlx_binder!(UpdateStatement);
impl_sqlx_binder!(InsertStatement);
impl_sqlx_binder!(DeleteStatement);
