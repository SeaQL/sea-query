use rbs::to_value;
use rbs::Value as RbValue;
use sea_query::*;
pub trait SqlxBinder {
    fn build_rbs<T: QueryBuilder>(&self, query_builder: T) -> (String, Vec<rbs::Value>);
    fn build_any_rbs(&self, query_builder: &dyn QueryBuilder) -> (String, Vec<rbs::Value>);
}

macro_rules! impl_rbs_binder {
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

impl_rbs_binder!(SelectStatement);
impl_rbs_binder!(UpdateStatement);
impl_rbs_binder!(InsertStatement);
impl_rbs_binder!(DeleteStatement);
impl_rbs_binder!(WithQuery);
trait ToRbV {
    fn to(self) -> RbValue;
}

impl<T> ToRbV for Option<T>
where
    T: Into<RbValue>,
{
    fn to(self) -> RbValue {
        match self {
            Some(v) => v.into(),
            None => RbValue::Null,
        }
    }
}

fn to_rb_values(values: Values) -> Vec<rbs::Value> {
    let mut args: Vec<rbs::Value> = vec![];
    for arg in values {
        match arg {
            Value::Bool(v) => args.push(v.to()),
            Value::TinyInt(v) => args.push(v.to()),
            Value::SmallInt(v) => args.push(v.to()),
            Value::Int(v) => args.push(v.to()),
            Value::BigInt(v) => args.push(v.to()),
            Value::TinyUnsigned(v) => args.push(v.to()),
            Value::SmallUnsigned(v) => args.push(v.to()),
            Value::Unsigned(v) => args.push(v.to()),
            Value::BigUnsigned(v) => args.push(v.to()),
            Value::Float(v) => args.push(v.to()),
            Value::Double(v) => args.push(v.to()),
            Value::String(v) => {
                let d = match v {
                    Some(v) => v.to_string().into(),
                    None => RbValue::Null,
                };
                args.push(d)
            }
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
                args.push(Value::ChronoDateTime(t).chrono_as_naive_utc_in_string().to());
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(t) => {
                args.push(Value::ChronoDateTimeUtc(t).chrono_as_naive_utc_in_string().to());
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(t) => {
                args.push(Value::ChronoDateTimeLocal(t).chrono_as_naive_utc_in_string().to());
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(t) => {
                args.push(Value::ChronoDateTimeWithTimeZone(t).chrono_as_naive_utc_in_string().to());
            }
            #[cfg(feature = "with-time")]
            Value::TimeDate(t) => {
                args.push(Value::TimeDate(t).time_as_naive_utc_in_string().to());
            }
            #[cfg(feature = "with-time")]
            Value::TimeTime(t) => {
                args.push(Value::TimeTime(t).time_as_naive_utc_in_string().to());
            }
            #[cfg(feature = "with-time")]
            Value::TimeDateTime(t) => {
                args.push(Value::TimeDateTime(t).time_as_naive_utc_in_string().to());
            }
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(t) => {
                args.push(Value::TimeDateTimeWithTimeZone(t).time_as_naive_utc_in_string().to());
            }
            #[cfg(feature = "with-uuid")]
            Value::Uuid(uuid) => {
                args.push(Value::Uuid(uuid).to_string().into());
            }
            #[cfg(feature = "with-rust_decimal")]
            Value::Decimal(_) => {
                panic!("rbs doesn't support Decimal arguments");
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
