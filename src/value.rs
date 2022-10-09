//! Container for all SQL value types.

#[cfg(feature = "with-json")]
use serde_json::Value as Json;
#[cfg(feature = "with-json")]
use std::str::from_utf8;

#[cfg(feature = "with-chrono")]
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};

#[cfg(feature = "with-time")]
use time::{OffsetDateTime, PrimitiveDateTime};

#[cfg(feature = "with-rust_decimal")]
use rust_decimal::Decimal;

#[cfg(feature = "with-bigdecimal")]
use bigdecimal::BigDecimal;

#[cfg(feature = "with-uuid")]
use uuid::Uuid;

#[cfg(feature = "with-ipnetwork")]
use ipnetwork::IpNetwork;

#[cfg(feature = "with-ipnetwork")]
use std::net::IpAddr;

#[cfg(feature = "with-mac_address")]
use mac_address::MacAddress;

use crate::{BlobSize, ColumnType, CommonSqlQueryBuilder, QueryBuilder};

/// Value variants
///
/// We want Value to be exactly 1 pointer sized, so anything larger should be boxed.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(Option<bool>),
    TinyInt(Option<i8>),
    SmallInt(Option<i16>),
    Int(Option<i32>),
    BigInt(Option<i64>),
    TinyUnsigned(Option<u8>),
    SmallUnsigned(Option<u16>),
    Unsigned(Option<u32>),
    BigUnsigned(Option<u64>),
    Float(Option<f32>),
    Double(Option<f64>),
    String(Option<Box<String>>),
    Char(Option<char>),

    #[cfg(feature = "postgres-array")]
    BoolArray(Option<Vec<bool>>),
    #[cfg(feature = "postgres-array")]
    TinyIntArray(Option<Vec<i8>>),
    #[cfg(feature = "postgres-array")]
    SmallIntArray(Option<Vec<i16>>),
    #[cfg(feature = "postgres-array")]
    IntArray(Option<Vec<i32>>),
    #[cfg(feature = "postgres-array")]
    BigIntArray(Option<Vec<i64>>),
    #[cfg(feature = "postgres-array")]
    SmallUnsignedArray(Option<Vec<u16>>),
    #[cfg(feature = "postgres-array")]
    UnsignedArray(Option<Vec<u32>>),
    #[cfg(feature = "postgres-array")]
    BigUnsignedArray(Option<Vec<u64>>),
    #[cfg(feature = "postgres-array")]
    FloatArray(Option<Vec<f32>>),
    #[cfg(feature = "postgres-array")]
    DoubleArray(Option<Vec<f64>>),
    #[cfg(feature = "postgres-array")]
    StringArray(Option<Vec<String>>),
    #[cfg(feature = "postgres-array")]
    CharArray(Option<Vec<char>>),

    #[allow(clippy::box_collection)]
    Bytes(Option<Box<Vec<u8>>>),

    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json(Option<Box<Json>>),
    #[cfg(all(feature = "with-json", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-json", feature = "postgres-array")))
    )]
    JsonArray(Option<Vec<Json>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDate(Option<Box<NaiveDate>>),
    #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-chrono", feature = "postgres-array")))
    )]
    ChronoDateArray(Option<Vec<NaiveDate>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoTime(Option<Box<NaiveTime>>),
    #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-chrono", feature = "postgres-array")))
    )]
    ChronoTimeArray(Option<Vec<NaiveTime>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTime(Option<Box<NaiveDateTime>>),
    #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-chrono", feature = "postgres-array")))
    )]
    ChronoDateTimeArray(Option<Vec<NaiveDateTime>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeUtc(Option<Box<DateTime<Utc>>>),
    #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-chrono", feature = "postgres-array")))
    )]
    ChronoDateTimeUtcArray(Option<Vec<DateTime<Utc>>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeLocal(Option<Box<DateTime<Local>>>),
    #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-chrono", feature = "postgres-array")))
    )]
    ChronoDateTimeLocalArray(Option<Vec<DateTime<Local>>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeWithTimeZone(Option<Box<DateTime<FixedOffset>>>),
    #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-chrono", feature = "postgres-array")))
    )]
    ChronoDateTimeWithTimeZoneArray(Option<Vec<DateTime<FixedOffset>>>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDate(Option<Box<time::Date>>),
    #[cfg(all(feature = "with-time", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-time", feature = "postgres-array")))
    )]
    TimeDateArray(Option<Vec<time::Date>>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeTime(Option<Box<time::Time>>),
    #[cfg(all(feature = "with-time", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-time", feature = "postgres-array")))
    )]
    TimeTimeArray(Option<Vec<time::Time>>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTime(Option<Box<PrimitiveDateTime>>),
    #[cfg(all(feature = "with-time", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-time", feature = "postgres-array")))
    )]
    TimeDateTimeArray(Option<Vec<PrimitiveDateTime>>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTimeWithTimeZone(Option<Box<OffsetDateTime>>),
    #[cfg(all(feature = "with-time", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-time", feature = "postgres-array")))
    )]
    TimeDateTimeWithTimeZoneArray(Option<Vec<OffsetDateTime>>),

    #[cfg(feature = "with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid(Option<Box<Uuid>>),
    #[cfg(all(feature = "with-uuid", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-uuid", feature = "postgres-array")))
    )]
    UuidArray(Option<Vec<Uuid>>),

    #[cfg(feature = "with-rust_decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
    Decimal(Option<Box<Decimal>>),
    #[cfg(all(feature = "with-rust_decimal", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-rust_decimal", feature = "postgres-array")))
    )]
    DecimalArray(Option<Vec<Decimal>>),

    #[cfg(feature = "with-bigdecimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
    BigDecimal(Option<Box<BigDecimal>>),
    #[cfg(all(feature = "with-bigdecimal", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-bigdecimal", feature = "postgres-array")))
    )]
    BigDecimalArray(Option<Vec<BigDecimal>>),

    #[cfg(feature = "with-ipnetwork")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
    IpNetwork(Option<Box<IpNetwork>>),
    #[cfg(all(feature = "with-ipnetwork", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-ipnetwork", feature = "postgres-array")))
    )]
    IpNetworkArray(Option<Vec<IpNetwork>>),

    #[cfg(feature = "with-mac_address")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
    MacAddress(Option<Box<MacAddress>>),
    #[cfg(all(feature = "with-mac_address", feature = "postgres-array"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(feature = "with-mac_address", feature = "postgres-array")))
    )]
    MacAddressArray(Option<Vec<MacAddress>>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", CommonSqlQueryBuilder.value_to_string(self))
    }
}

