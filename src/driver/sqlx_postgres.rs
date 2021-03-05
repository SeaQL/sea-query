use sqlx::{query::Query, query::QueryAs, Postgres, postgres::PgArguments};
use crate::Value;

macro_rules! bind_params_sqlx_postgres {
    ( $query:expr, $params:expr ) => {
        {
            let mut query = $query;
            for value in $params.iter() {
                query = match value {
                    Value::Null => query.bind(None::<bool>),
                    Value::Bool(v) => query.bind(v),
                    Value::TinyInt(v) => query.bind(v),
                    Value::SmallInt(v) => query.bind(v),
                    Value::Int(v) => query.bind(v),
                    Value::BigInt(v) => query.bind(v),
                    Value::TinyUnsigned(v) => query.bind(*v as u32),
                    Value::SmallUnsigned(v) => query.bind(*v as u32),
                    Value::Unsigned(v) => query.bind(v),
                    Value::BigUnsigned(v) => query.bind(*v as i64),
                    Value::Float(v) => query.bind(v),
                    Value::Double(v) => query.bind(v),
                    Value::String(v) => query.bind(v.as_str()),
                    Value::Bytes(v) => query.bind(v.as_ref()),
                    Value::Json(v) => query.bind(v.as_ref()),
                    #[cfg(feature="sqlx-chrono")]
                    Value::DateTime(v) => query.bind(v.as_ref()),
                };
            }
            query
        }
    };
}

type SqlxQuery<'a> = sqlx::query::Query<'a, Postgres, PgArguments>;
type SqlxQueryAs<'a, T> = sqlx::query::QueryAs<'a, Postgres, T, PgArguments>;

pub fn bind_query<'a>(query: SqlxQuery<'a>, params: &'a [Value]) -> SqlxQuery<'a> {
    bind_params_sqlx_postgres!(query, params)
}

pub fn bind_query_as<'a, T>(query: SqlxQueryAs<'a, T>, params: &'a [Value]) -> SqlxQueryAs<'a, T> {
    bind_params_sqlx_postgres!(query, params)
}
