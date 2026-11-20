use super::*;
use jiff_1::{Timestamp, civil};

type_to_value!(civil::Date, Jiff1Date, Date);
type_to_value!(civil::Time, Jiff1Time, Time);
type_to_box_value!(civil::DateTime, Jiff1DateTime, DateTime);
type_to_box_value!(Timestamp, Jiff1Timestamp, TimestampWithTimeZone);

impl DateLikeValue for civil::Date {}
impl TimeLikeValue for civil::Time {}
impl DateTimeLikeValue for civil::DateTime {}
impl DateTimeLikeValue for Timestamp {}

impl DateLikeValueNullable for Option<civil::Date> {}
impl TimeLikeValueNullable for Option<civil::Time> {}
impl DateTimeLikeValueNullable for Option<civil::DateTime> {}
impl DateTimeLikeValueNullable for Option<Timestamp> {}

impl Value {
    #[inline]
    pub fn jiff_1_date<T: Into<Option<civil::Date>>>(v: T) -> Value {
        Value::Jiff1Date(v.into())
    }

    #[inline]
    pub fn jiff_1_time<T: Into<Option<civil::Time>>>(v: T) -> Value {
        Value::Jiff1Time(v.into())
    }

    #[inline]
    pub fn jiff_1_date_time<T: Into<Option<civil::DateTime>>>(v: T) -> Value {
        Value::Jiff1DateTime(v.into().map(Into::into))
    }

    #[inline]
    pub fn jiff_1_timestamp<T: Into<Option<Timestamp>>>(v: T) -> Value {
        Value::Jiff1Timestamp(v.into().map(Into::into))
    }
}

impl Value {
    pub fn is_jiff_1_date(&self) -> bool {
        matches!(self, Self::Jiff1Date(_))
    }

    pub fn is_jiff_1_time(&self) -> bool {
        matches!(self, Self::Jiff1Time(_))
    }

    pub fn is_jiff_1_date_time(&self) -> bool {
        matches!(self, Self::Jiff1DateTime(_))
    }

    pub fn is_jiff_1_timestamp(&self) -> bool {
        matches!(self, Self::Jiff1Timestamp(_))
    }

    pub fn as_ref_jiff_1_date(&self) -> Option<&civil::Date> {
        match self {
            Self::Jiff1Date(v) => v.as_ref(),
            _ => panic!("not Value::Jiff1Date"),
        }
    }

    pub fn as_ref_jiff_1_time(&self) -> Option<&civil::Time> {
        match self {
            Self::Jiff1Time(v) => v.as_ref(),
            _ => panic!("not Value::Jiff1Time"),
        }
    }

    pub fn as_ref_jiff_1_date_time(&self) -> Option<&civil::DateTime> {
        match self {
            Self::Jiff1DateTime(v) => v.as_deref(),
            _ => panic!("not Value::Jiff1DateTime"),
        }
    }

    pub fn as_ref_jiff_1_timestamp(&self) -> Option<&Timestamp> {
        match self {
            Self::Jiff1Timestamp(v) => v.as_deref(),
            _ => panic!("not Value::Jiff1Timestamp"),
        }
    }
}
