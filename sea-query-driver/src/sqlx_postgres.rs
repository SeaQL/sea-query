use crate::utils::{BindParamArgs, SqlxDriverArgs};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub fn bind_params_sqlx_postgres_impl(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as BindParamArgs);
    let query = args.query;
    let params = args.params;

    let with_json = if cfg!(feature = "with-json") {
        quote! { Value::Json(v) => query.bind(v.as_deref()), }
    } else {
        quote! {}
    };

    let with_chrono = if cfg!(feature = "with-chrono") {
        quote! {
            Value::ChronoDate(v) => query.bind(v.as_deref()),
            Value::ChronoTime(v) => query.bind(v.as_deref()),
            Value::ChronoDateTime(v) => query.bind(v.as_deref()),
            Value::ChronoDateTimeUtc(v) => query.bind(v.as_deref()),
            Value::ChronoDateTimeLocal(v) => query.bind(v.as_deref()),
            Value::ChronoDateTimeWithTimeZone(v) => query.bind(v.as_deref()),
        }
    } else {
        quote! {}
    };

    let with_time = if cfg!(feature = "with-time") {
        quote! {
            Value::TimeDate(v) => query.bind(v.as_deref()),
            Value::TimeTime(v) => query.bind(v.as_deref()),
            Value::TimeDateTime(v) => query.bind(v.as_deref()),
            Value::TimeDateTimeWithTimeZone(v) => query.bind(v.as_deref()),
        }
    } else {
        quote! {}
    };

    let with_uuid = if cfg!(feature = "with-uuid") {
        quote! { Value::Uuid(v) => query.bind(v.as_deref()), }
    } else {
        quote! {}
    };

    let with_rust_decimal = if cfg!(feature = "with-rust_decimal") {
        quote! { Value::Decimal(v) => query.bind(v.as_deref()), }
    } else {
        quote! {}
    };

    let with_big_decimal = if cfg!(feature = "with-bigdecimal") {
        quote! { Value::BigDecimal(v) => query.bind(v.as_deref()), }
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
            Value::IpNetwork(v) => query.bind(v.as_deref()),
        }
    } else {
        quote! {}
    };

    let with_mac_address = if cfg!(feature = "with-mac_address") {
        quote! {
            Value::MacAddress(v) => query.bind(v.as_deref()),
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
                        query.bind($v.map(|v| v as $ty))
                    };
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
                    Value::String(v) => query.bind(v.as_deref()),
                    Value::Char(v) => query.bind(v.map(|v|v.to_string())),
                    Value::Bytes(v) => query.bind(v.as_deref()),
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
