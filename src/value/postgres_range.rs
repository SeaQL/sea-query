use super::*;
use crate::IntoIden;

impl From<RangeType> for Value {
    fn from(x: RangeType) -> Value {
        Value::Range(Some(x.into()))
    }
}

impl Nullable for RangeType {
    fn null() -> Value {
        Value::Range(None)
    }
}

impl ValueType for RangeType {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Range(Some(x)) => Ok(*x),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(RangeType).to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::Range
    }

    fn column_type() -> ColumnType {
        ColumnType::Custom("RANGE".into_iden())
    }
}