pub trait ValueType: Sized {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr>;

    fn unwrap(v: Value) -> Self {
        Self::try_from(v).unwrap()
    }

    fn type_name() -> String;

    fn column_type() -> ColumnType;
}

#[derive(Debug)]
pub struct ValueTypeErr;

impl std::error::Error for ValueTypeErr {}

impl std::fmt::Display for ValueTypeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Value type mismatch")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Values(pub Vec<Value>);

#[derive(Clone, Debug, PartialEq)]
pub enum ValueTuple {
    One(Value),
    Two(Value, Value),
    Three(Value, Value, Value),
    Four(Value, Value, Value, Value),
    Five(Value, Value, Value, Value, Value),
    Six(Value, Value, Value, Value, Value, Value),
}

pub trait IntoValueTuple {
    fn into_value_tuple(self) -> ValueTuple;
}

pub trait FromValueTuple: Sized {
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple;
}

pub trait Nullable {
    fn null() -> Value;
}

impl Value {
    pub fn unwrap<T>(self) -> T
    where
        T: ValueType,
    {
        T::unwrap(self)
    }
}

macro_rules! type_to_value {
    ( $type: ty, $name: ident, $col_type: expr ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::$name(Some(x))
            }
        }

        impl Nullable for $type {
            fn null() -> Value {
                Value::$name(None)
            }
        }

        impl ValueType for $type {
            fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                match v {
                    Value::$name(Some(x)) => Ok(x),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($type).to_owned()
            }

            fn column_type() -> ColumnType {
                use ColumnType::*;
                $col_type
            }
        }
    };
}

macro_rules! type_to_box_value {
    ( $type: ty, $name: ident, $col_type: expr ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::$name(Some(Box::new(x)))
            }
        }

        impl Nullable for $type {
            fn null() -> Value {
                Value::$name(None)
            }
        }

        impl ValueType for $type {
            fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                match v {
                    Value::$name(Some(x)) => Ok(*x),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($type).to_owned()
            }

            fn column_type() -> ColumnType {
                use ColumnType::*;
                $col_type
            }
        }
    };
}

type_to_value!(bool, Bool, Boolean);
type_to_value!(i8, TinyInt, TinyInteger(None));
type_to_value!(i16, SmallInt, SmallInteger(None));
type_to_value!(i32, Int, Integer(None));
type_to_value!(i64, BigInt, BigInteger(None));
type_to_value!(u8, TinyUnsigned, TinyUnsigned(None));
type_to_value!(u16, SmallUnsigned, SmallUnsigned(None));
type_to_value!(u32, Unsigned, Unsigned(None));
type_to_value!(u64, BigUnsigned, BigUnsigned(None));
type_to_value!(f32, Float, Float(None));
type_to_value!(f64, Double, Double(None));
type_to_value!(char, Char, Char(None));

impl<'a> From<&'a [u8]> for Value {
    fn from(x: &'a [u8]) -> Value {
        Value::Bytes(Some(Box::<Vec<u8>>::new(x.into())))
    }
}

impl<'a> From<&'a str> for Value {
    fn from(x: &'a str) -> Value {
        let string: String = x.into();
        Value::String(Some(Box::new(string)))
    }
}

impl<'a> Nullable for &'a str {
    fn null() -> Value {
        Value::String(None)
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value> + Nullable,
{
    fn from(x: Option<T>) -> Value {
        match x {
            Some(v) => v.into(),
            None => T::null(),
        }
    }
}

impl<T> ValueType for Option<T>
where
    T: ValueType + Nullable,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        if v == T::null() {
            Ok(None)
        } else {
            Ok(Some(T::try_from(v)?))
        }
    }

    fn type_name() -> String {
        format!("Option<{}>", T::type_name())
    }

    fn column_type() -> ColumnType {
        T::column_type()
    }
}

type_to_box_value!(Vec<u8>, Bytes, Binary(BlobSize::Blob(None)));
type_to_box_value!(String, String, String(None));

#[cfg(feature = "postgres-array")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
mod with_array {
    use super::*;

    type_to_value!(Vec<bool>, BoolArray, Array(Box::new(Boolean)));
    type_to_value!(Vec<i8>, TinyIntArray, Array(Box::new(TinyInteger(None))));
    type_to_value!(Vec<i16>, SmallIntArray, Array(Box::new(SmallInteger(None))));
    type_to_value!(Vec<i32>, IntArray, Array(Box::new(Integer(None))));
    type_to_value!(Vec<i64>, BigIntArray, Array(Box::new(BigInteger(None))));
    type_to_value!(
        Vec<u16>,
        SmallUnsignedArray,
        Array(Box::new(SmallUnsigned(None)))
    );
    type_to_value!(Vec<u32>, UnsignedArray, Array(Box::new(Unsigned(None))));
    type_to_value!(
        Vec<u64>,
        BigUnsignedArray,
        Array(Box::new(BigUnsigned(None)))
    );
    type_to_value!(Vec<f32>, FloatArray, Array(Box::new(Float(None))));
    type_to_value!(Vec<f64>, DoubleArray, Array(Box::new(Double(None))));
    type_to_value!(Vec<String>, StringArray, Array(Box::new(String(None))));
    type_to_value!(Vec<char>, CharArray, Array(Box::new(Char(None))));
}

#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
mod with_json {
    use super::*;

    type_to_box_value!(Json, Json, Json);

    #[cfg(feature = "postgres-array")]
    type_to_value!(Vec<Json>, JsonArray, Array(Box::new(Json)));
}

#[cfg(feature = "with-chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
mod with_chrono {
    use super::*;
    use chrono::{Local, Offset, Utc};

    type_to_box_value!(NaiveDate, ChronoDate, Date);
    type_to_box_value!(NaiveTime, ChronoTime, Time(None));
    type_to_box_value!(NaiveDateTime, ChronoDateTime, DateTime(None));

    impl From<DateTime<Utc>> for Value {
        fn from(v: DateTime<Utc>) -> Value {
            Value::ChronoDateTimeUtc(Some(Box::new(v)))
        }
    }

    impl From<DateTime<Local>> for Value {
        fn from(v: DateTime<Local>) -> Value {
            Value::ChronoDateTimeLocal(Some(Box::new(v)))
        }
    }

