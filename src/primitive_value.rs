//! Container for all SQL value types.

#[cfg(feature = "with-json")]
use serde_json::Value as Json;
#[cfg(feature = "with-json")]
use std::str::from_utf8;

#[cfg(feature = "with-chrono")]
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};

#[cfg(feature = "with-rust_decimal")]
use rust_decimal::Decimal;

#[cfg(feature = "with-bigdecimal")]
use bigdecimal::BigDecimal;

#[cfg(feature = "with-uuid")]
use uuid::Uuid;

/// Value variants
///
/// We want Value to be exactly 1 pointer sized, so anything larger should be boxed.
#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveValue {
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

    #[allow(clippy::box_vec)]
    Bytes(Option<Box<Vec<u8>>>),

    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json(Option<Box<Json>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    Date(Option<Box<NaiveDate>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    Time(Option<Box<NaiveTime>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    DateTime(Option<Box<NaiveDateTime>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    DateTimeWithTimeZone(Option<Box<DateTime<FixedOffset>>>),

    #[cfg(feature = "with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid(Option<Box<Uuid>>),

    #[cfg(feature = "with-rust_decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
    Decimal(Option<Box<Decimal>>),

    #[cfg(feature = "with-bigdecimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
    BigDecimal(Option<Box<BigDecimal>>),
}

pub trait ValueType: Sized {
    fn try_from(v: PrimitiveValue) -> Result<Self, ValueTypeErr>;

    fn unwrap(v: PrimitiveValue) -> Self {
        Self::try_from(v).unwrap()
    }

    fn type_name() -> String;
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
pub struct Values(pub Vec<PrimitiveValue>);

#[derive(Debug, PartialEq)]
pub enum ValueTuple {
    One(PrimitiveValue),
    Two(PrimitiveValue, PrimitiveValue),
    Three(PrimitiveValue, PrimitiveValue, PrimitiveValue),
}

pub trait IntoValueTuple {
    fn into_value_tuple(self) -> ValueTuple;
}

pub trait Nullable {
    fn null() -> PrimitiveValue;
}

impl PrimitiveValue {
    pub fn unwrap<T>(self) -> T
    where
        T: ValueType,
    {
        T::unwrap(self)
    }
}

macro_rules! type_to_value {
    ( $type: ty, $name: ident ) => {
        impl From<$type> for PrimitiveValue {
            fn from(x: $type) -> PrimitiveValue {
                PrimitiveValue::$name(Some(x))
            }
        }

        impl Nullable for $type {
            fn null() -> PrimitiveValue {
                PrimitiveValue::$name(None)
            }
        }

        impl ValueType for $type {
            fn try_from(v: PrimitiveValue) -> Result<Self, ValueTypeErr> {
                match v {
                    PrimitiveValue::$name(Some(x)) => Ok(x),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($type).to_owned()
            }
        }
    };
}

macro_rules! type_to_box_value {
    ( $type: ty, $name: ident ) => {
        impl From<$type> for PrimitiveValue {
            fn from(x: $type) -> PrimitiveValue {
                PrimitiveValue::$name(Some(Box::new(x)))
            }
        }

        impl Nullable for $type {
            fn null() -> PrimitiveValue {
                PrimitiveValue::$name(None)
            }
        }

        impl ValueType for $type {
            fn try_from(v: PrimitiveValue) -> Result<Self, ValueTypeErr> {
                match v {
                    PrimitiveValue::$name(Some(x)) => Ok(*x),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($type).to_owned()
            }
        }
    };
}

type_to_value!(bool, Bool);
type_to_value!(i8, TinyInt);
type_to_value!(i16, SmallInt);
type_to_value!(i32, Int);
type_to_value!(i64, BigInt);
type_to_value!(u8, TinyUnsigned);
type_to_value!(u16, SmallUnsigned);
type_to_value!(u32, Unsigned);
type_to_value!(u64, BigUnsigned);
type_to_value!(f32, Float);
type_to_value!(f64, Double);

impl<'a> From<&'a [u8]> for PrimitiveValue {
    fn from(x: &'a [u8]) -> PrimitiveValue {
        PrimitiveValue::Bytes(Some(Box::<Vec<u8>>::new(x.into())))
    }
}

impl<'a> From<&'a str> for PrimitiveValue {
    fn from(x: &'a str) -> PrimitiveValue {
        let string: String = x.into();
        PrimitiveValue::String(Some(Box::new(string)))
    }
}

impl<'a> Nullable for &'a str {
    fn null() -> PrimitiveValue {
        PrimitiveValue::String(None)
    }
}

impl<T> From<Option<T>> for PrimitiveValue
where
    T: Into<PrimitiveValue> + Nullable,
{
    fn from(x: Option<T>) -> PrimitiveValue {
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
    fn try_from(v: PrimitiveValue) -> Result<Self, ValueTypeErr> {
        if v == T::null() {
            Ok(None)
        } else {
            Ok(Some(T::try_from(v)?))
        }
    }

    fn type_name() -> String {
        format!("Option<{}>", T::type_name())
    }
}

type_to_box_value!(Vec<u8>, Bytes);
type_to_box_value!(String, String);

#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
mod with_json {
    use super::*;

    type_to_box_value!(Json, Json);
}

#[cfg(feature = "with-chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
mod with_chrono {
    use super::*;
    use chrono::{Offset, TimeZone};

    type_to_box_value!(NaiveDate, Date);
    type_to_box_value!(NaiveTime, Time);
    type_to_box_value!(NaiveDateTime, DateTime);

    impl<Tz> From<DateTime<Tz>> for PrimitiveValue
    where
        Tz: TimeZone,
    {
        fn from(x: DateTime<Tz>) -> PrimitiveValue {
            let v = DateTime::<FixedOffset>::from_utc(x.naive_utc(), x.offset().fix());
            PrimitiveValue::DateTimeWithTimeZone(Some(Box::new(v)))
        }
    }

    impl Nullable for DateTime<FixedOffset> {
        fn null() -> PrimitiveValue {
            PrimitiveValue::DateTimeWithTimeZone(None)
        }
    }

    impl ValueType for DateTime<FixedOffset> {
        fn try_from(v: PrimitiveValue) -> Result<Self, ValueTypeErr> {
            match v {
                PrimitiveValue::DateTimeWithTimeZone(Some(x)) => Ok(*x),
                _ => Err(ValueTypeErr),
            }
        }

        fn type_name() -> String {
            stringify!(DateTime<FixedOffset>).to_owned()
        }
    }
}

#[cfg(feature = "with-rust_decimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
mod with_rust_decimal {
    use super::*;

    type_to_box_value!(Decimal, Decimal);
}

#[cfg(feature = "with-bigdecimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
mod with_bigdecimal {
    use super::*;

    type_to_box_value!(BigDecimal, BigDecimal);
}

#[cfg(feature = "with-uuid")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
mod with_uuid {
    use super::*;

    type_to_box_value!(Uuid, Uuid);
}

impl PrimitiveValue {
    pub fn is_json(&self) -> bool {
        #[cfg(feature = "with-json")]
        return matches!(self, Self::Json(_));
        #[cfg(not(feature = "with-json"))]
        return false;
    }
    #[cfg(feature = "with-json")]
    pub fn as_ref_json(&self) -> &Json {
        match self {
            Self::Json(Some(v)) => v.as_ref(),
            _ => panic!("not Value::Json"),
        }
    }
    #[cfg(not(feature = "with-json"))]
    pub fn as_ref_json(&self) -> &bool {
        panic!("not Value::Json")
    }
}

impl PrimitiveValue {
    pub fn is_date(&self) -> bool {
        #[cfg(feature = "with-chrono")]
        return matches!(self, Self::Date(_));
        #[cfg(not(feature = "with-chrono"))]
        return false;
    }
    #[cfg(feature = "with-chrono")]
    pub fn as_ref_date(&self) -> &NaiveDate {
        match self {
            Self::Date(Some(v)) => v.as_ref(),
            _ => panic!("not Value::Date"),
        }
    }
    #[cfg(not(feature = "with-chrono"))]
    pub fn as_ref_date(&self) -> &bool {
        panic!("not Value::Date")
    }
}

impl PrimitiveValue {
    pub fn is_time(&self) -> bool {
        #[cfg(feature = "with-chrono")]
        return matches!(self, Self::Time(_));
        #[cfg(not(feature = "with-chrono"))]
        return false;
    }
    #[cfg(feature = "with-chrono")]
    pub fn as_ref_time(&self) -> &NaiveTime {
        match self {
            Self::Time(Some(v)) => v.as_ref(),
            _ => panic!("not Value::Time"),
        }
    }
    #[cfg(not(feature = "with-chrono"))]
    pub fn as_ref_time(&self) -> &bool {
        panic!("not Value::Time")
    }
}

impl PrimitiveValue {
    pub fn is_date_time(&self) -> bool {
        #[cfg(feature = "with-chrono")]
        return matches!(self, Self::DateTime(_));
        #[cfg(not(feature = "with-chrono"))]
        return false;
    }
    #[cfg(feature = "with-chrono")]
    pub fn as_ref_date_time(&self) -> &NaiveDateTime {
        match self {
            Self::DateTime(Some(v)) => v.as_ref(),
            _ => panic!("not Value::DateTime"),
        }
    }
    #[cfg(not(feature = "with-chrono"))]
    pub fn as_ref_date_time(&self) -> &bool {
        panic!("not Value::DateTime")
    }
}

impl PrimitiveValue {
    pub fn is_date_time_with_time_zone(&self) -> bool {
        #[cfg(feature = "with-chrono")]
        return matches!(self, Self::DateTimeWithTimeZone(_));
        #[cfg(not(feature = "with-chrono"))]
        return false;
    }
    #[cfg(feature = "with-chrono")]
    pub fn as_ref_date_time_with_time_zone(&self) -> &DateTime<FixedOffset> {
        match self {
            Self::DateTimeWithTimeZone(Some(v)) => v.as_ref(),
            _ => panic!("not Value::DateTimeWithTimeZone"),
        }
    }
    #[cfg(not(feature = "with-chrono"))]
    pub fn as_ref_date_time_with_time_zone(&self) -> &bool {
        panic!("not Value::DateTimeWithTimeZone")
    }
}

impl PrimitiveValue {
    pub fn is_decimal(&self) -> bool {
        #[cfg(feature = "with-rust_decimal")]
        return matches!(self, Self::Decimal(_));
        #[cfg(not(feature = "with-rust_decimal"))]
        return false;
    }
    #[cfg(feature = "with-rust_decimal")]
    pub fn as_ref_decimal(&self) -> &Decimal {
        match self {
            Self::Decimal(Some(v)) => v.as_ref(),
            _ => panic!("not Value::Decimal"),
        }
    }
    #[cfg(feature = "with-rust_decimal")]
    pub fn decimal_to_f64(&self) -> f64 {
        use rust_decimal::prelude::ToPrimitive;
        self.as_ref_decimal().to_f64().unwrap()
    }
    #[cfg(not(feature = "with-rust_decimal"))]
    pub fn as_ref_decimal(&self) -> &bool {
        panic!("not Value::Decimal")
    }
    #[cfg(not(feature = "with-rust_decimal"))]
    pub fn decimal_to_f64(&self) -> f64 {
        0.0
    }
}

impl PrimitiveValue {
    pub fn is_big_decimal(&self) -> bool {
        #[cfg(feature = "with-bigdecimal")]
        return matches!(self, Self::BigDecimal(_));
        #[cfg(not(feature = "with-bigdecimal"))]
        return false;
    }
    #[cfg(feature = "with-bigdecimal")]
    pub fn as_ref_big_decimal(&self) -> &BigDecimal {
        match self {
            Self::BigDecimal(Some(v)) => v.as_ref(),
            _ => panic!("not Value::BigDecimal"),
        }
    }
    #[cfg(feature = "with-bigdecimal")]
    pub fn big_decimal_to_f64(&self) -> f64 {
        use bigdecimal::ToPrimitive;
        self.as_ref_big_decimal().to_f64().unwrap()
    }
    #[cfg(not(feature = "with-bigdecimal"))]
    pub fn as_ref_big_decimal(&self) -> &bool {
        panic!("not Value::BigDecimal")
    }
    #[cfg(not(feature = "with-bigdecimal"))]
    pub fn big_decimal_to_f64(&self) -> f64 {
        0.0
    }
}

impl PrimitiveValue {
    pub fn is_uuid(&self) -> bool {
        #[cfg(feature = "with-uuid")]
        return matches!(self, Self::Uuid(_));
        #[cfg(not(feature = "with-uuid"))]
        return false;
    }
    #[cfg(feature = "with-uuid")]
    pub fn as_ref_uuid(&self) -> &Uuid {
        match self {
            Self::Uuid(Some(v)) => v.as_ref(),
            _ => panic!("not Value::Uuid"),
        }
    }
    #[cfg(not(feature = "with-uuid"))]
    pub fn as_ref_uuid(&self) -> &bool {
        panic!("not Value::Uuid")
    }
}

impl IntoIterator for ValueTuple {
    type Item = PrimitiveValue;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ValueTuple::One(v) => vec![v].into_iter(),
            ValueTuple::Two(v, w) => vec![v, w].into_iter(),
            ValueTuple::Three(u, v, w) => vec![u, v, w].into_iter(),
        }
    }
}

impl<V> IntoValueTuple for V
where
    V: Into<PrimitiveValue>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::One(self.into())
    }
}

impl<V, W> IntoValueTuple for (V, W)
where
    V: Into<PrimitiveValue>,
    W: Into<PrimitiveValue>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::Two(self.0.into(), self.1.into())
    }
}

impl<U, V, W> IntoValueTuple for (U, V, W)
where
    U: Into<PrimitiveValue>,
    V: Into<PrimitiveValue>,
    W: Into<PrimitiveValue>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::Three(self.0.into(), self.1.into(), self.2.into())
    }
}

