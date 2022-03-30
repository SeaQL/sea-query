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
                    macro_rules! opt_string_to_sql {
                        ( $v: expr ) => {
                            match $v {
                                Some(v) => Ok(ToSqlOutput::from(v)),
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
                        #[cfg(feature = "with-json")]
                        Value::Json(v) => ty_to_sql!(v),
                        #[cfg(feature = "with-chrono")]
                        Value::ChronoDate(v) => ty_to_sql!(v),
                        #[cfg(feature = "with-chrono")]
                        Value::ChronoTime(v) => ty_to_sql!(v),
                        #[cfg(feature = "with-chrono")]
                        Value::ChronoDateTime(v) => ty_to_sql!(v),
                        #[cfg(feature = "with-chrono")]
                        Value::ChronoDateTimeUtc(v) => ty_to_sql!(v),
                        #[cfg(feature = "with-chrono")]
                        Value::ChronoDateTimeLocal(v) => ty_to_sql!(v),
                        #[cfg(feature = "with-chrono")]
                        Value::ChronoDateTimeWithTimeZone(v) => ty_to_sql!(v),
                        #[cfg(feature = "with-time")]
                        Value::TimeDate(v) => opt_string_to_sql!(v),
                        #[cfg(feature = "with-time")]
                        Value::TimeTime(v) => opt_string_to_sql!(v),
                        #[cfg(feature = "with-time")]
                        Value::TimeDateTime(v) => opt_string_to_sql!(v),
                        #[cfg(feature = "with-time")]
                        Value::TimeDateTimeWithTimeZone(v) => opt_string_to_sql!(v),
                        #[cfg(feature = "with-uuid")]
                        Value::Uuid(v) => ty_to_sql!(v),
                        _ => unimplemented!(),
                    }
                }
            }
        }
    };
}
