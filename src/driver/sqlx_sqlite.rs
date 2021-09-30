#[macro_export]
macro_rules! bind_params_sqlx_sqlite {
    ( $query:expr, $params:expr ) => {{
        let mut query = $query;
        for value in $params.iter() {
            macro_rules! bind {
                ( $v: expr, $ty: ty ) => {
                    match $v {
                        Some(v) => query.bind((*v as $ty)),
                        None => query.bind(None::<$ty>),
                    }
                };
            }
            macro_rules! bind_box {
                ( $v: expr, $ty: ty ) => {
                    match $v {
                        Some(v) => query.bind(v.as_ref()),
                        None => query.bind(None::<$ty>),
                    }
                };
            }
            query = match value {
                Value::Bool(v) => bind!(v, bool),
                Value::TinyInt(v) => bind!(v, i8),
                Value::SmallInt(v) => bind!(v, i16),
                Value::Int(v) => bind!(v, i32),
                Value::BigInt(v) => bind!(v, i64),
                Value::TinyUnsigned(v) => bind!(v, u32),
                Value::SmallUnsigned(v) => bind!(v, u32),
                Value::Unsigned(v) => bind!(v, u32),
                Value::BigUnsigned(v) => bind!(v, i64),
                Value::Float(v) => bind!(v, f32),
                Value::Double(v) => bind!(v, f64),
                Value::String(v) => bind_box!(v, String),
                Value::Bytes(v) => bind_box!(v, Vec<u8>),
                _ => {
                    if value.is_json() {
                        query.bind(value.as_ref_json())
                    } else if value.is_date() {
                        query.bind(value.as_ref_date())
                    } else if value.is_time() {
                        query.bind(value.as_ref_time().format("%T.f").to_string())
                    } else if value.is_date_time() {
                        query.bind(value.as_ref_date_time())
                    } else if value.is_decimal() {
                        query.bind(value.decimal_to_f64())
                    } else if value.is_big_decimal() {
                        query.bind(value.big_decimal_to_f64())
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
macro_rules! sea_query_driver_sqlite {
    () => {
        mod sea_query_driver_sqlite {
            use sqlx::{query::Query, query::QueryAs, sqlite::SqliteArguments, Sqlite};
            use $crate::{Value, Values};

            type SqlxQuery<'a> = sqlx::query::Query<'a, Sqlite, SqliteArguments<'a>>;
            type SqlxQueryAs<'a, T> = sqlx::query::QueryAs<'a, Sqlite, T, SqliteArguments<'a>>;

            pub fn bind_query<'a>(query: SqlxQuery<'a>, params: &'a Values) -> SqlxQuery<'a> {
                $crate::bind_params_sqlx_sqlite!(query, params.0)
            }

            pub fn bind_query_as<'a, T>(
                query: SqlxQueryAs<'a, T>,
                params: &'a Values,
            ) -> SqlxQueryAs<'a, T> {
                $crate::bind_params_sqlx_sqlite!(query, params.0)
            }
        }
    };
}
