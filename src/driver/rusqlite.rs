#[macro_export]
macro_rules! sea_query_driver_rusqlite {
    () => {
        mod sea_query_driver_rusqlite {
            use rusqlite::{types::ToSqlOutput, Result, ToSql};
            use sea_query::{PrimitiveValue, Value};

            pub struct RusqliteValue(pub Value);

            pub struct RusqliteValues(pub Vec<RusqliteValue>);

            impl From<Vec<Value>> for RusqliteValues {
                fn from(values: Vec<Value>) -> RusqliteValues {
                    RusqliteValues(values.0.into_iter().map(|v| RusqliteValue(v)).collect())
                }
            }

            impl<'a> RusqliteValues {
                pub fn as_params(&'a self) -> Vec<&'a dyn ToSql> {
                    self.0
                        .iter()
                        .map(|x| {
                            let y: &dyn ToSql = x;
                            y
                        })
                        .collect()
                }
            }

            impl ToSql for RusqliteValue {
                fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
                    macro_rules! to_sql {
                        ( $v: expr, $ty: ty ) => {
                            match $v {
                                Some(v) => v.to_sql(),
                                None => None::<$ty>.to_sql(),
                            }
                        };
                    }
                    macro_rules! box_to_sql {
                        ( $v: expr, $ty: ty ) => {
                            match $v {
                                Some(v) => v.as_ref().to_sql(),
                                None => None::<$ty>.to_sql(),
                            }
                        };
                    }
                    match &self.0.primitive_value() {
                        PrimitiveValue::Bool(v) => to_sql!(v, bool),
                        PrimitiveValue::TinyInt(v) => to_sql!(v, i8),
                        PrimitiveValue::SmallInt(v) => to_sql!(v, i16),
                        PrimitiveValue::Int(v) => to_sql!(v, i32),
                        PrimitiveValue::BigInt(v) => to_sql!(v, i64),
                        PrimitiveValue::TinyUnsigned(v) => to_sql!(v, u32),
                        PrimitiveValue::SmallUnsigned(v) => to_sql!(v, u32),
                        PrimitiveValue::Unsigned(v) => to_sql!(v, u32),
                        PrimitiveValue::BigUnsigned(v) => to_sql!(v, i64),
                        PrimitiveValue::Float(v) => to_sql!(v, f32),
                        PrimitiveValue::Double(v) => to_sql!(v, f64),
                        PrimitiveValue::String(v) => box_to_sql!(v, String),
                        PrimitiveValue::Bytes(v) => box_to_sql!(v, Vec<u8>),
                        value => {
                            if value.is_json() {
                                (*value.as_ref_json()).to_sql()
                            } else if value.is_date() {
                                (*value.as_ref_date()).to_sql()
                            } else if value.is_time() {
                                (*value.as_ref_time()).to_sql()
                            } else if value.is_date_time() {
                                (*value.as_ref_date_time()).to_sql()
                            } else if value.is_date_time_with_time_zone() {
                                (*value.as_ref_date_time_with_time_zone()).to_sql()
                            } else if value.is_uuid() {
                                (*value.as_ref_uuid()).to_sql()
                            } else {
                                unimplemented!();
                            }
                        }
                    }
                }
            }
        }
    };
}