    impl From<DateTime<FixedOffset>> for Value {
        fn from(x: DateTime<FixedOffset>) -> Value {
            let v = DateTime::<FixedOffset>::from_utc(x.naive_utc(), x.offset().fix());
            Value::ChronoDateTimeWithTimeZone(Some(Box::new(v)))
        }
    }

    impl Nullable for DateTime<Utc> {
        fn null() -> Value {
            Value::ChronoDateTimeUtc(None)
        }
    }

    impl ValueType for DateTime<Utc> {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            match v {
                Value::ChronoDateTimeUtc(Some(x)) => Ok(*x),
                _ => Err(ValueTypeErr),
            }
        }

        fn type_name() -> String {
            stringify!(DateTime<Utc>).to_owned()
        }

        fn column_type() -> ColumnType {
            ColumnType::TimestampWithTimeZone(None)
        }
    }

    impl Nullable for DateTime<Local> {
        fn null() -> Value {
            Value::ChronoDateTimeLocal(None)
        }
    }

    impl ValueType for DateTime<Local> {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            match v {
                Value::ChronoDateTimeLocal(Some(x)) => Ok(*x),
                _ => Err(ValueTypeErr),
            }
        }

        fn type_name() -> String {
            stringify!(DateTime<Local>).to_owned()
        }

        fn column_type() -> ColumnType {
            ColumnType::TimestampWithTimeZone(None)
        }
    }

    impl Nullable for DateTime<FixedOffset> {
        fn null() -> Value {
            Value::ChronoDateTimeWithTimeZone(None)
        }
    }

    impl ValueType for DateTime<FixedOffset> {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            match v {
                Value::ChronoDateTimeWithTimeZone(Some(x)) => Ok(*x),
                _ => Err(ValueTypeErr),
            }
        }

        fn type_name() -> String {
            stringify!(DateTime<FixedOffset>).to_owned()
        }

        fn column_type() -> ColumnType {
            ColumnType::TimestampWithTimeZone(None)
        }
    }

    #[cfg(feature = "postgres-array")]
    type_to_value!(Vec<NaiveDate>, ChronoDateArray, Array(Box::new(Date)));
    #[cfg(feature = "postgres-array")]
    type_to_value!(Vec<NaiveTime>, ChronoTimeArray, Array(Box::new(Time(None))));
    #[cfg(feature = "postgres-array")]
    type_to_value!(
        Vec<NaiveDateTime>,
        ChronoDateTimeArray,
        Array(Box::new(DateTime(None)))
    );
    #[cfg(feature = "postgres-array")]
    type_to_value!(
        Vec<DateTime<Local>>,
        ChronoDateTimeLocalArray,
        Array(Box::new(TimestampWithTimeZone(None)))
    );
    #[cfg(feature = "postgres-array")]
    type_to_value!(
        Vec<DateTime<Utc>>,
        ChronoDateTimeUtcArray,
        Array(Box::new(TimestampWithTimeZone(None)))
    );
    #[cfg(feature = "postgres-array")]
    type_to_value!(
        Vec<DateTime<FixedOffset>>,
        ChronoDateTimeWithTimeZoneArray,
        Array(Box::new(TimestampWithTimeZone(None)))
    );
}

#[cfg(feature = "with-time")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
pub mod time_format {
    use time::format_description::FormatItem;
    use time::macros::format_description;

    pub static FORMAT_DATE: &[FormatItem<'static>] = format_description!("[year]-[month]-[day]");
    pub static FORMAT_TIME: &[FormatItem<'static>] =
        format_description!("[hour]:[minute]:[second]");
    pub static FORMAT_DATETIME: &[FormatItem<'static>] =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    pub static FORMAT_DATETIME_TZ: &[FormatItem<'static>] = format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"
    );
}

#[cfg(feature = "with-time")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
mod with_time {
    use super::*;

    type_to_box_value!(time::Date, TimeDate, Date);
    type_to_box_value!(time::Time, TimeTime, Time(None));
    type_to_box_value!(PrimitiveDateTime, TimeDateTime, DateTime(None));

    impl From<OffsetDateTime> for Value {
        fn from(v: OffsetDateTime) -> Value {
            Value::TimeDateTimeWithTimeZone(Some(Box::new(v)))
        }
    }

    impl Nullable for OffsetDateTime {
        fn null() -> Value {
            Value::TimeDateTimeWithTimeZone(None)
        }
    }

    impl ValueType for OffsetDateTime {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            match v {
                Value::TimeDateTimeWithTimeZone(Some(x)) => Ok(*x),
                _ => Err(ValueTypeErr),
            }
        }

        fn type_name() -> String {
            stringify!(OffsetDateTime).to_owned()
        }

        fn column_type() -> ColumnType {
            ColumnType::TimestampWithTimeZone(None)
        }
    }

    #[cfg(feature = "postgres-array")]
    type_to_value!(Vec<time::Date>, TimeDateArray, Array(Box::new(Date)));
    #[cfg(feature = "postgres-array")]
    type_to_value!(Vec<time::Time>, TimeTimeArray, Array(Box::new(Time(None))));
    #[cfg(feature = "postgres-array")]
    type_to_value!(
        Vec<PrimitiveDateTime>,
        TimeDateTimeArray,
        Array(Box::new(DateTime(None)))
    );
    #[cfg(feature = "postgres-array")]
    type_to_value!(
        Vec<OffsetDateTime>,
        TimeDateTimeWithTimeZoneArray,
        Array(Box::new(TimestampWithTimeZone(None)))
    );
}

#[cfg(feature = "with-rust_decimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
mod with_rust_decimal {
    use super::*;

    type_to_box_value!(Decimal, Decimal, Decimal(None));

    #[cfg(feature = "postgres-array")]
    type_to_value!(Vec<Decimal>, DecimalArray, Array(Box::new(Decimal(None))));
}

#[cfg(feature = "with-bigdecimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
mod with_bigdecimal {
    use super::*;

    type_to_box_value!(BigDecimal, BigDecimal, Decimal(None));

    #[cfg(feature = "postgres-array")]
    type_to_value!(
        Vec<BigDecimal>,
        BigDecimalArray,
        Array(Box::new(Decimal(None)))
    );
}

#[cfg(feature = "with-uuid")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
mod with_uuid {
    use super::*;

    type_to_box_value!(Uuid, Uuid, Uuid);
    #[cfg(feature = "postgres-array")]
    type_to_value!(Vec<Uuid>, UuidArray, Array(Box::new(Uuid)));
}

