use crate::{IsNull, SeaRc, Value, ValueTrait, ValueType};

impl From<String> for Value {
    fn from(v: String) -> Value {
        let object = Box::new(v) as _;
        Value {
            ty: String::value_type(),
            object: SeaRc::new(object),
        }
    }
}

impl ValueTrait for String {
    fn to_sql_string(&self) -> String {
        todo!()
    }

    fn value_type() -> ValueType {
        ValueType::String
    }

    fn value_is_null(&self) -> IsNull {
        IsNull::No
    }
}

impl ValueTrait for &str {
    fn to_sql_string(&self) -> String {
        todo!()
    }

    fn value_type() -> ValueType {
        ValueType::String
    }

    fn value_is_null(&self) -> IsNull {
        IsNull::No
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        let object = Box::new(v.to_string()) as _;
        Value {
            ty: <&str>::value_type(),
            object: SeaRc::new(object),
        }
    }
}
