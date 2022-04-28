//! Container for all SQL value types.
use std::fmt::Write;

#[cfg(feature = "with-json")]
use serde_json::Value as Json;
#[cfg(feature = "with-json")]
use std::str::from_utf8;

#[cfg(feature = "with-chrono")]
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};

#[cfg(feature = "with-time")]
use time::{offset, OffsetDateTime, PrimitiveDateTime};

#[cfg(feature = "with-rust_decimal")]
use rust_decimal::Decimal;

#[cfg(feature = "with-bigdecimal")]
use bigdecimal::BigDecimal;

#[cfg(feature = "with-uuid")]
use uuid::Uuid;

use crate::ColumnType;

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

    #[allow(clippy::box_collection)]
    Bytes(Option<Box<Vec<u8>>>),

    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json(Option<Box<Json>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDate(Option<Box<NaiveDate>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoTime(Option<Box<NaiveTime>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTime(Option<Box<NaiveDateTime>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeUtc(Option<Box<DateTime<Utc>>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeLocal(Option<Box<DateTime<Local>>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeWithTimeZone(Option<Box<DateTime<FixedOffset>>>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDate(Option<Box<time::Date>>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeTime(Option<Box<time::Time>>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTime(Option<Box<PrimitiveDateTime>>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTimeWithTimeZone(Option<Box<OffsetDateTime>>),

    #[cfg(feature = "with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid(Option<Box<Uuid>>),

    #[cfg(feature = "with-rust_decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
    Decimal(Option<Box<Decimal>>),

    #[cfg(feature = "with-bigdecimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
    BigDecimal(Option<Box<BigDecimal>>),

    #[cfg(feature = "postgres-array")]
    #[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
    Array(Option<Box<Vec<Value>>>),
}

#[cfg(feature = "oracle-overrides")]
pub mod oracle_overrides {
    use super::*;
    use r2d2_oracle::oracle::sql_type::ToSql;

    // /// 2021-07-14T23:59:59
    // const CLIENT_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
    // const DB_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    // pub fn to_oracle_format(time: chrono::NaiveDateTime) -> String {
    //     format!(
    //         r#"to_date('{time}', 'YYYY-MM-DD HH24:MI:SS')"#,
    //         time = time.format(DB_DATETIME_FORMAT),
    //     )
    // }

    impl ToSql for Value {
        fn oratype(
            &self,
            conn: &r2d2_oracle::oracle::Connection,
        ) -> r2d2_oracle::oracle::Result<r2d2_oracle::oracle::sql_type::OracleType> {
            match self {
                Value::Bool(v) => v.oratype(conn),
                Value::TinyInt(v) => v.oratype(conn),
                Value::SmallInt(v) => v.oratype(conn),
                Value::Int(v) => v.oratype(conn),
                Value::BigInt(v) => v.oratype(conn),
                Value::TinyUnsigned(v) => v.oratype(conn),
                Value::SmallUnsigned(v) => v.oratype(conn),
                Value::Unsigned(v) => v.oratype(conn),
                Value::BigUnsigned(v) => v.oratype(conn),
                Value::Float(v) => v.oratype(conn),
                Value::Double(v) => v.oratype(conn),
                Value::String(v) => v.as_ref().map(|v| v.as_ref().clone()).oratype(conn),
                Value::Bytes(v) => v.as_ref().map(|v| v.as_ref().clone()).oratype(conn),
                // #[cfg(feature = "with-json")]
                // Value::Json(v) => v.map(|v| v.as_ref().clone()).oratype(conn),
                Value::ChronoDate(v) => v.as_ref().map(|v| *v.as_ref()).oratype(conn),
                // Value::ChronoTime(v) => v.map(|v| v.as_ref().clone()).oratype(conn),
                Value::ChronoDateTime(v) => v.as_ref().map(|v| *v.as_ref()).oratype(conn),
                Value::ChronoDateTimeUtc(v) => v.as_ref().map(|v| *v.as_ref()).oratype(conn),
                Value::ChronoDateTimeLocal(v) => v.as_ref().map(|v| *v.as_ref()).oratype(conn),
                Value::ChronoDateTimeWithTimeZone(v) => {
                    v.as_ref().map(|v| *v.as_ref()).oratype(conn)
                }
                // Value::TimeDate(v) => v.map(|v| v.as_ref().clone()).oratype(conn),
                // Value::TimeTime(v) => v.map(|v| v.as_ref().clone()).oratype(conn),
                // Value::TimeDateTime(v) => v.map(|v| v.as_ref().clone()).oratype(conn),
                // Value::TimeDateTimeWithTimeZone(v) => v.map(|v| v.as_ref().clone()).oratype(conn),
                // Value::Uuid(v) => v.map(|v| v.as_ref().clone()).oratype(conn),
                // #[cfg(feature = "with-rust_decimal")]
                // Value::Decimal(v) => v.map(|v| v.as_ref().clone().into()).oratype(conn),
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(v) => v
                    .as_ref()
                    .map(|v| v.as_ref().clone().to_string()) // TODO: this is probably wrong
                    .oratype(conn),
                v => unimplemented!("{v:?}"),
            }
        }

        fn to_sql(
            &self,
            conn: &mut r2d2_oracle::oracle::SqlValue,
        ) -> r2d2_oracle::oracle::Result<()> {
            match self {
                Value::Bool(v) => v.to_sql(conn),
                Value::TinyInt(v) => v.to_sql(conn),
                Value::SmallInt(v) => v.to_sql(conn),
                Value::Int(v) => v.to_sql(conn),
                Value::BigInt(v) => v.to_sql(conn),
                Value::TinyUnsigned(v) => v.to_sql(conn),
                Value::SmallUnsigned(v) => v.to_sql(conn),
                Value::Unsigned(v) => v.to_sql(conn),
                Value::BigUnsigned(v) => v.to_sql(conn),
                Value::Float(v) => v.to_sql(conn),
                Value::Double(v) => v.to_sql(conn),
                Value::String(v) => v.as_ref().map(|v| v.as_ref().clone()).to_sql(conn),
                Value::Bytes(v) => v.as_ref().map(|v| v.as_ref().clone()).to_sql(conn),
                // #[cfg(feature = "with-json")]
                // Value::Json(v) => v.map(|v| v.as_ref().clone()).to_sql(conn),
                Value::ChronoDate(v) => v.as_ref().map(|v| *v.as_ref()).to_sql(conn),
                // Value::ChronoTime(v) => v.map(|v| v.as_ref().clone()).to_sql(conn),
                Value::ChronoDateTime(v) => v.as_ref().map(|v| *v.as_ref()).to_sql(conn),
                Value::ChronoDateTimeUtc(v) => v.as_ref().map(|v| *v.as_ref()).to_sql(conn),
                Value::ChronoDateTimeLocal(v) => v.as_ref().map(|v| *v.as_ref()).to_sql(conn),
                Value::ChronoDateTimeWithTimeZone(v) => {
                    v.as_ref().map(|v| *v.as_ref()).to_sql(conn)
                }
                // Value::TimeDate(v) => v.map(|v| v.as_ref().clone()).to_sql(conn),
                // Value::TimeTime(v) => v.map(|v| v.as_ref().clone()).to_sql(conn),
                // Value::TimeDateTime(v) => v.map(|v| v.as_ref().clone()).to_sql(conn),
                // Value::TimeDateTimeWithTimeZone(v) => v.map(|v| v.as_ref().clone()).to_sql(conn),
                // Value::Uuid(v) => v.map(|v| v.as_ref().clone()).to_sql(conn),
                // #[cfg(feature = "with-rust_decimal")]
                // Value::Decimal(v) => v.map(|v| v.as_ref().clone().into()).to_sql(conn),
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(v) => v
                    .as_ref()
                    .map(|v| v.as_ref().clone().to_string()) // TODO: this is probably wrong
                    .to_sql(conn),
                v => unimplemented!("{v:?}"),
            }
        }
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

#[derive(Debug, PartialEq)]
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

type_to_box_value!(Vec<u8>, Bytes, Binary(None));
type_to_box_value!(String, String, String(None));

#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
mod with_json {
    use super::*;

    type_to_box_value!(Json, Json, Json);
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
}

#[cfg(feature = "with-rust_decimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
mod with_rust_decimal {
    use super::*;

    type_to_box_value!(Decimal, Decimal, Decimal(None));
}

#[cfg(feature = "with-bigdecimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
mod with_bigdecimal {
    use super::*;

    type_to_box_value!(BigDecimal, BigDecimal, Decimal(None));
}

#[cfg(feature = "with-uuid")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
mod with_uuid {
    use super::*;

    type_to_box_value!(Uuid, Uuid, Uuid);
}

#[cfg(feature = "postgres-array")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
mod with_array {
    use super::*;

    // We only imlement conversion from Vec<T> to Array when T is not u8.
    // This is because for u8's case, there is already conversion to Byte defined above.
    // TODO When negative trait becomes a stable feature, following code can be much shorter.
    pub trait NotU8 {}

    impl NotU8 for bool {}
    impl NotU8 for i8 {}
    impl NotU8 for i16 {}
    impl NotU8 for i32 {}
    impl NotU8 for i64 {}
    impl NotU8 for u16 {}
    impl NotU8 for u32 {}
    impl NotU8 for u64 {}
    impl NotU8 for f32 {}
    impl NotU8 for f64 {}
    impl NotU8 for String {}

    #[cfg(feature = "with-json")]
    impl NotU8 for Json {}

    #[cfg(feature = "with-chrono")]
    impl NotU8 for NaiveDate {}

    #[cfg(feature = "with-chrono")]
    impl NotU8 for NaiveTime {}

    #[cfg(feature = "with-chrono")]
    impl NotU8 for NaiveDateTime {}

    #[cfg(feature = "with-chrono")]
    impl<Tz> NotU8 for DateTime<Tz> where Tz: chrono::TimeZone {}

    #[cfg(feature = "with-time")]
    impl NotU8 for time::Date {}

    #[cfg(feature = "with-time")]
    impl NotU8 for time::Time {}

    #[cfg(feature = "with-time")]
    impl NotU8 for PrimitiveDateTime {}

    #[cfg(feature = "with-time")]
    impl NotU8 for OffsetDateTime {}

    #[cfg(feature = "with-rust_decimal")]
    impl NotU8 for Decimal {}

    #[cfg(feature = "with-bigdecimal")]
    impl NotU8 for BigDecimal {}

    #[cfg(feature = "with-uuid")]
    impl NotU8 for Uuid {}

    impl<T> From<Vec<T>> for Value
    where
        T: Into<Value> + NotU8,
    {
        fn from(x: Vec<T>) -> Value {
            Value::Array(Some(Box::new(x.into_iter().map(|e| e.into()).collect())))
        }
    }

    impl<T> Nullable for Vec<T>
    where
        T: Into<Value> + NotU8,
    {
        fn null() -> Value {
            Value::Array(None)
        }
    }

    impl<T> ValueType for Vec<T>
    where
        T: NotU8 + ValueType,
    {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            match v {
                Value::Array(Some(v)) => Ok(v.into_iter().map(|e| e.unwrap()).collect()),
                _ => Err(ValueTypeErr),
            }
        }

        fn type_name() -> String {
            stringify!(Vec<T>).to_owned()
        }

        fn column_type() -> ColumnType {
            use ColumnType::*;
            Array(None)
        }
    }
}

#[allow(unused_macros)]
macro_rules! box_to_opt_ref {
    ( $v: expr ) => {
        match $v {
            Some(v) => Some(v.as_ref()),
            None => None,
        }
    };
}

#[cfg(feature = "with-json")]
impl Value {
    pub fn is_json(&self) -> bool {
        matches!(self, Self::Json(_))
    }

    pub fn as_ref_json(&self) -> Option<&Json> {
        match self {
            Self::Json(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::Json"),
        }
    }
}

#[cfg(feature = "with-chrono")]
impl Value {
    pub fn is_chrono_date(&self) -> bool {
        matches!(self, Self::ChronoDate(_))
    }

    pub fn as_ref_chrono_date(&self) -> Option<&NaiveDate> {
        match self {
            Self::ChronoDate(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::ChronoDate"),
        }
    }
}

#[cfg(feature = "with-time")]
impl Value {
    pub fn is_time_date(&self) -> bool {
        matches!(self, Self::TimeDate(_))
    }

    pub fn as_ref_time_date(&self) -> Option<&time::Date> {
        match self {
            Self::TimeDate(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::TimeDate"),
        }
    }
}

#[cfg(feature = "with-chrono")]
impl Value {
    pub fn is_chrono_time(&self) -> bool {
        matches!(self, Self::ChronoTime(_))
    }

    pub fn as_ref_chrono_time(&self) -> Option<&NaiveTime> {
        match self {
            Self::ChronoTime(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::ChronoTime"),
        }
    }
}

#[cfg(feature = "with-time")]
impl Value {
    pub fn is_time_time(&self) -> bool {
        matches!(self, Self::TimeTime(_))
    }

    pub fn as_ref_time_time(&self) -> Option<&time::Time> {
        match self {
            Self::TimeTime(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::TimeTime"),
        }
    }
}

#[cfg(feature = "with-chrono")]
impl Value {
    pub fn is_chrono_date_time(&self) -> bool {
        matches!(self, Self::ChronoDateTime(_))
    }

    pub fn as_ref_chrono_date_time(&self) -> Option<&NaiveDateTime> {
        match self {
            Self::ChronoDateTime(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::ChronoDateTime"),
        }
    }
}

#[cfg(feature = "with-time")]
impl Value {
    pub fn is_time_date_time(&self) -> bool {
        matches!(self, Self::TimeDateTime(_))
    }

    pub fn as_ref_time_date_time(&self) -> Option<&PrimitiveDateTime> {
        match self {
            Self::TimeDateTime(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::TimeDateTime"),
        }
    }
}

#[cfg(feature = "with-chrono")]
impl Value {
    pub fn is_chrono_date_time_utc(&self) -> bool {
        matches!(self, Self::ChronoDateTimeUtc(_))
    }

    pub fn as_ref_chrono_date_time_utc(&self) -> Option<&DateTime<Utc>> {
        match self {
            Self::ChronoDateTimeUtc(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::ChronoDateTimeUtc"),
        }
    }
}

#[cfg(feature = "with-chrono")]
impl Value {
    pub fn is_chrono_date_time_local(&self) -> bool {
        matches!(self, Self::ChronoDateTimeLocal(_))
    }

    pub fn as_ref_chrono_date_time_local(&self) -> Option<&DateTime<Local>> {
        match self {
            Self::ChronoDateTimeLocal(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::ChronoDateTimeLocal"),
        }
    }
}

#[cfg(feature = "with-chrono")]
impl Value {
    pub fn is_chrono_date_time_with_time_zone(&self) -> bool {
        matches!(self, Self::ChronoDateTimeWithTimeZone(_))
    }

    pub fn as_ref_chrono_date_time_with_time_zone(&self) -> Option<&DateTime<FixedOffset>> {
        match self {
            Self::ChronoDateTimeWithTimeZone(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::ChronoDateTimeWithTimeZone"),
        }
    }
}

#[cfg(feature = "with-time")]
impl Value {
    pub fn is_time_date_time_with_time_zone(&self) -> bool {
        matches!(self, Self::TimeDateTimeWithTimeZone(_))
    }

    pub fn as_ref_time_date_time_with_time_zone(&self) -> Option<&OffsetDateTime> {
        match self {
            Self::TimeDateTimeWithTimeZone(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::TimeDateTimeWithTimeZone"),
        }
    }
}

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

#[cfg(feature = "with-time")]
impl Value {
    pub fn time_as_naive_utc_in_string(&self) -> Option<String> {
        match self {
            Self::TimeDate(v) => v.as_ref().map(|v| v.format("%Y-%m-%d")),
            Self::TimeTime(v) => v.as_ref().map(|v| v.format("%H:%M:%S")),
            Self::TimeDateTime(v) => v.as_ref().map(|v| v.format("%Y-%m-%d %H:%M:%S")),
            Self::TimeDateTimeWithTimeZone(v) => v
                .as_ref()
                .map(|v| v.to_offset(offset!(+0)).format("%Y-%m-%d %H:%M:%S")),
            _ => panic!("not time Value"),
        }
    }
}

#[cfg(feature = "with-rust_decimal")]
impl Value {
    pub fn is_decimal(&self) -> bool {
        matches!(self, Self::Decimal(_))
    }
    pub fn as_ref_decimal(&self) -> Option<&Decimal> {
        match self {
            Self::Decimal(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::Decimal"),
        }
    }
    pub fn decimal_to_f64(&self) -> Option<f64> {
        use rust_decimal::prelude::ToPrimitive;
        self.as_ref_decimal().map(|d| d.to_f64().unwrap())
    }
}

#[cfg(feature = "with-bigdecimal")]
impl Value {
    pub fn is_big_decimal(&self) -> bool {
        matches!(self, Self::BigDecimal(_))
    }
    pub fn as_ref_big_decimal(&self) -> Option<&BigDecimal> {
        match self {
            Self::BigDecimal(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::BigDecimal"),
        }
    }
    pub fn big_decimal_to_f64(&self) -> Option<f64> {
        use bigdecimal::ToPrimitive;
        self.as_ref_big_decimal().map(|d| d.to_f64().unwrap())
    }
}

#[cfg(feature = "with-uuid")]
impl Value {
    pub fn is_uuid(&self) -> bool {
        matches!(self, Self::Uuid(_))
    }
    pub fn as_ref_uuid(&self) -> Option<&Uuid> {
        match self {
            Self::Uuid(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::Uuid"),
        }
    }
}

#[cfg(feature = "postgres-array")]
impl Value {
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn as_ref_array(&self) -> Option<&Vec<Value>> {
        match self {
            Self::Array(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::Array"),
        }
    }
}

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

/// Escape a SQL string literal
pub fn escape_string(string: &str) -> String {
    string
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\'', "\\'")
        .replace('\0', "\\0")
        .replace('\x08', "\\b")
        .replace('\x09', "\\t")
        .replace('\x1a', "\\z")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

/// Unescape a SQL string literal
pub fn unescape_string(input: &str) -> String {
    let mut escape = false;
    let mut output = String::new();
    for c in input.chars() {
        if !escape && c == '\\' {
            escape = true;
        } else if escape {
            write!(
                output,
                "{}",
                match c {
                    '0' => '\0',
                    'b' => '\x08',
                    't' => '\x09',
                    'z' => '\x1a',
                    'n' => '\n',
                    'r' => '\r',
                    c => c,
                }
            )
            .unwrap();
            escape = false;
        } else {
            write!(output, "{}", c).unwrap();
        }
    }
    output
}

/// Convert value to json value
#[allow(clippy::many_single_char_names)]
#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
pub fn sea_value_to_json_value(value: &Value) -> Json {
    use crate::{CommonSqlQueryBuilder, QueryBuilder};

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
        | Value::Bytes(None)
        | Value::Json(None) => Json::Null,
        #[cfg(feature = "with-rust_decimal")]
        Value::Decimal(None) => Json::Null,
        #[cfg(feature = "with-bigdecimal")]
        Value::BigDecimal(None) => Json::Null,
        #[cfg(feature = "with-uuid")]
        Value::Uuid(None) => Json::Null,
        #[cfg(feature = "postgres-array")]
        Value::Array(None) => Json::Null,
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
        Value::Bytes(Some(s)) => Json::String(from_utf8(s).unwrap().to_string()),
        Value::Json(Some(v)) => v.as_ref().clone(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDate(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeWithTimeZone(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeUtc(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeLocal(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeDate(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeDateTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeDateTimeWithTimeZone(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-rust_decimal")]
        Value::Decimal(Some(v)) => {
            use rust_decimal::prelude::ToPrimitive;
            v.as_ref().to_f64().unwrap().into()
        }
        #[cfg(feature = "with-bigdecimal")]
        Value::BigDecimal(Some(v)) => {
            use bigdecimal::ToPrimitive;
            v.as_ref().to_f64().unwrap().into()
        }
        #[cfg(feature = "with-uuid")]
        Value::Uuid(Some(v)) => Json::String(v.to_string()),
        #[cfg(feature = "postgres-array")]
        Value::Array(Some(v)) => {
            Json::Array(v.as_ref().iter().map(sea_value_to_json_value).collect())
        }
    }
}

impl Values {
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_1() {
        let test = r#" "abc" "#;
        assert_eq!(escape_string(test), r#" \"abc\" "#.to_owned());
        assert_eq!(unescape_string(escape_string(test).as_str()), test);
    }

    #[test]
    fn test_escape_2() {
        let test = "a\nb\tc";
        assert_eq!(escape_string(test), "a\\nb\\tc".to_owned());
        assert_eq!(unescape_string(escape_string(test).as_str()), test);
    }

    #[test]
    fn test_escape_3() {
        let test = "a\\b";
        assert_eq!(escape_string(test), "a\\\\b".to_owned());
        assert_eq!(unescape_string(escape_string(test).as_str()), test);
    }

    #[test]
    fn test_escape_4() {
        let test = "a\"b";
        assert_eq!(escape_string(test), "a\\\"b".to_owned());
        assert_eq!(unescape_string(escape_string(test).as_str()), test);
    }

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
        use time::{date, time};
        let timestamp = date!(2020 - 01 - 01).with_time(time!(2:2:2));
        let value: Value = timestamp.into();
        let out: PrimitiveDateTime = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-time")]
    fn test_time_utc_value() {
        use time::{date, time};
        let timestamp = date!(2022 - 01 - 02).with_time(time!(3:04:05)).assume_utc();
        let value: Value = timestamp.into();
        let out: OffsetDateTime = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-time")]
    fn test_time_local_value() {
        use time::{date, offset, time};
        let timestamp_utc = date!(2022 - 01 - 02).with_time(time!(3:04:05)).assume_utc();
        let timestamp_local: OffsetDateTime = timestamp_utc.to_offset(offset!(+3));
        let value: Value = timestamp_local.into();
        let out: OffsetDateTime = value.unwrap();
        assert_eq!(out, timestamp_local);
    }

    #[test]
    #[cfg(feature = "with-time")]
    fn test_time_timezone_value() {
        use time::{date, offset, time};
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

        let string = "2020-01-01 02:02:02 +0800";
        let timestamp = OffsetDateTime::parse(string, "%Y-%m-%d %H:%M:%S %z").unwrap();

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

    #[test]
    #[cfg(feature = "postgres-array")]
    fn test_array_value() {
        let array = vec![1, 2, 3, 4, 5];
        let v: Value = array.into();
        let out: Vec<i32> = v.unwrap();
        assert_eq!(out, vec![1, 2, 3, 4, 5]);
    }
}
