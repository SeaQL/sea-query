#[macro_export]
macro_rules! bind_params_sqlx_sqlite {
    ( $query:expr, $params:expr ) => {{
        let mut query = $query;
        for value in $params.iter() {
            macro_rules! bind {
                ( $v: expr, $ty: ty ) => {
                    match $v {
                        Some(v) => query.bind((v as $ty)),
                        None => query.bind(None::<$ty>),
                    }
                };
            }
            macro_rules! bind_box {
                ( $v: expr, $ty: ty ) => {
                    match $v {
                        Some(v) => query.bind(*v),
                        None => query.bind(None::<$ty>),
                    }
                };
            }
            let primitive_value = value.primitive_value();
            query = match primitive_value {
                PrimitiveValue::Bool(v) => bind!(v, bool),
                PrimitiveValue::TinyInt(v) => bind!(v, i8),
                PrimitiveValue::SmallInt(v) => bind!(v, i16),
                PrimitiveValue::Int(v) => bind!(v, i32),
                PrimitiveValue::BigInt(v) => bind!(v, i64),
                PrimitiveValue::TinyUnsigned(v) => bind!(v, u8),
                PrimitiveValue::SmallUnsigned(v) => bind!(v, u16),
                PrimitiveValue::Unsigned(v) => bind!(v, u32),
                PrimitiveValue::BigUnsigned(v) => bind!(v, u64),
                PrimitiveValue::Float(v) => bind!(v, f32),
                PrimitiveValue::Double(v) => bind!(v, f64),
                PrimitiveValue::String(v) => bind_box!(v, String),
                PrimitiveValue::Bytes(v) => bind_box!(v, Vec<u8>),
                #[cfg(feature = "with-json")]
                PrimitiveValue::Json(v) => bind_box!(v, serde_json::Value),
                #[cfg(feature = "with-chrono")]
                PrimitiveValue::Date(v) => bind_box!(v, chrono::NaiveDate),
                #[cfg(feature = "with-chrono")]
                PrimitiveValue::Time(v) => bind_box!(v, chrono::NaiveTime),
                #[cfg(feature = "with-chrono")]
                PrimitiveValue::DateTime(v) => bind_box!(v, chrono::NaiveDateTime),
                #[cfg(feature = "with-rust_decimal")]
                PrimitiveValue::Decimal(v) => bind_box!(v, rust_decimal::Decimal),
                #[cfg(feature = "with-bigdecimal")]
                PrimitiveValue::BigDecimal(v) => bind_box!(v, bigdecimal::BigDecimal),
                #[cfg(feature = "with-uuid")]
                PrimitiveValue::Uuid(v) => bind_box!(v, uuid::Uuid),
                _ => unimplemented!(),
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
            use $crate::{PrimitiveValue, Value};

            type SqlxQuery<'a> = sqlx::query::Query<'a, Sqlite, SqliteArguments<'a>>;
            type SqlxQueryAs<'a, T> = sqlx::query::QueryAs<'a, Sqlite, T, SqliteArguments<'a>>;

            pub fn bind_query<'a>(query: SqlxQuery<'a>, params: &'a [Value]) -> SqlxQuery<'a> {
                $crate::bind_params_sqlx_sqlite!(query, params)
            }

            pub fn bind_query_as<'a, T>(
                query: SqlxQueryAs<'a, T>,
                params: &'a [Value],
            ) -> SqlxQueryAs<'a, T> {
                $crate::bind_params_sqlx_sqlite!(query, params)
            }
        }
    };
}
