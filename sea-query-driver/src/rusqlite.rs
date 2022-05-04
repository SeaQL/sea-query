use crate::utils::RusqliteDriverArgs;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub fn sea_query_driver_rusqlite_impl(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as RusqliteDriverArgs);
    let rusqlite_path = args.driver;
    let sea_query_path = args.sea_query;

    let with_json = if cfg!(feature = "with-json") {
        quote! { Value::Json(v) => box_to_sql!(v), }
    } else {
        quote! {}
    };

    let with_chrono = if cfg!(feature = "with-chrono") {
        quote! {
            Value::ChronoDate(v) => box_to_sql!(v),
            Value::ChronoTime(v) => box_to_sql!(v),
            Value::ChronoDateTime(v) => box_to_sql!(v),
            Value::ChronoDateTimeUtc(v) => box_to_sql!(v),
            Value::ChronoDateTimeLocal(v) => box_to_sql!(v),
            Value::ChronoDateTimeWithTimeZone(v) => box_to_sql!(v),
        }
    } else {
        quote! {}
    };

    let with_time = if cfg!(feature = "with-time") {
        quote! {
            v @ Value::TimeDate(_) => opt_string_to_sql!(v.time_as_naive_utc_in_string()),
            v @ Value::TimeTime(_) => opt_string_to_sql!(v.time_as_naive_utc_in_string()),
            v @ Value::TimeDateTime(_) => opt_string_to_sql!(v.time_as_naive_utc_in_string()),
            v @ Value::TimeDateTimeWithTimeZone(_) => opt_string_to_sql!(v.time_as_naive_utc_in_string()),
        }
    } else {
        quote! {}
    };

    let with_uuid = if cfg!(feature = "with-uuid") {
        quote! {
            Value::Uuid(v) => box_to_sql!(v),
        }
    } else {
        quote! {}
    };

    let output = quote! {
        mod sea_query_driver_rusqlite {
            use #rusqlite_path::rusqlite::{types::{Null, ToSqlOutput}, Result, ToSql};
            use #sea_query_path::sea_query::{Value, Values};

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
                    macro_rules! box_to_sql {
                        ( $v: expr ) => {
                            match $v {
                                Some(v) => v.as_ref().to_sql(),
                                None => Null.to_sql(),
                            }
                        };
                    }

                    macro_rules! opt_string_to_sql {
                        ( $v: expr ) => {
                            match $v {
                                Some(v) => Ok(ToSqlOutput::from(v)),
                                None => Null.to_sql(),
                            }
                        };
                    }

                    match &self.0 {
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
                        Value::String(v) => box_to_sql!(v),
                        Value::Bytes(v) => box_to_sql!(v),
                        #with_json
                        #with_chrono
                        #with_time
                        #with_uuid
                        _ => unimplemented!(),
                    }
                }
            }
        }

    };

    output.into()
}
