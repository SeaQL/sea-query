use super::*;
use crate::RcOrArc;
#[cfg(feature = "with-json")]
use crate::backend::ValueEncoder;
use std::sync::Arc;

#[cfg(feature = "hashable-value")]
mod hash;

/// (type_name, values)
type EnumArray = Box<(Arc<str>, Box<[Option<Arc<Enum>>]>)>;

#[derive(Debug, Clone)]
#[cfg_attr(not(feature = "hashable-value"), derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Array {
    Bool(Box<[Option<bool>]>),
    TinyInt(Box<[Option<i8>]>),
    SmallInt(Box<[Option<i16>]>),
    Int(Box<[Option<i32>]>),
    BigInt(Box<[Option<i64>]>),
    TinyUnsigned(Box<[Option<u8>]>),
    SmallUnsigned(Box<[Option<u16>]>),
    Unsigned(Box<[Option<u32>]>),
    BigUnsigned(Box<[Option<u64>]>),
    Float(Box<[Option<f32>]>),
    Double(Box<[Option<f64>]>),
    String(Box<[Option<String>]>),
    Char(Box<[Option<char>]>),
    Bytes(Box<[Option<Vec<u8>>]>),
    Enum(EnumArray),
    #[cfg(feature = "with-json")]
    Json(Box<[Option<Json>]>),
    #[cfg(feature = "with-chrono")]
    ChronoDate(Box<[Option<NaiveDate>]>),
    #[cfg(feature = "with-chrono")]
    ChronoTime(Box<[Option<NaiveTime>]>),
    #[cfg(feature = "with-chrono")]
    ChronoDateTime(Box<[Option<NaiveDateTime>]>),
    #[cfg(feature = "with-chrono")]
    ChronoDateTimeUtc(Box<[Option<DateTime<Utc>>]>),
    #[cfg(feature = "with-chrono")]
    ChronoDateTimeLocal(Box<[Option<DateTime<Local>>]>),
    #[cfg(feature = "with-chrono")]
    ChronoDateTimeWithTimeZone(Box<[Option<DateTime<FixedOffset>>]>),
    #[cfg(feature = "with-time")]
    TimeDate(Box<[Option<time::Date>]>),
    #[cfg(feature = "with-time")]
    TimeTime(Box<[Option<time::Time>]>),
    #[cfg(feature = "with-time")]
    TimeDateTime(Box<[Option<PrimitiveDateTime>]>),
    #[cfg(feature = "with-time")]
    TimeDateTimeWithTimeZone(Box<[Option<OffsetDateTime>]>),
    #[cfg(feature = "with-jiff")]
    JiffDate(Box<[Option<jiff::civil::Date>]>),
    #[cfg(feature = "with-jiff")]
    JiffTime(Box<[Option<jiff::civil::Time>]>),
    #[cfg(feature = "with-jiff")]
    JiffDateTime(Box<[Option<jiff::civil::DateTime>]>),
    #[cfg(feature = "with-jiff")]
    JiffTimestamp(Box<[Option<Timestamp>]>),
    #[cfg(feature = "with-jiff")]
    JiffZoned(Box<[Option<Zoned>]>),
    #[cfg(feature = "with-uuid")]
    Uuid(Box<[Option<Uuid>]>),
    #[cfg(feature = "with-rust_decimal")]
    Decimal(Box<[Option<Decimal>]>),
    #[cfg(feature = "with-bigdecimal")]
    BigDecimal(Box<[Option<BigDecimal>]>),
    #[cfg(feature = "with-ipnetwork")]
    IpNetwork(Box<[Option<IpNetwork>]>),
    #[cfg(feature = "with-mac_address")]
    MacAddress(Box<[Option<MacAddress>]>),
    Null(ArrayType),
}

#[derive(Debug, Clone)]
pub(crate) enum ArrayValueVec {
    Bool(Vec<Option<bool>>),
    TinyInt(Vec<Option<i8>>),
    SmallInt(Vec<Option<i16>>),
    Int(Vec<Option<i32>>),
    BigInt(Vec<Option<i64>>),
    TinyUnsigned(Vec<Option<u8>>),
    SmallUnsigned(Vec<Option<u16>>),
    Unsigned(Vec<Option<u32>>),
    BigUnsigned(Vec<Option<u64>>),
    Float(Vec<Option<f32>>),
    Double(Vec<Option<f64>>),
    String(Vec<Option<String>>),
    Char(Vec<Option<char>>),
    Bytes(Vec<Option<Vec<u8>>>),
    Enum(Vec<Option<Arc<Enum>>>),
    #[cfg(feature = "with-json")]
    Json(Vec<Option<Json>>),
    #[cfg(feature = "with-chrono")]
    ChronoDate(Vec<Option<NaiveDate>>),
    #[cfg(feature = "with-chrono")]
    ChronoTime(Vec<Option<NaiveTime>>),
    #[cfg(feature = "with-chrono")]
    ChronoDateTime(Vec<Option<NaiveDateTime>>),
    #[cfg(feature = "with-chrono")]
    ChronoDateTimeUtc(Vec<Option<DateTime<Utc>>>),
    #[cfg(feature = "with-chrono")]
    ChronoDateTimeLocal(Vec<Option<DateTime<Local>>>),
    #[cfg(feature = "with-chrono")]
    ChronoDateTimeWithTimeZone(Vec<Option<DateTime<FixedOffset>>>),
    #[cfg(feature = "with-time")]
    TimeDate(Vec<Option<time::Date>>),
    #[cfg(feature = "with-time")]
    TimeTime(Vec<Option<time::Time>>),
    #[cfg(feature = "with-time")]
    TimeDateTime(Vec<Option<PrimitiveDateTime>>),
    #[cfg(feature = "with-time")]
    TimeDateTimeWithTimeZone(Vec<Option<OffsetDateTime>>),
    #[cfg(feature = "with-jiff")]
    JiffDate(Vec<Option<jiff::civil::Date>>),
    #[cfg(feature = "with-jiff")]
    JiffTime(Vec<Option<jiff::civil::Time>>),
    #[cfg(feature = "with-jiff")]
    JiffDateTime(Vec<Option<jiff::civil::DateTime>>),
    #[cfg(feature = "with-jiff")]
    JiffTimestamp(Vec<Option<Timestamp>>),
    #[cfg(feature = "with-jiff")]
    JiffZoned(Vec<Option<Zoned>>),
    #[cfg(feature = "with-uuid")]
    Uuid(Vec<Option<Uuid>>),
    #[cfg(feature = "with-rust_decimal")]
    Decimal(Vec<Option<Decimal>>),
    #[cfg(feature = "with-bigdecimal")]
    BigDecimal(Vec<Option<BigDecimal>>),
    #[cfg(feature = "with-ipnetwork")]
    IpNetwork(Vec<Option<IpNetwork>>),
    #[cfg(feature = "with-mac_address")]
    MacAddress(Vec<Option<MacAddress>>),
}

