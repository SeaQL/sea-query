use super::*;
use jiff::{Timestamp, civil};

type_to_value!(civil::Date, JiffDate, Date);
type_to_value!(civil::Time, JiffTime, Time);
type_to_box_value!(civil::DateTime, JiffDateTime, Timestamp);
type_to_box_value!(Timestamp, JiffTimestamp, TimestampWithTimeZone);

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
    pub fn jiff_date<T: Into<Option<civil::Date>>>(v: T) -> Value {
        Value::JiffDate(v.into())
    }

    #[inline]
    pub fn jiff_time<T: Into<Option<civil::Time>>>(v: T) -> Value {
        Value::JiffTime(v.into())
    }

    #[inline]
    pub fn jiff_date_time<T: Into<Option<civil::DateTime>>>(v: T) -> Value {
        Value::JiffDateTime(v.into().map(Into::into))
    }

    #[inline]
    pub fn jiff_timestamp<T: Into<Option<Timestamp>>>(v: T) -> Value {
        Value::JiffTimestamp(v.into().map(Into::into))
    }
}

impl Value {
    pub fn is_jiff_date(&self) -> bool {
        matches!(self, Self::JiffDate(_))
    }

    pub fn is_jiff_time(&self) -> bool {
        matches!(self, Self::JiffTime(_))
    }

    pub fn is_jiff_date_time(&self) -> bool {
        matches!(self, Self::JiffDateTime(_))
    }

    pub fn is_jiff_timestamp(&self) -> bool {
        matches!(self, Self::JiffTimestamp(_))
    }

    pub fn as_ref_jiff_date(&self) -> Option<&civil::Date> {
        match self {
            Self::JiffDate(v) => v.as_ref(),
            _ => panic!("not Value::JiffDate"),
        }
    }

    pub fn as_ref_jiff_time(&self) -> Option<&civil::Time> {
        match self {
            Self::JiffTime(v) => v.as_ref(),
            _ => panic!("not Value::JiffTime"),
        }
    }

    pub fn as_ref_jiff_date_time(&self) -> Option<&civil::DateTime> {
        match self {
            Self::JiffDateTime(v) => v.as_deref(),
            _ => panic!("not Value::JiffDateTime"),
        }
    }

    pub fn as_ref_jiff_timestamp(&self) -> Option<&Timestamp> {
        match self {
            Self::JiffTimestamp(v) => v.as_deref(),
            _ => panic!("not Value::JiffTimestamp"),
        }
    }
}

impl Value {
    #[cfg(test)]
    pub(crate) fn jiff_value_to_string(&self) -> Option<String> {
        match self {
            Self::JiffDate(v) => v.as_ref().map(|v| v.to_string()),
            Self::JiffTime(v) => v.as_ref().map(|v| v.to_string()),
            Self::JiffDateTime(v) => v.as_ref().map(|v| v.to_string()),
            Self::JiffTimestamp(v) => v.as_ref().map(|v| v.to_string()),
            _ => panic!("not jiff Value"),
        }
    }
}