#[cfg(feature = "with-ipnetwork")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
mod with_ipnetwork {
    use super::*;

    type_to_box_value!(IpNetwork, IpNetwork, Inet);

    #[cfg(feature = "postgres-array")]
    type_to_value!(Vec<IpNetwork>, IpNetworkArray, Array(Box::new(Inet)));
}

#[cfg(feature = "with-mac_address")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
mod with_mac_address {
    use super::*;

    type_to_box_value!(MacAddress, MacAddress, MacAddr);

    #[cfg(feature = "postgres-array")]
    type_to_value!(Vec<MacAddress>, MacAddressArray, Array(Box::new(MacAddr)));
}

macro_rules! impl_value_methods {
    ($name:ident, $is_fn:ident, $as_ref_fn:ident, $as_ref_ty:ty) => {
        impl Value {
            pub fn $is_fn(&self) -> bool {
                matches!(self, Self::$name(_))
            }
            pub fn $as_ref_fn(&self) -> Option<&$as_ref_ty> {
                match self {
                    Self::$name(v) => v.as_deref(),
                    _ => panic!("is not Value::{}", stringify!($name)),
                }
            }
        }
    };
}

#[cfg(feature = "with-json")]
impl_value_methods!(Json, is_json, as_ref_json, Json);
#[cfg(all(feature = "with-json", feature = "postgres-array"))]
impl_value_methods!(JsonArray, is_json_array, as_ref_json_array, [Json]);

#[cfg(feature = "with-chrono")]
impl_value_methods!(ChronoDate, is_chrono_date, as_ref_chrono_date, NaiveDate);
#[cfg(feature = "with-chrono")]
impl_value_methods!(
    ChronoDateTime,
    is_chrono_date_time,
    as_ref_chrono_date_time,
    NaiveDateTime
);
#[cfg(feature = "with-chrono")]
impl_value_methods!(ChronoTime, is_chrono_time, as_ref_chrono_time, NaiveTime);
#[cfg(feature = "with-chrono")]
impl_value_methods!(
    ChronoDateTimeUtc,
    is_chrono_date_time_utc,
    as_ref_chrono_date_time_utc,
    DateTime<Utc>
);
#[cfg(feature = "with-chrono")]
impl_value_methods!(
    ChronoDateTimeLocal,
    is_chrono_date_time_local,
    as_ref_chrono_date_time_local,
    DateTime<Local>
);
#[cfg(feature = "with-chrono")]
impl_value_methods!(
    ChronoDateTimeWithTimeZone,
    is_chrono_date_time_with_time_zone,
    as_ref_chrono_date_time_with_time_zone,
    DateTime<FixedOffset>
);
#[cfg(feature = "with-chrono")]
impl Value {
    pub fn chrono_as_naive_utc_in_string(&self) -> Option<String> {
        match self {
            Self::ChronoDate(v) => v.as_ref().map(|v| v.to_string()),
            Self::ChronoTime(v) => v.as_ref().map(|v| v.to_string()),
            Self::ChronoDateTime(v) => v.as_ref().map(|v| v.to_string()),
            Self::ChronoDateTimeUtc(v) => v.as_ref().map(|v| v.naive_utc().to_string()),
            Self::ChronoDateTimeLocal(v) => v.as_ref().map(|v| v.naive_utc().to_string()),
            Self::ChronoDateTimeWithTimeZone(v) => v.as_ref().map(|v| v.naive_utc().to_string()),
            _ => panic!("not chrono Value"),
        }
    }
}
#[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
impl_value_methods!(
    ChronoDateArray,
    is_chrono_date_array,
    as_ref_chrono_date_array,
    [NaiveDate]
);
#[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
impl_value_methods!(
    ChronoDateTimeArray,
    is_chrono_date_time_array,
    as_ref_chrono_date_time_array,
    [NaiveDateTime]
);
#[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
impl_value_methods!(
    ChronoTimeArray,
    is_chrono_time_array,
    as_ref_chrono_time_array,
    [NaiveTime]
);
#[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
impl_value_methods!(
    ChronoDateTimeUtcArray,
    is_chrono_date_time_utc_array,
    as_ref_chrono_date_time_utc_array,
    [DateTime<Utc>]
);
#[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
impl_value_methods!(
    ChronoDateTimeLocalArray,
    is_chrono_date_time_local_array,
    as_ref_chrono_date_time_local_array,
    [DateTime<Local>]
);
#[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
impl_value_methods!(
    ChronoDateTimeWithTimeZoneArray,
    is_chrono_date_time_with_time_zone_array,
    as_ref_chrono_date_time_with_time_zone_array,
    [DateTime<FixedOffset>]
);

#[cfg(feature = "with-time")]
impl_value_methods!(TimeDate, is_time_date, as_ref_time_date, time::Date);
#[cfg(feature = "with-time")]
impl_value_methods!(TimeTime, is_time_time, as_ref_time_time, time::Time);
#[cfg(feature = "with-time")]
impl_value_methods!(
    TimeDateTime,
    is_time_date_time,
    as_ref_time_date_time,
    PrimitiveDateTime
);
#[cfg(feature = "with-time")]
impl_value_methods!(
    TimeDateTimeWithTimeZone,
    is_time_date_time_with_time_zone,
    as_ref_time_date_time_with_time_zone,
    OffsetDateTime
);
#[cfg(feature = "with-time")]
impl Value {
    pub fn time_as_naive_utc_in_string(&self) -> Option<String> {
        match self {
            Self::TimeDate(v) => v
                .as_ref()
                .and_then(|v| v.format(time_format::FORMAT_DATE).ok()),
            Self::TimeTime(v) => v
                .as_ref()
                .and_then(|v| v.format(time_format::FORMAT_TIME).ok()),
            Self::TimeDateTime(v) => v
                .as_ref()
                .and_then(|v| v.format(time_format::FORMAT_DATETIME).ok()),
            Self::TimeDateTimeWithTimeZone(v) => v.as_ref().and_then(|v| {
                v.to_offset(time::macros::offset!(UTC))
                    .format(time_format::FORMAT_DATETIME_TZ)
                    .ok()
            }),
            _ => panic!("not time Value"),
        }
    }
}
#[cfg(all(feature = "with-time", feature = "postgres-array"))]
impl_value_methods!(
    TimeDateArray,
    is_time_date_array,
    as_ref_time_date_array,
    [time::Date]
);
#[cfg(all(feature = "with-time", feature = "postgres-array"))]
impl_value_methods!(
    TimeTimeArray,
    is_time_time_array,
    as_ref_time_time_array,
    [time::Time]
);
#[cfg(all(feature = "with-time", feature = "postgres-array"))]
impl_value_methods!(
    TimeDateTimeArray,
    is_time_date_time_array,
    as_ref_time_date_time_array,
    [PrimitiveDateTime]
);
#[cfg(all(feature = "with-time", feature = "postgres-array"))]
impl_value_methods!(
    TimeDateTimeWithTimeZoneArray,
    is_time_date_time_with_time_zone_array,
    as_ref_time_date_time_with_time_zone_array,
    [OffsetDateTime]
);

