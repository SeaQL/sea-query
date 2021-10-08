#[macro_export]
macro_rules! sea_query_driver_rusqlite {
    () => {
        mod sea_query_driver_rusqlite {
            use rusqlite::{types::ToSqlOutput, Result, ToSql};
            use sea_query::{Value, Values};

            pub struct RusqliteValue(pub Value);

            pub struct RusqliteValues(pub Vec<RusqliteValue>);

            impl From<Values> for RusqliteValues {
                fn from(values: Values) -> RusqliteValues {
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
                    macro_rules! ty_to_sql {
                        ( $v: expr ) => {
                            match $v {
                                Some(v) => v.to_sql(),
                                None => None::<bool>.to_sql(),
                            }
                        };
                    }
                    match &self.0 {
                        Value::Bool(v) => to_sql!(v, bool),
                        Value::TinyInt(v) => to_sql!(v, i8),
                        Value::SmallInt(v) => to_sql!(v, i16),
                        Value::Int(v) => to_sql!(v, i32),
                        Value::BigInt(v) => to_sql!(v, i64),
                        Value::TinyUnsigned(v) => to_sql!(v, u32),
                        Value::SmallUnsigned(v) => to_sql!(v, u32),
                        Value::Unsigned(v) => to_sql!(v, u32),
                        Value::BigUnsigned(v) => to_sql!(v, i64),
                        Value::Float(v) => to_sql!(v, f32),
                        Value::Double(v) => to_sql!(v, f64),
                        Value::String(v) => box_to_sql!(v, String),
                        Value::Bytes(v) => box_to_sql!(v, Vec<u8>),
                        _ => {
                            if self.0.is_json() {
                                ty_to_sql!(self.0.as_ref_json())
                            } else if self.0.is_date() {
                                ty_to_sql!(self.0.as_ref_date())
                            } else if self.0.is_time() {
                                ty_to_sql!(self.0.as_ref_time())
                            } else if self.0.is_date_time() {
                                ty_to_sql!(self.0.as_ref_date_time())
                            } else if self.0.is_date_time_with_time_zone() {
                                ty_to_sql!(self.0.as_ref_date_time_with_time_zone())
                            } else if self.0.is_uuid() {
                                ty_to_sql!(self.0.as_ref_uuid())
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