pub struct ArrayIterValue<'a>(Box<dyn Iterator<Item = Option<Value>> + 'a>);

impl Array {
    pub fn array_type(&self) -> ArrayType {
        match self {
            Array::Bool(_) => ArrayType::Bool,
            Array::TinyInt(_) => ArrayType::TinyInt,
            Array::SmallInt(_) => ArrayType::SmallInt,
            Array::Int(_) => ArrayType::Int,
            Array::BigInt(_) => ArrayType::BigInt,
            Array::TinyUnsigned(_) => ArrayType::TinyUnsigned,
            Array::SmallUnsigned(_) => ArrayType::SmallUnsigned,
            Array::Unsigned(_) => ArrayType::Unsigned,
            Array::BigUnsigned(_) => ArrayType::BigUnsigned,
            Array::Float(_) => ArrayType::Float,
            Array::Double(_) => ArrayType::Double,
            Array::String(_) => ArrayType::String,
            Array::Char(_) => ArrayType::Char,
            Array::Bytes(_) => ArrayType::Bytes,
            Array::Enum(boxed) => ArrayType::Enum(boxed.as_ref().0.clone()),
            #[cfg(feature = "with-json")]
            Array::Json(_) => ArrayType::Json,
            #[cfg(feature = "with-chrono")]
            Array::ChronoDate(_) => ArrayType::ChronoDate,
            #[cfg(feature = "with-chrono")]
            Array::ChronoTime(_) => ArrayType::ChronoTime,
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTime(_) => ArrayType::ChronoDateTime,
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeUtc(_) => ArrayType::ChronoDateTimeUtc,
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeLocal(_) => ArrayType::ChronoDateTimeLocal,
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeWithTimeZone(_) => ArrayType::ChronoDateTimeWithTimeZone,
            #[cfg(feature = "with-time")]
            Array::TimeDate(_) => ArrayType::TimeDate,
            #[cfg(feature = "with-time")]
            Array::TimeTime(_) => ArrayType::TimeTime,
            #[cfg(feature = "with-time")]
            Array::TimeDateTime(_) => ArrayType::TimeDateTime,
            #[cfg(feature = "with-time")]
            Array::TimeDateTimeWithTimeZone(_) => ArrayType::TimeDateTimeWithTimeZone,
            #[cfg(feature = "with-jiff")]
            Array::JiffDate(_) => ArrayType::JiffDate,
            #[cfg(feature = "with-jiff")]
            Array::JiffTime(_) => ArrayType::JiffTime,
            #[cfg(feature = "with-jiff")]
            Array::JiffDateTime(_) => ArrayType::JiffDateTime,
            #[cfg(feature = "with-jiff")]
            Array::JiffTimestamp(_) => ArrayType::JiffTimestamp,
            #[cfg(feature = "with-jiff")]
            Array::JiffZoned(_) => ArrayType::JiffZoned,
            #[cfg(feature = "with-uuid")]
            Array::Uuid(_) => ArrayType::Uuid,
            #[cfg(feature = "with-rust_decimal")]
            Array::Decimal(_) => ArrayType::Decimal,
            #[cfg(feature = "with-bigdecimal")]
            Array::BigDecimal(_) => ArrayType::BigDecimal,
            #[cfg(feature = "with-ipnetwork")]
            Array::IpNetwork(_) => ArrayType::IpNetwork,
            #[cfg(feature = "with-mac_address")]
            Array::MacAddress(_) => ArrayType::MacAddress,
            Array::Null(ty) => ty.clone(),
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Array::Null(_))
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Array::Bool(v) => v.is_empty(),
            Array::TinyInt(v) => v.is_empty(),
            Array::SmallInt(v) => v.is_empty(),
            Array::Int(v) => v.is_empty(),
            Array::BigInt(v) => v.is_empty(),
            Array::TinyUnsigned(v) => v.is_empty(),
            Array::SmallUnsigned(v) => v.is_empty(),
            Array::Unsigned(v) => v.is_empty(),
            Array::BigUnsigned(v) => v.is_empty(),
            Array::Float(v) => v.is_empty(),
            Array::Double(v) => v.is_empty(),
            Array::String(v) => v.is_empty(),
            Array::Char(v) => v.is_empty(),
            Array::Bytes(v) => v.is_empty(),
            Array::Enum(b) => b.as_ref().1.is_empty(),
            #[cfg(feature = "with-json")]
            Array::Json(v) => v.is_empty(),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDate(v) => v.is_empty(),
            #[cfg(feature = "with-chrono")]
            Array::ChronoTime(v) => v.is_empty(),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTime(v) => v.is_empty(),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeUtc(v) => v.is_empty(),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeLocal(v) => v.is_empty(),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeWithTimeZone(v) => v.is_empty(),
            #[cfg(feature = "with-time")]
            Array::TimeDate(v) => v.is_empty(),
            #[cfg(feature = "with-time")]
            Array::TimeTime(v) => v.is_empty(),
            #[cfg(feature = "with-time")]
            Array::TimeDateTime(v) => v.is_empty(),
            #[cfg(feature = "with-time")]
            Array::TimeDateTimeWithTimeZone(v) => v.is_empty(),
            #[cfg(feature = "with-jiff")]
            Array::JiffDate(v) => v.is_empty(),
            #[cfg(feature = "with-jiff")]
            Array::JiffTime(v) => v.is_empty(),
            #[cfg(feature = "with-jiff")]
            Array::JiffDateTime(v) => v.is_empty(),
            #[cfg(feature = "with-jiff")]
            Array::JiffTimestamp(v) => v.is_empty(),
            #[cfg(feature = "with-jiff")]
            Array::JiffZoned(v) => v.is_empty(),
            #[cfg(feature = "with-uuid")]
            Array::Uuid(v) => v.is_empty(),
            #[cfg(feature = "with-rust_decimal")]
            Array::Decimal(v) => v.is_empty(),
            #[cfg(feature = "with-bigdecimal")]
            Array::BigDecimal(v) => v.is_empty(),
            #[cfg(feature = "with-ipnetwork")]
            Array::IpNetwork(v) => v.is_empty(),
            #[cfg(feature = "with-mac_address")]
            Array::MacAddress(v) => v.is_empty(),
            Array::Null(_) => true,
        }
    }

    pub fn iter_value(&self) -> ArrayIterValue<'_> {
        fn map_value<T>(t: &Option<T>) -> Option<Value>
        where
            T: Clone,
            Value: From<T>,
        {
            t.to_owned().map(Value::from)
        }

        ArrayIterValue(match self {
            Array::Bool(v) => Box::new(v.iter().map(map_value)),
            Array::TinyInt(v) => Box::new(v.iter().map(map_value)),
            Array::SmallInt(v) => Box::new(v.iter().map(map_value)),
            Array::Int(v) => Box::new(v.iter().map(map_value)),
            Array::BigInt(v) => Box::new(v.iter().map(map_value)),
            Array::TinyUnsigned(v) => Box::new(v.iter().map(map_value)),
            Array::SmallUnsigned(v) => Box::new(v.iter().map(map_value)),
            Array::Unsigned(v) => Box::new(v.iter().map(map_value)),
            Array::BigUnsigned(v) => Box::new(v.iter().map(map_value)),
            Array::Float(v) => Box::new(v.iter().map(map_value)),
            Array::Double(v) => Box::new(v.iter().map(map_value)),
            Array::String(v) => Box::new(v.iter().map(map_value)),
            Array::Char(v) => Box::new(v.iter().map(map_value)),
            Array::Bytes(v) => Box::new(v.iter().map(map_value)),
            Array::Enum(v) => {
                let (_, arr) = v.as_ref();
                Box::new(arr.iter().map(|v| Some(Value::Enum(v.clone()))))
            }
            #[cfg(feature = "with-json")]
            Array::Json(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDate(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-chrono")]
            Array::ChronoTime(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTime(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeUtc(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeLocal(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeWithTimeZone(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-time")]
            Array::TimeDate(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-time")]
            Array::TimeTime(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-time")]
            Array::TimeDateTime(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-time")]
            Array::TimeDateTimeWithTimeZone(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-jiff")]
            Array::JiffDate(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-jiff")]
            Array::JiffTime(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-jiff")]
            Array::JiffDateTime(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-jiff")]
            Array::JiffTimestamp(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-jiff")]
            Array::JiffZoned(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-uuid")]
            Array::Uuid(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-rust_decimal")]
            Array::Decimal(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-bigdecimal")]
            Array::BigDecimal(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-ipnetwork")]
            Array::IpNetwork(v) => Box::new(v.iter().map(map_value)),
            #[cfg(feature = "with-mac_address")]
            Array::MacAddress(v) => Box::new(v.iter().map(map_value)),
            Array::Null(_) => Box::new(std::iter::empty()),
        })
    }

    fn into_array_value_vec(self) -> ArrayValueVec {
        match self {
            Array::Bool(v) => ArrayValueVec::Bool(Vec::from(v)),
            Array::TinyInt(v) => ArrayValueVec::TinyInt(Vec::from(v)),
            Array::SmallInt(v) => ArrayValueVec::SmallInt(Vec::from(v)),
            Array::Int(v) => ArrayValueVec::Int(Vec::from(v)),
            Array::BigInt(v) => ArrayValueVec::BigInt(Vec::from(v)),
            Array::TinyUnsigned(v) => ArrayValueVec::TinyUnsigned(Vec::from(v)),
            Array::SmallUnsigned(v) => ArrayValueVec::SmallUnsigned(Vec::from(v)),
            Array::Unsigned(v) => ArrayValueVec::Unsigned(Vec::from(v)),
            Array::BigUnsigned(v) => ArrayValueVec::BigUnsigned(Vec::from(v)),
            Array::Float(v) => ArrayValueVec::Float(Vec::from(v)),
            Array::Double(v) => ArrayValueVec::Double(Vec::from(v)),
            Array::String(v) => ArrayValueVec::String(Vec::from(v)),
            Array::Char(v) => ArrayValueVec::Char(Vec::from(v)),
            Array::Bytes(v) => ArrayValueVec::Bytes(Vec::from(v)),
            Array::Enum(boxed) => ArrayValueVec::Enum(Vec::from(boxed.1)),
            #[cfg(feature = "with-json")]
            Array::Json(v) => ArrayValueVec::Json(Vec::from(v)),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDate(v) => ArrayValueVec::ChronoDate(Vec::from(v)),
            #[cfg(feature = "with-chrono")]
            Array::ChronoTime(v) => ArrayValueVec::ChronoTime(Vec::from(v)),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTime(v) => ArrayValueVec::ChronoDateTime(Vec::from(v)),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeUtc(v) => ArrayValueVec::ChronoDateTimeUtc(Vec::from(v)),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeLocal(v) => ArrayValueVec::ChronoDateTimeLocal(Vec::from(v)),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeWithTimeZone(v) => {
                ArrayValueVec::ChronoDateTimeWithTimeZone(Vec::from(v))
            }
            #[cfg(feature = "with-time")]
            Array::TimeDate(v) => ArrayValueVec::TimeDate(Vec::from(v)),
            #[cfg(feature = "with-time")]
            Array::TimeTime(v) => ArrayValueVec::TimeTime(Vec::from(v)),
            #[cfg(feature = "with-time")]
            Array::TimeDateTime(v) => ArrayValueVec::TimeDateTime(Vec::from(v)),
            #[cfg(feature = "with-time")]
            Array::TimeDateTimeWithTimeZone(v) => {
                ArrayValueVec::TimeDateTimeWithTimeZone(Vec::from(v))
            }
            #[cfg(feature = "with-jiff")]
            Array::JiffDate(v) => ArrayValueVec::JiffDate(Vec::from(v)),
            #[cfg(feature = "with-jiff")]
            Array::JiffTime(v) => ArrayValueVec::JiffTime(Vec::from(v)),
            #[cfg(feature = "with-jiff")]
            Array::JiffDateTime(v) => ArrayValueVec::JiffDateTime(Vec::from(v)),
            #[cfg(feature = "with-jiff")]
            Array::JiffTimestamp(v) => ArrayValueVec::JiffTimestamp(Vec::from(v)),
            #[cfg(feature = "with-jiff")]
            Array::JiffZoned(v) => ArrayValueVec::JiffZoned(Vec::from(v)),
            #[cfg(feature = "with-uuid")]
            Array::Uuid(v) => ArrayValueVec::Uuid(Vec::from(v)),
            #[cfg(feature = "with-rust_decimal")]
            Array::Decimal(v) => ArrayValueVec::Decimal(Vec::from(v)),
            #[cfg(feature = "with-bigdecimal")]
            Array::BigDecimal(v) => ArrayValueVec::BigDecimal(Vec::from(v)),
            #[cfg(feature = "with-ipnetwork")]
            Array::IpNetwork(v) => ArrayValueVec::IpNetwork(Vec::from(v)),
            #[cfg(feature = "with-mac_address")]
            Array::MacAddress(v) => ArrayValueVec::MacAddress(Vec::from(v)),
            Array::Null(_) => panic!("Array cannot be Null"),
        }
    }

    #[cfg(feature = "with-json")]
    pub(crate) fn to_json_value(&self) -> Json {
        fn map_slice_of_opts<T, F>(slice: &[Option<T>], mut f: F) -> Json
        where
            F: FnMut(&T) -> Json,
        {
            slice
                .iter()
                .map(|o| match o.as_ref() {
                    Some(v) => f(v),
                    None => Json::Null,
                })
                .collect()
        }

        fn encode_to_string<F>(f: F) -> String
        where
            F: FnOnce(&CommonSqlQueryBuilder, &mut String),
        {
            let mut s = String::new();
            let enc = CommonSqlQueryBuilder;
            f(&enc, &mut s);
            s
        }

        match self {
            Array::Bool(v) => map_slice_of_opts(v, |&b| Json::Bool(b)),
            Array::TinyInt(v) => map_slice_of_opts(v, |&x| x.into()),
            Array::SmallInt(v) => map_slice_of_opts(v, |&x| x.into()),
            Array::Int(v) => map_slice_of_opts(v, |&x| x.into()),
            Array::BigInt(v) => map_slice_of_opts(v, |&x| x.into()),
            Array::TinyUnsigned(v) => map_slice_of_opts(v, |&x| x.into()),
            Array::SmallUnsigned(v) => map_slice_of_opts(v, |&x| x.into()),
            Array::Unsigned(v) => map_slice_of_opts(v, |&x| x.into()),
            Array::BigUnsigned(v) => map_slice_of_opts(v, |&x| x.into()),
            Array::Float(v) => map_slice_of_opts(v, |&x| x.into()),
            Array::Double(v) => map_slice_of_opts(v, |&x| x.into()),
            Array::String(v) => map_slice_of_opts(v, |s| Json::String(s.clone())),
            Array::Char(v) => map_slice_of_opts(v, |&c| Json::String(c.to_string())),
            Array::Bytes(v) => map_slice_of_opts(v, |bytes| {
                Json::String(std::str::from_utf8(bytes).unwrap().to_string())
            }),
            Array::Enum(v) => {
                let (_, arr) = v.as_ref();
                map_slice_of_opts(arr, |e| Json::String(e.value.to_string()))
            }
            #[cfg(feature = "with-json")]
            Array::Json(v) => map_slice_of_opts(v, |j| j.clone()),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDate(v) => map_slice_of_opts(v, |&d| {
                Json::String(encode_to_string(|enc, buf| enc.write_naive_date_to(buf, d)))
            }),
            #[cfg(feature = "with-chrono")]
            Array::ChronoTime(v) => map_slice_of_opts(v, |&t| {
                Json::String(encode_to_string(|enc, buf| enc.write_naive_time_to(buf, t)))
            }),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTime(v) => map_slice_of_opts(v, |&dt| {
                Json::String(encode_to_string(|enc, buf| {
                    enc.write_naive_datetime_to(buf, dt)
                }))
            }),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeUtc(v) => map_slice_of_opts(v, |dt| {
                Json::String(encode_to_string(|enc, buf| {
                    enc.write_datetime_utc_to(buf, dt)
                }))
            }),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeLocal(v) => map_slice_of_opts(v, |dt| {
                Json::String(encode_to_string(|enc, buf| {
                    enc.write_datetime_local_to(buf, dt)
                }))
            }),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeWithTimeZone(v) => map_slice_of_opts(v, |dt| {
                Json::String(encode_to_string(|enc, buf| {
                    enc.write_datetime_fixed_to(buf, dt)
                }))
            }),
            #[cfg(feature = "with-time")]
            Array::TimeDate(v) => map_slice_of_opts(v, |&d| {
                Json::String(encode_to_string(|enc, buf| enc.write_time_date_to(buf, d)))
            }),
            #[cfg(feature = "with-time")]
            Array::TimeTime(v) => map_slice_of_opts(v, |&t| {
                Json::String(encode_to_string(|enc, buf| enc.write_time_time_to(buf, t)))
            }),
            #[cfg(feature = "with-time")]
            Array::TimeDateTime(v) => map_slice_of_opts(v, |&dt| {
                Json::String(encode_to_string(|enc, buf| {
                    enc.write_time_datetime_to(buf, dt)
                }))
            }),
            #[cfg(feature = "with-time")]
            Array::TimeDateTimeWithTimeZone(v) => map_slice_of_opts(v, |&dt| {
                Json::String(encode_to_string(|enc, buf| {
                    enc.write_time_datetime_tz_to(buf, dt)
                }))
            }),
            #[cfg(feature = "with-jiff")]
            Array::JiffDate(v) => map_slice_of_opts(v, |&d| {
                Json::String(encode_to_string(|enc, buf| enc.write_jiff_date_to(buf, d)))
            }),
            #[cfg(feature = "with-jiff")]
            Array::JiffTime(v) => map_slice_of_opts(v, |&t| {
                Json::String(encode_to_string(|enc, buf| enc.write_jiff_time_to(buf, t)))
            }),
            #[cfg(feature = "with-jiff")]
            Array::JiffDateTime(v) => map_slice_of_opts(v, |&dt| {
                Json::String(encode_to_string(|enc, buf| {
                    enc.write_jiff_datetime_to(buf, dt)
                }))
            }),
            #[cfg(feature = "with-jiff")]
            Array::JiffTimestamp(v) => map_slice_of_opts(v, |&ts| {
                Json::String(encode_to_string(|enc, buf| {
                    enc.write_jiff_timestamp_to(buf, ts)
                }))
            }),
            #[cfg(feature = "with-jiff")]
            Array::JiffZoned(v) => map_slice_of_opts(v, |z| {
                Json::String(encode_to_string(|enc, buf| enc.write_jiff_zoned_to(buf, z)))
            }),
            #[cfg(feature = "with-uuid")]
            Array::Uuid(v) => map_slice_of_opts(v, |&u| Json::String(u.to_string())),
            #[cfg(feature = "with-rust_decimal")]
            Array::Decimal(v) => map_slice_of_opts(v, |&d| {
                use rust_decimal::prelude::ToPrimitive;
                Json::Number(serde_json::Number::from_f64(d.to_f64().unwrap()).unwrap())
            }),
            #[cfg(feature = "with-bigdecimal")]
            Array::BigDecimal(v) => map_slice_of_opts(v, |bd| {
                use bigdecimal::ToPrimitive;
                Json::Number(serde_json::Number::from_f64(bd.to_f64().unwrap()).unwrap())
            }),
            #[cfg(feature = "with-ipnetwork")]
            Array::IpNetwork(v) => map_slice_of_opts(v, |&ip| {
                Json::String(encode_to_string(|enc, buf| enc.write_ipnetwork_to(buf, ip)))
            }),
            #[cfg(feature = "with-mac_address")]
            Array::MacAddress(v) => map_slice_of_opts(v, |&mac| {
                Json::String(encode_to_string(|enc, buf| {
                    enc.write_mac_address_to(buf, mac)
                }))
            }),
            Array::Null(_) => Json::Null,
        }
    }

    pub fn dummy_value(&self) -> Self {
        match self {
            Array::Bool(_) => Array::Bool(Box::new([])),
            Array::TinyInt(_) => Array::TinyInt(Box::new([])),
            Array::SmallInt(_) => Array::SmallInt(Box::new([])),
            Array::Int(_) => Array::Int(Box::new([])),
            Array::BigInt(_) => Array::BigInt(Box::new([])),
            Array::TinyUnsigned(_) => Array::TinyUnsigned(Box::new([])),
            Array::SmallUnsigned(_) => Array::SmallUnsigned(Box::new([])),
            Array::Unsigned(_) => Array::Unsigned(Box::new([])),
            Array::BigUnsigned(_) => Array::BigUnsigned(Box::new([])),
            Array::Float(_) => Array::Float(Box::new([])),
            Array::Double(_) => Array::Double(Box::new([])),
            Array::String(_) => Array::String(Box::new([])),
            Array::Char(_) => Array::Char(Box::new([])),
            Array::Bytes(_) => Array::Bytes(Box::new([])),
            Array::Enum(val) => {
                let val = val.as_ref();
                Array::Enum(Box::new((val.0.clone(), Box::new([]))))
            }
            #[cfg(feature = "with-json")]
            Array::Json(_) => Array::Json(Box::new([])),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDate(_) => Array::ChronoDate(Box::new([])),
            #[cfg(feature = "with-chrono")]
            Array::ChronoTime(_) => Array::ChronoTime(Box::new([])),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTime(_) => Array::ChronoDateTime(Box::new([])),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeUtc(_) => Array::ChronoDateTimeUtc(Box::new([])),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeLocal(_) => Array::ChronoDateTimeLocal(Box::new([])),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeWithTimeZone(_) => Array::ChronoDateTimeWithTimeZone(Box::new([])),
            #[cfg(feature = "with-time")]
            Array::TimeDate(_) => Array::TimeDate(Box::new([])),
            #[cfg(feature = "with-time")]
            Array::TimeTime(_) => Array::TimeTime(Box::new([])),
            #[cfg(feature = "with-time")]
            Array::TimeDateTime(_) => Array::TimeDateTime(Box::new([])),
            #[cfg(feature = "with-time")]
            Array::TimeDateTimeWithTimeZone(_) => Array::TimeDateTimeWithTimeZone(Box::new([])),
            #[cfg(feature = "with-jiff")]
            Array::JiffDate(_) => Array::JiffDate(Box::new([])),
            #[cfg(feature = "with-jiff")]
            Array::JiffTime(_) => Array::JiffTime(Box::new([])),
            #[cfg(feature = "with-jiff")]
            Array::JiffDateTime(_) => Array::JiffDateTime(Box::new([])),
            #[cfg(feature = "with-jiff")]
            Array::JiffTimestamp(_) => Array::JiffTimestamp(Box::new([])),
            #[cfg(feature = "with-jiff")]
            Array::JiffZoned(_) => Array::JiffZoned(Box::new([])),
            #[cfg(feature = "with-uuid")]
            Array::Uuid(_) => Array::Uuid(Box::new([])),
            #[cfg(feature = "with-rust_decimal")]
            Array::Decimal(_) => Array::Decimal(Box::new([])),
            #[cfg(feature = "with-bigdecimal")]
            Array::BigDecimal(_) => Array::BigDecimal(Box::new([])),
            #[cfg(feature = "with-ipnetwork")]
            Array::IpNetwork(_) => Array::IpNetwork(Box::new([])),
            #[cfg(feature = "with-mac_address")]
            Array::MacAddress(_) => Array::MacAddress(Box::new([])),
            Array::Null(ty) => Array::Null(ty.clone()),
        }
    }
}

impl Value {
    fn into_array_value_vec(self) -> ArrayValueVec {
        fn to_vec<T>(v: Option<T>) -> Vec<Option<T>> {
            vec![v]
        }

        match self {
            Self::Bool(v) => ArrayValueVec::Bool(to_vec(v)),
            Self::TinyInt(v) => ArrayValueVec::TinyInt(to_vec(v)),
            Self::SmallInt(v) => ArrayValueVec::SmallInt(to_vec(v)),
            Self::Int(v) => ArrayValueVec::Int(to_vec(v)),
            Self::BigInt(v) => ArrayValueVec::BigInt(to_vec(v)),
            Self::TinyUnsigned(v) => ArrayValueVec::TinyUnsigned(to_vec(v)),
            Self::SmallUnsigned(v) => ArrayValueVec::SmallUnsigned(to_vec(v)),
            Self::Unsigned(v) => ArrayValueVec::Unsigned(to_vec(v)),
            Self::BigUnsigned(v) => ArrayValueVec::BigUnsigned(to_vec(v)),
            Self::Float(v) => ArrayValueVec::Float(to_vec(v)),
            Self::Double(v) => ArrayValueVec::Double(to_vec(v)),
            Self::String(v) => ArrayValueVec::String(to_vec(v)),
            Self::Char(v) => ArrayValueVec::Char(to_vec(v)),
            Self::Enum(v) => ArrayValueVec::Enum(to_vec(v)),
            Self::Bytes(v) => ArrayValueVec::Bytes(to_vec(v)),

            #[cfg(feature = "with-json")]
            Self::Json(v) => ArrayValueVec::Json(to_vec(v)),

            #[cfg(feature = "with-chrono")]
            Self::ChronoDate(v) => ArrayValueVec::ChronoDate(to_vec(v)),

            #[cfg(feature = "with-chrono")]
            Self::ChronoTime(v) => ArrayValueVec::ChronoTime(to_vec(v)),

            #[cfg(feature = "with-chrono")]
            Self::ChronoDateTime(v) => ArrayValueVec::ChronoDateTime(to_vec(v)),

            #[cfg(feature = "with-chrono")]
            Self::ChronoDateTimeUtc(v) => ArrayValueVec::ChronoDateTimeUtc(to_vec(v)),

            #[cfg(feature = "with-chrono")]
            Self::ChronoDateTimeLocal(v) => ArrayValueVec::ChronoDateTimeLocal(to_vec(v)),

            #[cfg(feature = "with-chrono")]
            Self::ChronoDateTimeWithTimeZone(v) => {
                ArrayValueVec::ChronoDateTimeWithTimeZone(to_vec(v))
            }

            #[cfg(feature = "with-time")]
            Self::TimeDate(v) => ArrayValueVec::TimeDate(to_vec(v)),

            #[cfg(feature = "with-time")]
            Self::TimeTime(v) => ArrayValueVec::TimeTime(to_vec(v)),

            #[cfg(feature = "with-time")]
            Self::TimeDateTime(v) => ArrayValueVec::TimeDateTime(to_vec(v)),

            #[cfg(feature = "with-time")]
            Self::TimeDateTimeWithTimeZone(v) => ArrayValueVec::TimeDateTimeWithTimeZone(to_vec(v)),

            #[cfg(feature = "with-jiff")]
            Self::JiffDate(v) => ArrayValueVec::JiffDate(to_vec(v)),

            #[cfg(feature = "with-jiff")]
            Self::JiffTime(v) => ArrayValueVec::JiffTime(to_vec(v)),

            #[cfg(feature = "with-jiff")]
            Self::JiffDateTime(v) => ArrayValueVec::JiffDateTime(to_vec(v)),

            #[cfg(feature = "with-jiff")]
            Self::JiffTimestamp(v) => ArrayValueVec::JiffTimestamp(to_vec(v)),

            #[cfg(feature = "with-jiff")]
            Self::JiffZoned(v) => ArrayValueVec::JiffZoned(to_vec(v)),

            #[cfg(feature = "with-uuid")]
            Self::Uuid(v) => ArrayValueVec::Uuid(to_vec(v)),

            #[cfg(feature = "with-rust_decimal")]
            Self::Decimal(v) => ArrayValueVec::Decimal(to_vec(v)),

            #[cfg(feature = "with-bigdecimal")]
            Self::BigDecimal(v) => ArrayValueVec::BigDecimal(to_vec(v)),

            #[cfg(feature = "postgres-array")]
            Self::Array(v) => v.into_array_value_vec(),

            #[cfg(feature = "postgres-vector")]
            Self::Vector(_) => panic!("Array of Vector is not supported"),

            #[cfg(feature = "with-ipnetwork")]
            Self::IpNetwork(v) => ArrayValueVec::IpNetwork(to_vec(v)),

            #[cfg(feature = "with-mac_address")]
            Self::MacAddress(v) => ArrayValueVec::MacAddress(to_vec(v)),

            #[cfg(feature = "postgres-range")]
            Self::Range(_) => panic!("Array of Vector is not supported"),
        }
    }
}

impl ArrayValueVec {
    pub fn push(&mut self, v: Value) {
        match self {
            Self::Bool(a) => a.push(v.unwrap()),
            Self::TinyInt(a) => a.push(v.unwrap()),
            Self::SmallInt(a) => a.push(v.unwrap()),
            Self::Int(a) => a.push(v.unwrap()),
            Self::BigInt(a) => a.push(v.unwrap()),
            Self::TinyUnsigned(a) => a.push(v.unwrap()),
            Self::SmallUnsigned(a) => a.push(v.unwrap()),
            Self::Unsigned(a) => a.push(v.unwrap()),
            Self::BigUnsigned(a) => a.push(v.unwrap()),
            Self::Float(a) => a.push(v.unwrap()),
            Self::Double(a) => a.push(v.unwrap()),
            Self::String(a) => a.push(v.unwrap()),
            Self::Char(a) => a.push(v.unwrap()),
            Self::Bytes(a) => a.push(v.unwrap()),
            Self::Enum(a) => a.push(match v {
                Value::Enum(v) => v,
                _ => panic!("Not Enum"),
            }),
            #[cfg(feature = "with-json")]
            Self::Json(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-chrono")]
            Self::ChronoDate(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-chrono")]
            Self::ChronoTime(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-chrono")]
            Self::ChronoDateTime(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-chrono")]
            Self::ChronoDateTimeUtc(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-chrono")]
            Self::ChronoDateTimeLocal(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-chrono")]
            Self::ChronoDateTimeWithTimeZone(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-time")]
            Self::TimeDate(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-time")]
            Self::TimeTime(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-time")]
            Self::TimeDateTime(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-time")]
            Self::TimeDateTimeWithTimeZone(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-jiff")]
            Self::JiffDate(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-jiff")]
            Self::JiffTime(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-jiff")]
            Self::JiffDateTime(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-jiff")]
            Self::JiffTimestamp(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-jiff")]
            Self::JiffZoned(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-uuid")]
            Self::Uuid(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-rust_decimal")]
            Self::Decimal(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-bigdecimal")]
            Self::BigDecimal(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-ipnetwork")]
            Self::IpNetwork(a) => a.push(v.unwrap()),
            #[cfg(feature = "with-mac_address")]
            Self::MacAddress(a) => a.push(v.unwrap()),
        }
    }

    pub fn into_array(self) -> Array {
        match self {
            Self::Bool(a) => Array::Bool(a.into()),
            Self::TinyInt(a) => Array::TinyInt(a.into()),
            Self::SmallInt(a) => Array::SmallInt(a.into()),
            Self::Int(a) => Array::Int(a.into()),
            Self::BigInt(a) => Array::BigInt(a.into()),
            Self::TinyUnsigned(a) => Array::TinyUnsigned(a.into()),
            Self::SmallUnsigned(a) => Array::SmallUnsigned(a.into()),
            Self::Unsigned(a) => Array::Unsigned(a.into()),
            Self::BigUnsigned(a) => Array::BigUnsigned(a.into()),
            Self::Float(a) => Array::Float(a.into()),
            Self::Double(a) => Array::Double(a.into()),
            Self::String(a) => Array::String(a.into()),
            Self::Char(a) => Array::Char(a.into()),
            Self::Bytes(a) => Array::Bytes(a.into()),
            Self::Enum(a) => Array::Enum(Box::new((
                a.first()
                    .expect("Array empty")
                    .as_ref()
                    .unwrap()
                    .type_name
                    .as_ref()
                    .expect("No type_name?")
                    .clone(),
                a.into(),
            ))),
            #[cfg(feature = "with-json")]
            Self::Json(a) => Array::Json(a.into()),
            #[cfg(feature = "with-chrono")]
            Self::ChronoDate(a) => Array::ChronoDate(a.into()),
            #[cfg(feature = "with-chrono")]
            Self::ChronoTime(a) => Array::ChronoTime(a.into()),
            #[cfg(feature = "with-chrono")]
            Self::ChronoDateTime(a) => Array::ChronoDateTime(a.into()),
            #[cfg(feature = "with-chrono")]
            Self::ChronoDateTimeUtc(a) => Array::ChronoDateTimeUtc(a.into()),
            #[cfg(feature = "with-chrono")]
            Self::ChronoDateTimeLocal(a) => Array::ChronoDateTimeLocal(a.into()),
            #[cfg(feature = "with-chrono")]
            Self::ChronoDateTimeWithTimeZone(a) => Array::ChronoDateTimeWithTimeZone(a.into()),
            #[cfg(feature = "with-time")]
            Self::TimeDate(a) => Array::TimeDate(a.into()),
            #[cfg(feature = "with-time")]
            Self::TimeTime(a) => Array::TimeTime(a.into()),
            #[cfg(feature = "with-time")]
            Self::TimeDateTime(a) => Array::TimeDateTime(a.into()),
            #[cfg(feature = "with-time")]
            Self::TimeDateTimeWithTimeZone(a) => Array::TimeDateTimeWithTimeZone(a.into()),
            #[cfg(feature = "with-jiff")]
            Self::JiffDate(a) => Array::JiffDate(a.into()),
            #[cfg(feature = "with-jiff")]
            Self::JiffTime(a) => Array::JiffTime(a.into()),
            #[cfg(feature = "with-jiff")]
            Self::JiffDateTime(a) => Array::JiffDateTime(a.into()),
            #[cfg(feature = "with-jiff")]
            Self::JiffTimestamp(a) => Array::JiffTimestamp(a.into()),
            #[cfg(feature = "with-jiff")]
            Self::JiffZoned(a) => Array::JiffZoned(a.into()),
            #[cfg(feature = "with-uuid")]
            Self::Uuid(a) => Array::Uuid(a.into()),
            #[cfg(feature = "with-rust_decimal")]
            Self::Decimal(a) => Array::Decimal(a.into()),
            #[cfg(feature = "with-bigdecimal")]
            Self::BigDecimal(a) => Array::BigDecimal(a.into()),
            #[cfg(feature = "with-ipnetwork")]
            Self::IpNetwork(a) => Array::IpNetwork(a.into()),
            #[cfg(feature = "with-mac_address")]
            Self::MacAddress(a) => Array::MacAddress(a.into()),
        }
    }
}

impl From<Array> for Value {
    fn from(value: Array) -> Self {
        Value::Array(value)
    }
}

impl std::fmt::Debug for ArrayIterValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArrayIterValue")
    }
}

impl Iterator for ArrayIterValue<'_> {
    type Item = Option<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// Trait for custom types that can be used as PostgreSQL array elements.
///
/// When implemented, SeaQuery will provide:
/// - `ValueType` for `Vec<T>` and `Vec<Option<T>>`
/// - `From` implementations for `Vec<T>`, `Vec<Option<T>>`, `Box<[T]>`, `Box<[Option<T>]>`, `[T; N]`, and
///   `[Option<T>; N]` into `Value` and `Array`
pub trait ArrayElement: Sized {
    /// The underlying element type stored in the array.
    ///
    /// Usually this is a built-in type like `String`, `i32`, `Uuid`, ...
    type ArrayValueType: ArrayValue;

    /// Convert self into the underlying array element type.
    fn into_array_value(self) -> Self::ArrayValueType;

    /// Convert from a Value to `Vec<Option<Self>>`
    fn try_from_value(v: Value) -> Result<Vec<Option<Self>>, ValueTypeErr>;
}

/// Internal helper trait used by [`ArrayElement`] to build [`Array`] without specialization.
///
/// This trait is sealed and not intended to be implemented by downstream crates. To support a
/// custom array element type, implement [`ArrayElement`] and set `ArrayValueType` to one of the
/// built-in array value types supported by SeaQuery.
pub trait ArrayValue: crate::sealed::Sealed + Sized {
    fn array_type() -> ArrayType;
    #[doc(hidden)]
    fn into_array(iter: impl IntoIterator<Item = Option<Self>>) -> Array;
}

impl<T: ArrayElement + ValueType> ValueType for Vec<Option<T>> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        T::try_from_value(v)
    }

    fn type_name() -> String {
        format!("Vec<Option<{}>>", T::type_name())
    }

    fn array_type() -> ArrayType {
        T::ArrayValueType::array_type()
    }

    fn column_type() -> ColumnType {
        ColumnType::Array(RcOrArc::new(T::column_type()))
    }
}

impl<T: ArrayElement + ValueType> ValueType for Vec<T> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        let vec_opt = T::try_from_value(v)?;
        vec_opt
            .into_iter()
            .map(|opt| opt.ok_or(ValueTypeErr))
            .collect()
    }

    fn type_name() -> String {
        format!("Vec<{}>", T::type_name())
    }

    fn array_type() -> ArrayType {
        T::ArrayValueType::array_type()
    }

    fn column_type() -> ColumnType {
        ColumnType::Array(RcOrArc::new(T::column_type()))
    }
}

impl<T> From<Vec<T>> for Value
where
    T: ArrayElement,
{
    fn from(vec: Vec<T>) -> Value {
        Array::from(vec).into()
    }
}

impl<T> From<Vec<Option<T>>> for Value
where
    T: ArrayElement,
{
    fn from(vec: Vec<Option<T>>) -> Value {
        Array::from(vec).into()
    }
}

impl<T> From<Box<[T]>> for Value
where
    T: ArrayElement,
{
    fn from(vec: Box<[T]>) -> Value {
        Array::from(vec).into()
    }
}

impl<T> From<Box<[Option<T>]>> for Value
where
    T: ArrayElement,
{
    fn from(vec: Box<[Option<T>]>) -> Value {
        Array::from(vec).into()
    }
}

impl<T, const N: usize> From<[T; N]> for Value
where
    T: ArrayElement,
{
    fn from(x: [T; N]) -> Value {
        let iter = x.into_iter().map(|item| item.into_array_value()).map(Some);
        ArrayValue::into_array(iter).into()
    }
}

impl<T, const N: usize> From<[Option<T>; N]> for Value
where
    T: ArrayElement,
{
    fn from(x: [Option<T>; N]) -> Value {
        let iter = x
            .into_iter()
            .map(|opt| opt.map(|item| item.into_array_value()));
        ArrayValue::into_array(iter).into()
    }
}

impl<T> From<Vec<T>> for Array
where
    T: ArrayElement,
{
    fn from(vec: Vec<T>) -> Array {
        let converted = vec.into_iter().map(|x| x.into_array_value()).map(Some);
        ArrayValue::into_array(converted)
    }
}

impl<T> From<Vec<Option<T>>> for Array
where
    T: ArrayElement,
{
    fn from(vec: Vec<Option<T>>) -> Array {
        let converted = vec.into_iter().map(|opt| opt.map(|e| e.into_array_value()));
        ArrayValue::into_array(converted)
    }
}

impl<T> From<Box<[T]>> for Array
where
    T: ArrayElement,
{
    fn from(slice: Box<[T]>) -> Array {
        ArrayValue::into_array(slice.into_iter().map(|x| x.into_array_value()).map(Some))
    }
}

impl<T> From<Box<[Option<T>]>> for Array
where
    T: ArrayElement,
{
    fn from(slice: Box<[Option<T>]>) -> Array {
        let converted = slice
            .into_iter()
            .map(|opt| opt.map(|e| e.into_array_value()));

        ArrayValue::into_array(converted)
    }
}

impl<T, const N: usize> From<[T; N]> for Array
where
    T: ArrayElement,
{
    fn from(x: [T; N]) -> Array {
        let iter = x.into_iter().map(|item| item.into_array_value()).map(Some);
        ArrayValue::into_array(iter)
    }
}

impl<T, const N: usize> From<[Option<T>; N]> for Array
where
    T: ArrayElement,
{
    fn from(x: [Option<T>; N]) -> Array {
        let iter = x
            .into_iter()
            .map(|opt| opt.map(|item| item.into_array_value()));
        ArrayValue::into_array(iter)
    }
}

impl<T> std::iter::FromIterator<T> for Array
where
    T: ArrayElement,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter
            .into_iter()
            .map(|item| item.into_array_value())
            .map(Some);
        ArrayValue::into_array(iter)
    }
}

impl<T> std::iter::FromIterator<Option<T>> for Array
where
    T: ArrayElement,
{
    fn from_iter<I: IntoIterator<Item = Option<T>>>(iter: I) -> Self {
        let iter = iter
            .into_iter()
            .map(|opt| opt.map(|item| item.into_array_value()));
        ArrayValue::into_array(iter)
    }
}

impl From<Vec<Value>> for Array {
    fn from(values: Vec<Value>) -> Self {
        let mut values = values.into_iter();
        let mut arr = match values.next() {
            Some(value) => value.into_array_value_vec(),
            None => return Array::Null(ArrayType::Int), // FIXME
        };

        for value in values {
            arr.push(value);
        }

        arr.into_array()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_array_value() {
        let mut values: Vec<Value> = Vec::new();
        values.push(1i32.into());
        values.push(2i32.into());
        values.push(3i32.into());
        let array = Value::Array(values.into());
        assert_eq!(
            format!("{array:?}"),
            "Array(Int([Some(1), Some(2), Some(3)]))"
        );
        assert_eq!(array, Value::from(vec![1i32, 2, 3]));
    }
}