#[cfg(feature = "with-rust_decimal")]
impl_value_methods!(Decimal, is_decimal, as_ref_decimal, Decimal);
#[cfg(feature = "with-rust_decimal")]
impl Value {
    pub fn decimal_to_f64(&self) -> Option<f64> {
        use rust_decimal::prelude::ToPrimitive;

        self.as_ref_decimal().and_then(|d| d.to_f64())
    }
}
#[cfg(all(feature = "with-rust_decimal", feature = "postgres-array"))]
impl_value_methods!(
    DecimalArray,
    is_decimal_array,
    as_ref_decimal_array,
    [Decimal]
);

#[cfg(feature = "with-bigdecimal")]
impl_value_methods!(BigDecimal, is_big_decimal, as_ref_big_decimal, BigDecimal);
#[cfg(feature = "with-bigdecimal")]
impl Value {
    pub fn big_decimal_to_f64(&self) -> Option<f64> {
        use bigdecimal::ToPrimitive;
        self.as_ref_big_decimal().map(|d| d.to_f64().unwrap())
    }
}
#[cfg(all(feature = "with-bigdecimal", feature = "postgres-array"))]
impl_value_methods!(
    BigDecimalArray,
    is_big_decimal_array,
    as_ref_big_decimal_array,
    [BigDecimal]
);

#[cfg(feature = "with-uuid")]
impl_value_methods!(Uuid, is_uuid, as_ref_uuid, Uuid);
#[cfg(all(feature = "with-uuid", feature = "postgres-array"))]
impl_value_methods!(UuidArray, is_uuid_array, as_ref_uuid_array, [Uuid]);

#[cfg(feature = "with-ipnetwork")]
impl_value_methods!(IpNetwork, is_ipnetwork, as_ref_ipnetwork, IpNetwork);
#[cfg(feature = "with-ipnetwork")]
impl Value {
    pub fn as_ipaddr(&self) -> Option<IpAddr> {
        self.as_ref_ipnetwork().map(|v| v.network())
    }
}
#[cfg(all(feature = "with-ipnetwork", feature = "postgres-array"))]
impl_value_methods!(
    IpNetworkArray,
    is_ipnetwork_array,
    as_ref_ipnetwork_array,
    [IpNetwork]
);

#[cfg(feature = "with-mac_address")]
impl_value_methods!(MacAddress, is_mac_address, as_ref_mac_address, MacAddress);
#[cfg(all(feature = "with-mac_address", feature = "postgres-array"))]
impl_value_methods!(
    MacAddressArray,
    is_mac_address_array,
    as_ref_mac_address_array,
    [MacAddress]
);

impl IntoIterator for ValueTuple {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ValueTuple::One(v) => vec![v].into_iter(),
            ValueTuple::Two(v, w) => vec![v, w].into_iter(),
            ValueTuple::Three(u, v, w) => vec![u, v, w].into_iter(),
            ValueTuple::Four(u, v, w, x) => vec![u, v, w, x].into_iter(),
            ValueTuple::Five(u, v, w, x, y) => vec![u, v, w, x, y].into_iter(),
            ValueTuple::Six(u, v, w, x, y, z) => vec![u, v, w, x, y, z].into_iter(),
        }
    }
}

impl IntoValueTuple for ValueTuple {
    fn into_value_tuple(self) -> ValueTuple {
        self
    }
}

impl<V> IntoValueTuple for V
where
    V: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::One(self.into())
    }
}

impl<V, W> IntoValueTuple for (V, W)
where
    V: Into<Value>,
    W: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::Two(self.0.into(), self.1.into())
    }
}

impl<U, V, W> IntoValueTuple for (U, V, W)
where
    U: Into<Value>,
    V: Into<Value>,
    W: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::Three(self.0.into(), self.1.into(), self.2.into())
    }
}

impl<U, V, W, X> IntoValueTuple for (U, V, W, X)
where
    U: Into<Value>,
    V: Into<Value>,
    W: Into<Value>,
    X: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::Four(self.0.into(), self.1.into(), self.2.into(), self.3.into())
    }
}

impl<U, V, W, X, Y> IntoValueTuple for (U, V, W, X, Y)
where
    U: Into<Value>,
    V: Into<Value>,
    W: Into<Value>,
    X: Into<Value>,
    Y: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::Five(
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
        )
    }
}

impl<U, V, W, X, Y, Z> IntoValueTuple for (U, V, W, X, Y, Z)
where
    U: Into<Value>,
    V: Into<Value>,
    W: Into<Value>,
    X: Into<Value>,
    Y: Into<Value>,
    Z: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::Six(
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
        )
    }
}

impl<V> FromValueTuple for V
where
    V: Into<Value> + ValueType,
{
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple,
    {
        match i.into_value_tuple() {
            ValueTuple::One(u) => u.unwrap(),
            _ => panic!("not ValueTuple::One"),
        }
    }
}

impl<V, W> FromValueTuple for (V, W)
where
    V: Into<Value> + ValueType,
    W: Into<Value> + ValueType,
{
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple,
    {
        match i.into_value_tuple() {
            ValueTuple::Two(v, w) => (v.unwrap(), w.unwrap()),
            _ => panic!("not ValueTuple::Two"),
        }
    }
}

impl<U, V, W> FromValueTuple for (U, V, W)
where
    U: Into<Value> + ValueType,
    V: Into<Value> + ValueType,
    W: Into<Value> + ValueType,
{
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple,
    {
        match i.into_value_tuple() {
            ValueTuple::Three(u, v, w) => (u.unwrap(), v.unwrap(), w.unwrap()),
            _ => panic!("not ValueTuple::Three"),
        }
    }
}

