use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, parse_macro_input, token};

pub struct Args {
    pub(crate) query: syn::Expr,
    pub(crate) params: syn::Expr,
}

impl parse::Parse for Args {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let query: syn::Expr = input.parse()?;
        let _: token::Comma = input.parse()?;
        let params: syn::Expr = input.parse()?;

        Ok(Args { query, params })
    }
}

pub fn bind_params_sqlx_impl(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as Args);
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
                    ( $v: expr ) => {
                        match $v {
                            Some(v) => query.bind(v.as_ref()),
                            None => query.bind(None::<bool>),
                        }
                    };
                }
                query = match value {
                    Value::Bool(v) => bind!(v, bool),
                    Value::TinyInt(v) => bind!(v, i8),
                    Value::SmallInt(v) => bind!(v, i16),
                    Value::Int(v) => bind!(v, i32),
                    Value::BigInt(v) => bind!(v, i64),
                    Value::TinyUnsigned(v) => bind!(v, u8),
                    Value::SmallUnsigned(v) => bind!(v, u16),
                    Value::Unsigned(v) => bind!(v, u32),
                    Value::BigUnsigned(v) => bind!(v, u64),
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
                    #[allow(unreachable_patterns)]
                    _ => unimplemented!(),
                };
            };
            query
        }
    };
    output.into()
}
