use super::*;
#[cfg(feature = "with-json")]
use crate::backend::ValueEncoder;
use std::sync::Arc;

type EnumArray = Box<(Arc<str>, Box<[Option<Arc<Enum>>]>)>;

#[derive(Debug, Clone)]
#[cfg_attr(not(feature = "hashable-value"), derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
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
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json(Box<[Option<Json>]>),
    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDate(Box<[Option<NaiveDate>]>),
    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoTime(Box<[Option<NaiveTime>]>),
    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTime(Box<[Option<NaiveDateTime>]>),
    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeUtc(Box<[Option<DateTime<Utc>>]>),
    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeLocal(Box<[Option<DateTime<Local>>]>),
    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeWithTimeZone(Box<[Option<DateTime<FixedOffset>>]>),
    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDate(Box<[Option<time::Date>]>),
    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeTime(Box<[Option<time::Time>]>),
    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTime(Box<[Option<PrimitiveDateTime>]>),
    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTimeWithTimeZone(Box<[Option<OffsetDateTime>]>),
    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffDate(Box<[Option<jiff::civil::Date>]>),
    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffTime(Box<[Option<jiff::civil::Time>]>),
    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffDateTime(Box<[Option<jiff::civil::DateTime>]>),
    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffTimestamp(Box<[Option<Timestamp>]>),
    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffZoned(Box<[Option<Zoned>]>),
    #[cfg(feature = "with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid(Box<[Option<Uuid>]>),
    #[cfg(feature = "with-rust_decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
    Decimal(Box<[Option<Decimal>]>),
    #[cfg(feature = "with-bigdecimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
    BigDecimal(Box<[Option<BigDecimal>]>),
    #[cfg(feature = "with-ipnetwork")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
    IpNetwork(Box<[Option<IpNetwork>]>),
    #[cfg(feature = "with-mac_address")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
    MacAddress(Box<[Option<MacAddress>]>),
}

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
        }
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
                Json::String(encode_to_string(|enc, buf| enc.write_mac_to(buf, mac)))
            }),
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
        }
    }
}
#[cfg(feature = "hashable-value")]
mod hash {
    use ordered_float::{FloatCore, OrderedFloat};

    use super::Array;

    #[inline]
    fn map_option_ordered_float_vec<T>(
        vec: &[Option<T>],
    ) -> impl Iterator<Item = Option<OrderedFloat<T>>> + '_
    where
        T: FloatCore,
    {
        vec.iter().copied().map(|x| x.map(OrderedFloat))
    }

    #[inline]
    fn cmp_option_ordered_float_vec<T>(left: &[Option<T>], right: &[Option<T>]) -> bool
    where
        T: FloatCore,
    {
        map_option_ordered_float_vec(left).eq(map_option_ordered_float_vec(right))
    }