impl<U, V, W, X> FromValueTuple for (U, V, W, X)
where
    U: Into<Value> + ValueType,
    V: Into<Value> + ValueType,
    W: Into<Value> + ValueType,
    X: Into<Value> + ValueType,
{
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple,
    {
        match i.into_value_tuple() {
            ValueTuple::Four(u, v, w, x) => (u.unwrap(), v.unwrap(), w.unwrap(), x.unwrap()),
            _ => panic!("not ValueTuple::Four"),
        }
    }
}

impl<U, V, W, X, Y> FromValueTuple for (U, V, W, X, Y)
where
    U: Into<Value> + ValueType,
    V: Into<Value> + ValueType,
    W: Into<Value> + ValueType,
    X: Into<Value> + ValueType,
    Y: Into<Value> + ValueType,
{
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple,
    {
        match i.into_value_tuple() {
            ValueTuple::Five(u, v, w, x, y) => {
                (u.unwrap(), v.unwrap(), w.unwrap(), x.unwrap(), y.unwrap())
            }
            _ => panic!("not ValueTuple::Five"),
        }
    }
}

impl<U, V, W, X, Y, Z> FromValueTuple for (U, V, W, X, Y, Z)
where
    U: Into<Value> + ValueType,
    V: Into<Value> + ValueType,
    W: Into<Value> + ValueType,
    X: Into<Value> + ValueType,
    Y: Into<Value> + ValueType,
    Z: Into<Value> + ValueType,
{
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple,
    {
        match i.into_value_tuple() {
            ValueTuple::Six(u, v, w, x, y, z) => (
                u.unwrap(),
                v.unwrap(),
                w.unwrap(),
                x.unwrap(),
                y.unwrap(),
                z.unwrap(),
            ),
            _ => panic!("not ValueTuple::Six"),
        }
    }
}

