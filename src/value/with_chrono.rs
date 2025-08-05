use super::*;
use chrono::{Local, Offset, Utc};

type_to_value!(NaiveDate, ChronoDate, Date);
type_to_value!(NaiveTime, ChronoTime, Time);
type_to_value!(NaiveDateTime, ChronoDateTime, DateTime);

impl From<DateTime<Utc>> for Value {
    fn from(v: DateTime<Utc>) -> Value {
        Value::ChronoDateTimeUtc(Some(v))
    }
}

impl From<DateTime<Local>> for Value {
    fn from(v: DateTime<Local>) -> Value {
        Value::ChronoDateTimeLocal(Some(v))
    }
}

impl From<DateTime<FixedOffset>> for Value {
    fn from(x: DateTime<FixedOffset>) -> Value {
        let v = DateTime::<FixedOffset>::from_naive_utc_and_offset(x.naive_utc(), x.offset().fix());
        Value::ChronoDateTimeWithTimeZone(Some(v))
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
            Value::ChronoDateTimeUtc(Some(x)) => Ok(x),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(DateTime<Utc>).to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::ChronoDateTimeUtc
    }

    fn column_type() -> ColumnType {
        ColumnType::TimestampWithTimeZone
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
            Value::ChronoDateTimeLocal(Some(x)) => Ok(x),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(DateTime<Local>).to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::ChronoDateTimeLocal
    }

    fn column_type() -> ColumnType {
        ColumnType::TimestampWithTimeZone
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
            Value::ChronoDateTimeWithTimeZone(Some(x)) => Ok(x),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(DateTime<FixedOffset>).to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::ChronoDateTimeWithTimeZone
    }

    fn column_type() -> ColumnType {
        ColumnType::TimestampWithTimeZone
    }
}

impl Value {
    pub fn is_chrono_date(&self) -> bool {
        matches!(self, Self::ChronoDate(_))
    }

    pub fn is_chrono_time(&self) -> bool {
        matches!(self, Self::ChronoTime(_))
    }

    pub fn is_chrono_date_time(&self) -> bool {
        matches!(self, Self::ChronoDateTime(_))
    }

    pub fn is_chrono_date_time_utc(&self) -> bool {
        matches!(self, Self::ChronoDateTimeUtc(_))
    }

    pub fn is_chrono_date_time_with_time_zone(&self) -> bool {
        matches!(self, Self::ChronoDateTimeWithTimeZone(_))
    }

    pub fn is_chrono_date_time_local(&self) -> bool {
        matches!(self, Self::ChronoDateTimeLocal(_))
    }

    pub fn as_ref_chrono_date(&self) -> Option<&NaiveDate> {
        match self {
            Self::ChronoDate(v) => v.as_ref(),
            _ => panic!("not Value::ChronoDate"),
        }
    }

    pub fn as_ref_chrono_time(&self) -> Option<&NaiveTime> {
        match self {
            Self::ChronoTime(v) => v.as_ref(),
            _ => panic!("not Value::ChronoTime"),
        }
    }

    pub fn as_ref_chrono_date_time(&self) -> Option<&NaiveDateTime> {
        match self {
            Self::ChronoDateTime(v) => v.as_ref(),
            _ => panic!("not Value::ChronoDateTime"),
        }
    }

    pub fn as_ref_chrono_date_time_utc(&self) -> Option<&DateTime<Utc>> {
        match self {
            Self::ChronoDateTimeUtc(v) => v.as_ref(),
            _ => panic!("not Value::ChronoDateTimeUtc"),
        }
    }

    pub fn as_ref_chrono_date_time_with_time_zone(&self) -> Option<&DateTime<FixedOffset>> {
        match self {
            Self::ChronoDateTimeWithTimeZone(v) => v.as_ref(),
            _ => panic!("not Value::ChronoDateTimeWithTimeZone"),
        }
    }

    pub fn as_ref_chrono_date_time_local(&self) -> Option<&DateTime<Local>> {
        match self {
            Self::ChronoDateTimeLocal(v) => v.as_ref(),
            _ => panic!("not Value::ChronoDateTimeLocal"),
        }
    }

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
