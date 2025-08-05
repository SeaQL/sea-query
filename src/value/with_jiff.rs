use super::*;
use jiff::{Timestamp, Zoned, civil};

type_to_value!(civil::Date, JiffDate, Date);
type_to_value!(civil::Time, JiffTime, Time);
type_to_box_value!(civil::DateTime, JiffDateTime, DateTime);
type_to_box_value!(Timestamp, JiffTimestamp, Timestamp);
type_to_box_value!(Zoned, JiffZoned, TimestampWithTimeZone);

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

    #[inline]
    pub fn jiff_zoned<T: Into<Option<Zoned>>>(v: T) -> Value {
        Value::JiffZoned(v.into().map(Into::into))
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

    pub fn is_jiff_zoned(&self) -> bool {
        matches!(self, Self::JiffZoned(_))
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

    pub fn as_ref_jiff_zoned(&self) -> Option<&Zoned> {
        match self {
            Self::JiffZoned(v) => v.as_deref(),
            _ => panic!("not Value::JiffZoned"),
        }
    }
}

pub(crate) const JIFF_DATE_TIME_FMT_STR: &str = "%Y-%m-%d %H:%M:%S%.6f";
pub(crate) const JIFF_TIMESTAMP_FMT_STR: &str = "%Y-%m-%d %H:%M:%S%.6f";
pub(crate) const JIFF_ZONE_FMT_STR: &str = "%Y-%m-%d %H:%M:%S%.6f %:z";

impl Value {
    #[cfg(test)]
    pub(crate) fn jiff_value_to_string(&self) -> Option<String> {
        match self {
            Self::JiffDate(v) => v.as_ref().map(|v| v.to_string()),
            Self::JiffTime(v) => v.as_ref().map(|v| v.to_string()),
            Self::JiffDateTime(v) => v
                .as_ref()
                .map(|v| v.strftime(JIFF_DATE_TIME_FMT_STR).to_string()),
            Self::JiffTimestamp(v) => v
                .as_ref()
                .map(|v| v.strftime(JIFF_TIMESTAMP_FMT_STR).to_string()),
            Self::JiffZoned(v) => v
                .as_ref()
                .map(|v| v.strftime(JIFF_ZONE_FMT_STR).to_string()),
            _ => panic!("not jiff Value"),
        }
    }
}

#[cfg(test)]
mod tests {

    use jiff::fmt::strtime;

    #[test]
    fn jiff_fmt() {
        use super::*;
        assert_eq!(
            Value::jiff_date(jiff::civil::date(2020, 1, 1)).jiff_value_to_string(),
            Some("2020-01-01".to_owned())
        );
        assert_eq!(
            Value::jiff_time(jiff::civil::time(1, 2, 3, 123456 * 1000)).jiff_value_to_string(),
            Some("01:02:03.123456".to_owned())
        );
        assert_eq!(
            Value::jiff_date_time(jiff::civil::date(2020, 1, 1).at(1, 2, 3, 123456 * 1000))
                .jiff_value_to_string(),
            Some("2020-01-01 01:02:03.123456".to_owned())
        );

        assert_eq!(
            Value::jiff_timestamp(jiff::Timestamp::constant(0, 123456 * 1000))
                .jiff_value_to_string(),
            Some("1970-01-01 00:00:00.123456".to_owned())
        );

        assert_eq!(
            Value::jiff_zoned(
                strtime::parse(JIFF_ZONE_FMT_STR, "1970-01-01 00:00:00.123456 +00:00")
                    .unwrap()
                    .to_zoned()
                    .unwrap()
            )
            .jiff_value_to_string(),
            Some("1970-01-01 00:00:00.123456 +00:00".to_owned())
        );
    }
}