/// Convert value to json value
#[allow(clippy::many_single_char_names)]
#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
pub fn sea_value_to_json_value(value: &Value) -> Json {
    match value {
        Value::Bool(None)
        | Value::TinyInt(None)
        | Value::SmallInt(None)
        | Value::Int(None)
        | Value::BigInt(None)
        | Value::TinyUnsigned(None)
        | Value::SmallUnsigned(None)
        | Value::Unsigned(None)
        | Value::BigUnsigned(None)
        | Value::Float(None)
        | Value::Double(None)
        | Value::String(None)
        | Value::Char(None)
        | Value::Bytes(None)
        | Value::Json(None) => Json::Null,
        #[cfg(feature = "postgres-array")]
        Value::BoolArray(None)
        | Value::TinyIntArray(None)
        | Value::SmallIntArray(None)
        | Value::IntArray(None)
        | Value::BigIntArray(None)
        | Value::SmallUnsignedArray(None)
        | Value::UnsignedArray(None)
        | Value::BigUnsignedArray(None)
        | Value::FloatArray(None)
        | Value::DoubleArray(None)
        | Value::StringArray(None)
        | Value::CharArray(None)
        | Value::JsonArray(None) => Json::Null,
        #[cfg(feature = "with-rust_decimal")]
        Value::Decimal(None) => Json::Null,
        #[cfg(feature = "with-bigdecimal")]
        Value::BigDecimal(None) => Json::Null,
        #[cfg(feature = "with-uuid")]
        Value::Uuid(None) => Json::Null,
        #[cfg(feature = "with-ipnetwork")]
        Value::IpNetwork(None) => Json::Null,
        #[cfg(feature = "with-mac_address")]
        Value::MacAddress(None) => Json::Null,
        #[cfg(all(feature = "with-uuid", feature = "postgres-array"))]
        Value::UuidArray(None) => Json::Null,

        Value::Bool(Some(b)) => Json::Bool(*b),
        Value::TinyInt(Some(v)) => (*v).into(),
        Value::SmallInt(Some(v)) => (*v).into(),
        Value::Int(Some(v)) => (*v).into(),
        Value::BigInt(Some(v)) => (*v).into(),
        Value::TinyUnsigned(Some(v)) => (*v).into(),
        Value::SmallUnsigned(Some(v)) => (*v).into(),
        Value::Unsigned(Some(v)) => (*v).into(),
        Value::BigUnsigned(Some(v)) => (*v).into(),
        Value::Float(Some(v)) => (*v).into(),
        Value::Double(Some(v)) => (*v).into(),
        Value::String(Some(s)) => Json::String(s.as_ref().clone()),
        Value::Char(Some(v)) => Json::String(v.to_string()),
        Value::Bytes(Some(s)) => Json::String(from_utf8(s).unwrap().to_string()),
        Value::Json(Some(v)) => v.as_ref().clone(),
        #[cfg(feature = "postgres-array")]
        Value::JsonArray(Some(v)) => v.as_slice().into(),
        #[cfg(feature = "postgres-array")]
        Value::BoolArray(Some(v)) => v.as_slice().into(),
        #[cfg(feature = "postgres-array")]
        Value::TinyIntArray(Some(v)) => v.as_slice().into(),
        #[cfg(feature = "postgres-array")]
        Value::SmallIntArray(Some(v)) => v.as_slice().into(),
        #[cfg(feature = "postgres-array")]
        Value::IntArray(Some(v)) => v.as_slice().into(),
        #[cfg(feature = "postgres-array")]
        Value::BigIntArray(Some(v)) => v.as_slice().into(),
        #[cfg(feature = "postgres-array")]
        Value::SmallUnsignedArray(Some(v)) => v.as_slice().into(),
        #[cfg(feature = "postgres-array")]
        Value::UnsignedArray(Some(v)) => v.as_slice().into(),
        #[cfg(feature = "postgres-array")]
        Value::BigUnsignedArray(Some(v)) => v.as_slice().into(),
        #[cfg(feature = "postgres-array")]
        Value::FloatArray(Some(v)) => v.as_slice().into(),
        #[cfg(feature = "postgres-array")]
        Value::DoubleArray(Some(v)) => v.as_slice().into(),
        #[cfg(feature = "postgres-array")]
        Value::StringArray(Some(v)) => v.as_slice().into(),
        #[cfg(feature = "postgres-array")]
        Value::CharArray(Some(v)) => v
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDate(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
        Value::ChronoDateArray(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
        Value::ChronoTimeArray(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
        Value::ChronoDateTimeArray(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeWithTimeZone(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
        Value::ChronoDateTimeWithTimeZoneArray(_) => {
            CommonSqlQueryBuilder.value_to_string(value).into()
        }
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeUtc(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
        Value::ChronoDateTimeUtcArray(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeLocal(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
        Value::ChronoDateTimeLocalArray(_) => CommonSqlQueryBuilder.value_to_string(value).into(),

        #[cfg(feature = "with-time")]
        Value::TimeDate(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(all(feature = "with-time", feature = "postgres-array"))]
        Value::TimeDateArray(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(all(feature = "with-time", feature = "postgres-array"))]
        Value::TimeTimeArray(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeDateTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(all(feature = "with-time", feature = "postgres-array"))]
        Value::TimeDateTimeArray(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeDateTimeWithTimeZone(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(all(feature = "with-time", feature = "postgres-array"))]
        Value::TimeDateTimeWithTimeZoneArray(_) => {
            CommonSqlQueryBuilder.value_to_string(value).into()
        }

        #[cfg(feature = "with-rust_decimal")]
        Value::Decimal(Some(v)) => {
            use rust_decimal::prelude::ToPrimitive;
            v.as_ref().to_f64().unwrap().into()
        }
        #[cfg(all(feature = "with-rust_decimal", feature = "postgres-array"))]
        Value::DecimalArray(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-bigdecimal")]
        Value::BigDecimal(Some(v)) => {
            use bigdecimal::ToPrimitive;
            v.as_ref().to_f64().unwrap().into()
        }
        #[cfg(all(feature = "with-bigdecimal", feature = "postgres-array"))]
        Value::BigDecimalArray(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-uuid")]
        Value::Uuid(Some(v)) => Json::String(v.to_string()),
        #[cfg(all(feature = "with-uuid", feature = "postgres-array"))]
        Value::UuidArray(Some(v)) => v
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .into(),
        #[cfg(feature = "with-ipnetwork")]
        Value::IpNetwork(Some(_)) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(all(feature = "with-ipnetwork", feature = "postgres-array"))]
        Value::IpNetworkArray(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-mac_address")]
        Value::MacAddress(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(all(feature = "with-mac_address", feature = "postgres-array"))]
        Value::MacAddressArray(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
    }
}

impl Values {
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }
}

impl IntoIterator for Values {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        macro_rules! test_value {
            ( $type: ty, $val: literal ) => {
                let val: $type = $val;
                let v: Value = val.into();
                let out: $type = v.unwrap();
                assert_eq!(out, val);
            };
        }

        test_value!(u8, 255);
        test_value!(u16, 65535);
        test_value!(i8, 127);
        test_value!(i16, 32767);
        test_value!(i32, 1073741824);
        test_value!(i64, 8589934592);
    }

    #[test]
    fn test_option_value() {
        macro_rules! test_some_value {
            ( $type: ty, $val: literal ) => {
                let val: Option<$type> = Some($val);
                let v: Value = val.into();
                let out: $type = v.unwrap();
                assert_eq!(out, val.unwrap());
            };
        }

        macro_rules! test_none {
            ( $type: ty, $name: ident ) => {
                let val: Option<$type> = None;
                let v: Value = val.into();
                assert_eq!(v, Value::$name(None));
            };
        }

        test_some_value!(u8, 255);
        test_some_value!(u16, 65535);
        test_some_value!(i8, 127);
        test_some_value!(i16, 32767);
        test_some_value!(i32, 1073741824);
        test_some_value!(i64, 8589934592);

        test_none!(u8, TinyUnsigned);
        test_none!(u16, SmallUnsigned);
        test_none!(i8, TinyInt);
        test_none!(i16, SmallInt);
        test_none!(i32, Int);
        test_none!(i64, BigInt);
    }

    #[test]
    fn test_box_value() {
        let val: String = "hello".to_owned();
        let v: Value = val.clone().into();
        let out: String = v.unwrap();
        assert_eq!(out, val);
    }

    #[test]
    fn test_value_tuple() {
        assert_eq!(
            1i32.into_value_tuple(),
            ValueTuple::One(Value::Int(Some(1)))
        );
        assert_eq!(
            "b".into_value_tuple(),
            ValueTuple::One(Value::String(Some(Box::new("b".to_owned()))))
        );
        assert_eq!(
            (1i32, "b").into_value_tuple(),
            ValueTuple::Two(
                Value::Int(Some(1)),
                Value::String(Some(Box::new("b".to_owned())))
            )
        );
        assert_eq!(
            (1i32, 2.4f64, "b").into_value_tuple(),
            ValueTuple::Three(
                Value::Int(Some(1)),
                Value::Double(Some(2.4)),
                Value::String(Some(Box::new("b".to_owned())))
            )
        );
        assert_eq!(
            (1i32, 2.4f64, "b", 123u8).into_value_tuple(),
            ValueTuple::Four(
                Value::Int(Some(1)),
                Value::Double(Some(2.4)),
                Value::String(Some(Box::new("b".to_owned()))),
                Value::TinyUnsigned(Some(123))
            )
        );
        assert_eq!(
            (1i32, 2.4f64, "b", 123u8, 456u16).into_value_tuple(),
            ValueTuple::Five(
                Value::Int(Some(1)),
                Value::Double(Some(2.4)),
                Value::String(Some(Box::new("b".to_owned()))),
                Value::TinyUnsigned(Some(123)),
                Value::SmallUnsigned(Some(456))
            )
        );
        assert_eq!(
            (1i32, 2.4f64, "b", 123u8, 456u16, 789u32).into_value_tuple(),
            ValueTuple::Six(
                Value::Int(Some(1)),
                Value::Double(Some(2.4)),
                Value::String(Some(Box::new("b".to_owned()))),
                Value::TinyUnsigned(Some(123)),
                Value::SmallUnsigned(Some(456)),
                Value::Unsigned(Some(789))
            )
        );
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_from_value_tuple() {
        let mut val = 1i32;
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);

        let mut val = "b".to_owned();
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);

        let mut val = (1i32, "b".to_owned());
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);

        let mut val = (1i32, 2.4f64, "b".to_owned());
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);

        let mut val = (1i32, 2.4f64, "b".to_owned(), 123u8);
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);

        let mut val = (1i32, 2.4f64, "b".to_owned(), 123u8, 456u16);
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);

        let mut val = (1i32, 2.4f64, "b".to_owned(), 123u8, 456u16, 789u32);
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);
    }

    #[test]
    fn test_value_tuple_iter() {
        let mut iter = (1i32).into_value_tuple().into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
        assert_eq!(iter.next(), None);

        let mut iter = (1i32, 2.4f64).into_value_tuple().into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
        assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
        assert_eq!(iter.next(), None);

        let mut iter = (1i32, 2.4f64, "b").into_value_tuple().into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
        assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
        assert_eq!(
            iter.next().unwrap(),
            Value::String(Some(Box::new("b".to_owned())))
        );
        assert_eq!(iter.next(), None);

        let mut iter = (1i32, 2.4f64, "b", 123u8).into_value_tuple().into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
        assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
        assert_eq!(
            iter.next().unwrap(),
            Value::String(Some(Box::new("b".to_owned())))
        );
        assert_eq!(iter.next().unwrap(), Value::TinyUnsigned(Some(123)));
        assert_eq!(iter.next(), None);

        let mut iter = (1i32, 2.4f64, "b", 123u8, 456u16)
            .into_value_tuple()
            .into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
        assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
        assert_eq!(
            iter.next().unwrap(),
            Value::String(Some(Box::new("b".to_owned())))
        );
        assert_eq!(iter.next().unwrap(), Value::TinyUnsigned(Some(123)));
        assert_eq!(iter.next().unwrap(), Value::SmallUnsigned(Some(456)));
        assert_eq!(iter.next(), None);

        let mut iter = (1i32, 2.4f64, "b", 123u8, 456u16, 789u32)
            .into_value_tuple()
            .into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
        assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
        assert_eq!(
            iter.next().unwrap(),
            Value::String(Some(Box::new("b".to_owned())))
        );
        assert_eq!(iter.next().unwrap(), Value::TinyUnsigned(Some(123)));
        assert_eq!(iter.next().unwrap(), Value::SmallUnsigned(Some(456)));
        assert_eq!(iter.next().unwrap(), Value::Unsigned(Some(789)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[cfg(feature = "with-json")]
    fn test_json_value() {
        let json = serde_json::json! {{
            "a": 25.0,
            "b": "hello",
        }};
        let value: Value = json.clone().into();
        let out: Json = value.unwrap();
        assert_eq!(out, json);
    }

    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_chrono_value() {
        let timestamp = chrono::NaiveDate::from_ymd(2020, 1, 1).and_hms(2, 2, 2);
        let value: Value = timestamp.into();
        let out: NaiveDateTime = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_chrono_utc_value() {
        let timestamp =
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2022, 1, 2).and_hms(3, 4, 5), Utc);
        let value: Value = timestamp.into();
        let out: DateTime<Utc> = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_chrono_local_value() {
        let timestamp_utc =
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2022, 1, 2).and_hms(3, 4, 5), Utc);
        let timestamp_local: DateTime<Local> = timestamp_utc.into();
        let value: Value = timestamp_local.into();
        let out: DateTime<Local> = value.unwrap();
        assert_eq!(out, timestamp_local);
    }

    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_chrono_timezone_value() {
        let timestamp = DateTime::parse_from_rfc3339("2020-01-01T02:02:02+08:00").unwrap();
        let value: Value = timestamp.into();
        let out: DateTime<FixedOffset> = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_chrono_query() {
        use crate::*;

        let string = "2020-01-01T02:02:02+08:00";
        let timestamp = DateTime::parse_from_rfc3339(string).unwrap();

        let query = Query::select().expr(Expr::val(timestamp)).to_owned();

        let formatted = "2020-01-01 02:02:02 +08:00";

        assert_eq!(
            query.to_string(MysqlQueryBuilder),
            format!("SELECT '{}'", formatted)
        );
        assert_eq!(
            query.to_string(PostgresQueryBuilder),
            format!("SELECT '{}'", formatted)
        );
        assert_eq!(
            query.to_string(SqliteQueryBuilder),
            format!("SELECT '{}'", formatted)
        );
    }

    #[test]
    #[cfg(feature = "with-time")]
    fn test_time_value() {
        use time::macros::{date, time};
        let timestamp = date!(2020 - 01 - 01).with_time(time!(2:2:2));
        let value: Value = timestamp.into();
        let out: PrimitiveDateTime = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-time")]
    fn test_time_utc_value() {
        use time::macros::{date, time};
        let timestamp = date!(2022 - 01 - 02).with_time(time!(3:04:05)).assume_utc();
        let value: Value = timestamp.into();
        let out: OffsetDateTime = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-time")]
    fn test_time_local_value() {
        use time::macros::{date, offset, time};
        let timestamp_utc = date!(2022 - 01 - 02).with_time(time!(3:04:05)).assume_utc();
        let timestamp_local: OffsetDateTime = timestamp_utc.to_offset(offset!(+3));
        let value: Value = timestamp_local.into();
        let out: OffsetDateTime = value.unwrap();
        assert_eq!(out, timestamp_local);
    }

    #[test]
    #[cfg(feature = "with-time")]
    fn test_time_timezone_value() {
        use time::macros::{date, offset, time};
        let timestamp = date!(2022 - 01 - 02)
            .with_time(time!(3:04:05))
            .assume_offset(offset!(+8));
        let value: Value = timestamp.into();
        let out: OffsetDateTime = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-time")]
    fn test_time_query() {
        use crate::*;
        use time::macros::datetime;

        let timestamp = datetime!(2020-01-01 02:02:02 +8);
        let query = Query::select().expr(Expr::val(timestamp)).to_owned();
        let formatted = "2020-01-01 02:02:02 +0800";

        assert_eq!(
            query.to_string(MysqlQueryBuilder),
            format!("SELECT '{}'", formatted)
        );
        assert_eq!(
            query.to_string(PostgresQueryBuilder),
            format!("SELECT '{}'", formatted)
        );
        assert_eq!(
            query.to_string(SqliteQueryBuilder),
            format!("SELECT '{}'", formatted)
        );
    }

    #[test]
    #[cfg(feature = "with-uuid")]
    fn test_uuid_value() {
        let uuid = uuid::Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
        let value: Value = uuid.into();
        let out: uuid::Uuid = value.unwrap();
        assert_eq!(out, uuid);
    }

    #[test]
    #[cfg(feature = "with-rust_decimal")]
    fn test_decimal_value() {
        use std::str::FromStr;

        let num = "2.02";
        let val = Decimal::from_str(num).unwrap();
        let v: Value = val.into();
        let out: Decimal = v.unwrap();
        assert_eq!(out.to_string(), num);
    }
}
