use crate::utils::{BindParamArgs, SqlxDriverArgs};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub fn bind_params_sqlx_mysql_impl(input: TokenStream) -> TokenStream {
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
            v @ Value::ChronoDateTimeWithTimeZone(_) => query.bind(v.chrono_as_naive_utc_in_string()),
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

    let output = quote! {
        {
            let mut query = #query;
            for value in #params.iter() {
                query = match value {
                    Value::Bool(v) => query.bind(v.as_ref()),
                    Value::TinyInt(v) => query.bind(v.as_ref()),
                    Value::SmallInt(v) => query.bind(v.as_ref()),
                    Value::Int(v) => query.bind(v.as_ref()),
                    Value::BigInt(v) => query.bind(v.as_ref()),
                    Value::TinyUnsigned(v) => query.bind(v.as_ref()),
                    Value::SmallUnsigned(v) => query.bind(v.as_ref()),
                    Value::Unsigned(v) => query.bind(v.as_ref()),
                    Value::BigUnsigned(v) => query.bind(v.as_ref()),
                    Value::Float(v) => query.bind(v.as_ref()),
                    Value::Double(v) => query.bind(v.as_ref()),
                    Value::String(v) => query.bind(v.as_deref()),
                    Value::Char(v) => query.bind(v.map(|v| v.to_string())),
                    Value::Bytes(v) => query.bind(v.as_deref()),
                    #with_json
                    #with_chrono
                    #with_time
                    #with_uuid
                    #with_rust_decimal
                    #with_big_decimal
                    #[allow(unreachable_patterns)]
                    _ => unimplemented!(),
                };
            };
            query
        }
    };
    output.into()
}

pub fn sea_query_driver_mysql_impl(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as SqlxDriverArgs);
    let sqlx_path = args.driver;
    let sea_query_path = args.sea_query;

    let output = quote! {
        mod sea_query_driver_mysql {
            use #sqlx_path::sqlx::{mysql::MySqlArguments, MySql};
            use #sea_query_path::sea_query::{Value, Values};

            type SqlxQuery<'a> = #sqlx_path::sqlx::query::Query<'a, MySql, MySqlArguments>;
            type SqlxQueryAs<'a, T> = #sqlx_path::sqlx::query::QueryAs<'a, MySql, T, MySqlArguments>;

            pub fn bind_query<'a>(query: SqlxQuery<'a>, params: &'a Values) -> SqlxQuery<'a> {
                #sea_query_path::sea_query::bind_params_sqlx_mysql!(query, params.0)
            }

            pub fn bind_query_as<'a, T>(
                query: SqlxQueryAs<'a, T>,
                params: &'a Values,
            ) -> SqlxQueryAs<'a, T> {
                #sea_query_path::sea_query::bind_params_sqlx_mysql!(query, params.0)
            }
        }
    };

    output.into()
}
