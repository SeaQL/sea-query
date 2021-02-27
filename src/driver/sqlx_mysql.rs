#[macro_export]
macro_rules! bind_params {
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
                    Value::TinyUnsigned(v) => query.bind(v),
                    Value::SmallUnsigned(v) => query.bind(v),
                    Value::Unsigned(v) => query.bind(v),
                    Value::BigUnsigned(v) => query.bind(v),
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