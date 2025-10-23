use super::*;

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
    Enum(Box<(Arc<str>, Box<[Option<Arc<Enum>>]>)>),
    Array(Box<(ArrayType, Box<[Option<Array>]>)>),
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
            Array::Array(arr) => arr.as_ref().0.clone(),
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
            Array::Enum(boxed) => boxed.as_ref().1.is_empty(),
            Array::Array(v) => v.as_ref().1.is_empty(),
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

    pub fn try_from_parts(ty: ArrayType, vals: Vec<Value>) -> Result<Self, ValueTypeErr> {
        match ty {
            ArrayType::Bool => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::Bool(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::Bool(v.into_boxed_slice()))
            }
            ArrayType::TinyInt => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::TinyInt(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::TinyInt(v.into_boxed_slice()))
            }
            ArrayType::SmallInt => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::SmallInt(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::SmallInt(v.into_boxed_slice()))
            }
            ArrayType::Int => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::Int(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::Int(v.into_boxed_slice()))
            }
            ArrayType::BigInt => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::BigInt(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::BigInt(v.into_boxed_slice()))
            }
            ArrayType::TinyUnsigned => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::TinyUnsigned(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::TinyUnsigned(v.into_boxed_slice()))
            }
            ArrayType::SmallUnsigned => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::SmallUnsigned(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::SmallUnsigned(v.into_boxed_slice()))
            }
            ArrayType::Unsigned => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::Unsigned(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::Unsigned(v.into_boxed_slice()))
            }
            ArrayType::BigUnsigned => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::BigUnsigned(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::BigUnsigned(v.into_boxed_slice()))
            }
            ArrayType::Float => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::Float(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::Float(v.into_boxed_slice()))
            }
            ArrayType::Double => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::Double(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::Double(v.into_boxed_slice()))
            }
            ArrayType::String => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::String(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::String(v.into_boxed_slice()))
            }
            ArrayType::Char => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::Char(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::Char(v.into_boxed_slice()))
            }
            ArrayType::Bytes => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::Bytes(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::Bytes(v.into_boxed_slice()))
            }
            #[cfg(feature = "backend-postgres")]
            ArrayType::Enum(name) => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::Enum(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::Enum(Box::new((name, v.into_boxed_slice()))))
            }
            #[cfg(feature = "with-json")]
            ArrayType::Json => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::Json(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::Json(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-chrono")]
            ArrayType::ChronoDate => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::ChronoDate(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::ChronoDate(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-chrono")]
            ArrayType::ChronoTime => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::ChronoTime(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::ChronoTime(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-chrono")]
            ArrayType::ChronoDateTime => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::ChronoDateTime(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::ChronoDateTime(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-chrono")]
            ArrayType::ChronoDateTimeUtc => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::ChronoDateTimeUtc(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::ChronoDateTimeUtc(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-chrono")]
            ArrayType::ChronoDateTimeLocal => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::ChronoDateTimeLocal(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::ChronoDateTimeLocal(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-chrono")]
            ArrayType::ChronoDateTimeWithTimeZone => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::ChronoDateTimeWithTimeZone(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::ChronoDateTimeWithTimeZone(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-time")]
            ArrayType::TimeDate => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::TimeDate(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::TimeDate(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-time")]
            ArrayType::TimeTime => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::TimeTime(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::TimeTime(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-time")]
            ArrayType::TimeDateTime => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::TimeDateTime(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::TimeDateTime(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-time")]
            ArrayType::TimeDateTimeWithTimeZone => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::TimeDateTimeWithTimeZone(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::TimeDateTimeWithTimeZone(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-jiff")]
            ArrayType::JiffDate => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::JiffDate(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::JiffDate(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-jiff")]
            ArrayType::JiffTime => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::JiffTime(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::JiffTime(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-jiff")]
            ArrayType::JiffDateTime => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::JiffDateTime(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::JiffDateTime(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-jiff")]
            ArrayType::JiffTimestamp => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::JiffTimestamp(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::JiffTimestamp(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-jiff")]
            ArrayType::JiffZoned => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::JiffZoned(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::JiffZoned(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-uuid")]
            ArrayType::Uuid => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::Uuid(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::Uuid(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-rust_decimal")]
            ArrayType::Decimal => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::Decimal(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::Decimal(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-bigdecimal")]
            ArrayType::BigDecimal => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::BigDecimal(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::BigDecimal(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-ipnetwork")]
            ArrayType::IpNetwork => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::IpNetwork(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::IpNetwork(v.into_boxed_slice()))
            }
            #[cfg(feature = "with-mac_address")]
            ArrayType::MacAddress => {
                let mut v = Vec::with_capacity(vals.len());
                for e in vals {
                    match e {
                        Value::MacAddress(x) => v.push(x),
                        _ => return Err(ValueTypeErr),
                    }
                }
                Ok(Array::MacAddress(v.into_boxed_slice()))
            }
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
            Array::Array(val) => {
                let val = val.as_ref();
                Array::Array(Box::new((val.0.clone(), Box::new([]))))
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

    #[cfg(feature = "with-json")]
    #[allow(unused)]
    pub(crate) fn to_json_values(&self) -> Vec<serde_json::Value> {
        match self {
            Array::Bool(items) => todo!(),
            Array::TinyInt(items) => todo!(),
            Array::SmallInt(items) => todo!(),
            Array::Int(items) => todo!(),
            Array::BigInt(items) => todo!(),
            Array::TinyUnsigned(items) => todo!(),
            Array::SmallUnsigned(items) => todo!(),
            Array::Unsigned(items) => todo!(),
            Array::BigUnsigned(items) => todo!(),
            Array::Float(items) => todo!(),
            Array::Double(items) => todo!(),
            Array::String(items) => todo!(),
            Array::Char(items) => todo!(),
            Array::Bytes(items) => todo!(),
            Array::Enum(_) => todo!(),
            Array::Array(items) => todo!(),
            Array::Json(values) => todo!(),
            Array::ChronoDate(naive_dates) => todo!(),
            Array::ChronoTime(naive_times) => todo!(),
            Array::ChronoDateTime(naive_date_times) => todo!(),
            Array::ChronoDateTimeUtc(date_times) => todo!(),
            Array::ChronoDateTimeLocal(date_times) => todo!(),
            Array::ChronoDateTimeWithTimeZone(date_times) => todo!(),
            Array::TimeDate(dates) => todo!(),
            Array::TimeTime(times) => todo!(),
            Array::TimeDateTime(primitive_date_times) => todo!(),
            Array::TimeDateTimeWithTimeZone(offset_date_times) => todo!(),
            Array::JiffDate(dates) => todo!(),
            Array::JiffTime(times) => todo!(),
            Array::JiffDateTime(date_times) => todo!(),
            Array::JiffTimestamp(timestamps) => todo!(),
            Array::JiffZoned(zoneds) => todo!(),
            Array::Uuid(uuids) => todo!(),
            Array::Decimal(decimals) => todo!(),
            Array::BigDecimal(big_decimals) => todo!(),
            Array::IpNetwork(ip_networks) => todo!(),
            Array::MacAddress(items) => todo!(),
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
                (Self::Array(l0), Self::Array(r0)) => l0 == r0,
                (Self::Json(l0), Self::Json(r0)) => l0 == r0,
                (Self::ChronoDate(l0), Self::ChronoDate(r0)) => l0 == r0,
                (Self::ChronoTime(l0), Self::ChronoTime(r0)) => l0 == r0,
                (Self::ChronoDateTime(l0), Self::ChronoDateTime(r0)) => l0 == r0,
                (Self::ChronoDateTimeUtc(l0), Self::ChronoDateTimeUtc(r0)) => l0 == r0,
                (Self::ChronoDateTimeLocal(l0), Self::ChronoDateTimeLocal(r0)) => l0 == r0,
                (Self::ChronoDateTimeWithTimeZone(l0), Self::ChronoDateTimeWithTimeZone(r0)) => {
                    l0 == r0
                }
                (Self::TimeDate(l0), Self::TimeDate(r0)) => l0 == r0,
                (Self::TimeTime(l0), Self::TimeTime(r0)) => l0 == r0,
                (Self::TimeDateTime(l0), Self::TimeDateTime(r0)) => l0 == r0,
                (Self::TimeDateTimeWithTimeZone(l0), Self::TimeDateTimeWithTimeZone(r0)) => {
                    l0 == r0
                }
                (Self::JiffDate(l0), Self::JiffDate(r0)) => l0 == r0,
                (Self::JiffTime(l0), Self::JiffTime(r0)) => l0 == r0,
                (Self::JiffDateTime(l0), Self::JiffDateTime(r0)) => l0 == r0,
                (Self::JiffTimestamp(l0), Self::JiffTimestamp(r0)) => l0 == r0,
                (Self::JiffZoned(l0), Self::JiffZoned(r0)) => l0 == r0,
                (Self::Uuid(l0), Self::Uuid(r0)) => l0 == r0,
                (Self::Decimal(l0), Self::Decimal(r0)) => l0 == r0,
                (Self::BigDecimal(l0), Self::BigDecimal(r0)) => l0 == r0,
                (Self::IpNetwork(l0), Self::IpNetwork(r0)) => l0 == r0,
                (Self::MacAddress(l0), Self::MacAddress(r0)) => l0 == r0,
                _ => false,
            }
        }
    }

    impl Eq for Array {}

    impl std::hash::Hash for Array {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
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
                Array::Float(items) => items
                    .iter()
                    .copied()
                    .map(|x| x.map(OrderedFloat))
                    .collect::<Vec<_>>()
                    .hash(state),
                Array::Double(items) => items
                    .iter()
                    .copied()
                    .map(|x| x.map(OrderedFloat))
                    .collect::<Vec<_>>()
                    .hash(state),
                Array::String(items) => items.hash(state),
                Array::Char(items) => items.hash(state),
                Array::Bytes(items) => items.hash(state),
                Array::Enum(items) => items.hash(state),
                Array::Array(items) => items.hash(state),
                Array::Json(items) => items.hash(state),
                Array::ChronoDate(items) => items.hash(state),
                Array::ChronoTime(items) => items.hash(state),
                Array::ChronoDateTime(items) => items.hash(state),
                Array::ChronoDateTimeUtc(items) => items.hash(state),
                Array::ChronoDateTimeLocal(items) => items.hash(state),
                Array::ChronoDateTimeWithTimeZone(items) => items.hash(state),
                Array::TimeDate(items) => items.hash(state),
                Array::TimeTime(items) => items.hash(state),
                Array::TimeDateTime(items) => items.hash(state),
                Array::TimeDateTimeWithTimeZone(items) => items.hash(state),
                Array::JiffDate(items) => items.hash(state),
                Array::JiffTime(items) => items.hash(state),
                Array::JiffDateTime(items) => items.hash(state),
                Array::JiffTimestamp(items) => items.hash(state),
                Array::JiffZoned(items) => items.hash(state),
                Array::Uuid(items) => items.hash(state),
                Array::Decimal(items) => items.hash(state),
                Array::BigDecimal(items) => items.hash(state),
                Array::IpNetwork(items) => items.hash(state),
                Array::MacAddress(items) => items.hash(state),
            }
        }
    }
}