    impl PartialEq for Array {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
                (Self::TinyInt(l0), Self::TinyInt(r0)) => l0 == r0,
                (Self::SmallInt(l0), Self::SmallInt(r0)) => l0 == r0,
                (Self::Int(l0), Self::Int(r0)) => l0 == r0,
                (Self::BigInt(l0), Self::BigInt(r0)) => l0 == r0,
                (Self::TinyUnsigned(l0), Self::TinyUnsigned(r0)) => l0 == r0,
                (Self::SmallUnsigned(l0), Self::SmallUnsigned(r0)) => l0 == r0,
                (Self::Unsigned(l0), Self::Unsigned(r0)) => l0 == r0,
                (Self::BigUnsigned(l0), Self::BigUnsigned(r0)) => l0 == r0,
                (Self::Float(l0), Self::Float(r0)) => cmp_option_ordered_float_vec(l0, r0),
                (Self::Double(l0), Self::Double(r0)) => cmp_option_ordered_float_vec(l0, r0),
                (Self::String(l0), Self::String(r0)) => l0 == r0,
                (Self::Char(l0), Self::Char(r0)) => l0 == r0,
                (Self::Bytes(l0), Self::Bytes(r0)) => l0 == r0,
                (Self::Enum(l0), Self::Enum(r0)) => l0 == r0,
                #[cfg(feature = "with-json")]
                (Self::Json(l0), Self::Json(r0)) => l0 == r0,
                #[cfg(feature = "with-chrono")]
                (Self::ChronoDate(l0), Self::ChronoDate(r0)) => l0 == r0,
                #[cfg(feature = "with-chrono")]
                (Self::ChronoTime(l0), Self::ChronoTime(r0)) => l0 == r0,
                #[cfg(feature = "with-chrono")]
                (Self::ChronoDateTime(l0), Self::ChronoDateTime(r0)) => l0 == r0,
                #[cfg(feature = "with-chrono")]
                (Self::ChronoDateTimeUtc(l0), Self::ChronoDateTimeUtc(r0)) => l0 == r0,
                #[cfg(feature = "with-chrono")]
                (Self::ChronoDateTimeLocal(l0), Self::ChronoDateTimeLocal(r0)) => l0 == r0,
                #[cfg(feature = "with-chrono")]
                (Self::ChronoDateTimeWithTimeZone(l0), Self::ChronoDateTimeWithTimeZone(r0)) => {
                    l0 == r0
                }
                #[cfg(feature = "with-time")]
                (Self::TimeDate(l0), Self::TimeDate(r0)) => l0 == r0,
                #[cfg(feature = "with-time")]
                (Self::TimeTime(l0), Self::TimeTime(r0)) => l0 == r0,
                #[cfg(feature = "with-time")]
                (Self::TimeDateTime(l0), Self::TimeDateTime(r0)) => l0 == r0,
                #[cfg(feature = "with-time")]
                (Self::TimeDateTimeWithTimeZone(l0), Self::TimeDateTimeWithTimeZone(r0)) => {
                    l0 == r0
                }
                #[cfg(feature = "with-jiff")]
                (Self::JiffDate(l0), Self::JiffDate(r0)) => l0 == r0,
                #[cfg(feature = "with-jiff")]
                (Self::JiffTime(l0), Self::JiffTime(r0)) => l0 == r0,
                #[cfg(feature = "with-jiff")]
                (Self::JiffDateTime(l0), Self::JiffDateTime(r0)) => l0 == r0,
                #[cfg(feature = "with-jiff")]
                (Self::JiffTimestamp(l0), Self::JiffTimestamp(r0)) => l0 == r0,
                #[cfg(feature = "with-jiff")]
                (Self::JiffZoned(l0), Self::JiffZoned(r0)) => l0 == r0,
                #[cfg(feature = "with-uuid")]
                (Self::Uuid(l0), Self::Uuid(r0)) => l0 == r0,
                #[cfg(feature = "with-rust_decimal")]
                (Self::Decimal(l0), Self::Decimal(r0)) => l0 == r0,
                #[cfg(feature = "with-bigdecimal")]
                (Self::BigDecimal(l0), Self::BigDecimal(r0)) => l0 == r0,
                #[cfg(feature = "with-ipnetwork")]
                (Self::IpNetwork(l0), Self::IpNetwork(r0)) => l0 == r0,
                #[cfg(feature = "with-mac_address")]
                (Self::MacAddress(l0), Self::MacAddress(r0)) => l0 == r0,
                _ => false,
            }
        }
    }

    impl Eq for Array {}

    impl std::hash::Hash for Array {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            use ordered_float::OrderedFloat;

            std::mem::discriminant(self).hash(state);
            match self {
                Array::Bool(items) => items.hash(state),
                Array::TinyInt(items) => items.hash(state),
                Array::SmallInt(items) => items.hash(state),
                Array::Int(items) => items.hash(state),
                Array::BigInt(items) => items.hash(state),
                Array::TinyUnsigned(items) => items.hash(state),
                Array::SmallUnsigned(items) => items.hash(state),
                Array::Unsigned(items) => items.hash(state),
                Array::BigUnsigned(items) => items.hash(state),
                Array::Float(items) => {
                    for x in items.iter() {
                        x.map(OrderedFloat).hash(state)
                    }
                }
                Array::Double(items) => {
                    for x in items.iter() {
                        x.map(OrderedFloat).hash(state)
                    }
                }
                Array::String(items) => items.hash(state),
                Array::Char(items) => items.hash(state),
                Array::Bytes(items) => items.hash(state),
                Array::Enum(items) => items.hash(state),
                #[cfg(feature = "with-json")]
                Array::Json(items) => items.hash(state),
                #[cfg(feature = "with-chrono")]
                Array::ChronoDate(items) => items.hash(state),
                #[cfg(feature = "with-chrono")]
                Array::ChronoTime(items) => items.hash(state),
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTime(items) => items.hash(state),
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTimeUtc(items) => items.hash(state),
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTimeLocal(items) => items.hash(state),
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTimeWithTimeZone(items) => items.hash(state),
                #[cfg(feature = "with-time")]
                Array::TimeDate(items) => items.hash(state),
                #[cfg(feature = "with-time")]
                Array::TimeTime(items) => items.hash(state),
                #[cfg(feature = "with-time")]
                Array::TimeDateTime(items) => items.hash(state),
                #[cfg(feature = "with-time")]
                Array::TimeDateTimeWithTimeZone(items) => items.hash(state),
                #[cfg(feature = "with-jiff")]
                Array::JiffDate(items) => items.hash(state),
                #[cfg(feature = "with-jiff")]
                Array::JiffTime(items) => items.hash(state),
                #[cfg(feature = "with-jiff")]
                Array::JiffDateTime(items) => items.hash(state),
                #[cfg(feature = "with-jiff")]
                Array::JiffTimestamp(items) => items.hash(state),
                #[cfg(feature = "with-jiff")]
                Array::JiffZoned(items) => items.hash(state),
                #[cfg(feature = "with-uuid")]
                Array::Uuid(items) => items.hash(state),
                #[cfg(feature = "with-rust_decimal")]
                Array::Decimal(items) => items.hash(state),
                #[cfg(feature = "with-bigdecimal")]
                Array::BigDecimal(items) => items.hash(state),
                #[cfg(feature = "with-ipnetwork")]
                Array::IpNetwork(items) => items.hash(state),
                #[cfg(feature = "with-mac_address")]
                Array::MacAddress(items) => items.hash(state),
            }
        }
    }
}
