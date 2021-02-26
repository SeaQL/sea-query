#[macro_export]
macro_rules! bind_params {
    ( $query:expr, $params:expr ) => {
        {
            let mut query = $query;
            for value in $params.iter() {
                query = match value {
                    Value::Null => query.bind(None::<i32>),
                    Value::Bool(b) => query.bind(b),
                    Value::TinyInt(v) => query.bind(*v as i32),
                    Value::SmallInt(v) => query.bind(*v as i32),
                    Value::Int(v) => query.bind(v),
                    Value::BigInt(v) => query.bind(v),
                    Value::TinyUnsigned(v) => query.bind(*v as i64),
                    Value::SmallUnsigned(v) => query.bind(*v as i64),
                    Value::Unsigned(v) => query.bind(*v as i64),
                    Value::BigUnsigned(v) => query.bind(format!("{}", v)),
                    Value::Float(v) => query.bind(v),
                    Value::Double(v) => query.bind(v),
                    Value::String(v) => query.bind(v.as_str()),
                    Value::Bytes(v) => query.bind(std::str::from_utf8(v).unwrap()),
                };
            }
            query
        }
    };
}