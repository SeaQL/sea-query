#[macro_export]
macro_rules! bind_params_sqlx_mysql {
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
                Value::TinyUnsigned(v) => query.bind(v),
                Value::SmallUnsigned(v) => query.bind(v),
                Value::Unsigned(v) => query.bind(v),
                Value::BigUnsigned(v) => query.bind(v),
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
macro_rules! sea_query_driver_mysql {
    () => {
        mod sea_query_driver_mysql {
            use sqlx::{mysql::MySqlArguments, query::Query, query::QueryAs, MySql};
            use $crate::{Value, Values};

            type SqlxQuery<'a> = Query<'a, MySql, MySqlArguments>;
            type SqlxQueryAs<'a, T> = QueryAs<'a, MySql, T, MySqlArguments>;

            pub fn bind_query<'a>(query: SqlxQuery<'a>, params: &'a Values) -> SqlxQuery<'a> {
                $crate::bind_params_sqlx_mysql!(query, params.0)
            }

            pub fn bind_query_as<'a, T>(
                query: SqlxQueryAs<'a, T>,
                params: &'a Values,
            ) -> SqlxQueryAs<'a, T> {
                $crate::bind_params_sqlx_mysql!(query, params.0)
            }
        }
    };
}
