#[macro_export]
macro_rules! bind_params {
    ( $query:expr, $params:expr ) => {
        {
            let mut query = $query;
            for value in $params.iter() {
                query = match value {
                    Value::Null => query.bind(None::<i32>),
                    Value::Bytes(v) => query.bind(std::str::from_utf8(v).unwrap()),
                    Value::Int(v) => query.bind(v),
                    Value::UInt(v) => query.bind(format!("{}", v)),
                    Value::Float(v) => query.bind(v),
                    Value::Double(v) => query.bind(v),
                    Value::Date(year, month, day, hour, minutes, seconds, _micro_seconds) => 
                        query.bind(format!(
                            "{:04}{:02}{:02} {:02}{:02}{:02}",
                            year, month, day, hour, minutes, seconds
                        )),
                    Value::Time(negative, days, hours, minutes, seconds, _micro_seconds) => 
                        query.bind(format!(
                            "{}{:02}{:02} {:02}{:02}.{:03}",
                            if *negative { "-" } else { "" }, days, hours, minutes, seconds, _micro_seconds / 1000
                        )),
                };
            }
            query
        }
    };
}