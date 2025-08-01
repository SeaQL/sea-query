use super::*;

type_to_value!(time::Date, TimeDate, Date);
type_to_value!(time::Time, TimeTime, Time);
type_to_value!(PrimitiveDateTime, TimeDateTime, DateTime);

impl From<OffsetDateTime> for Value {
    fn from(v: OffsetDateTime) -> Value {
        Value::time_date_time_with_time_zone(v)
    }
}

impl Nullable for OffsetDateTime {
    fn null() -> Value {
        Value::time_date_time_with_time_zone(None)
    }
}

impl ValueType for OffsetDateTime {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::TimeDateTimeWithTimeZone(Some(x)) => Ok(x),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(OffsetDateTime).to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::TimeDateTimeWithTimeZone
    }

    fn column_type() -> ColumnType {
        ColumnType::TimestampWithTimeZone
    }
}

impl Value {
    pub fn is_time_date(&self) -> bool {
        matches!(self, Self::TimeDate(_))
    }

    pub fn as_ref_time_date(&self) -> Option<&time::Date> {
        match self {
            Self::TimeDate(v) => v.as_ref(),
            _ => panic!("not Value::TimeDate"),
        }
    }
}

impl Value {
    pub fn is_time_time(&self) -> bool {
        matches!(self, Self::TimeTime(_))
    }

    pub fn as_ref_time_time(&self) -> Option<&time::Time> {
        match self {
            Self::TimeTime(v) => v.as_ref(),
            _ => panic!("not Value::TimeTime"),
        }
    }
}

impl Value {
    pub fn is_time_date_time(&self) -> bool {
        matches!(self, Self::TimeDateTime(_))
    }

    pub fn as_ref_time_date_time(&self) -> Option<&PrimitiveDateTime> {
        match self {
            Self::TimeDateTime(v) => v.as_ref(),
            _ => panic!("not Value::TimeDateTime"),
        }
    }
}

impl Value {
    pub fn is_time_date_time_with_time_zone(&self) -> bool {
        matches!(self, Self::TimeDateTimeWithTimeZone(_))
    }

    pub fn as_ref_time_date_time_with_time_zone(&self) -> Option<&OffsetDateTime> {
        match self {
            Self::TimeDateTimeWithTimeZone(v) => v.as_ref(),
            _ => panic!("not Value::TimeDateTimeWithTimeZone"),
        }
    }
}

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
