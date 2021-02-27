#[macro_export]
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
                };
            }
            query
        }
    };
}