use rbs::to_value;
use sea_query::*;
pub trait SqlxBinder {
    fn build_rbs<T: QueryBuilder>(&self, query_builder: T) -> (String, Vec<rbs::Value>);
    fn build_any_rbs(&self, query_builder: &dyn QueryBuilder) -> (String, Vec<rbs::Value>);
}

macro_rules! impl_sqlx_binder {
    ($l:ident) => {
        impl SqlxBinder for $l {
            fn build_rbs<T: QueryBuilder>(&self, query_builder: T) -> (String, Vec<rbs::Value>) {
                let (query, values) = self.build(query_builder);
                (query, to_rb_values(values))
            }

            fn build_any_rbs(&self, query_builder: &dyn QueryBuilder) -> (String, Vec<rbs::Value>) {
                let (query, values) = self.build_any(query_builder);
                (query, to_rb_values(values))
            }
        }
    };
}

impl_sqlx_binder!(SelectStatement);
impl_sqlx_binder!(UpdateStatement);
impl_sqlx_binder!(InsertStatement);
impl_sqlx_binder!(DeleteStatement);
impl_sqlx_binder!(WithQuery);

fn to_rb_values(values: Values) -> Vec<rbs::Value> {
    let mut args: Vec<rbs::Value> = vec![];
    for arg in values {
        match arg {
            Value::Bool(v) => args.push(to_value!(v)),
            Value::TinyInt(v) => args.push(to_value!(v)),
            Value::SmallInt(v) => args.push(to_value!(v)),
            Value::Int(v) => args.push(to_value!(v)),
            Value::BigInt(v) => args.push(to_value!(v)),
            Value::TinyUnsigned(v) => args.push(to_value!(v)),
            Value::SmallUnsigned(v) => args.push(to_value!(v)),
            Value::Unsigned(v) => args.push(to_value!(v)),
            Value::BigUnsigned(v) => args.push(to_value!(v)),
            Value::Float(v) => args.push(to_value!(v)),
            Value::Double(v) => args.push(to_value!(v)),
            Value::String(v) => args.push(to_value!(v)),
            Value::Char(v) => args.push(to_value!(v)),
            Value::Bytes(v) => args.push(to_value!(v)),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDate(v) => args.push(to_value!(
                Value::ChronoDate(v).chrono_as_naive_utc_in_string()
            )),
            #[cfg(feature = "with-chrono")]
            Value::ChronoTime(v) => {
                args.push(to_value!(
                    Value::ChronoTime(v).chrono_as_naive_utc_in_string()
                ));
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(t) => {
                args.push(to_value!(
                    Value::ChronoDateTime(t).chrono_as_naive_utc_in_string()
                ));
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(t) => {
                args.push(to_value!(
                    Value::ChronoDateTimeUtc(t).chrono_as_naive_utc_in_string()
                ));
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(t) => {
                args.push(to_value!(
                    Value::ChronoDateTimeLocal(t).chrono_as_naive_utc_in_string()
                ));
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(t) => {
                args.push(to_value!(
                    Value::ChronoDateTimeWithTimeZone(t).chrono_as_naive_utc_in_string()
                ));
            }
            #[cfg(feature = "with-time")]
            Value::TimeDate(t) => {
                args.push(to_value!(Value::TimeDate(t).time_as_naive_utc_in_string()));
            }
            #[cfg(feature = "with-time")]
            Value::TimeTime(t) => {
                args.push(to_value!(Value::TimeTime(t).time_as_naive_utc_in_string()));
            }
            #[cfg(feature = "with-time")]
            Value::TimeDateTime(t) => {
                args.push(to_value!(
                    Value::TimeDateTime(t).time_as_naive_utc_in_string()
                ));
            }
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(t) => {
                args.push(to_value!(
                    Value::TimeDateTimeWithTimeZone(t).time_as_naive_utc_in_string()
                ));
            }
            #[cfg(feature = "with-uuid")]
            Value::Uuid(uuid) => {
                args.push(to_value!(Value::Uuid(uuid).to_string()));
            }
            #[cfg(feature = "with-rust_decimal")]
            Value::Decimal(d) => {
                args.push(to_value!(d));
            }
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(d) => {
                args.push(to_value!(Value::BigDecimal(d).to_string()));
            }
            #[cfg(feature = "with-json")]
            Value::Json(j) => {
                args.push(to_value!(j));
            }
            #[cfg(feature = "postgres-array")]
            Value::Array(_, _) => {
                panic!("Mysql doesn't support array arguments");
            }
            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(_) => {
                panic!("Mysql doesn't support IpNetwork arguments");
            }
            #[cfg(feature = "with-mac_address")]
            Value::MacAddress(_) => {
                panic!("Mysql doesn't support MacAddress arguments");
            }
        }
    }
    args
}