/// Convert value to json value
#[allow(clippy::many_single_char_names)]
#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
#[allow(deprecated)]
pub fn sea_value_to_json_value(value: &PrimitiveValue) -> Json {
    use crate::{CommonSqlQueryBuilder, QueryBuilder};

    match value {
        PrimitiveValue::Bool(None)
        | PrimitiveValue::TinyInt(None)
        | PrimitiveValue::SmallInt(None)
        | PrimitiveValue::Int(None)
        | PrimitiveValue::BigInt(None)
        | PrimitiveValue::TinyUnsigned(None)
        | PrimitiveValue::SmallUnsigned(None)
        | PrimitiveValue::Unsigned(None)
        | PrimitiveValue::BigUnsigned(None)
        | PrimitiveValue::Float(None)
        | PrimitiveValue::Double(None)
        | PrimitiveValue::String(None)
        | PrimitiveValue::Bytes(None)
        | PrimitiveValue::Json(None) => Json::Null,
        #[cfg(feature = "with-rust_decimal")]
        PrimitiveValue::Decimal(None) => Json::Null,
        #[cfg(feature = "with-bigdecimal")]
        PrimitiveValue::BigDecimal(None) => Json::Null,
        #[cfg(feature = "with-uuid")]
        PrimitiveValue::Uuid(None) => Json::Null,
        PrimitiveValue::Bool(Some(b)) => Json::Bool(*b),
        PrimitiveValue::TinyInt(Some(v)) => (*v).into(),
        PrimitiveValue::SmallInt(Some(v)) => (*v).into(),
        PrimitiveValue::Int(Some(v)) => (*v).into(),
        PrimitiveValue::BigInt(Some(v)) => (*v).into(),
        PrimitiveValue::TinyUnsigned(Some(v)) => (*v).into(),
        PrimitiveValue::SmallUnsigned(Some(v)) => (*v).into(),
        PrimitiveValue::Unsigned(Some(v)) => (*v).into(),
        PrimitiveValue::BigUnsigned(Some(v)) => (*v).into(),
        PrimitiveValue::Float(Some(v)) => (*v).into(),
        PrimitiveValue::Double(Some(v)) => (*v).into(),
        PrimitiveValue::String(Some(s)) => Json::String(s.as_ref().clone()),
        PrimitiveValue::Bytes(Some(s)) => Json::String(from_utf8(s).unwrap().to_string()),
        PrimitiveValue::Json(Some(v)) => v.as_ref().clone(),
        #[cfg(feature = "with-chrono")]
        PrimitiveValue::Date(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        PrimitiveValue::Time(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        PrimitiveValue::DateTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        PrimitiveValue::DateTimeWithTimeZone(_) => {
            CommonSqlQueryBuilder.value_to_string(value).into()
        }
        #[cfg(feature = "with-rust_decimal")]
        PrimitiveValue::Decimal(Some(v)) => {
            use rust_decimal::prelude::ToPrimitive;
            v.as_ref().to_f64().unwrap().into()
        }
        #[cfg(feature = "with-bigdecimal")]
        PrimitiveValue::BigDecimal(Some(v)) => {
            use bigdecimal::ToPrimitive;
            v.as_ref().to_f64().unwrap().into()
        }
        #[cfg(feature = "with-uuid")]
        PrimitiveValue::Uuid(Some(v)) => Json::String(v.to_string()),
    }
}

impl Values {
    pub fn iter(&self) -> impl Iterator<Item = &PrimitiveValue> {
        self.0.iter()
    }
}
