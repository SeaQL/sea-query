use super::*;

pub use geo_types::Point;

impl From<Point> for Value {
    fn from(x: Point) -> Value {
        Value::Point(Some(Box::new(x)))
    }
}

impl Nullable for Point {
    fn null() -> Value {
        Value::Point(None)
    }
}

impl ValueType for Point {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Point(Some(x)) => Ok(*x),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(Point).to_owned()
    }

    fn array_type() -> ArrayType {
        unimplemented!()
    }

    fn column_type() -> ColumnType {
        ColumnType::Point
    }
}
