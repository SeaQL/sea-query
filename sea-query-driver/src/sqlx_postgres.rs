use crate::utils::{BindParamArgs, SqlxDriverArgs};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub fn bind_params_sqlx_postgres_impl(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as BindParamArgs);
    let query = args.query;
    let params = args.params;

    let with_json = if cfg!(feature = "with-json") {
        quote! { Value::Json(v) => bind_box!(v), }
    } else {
        quote! {}
    };

    let with_chrono = if cfg!(feature = "with-chrono") {
        quote! {
            Value::ChronoDate(v) => bind_box!(v),
            Value::ChronoTime(v) => bind_box!(v),
            Value::ChronoDateTime(v) => bind_box!(v),
            Value::ChronoDateTimeUtc(v) => bind_box!(v),
            Value::ChronoDateTimeLocal(v) => bind_box!(v),
            Value::ChronoDateTimeWithTimeZone(v) => bind_box!(v),
        }
    } else {
        quote! {}
    };

    let with_time = if cfg!(feature = "with-time") {
        quote! {
            Value::TimeDate(v) => bind_box!(v),
            Value::TimeTime(v) => bind_box!(v),
            Value::TimeDateTime(v) => bind_box!(v),
            Value::TimeDateTimeWithTimeZone(v) => bind_box!(v),
        }
    } else {
        quote! {}
    };

    let with_uuid = if cfg!(feature = "with-uuid") {
        quote! { Value::Uuid(v) => bind_box!(v), }
    } else {
        quote! {}
    };

    let with_rust_decimal = if cfg!(feature = "with-rust_decimal") {
        quote! { Value::Decimal(v) => bind_box!(v), }
    } else {
        quote! {}
    };

    let with_big_decimal = if cfg!(feature = "with-bigdecimal") {
        quote! { Value::BigDecimal(v) => bind_box!(v), }
    } else {
        quote! {}
    };

    let with_postgres_array = if cfg!(feature = "postgres-array") {
        quote! { Value::Array(_) => unimplemented!("SQLx array is not supported"), }
    } else {
        quote! {}
    };

    let with_ipnetwork = if cfg!(feature = "with-ipnetwork") {
        quote! {
            Value::Ipv4Network(v) => bind_box!(v),
            Value::Ipv6Network(v) => bind_box!(v),
        }
    } else {
        quote! {}
    };

    let with_mac_address = if cfg!(feature = "with-mac_address") {
        quote! {
            Value::MacAddress(v) => bind_box!(v),
        }
    } else {
        quote! {}
    };

    let output = quote! {
        {
            let mut query = #query;
            for value in #params.iter() {
                macro_rules! bind {
                    ( $v: expr, $ty: ty ) => {
                        match $v {
                            Some(v) => query.bind(*v as $ty),
                            None => query.bind(None::<$ty>),
                        }
                    };
                }
                macro_rules! bind_box {
                    ( $v: expr ) => {{
                        let v = match $v {
                            Some(v) => Some(v.as_ref()),
                            None => None,
                        };
                        query.bind(v)
                    }};
                }
                query = match value {
                    Value::Bool(v) => bind!(v, bool),
                    Value::TinyInt(v) => bind!(v, i8),
                    Value::SmallInt(v) => bind!(v, i16),
                    Value::Int(v) => bind!(v, i32),
                    Value::BigInt(v) => bind!(v, i64),
                    Value::TinyUnsigned(v) => bind!(v, u32),
                    Value::SmallUnsigned(v) => bind!(v, u32),
                    Value::Unsigned(v) => bind!(v, u32),
                    Value::BigUnsigned(v) => bind!(v, i64),
                    Value::Float(v) => bind!(v, f32),
                    Value::Double(v) => bind!(v, f64),
                    Value::String(v) => bind_box!(v),
                    Value::Bytes(v) => bind_box!(v),
                    #with_json
                    #with_chrono
                    #with_time
                    #with_uuid
                    #with_rust_decimal
                    #with_big_decimal
                    #with_postgres_array
                    #with_ipnetwork
                    #with_mac_address
                    #[allow(unreachable_patterns)]
                    _ => unimplemented!(),
                };
            };
            query
        }
    };
    output.into()
}

pub fn sea_query_driver_postgres_impl(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as SqlxDriverArgs);
    let sqlx_path = args.driver;
    let sea_query_path = args.sea_query;

    let output = quote! {
        mod sea_query_driver_postgres {
            use #sqlx_path::sqlx::{postgres::PgArguments, Postgres};
            use #sea_query_path::sea_query::{Value, Values};

            type SqlxQuery<'a> = #sqlx_path::sqlx::query::Query<'a, Postgres, PgArguments>;
            type SqlxQueryAs<'a, T> = #sqlx_path::sqlx::query::QueryAs<'a, Postgres, T, PgArguments>;

            pub fn bind_query<'a>(query: SqlxQuery<'a>, params: &'a Values) -> SqlxQuery<'a> {
                #sea_query_path::sea_query::bind_params_sqlx_postgres!(query, params.0)
            }

            pub fn bind_query_as<'a, T>(
                query: SqlxQueryAs<'a, T>,
                params: &'a Values,
            ) -> SqlxQueryAs<'a, T> {
                #sea_query_path::sea_query::bind_params_sqlx_postgres!(query, params.0)
            }
        }
    };
    output.into()
}
