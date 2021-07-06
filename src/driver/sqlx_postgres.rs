#[macro_export]
macro_rules! bind_params_sqlx_postgres {
    ( $query:expr, $params:expr ) => {{
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
                _ => {
                    if value.is_json() {
                        query.bind(value.as_ref_json())
                    } else if value.is_date_time() {
                        query.bind(value.as_ref_date_time())
                    } else if value.is_decimal() {
                        query.bind(value.as_ref_decimal())
                    } else if value.is_uuid() {
                        query.bind(value.as_ref_uuid())
                    } else {
                        unimplemented!();
                    }
                }
            };
        }
        query
    }};
}

#[macro_export]
macro_rules! sea_query_driver_postgres {
    () => {
        mod sea_query_driver_postgres {
            use sqlx::{postgres::PgArguments, query::Query, query::QueryAs, Postgres};
            use $crate::{Value, Values};

            type SqlxQuery<'a> = sqlx::query::Query<'a, Postgres, PgArguments>;
            type SqlxQueryAs<'a, T> = sqlx::query::QueryAs<'a, Postgres, T, PgArguments>;

            pub fn bind_query<'a>(query: SqlxQuery<'a>, params: &'a Values) -> SqlxQuery<'a> {
                $crate::bind_params_sqlx_postgres!(query, params.0)
            }

            pub fn bind_query_as<'a, T>(
                query: SqlxQueryAs<'a, T>,
                params: &'a Values,
            ) -> SqlxQueryAs<'a, T> {
                $crate::bind_params_sqlx_postgres!(query, params.0)
            }
        }
    };
}
