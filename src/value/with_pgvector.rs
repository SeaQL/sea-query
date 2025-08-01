use super::*;

impl From<pgvector::Vector> for Value {
    fn from(x: pgvector::Vector) -> Value {
        Value::vector(x)
    }
}

impl Nullable for pgvector::Vector {
    fn null() -> Value {
        Value::vector(None)
    }
}

impl ValueType for pgvector::Vector {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Vector(Some(x)) => Ok(x),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(Vector).to_owned()
    }

    fn array_type() -> ArrayType {
        unimplemented!("Vector does not have array type")
    }

    fn column_type() -> ColumnType {
        ColumnType::Vector(None)
    }
}
