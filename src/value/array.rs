use super::*;

#[derive(Debug, Clone, PartialEq)]
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
    Enum(Arc<str>, Box<[Option<Arc<Enum>>]>),
    Array(Box<[Option<Array>]>),
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
    JiffDateTime(Box<[Option<Box<jiff::civil::DateTime>>]>),
    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffTimestamp(Box<[Option<Box<Timestamp>>]>),
    #[cfg(feature = "with-jiff")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-jiff")))]
    JiffZoned(Box<[Option<Box<Zoned>>]>),
    #[cfg(feature = "with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid(Box<[Option<Uuid>]>),
    #[cfg(feature = "with-rust_decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
    Decimal(Box<[Option<Decimal>]>),
    #[cfg(feature = "with-bigdecimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
    BigDecimal(Box<[Option<Box<BigDecimal>>]>),
    #[cfg(feature = "with-ipnetwork")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
    IpNetwork(Box<[Option<IpNetwork>]>),
    #[cfg(feature = "with-mac_address")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
    MacAddress(Box<[Option<MacAddress>]>),
}
