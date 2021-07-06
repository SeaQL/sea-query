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
                    match &self.0 {
                        Value::Null => None::<bool>.to_sql(),
                        Value::Bool(v) => v.to_sql(),
                        Value::TinyInt(v) => v.to_sql(),
                        Value::SmallInt(v) => v.to_sql(),
                        Value::Int(v) => v.to_sql(),
                        Value::BigInt(v) => v.to_sql(),
                        Value::TinyUnsigned(v) => v.to_sql(),
                        Value::SmallUnsigned(v) => v.to_sql(),
                        Value::Unsigned(v) => v.to_sql(),
                        Value::BigUnsigned(v) => v.to_sql(),
                        Value::Float(v) => v.to_sql(),
                        Value::Double(v) => v.to_sql(),
                        Value::String(v) => v.as_str().to_sql(),
                        Value::Bytes(v) => v.as_ref().to_sql(),
                        _ => {
                            if self.0.is_json() {
                                (*self.0.as_ref_json()).to_sql()
                            } else if self.0.is_date_time() {
                                (*self.0.as_ref_date_time()).to_sql()
                            } else if self.0.is_decimal() {
                                unimplemented!("Not supported");
                            } else if self.0.is_uuid() {
                                (*self.0.as_ref_uuid()).to_sql()
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
