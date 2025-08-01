use std::convert::TryFrom;
use std::fmt;

use crate::PgInterval;

impl fmt::Display for PgInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fields = match self {
            Self::Year => "YEAR",
            Self::Month => "MONTH",
            Self::Day => "DAY",
            Self::Hour => "HOUR",
            Self::Minute => "MINUTE",
            Self::Second => "SECOND",
            Self::YearToMonth => "YEAR TO MONTH",
            Self::DayToHour => "DAY TO HOUR",
            Self::DayToMinute => "DAY TO MINUTE",
            Self::DayToSecond => "DAY TO SECOND",
            Self::HourToMinute => "HOUR TO MINUTE",
            Self::HourToSecond => "HOUR TO SECOND",
            Self::MinuteToSecond => "MINUTE TO SECOND",
        };
        write!(f, "{fields}")
    }
}

impl TryFrom<String> for PgInterval {
    type Error = String;

    fn try_from(field: String) -> Result<Self, Self::Error> {
        Self::try_from(field.as_str())
    }
}

impl TryFrom<&String> for PgInterval {
    type Error = String;

    fn try_from(field: &String) -> Result<Self, Self::Error> {
        Self::try_from(field.as_str())
    }
}

impl TryFrom<&str> for PgInterval {
    type Error = String;

    fn try_from(field: &str) -> Result<Self, Self::Error> {
        match field.trim_start().trim_end().to_uppercase().as_ref() {
            "YEAR" => Ok(Self::Year),
            "MONTH" => Ok(Self::Month),
            "DAY" => Ok(Self::Day),
            "HOUR" => Ok(Self::Hour),
            "MINUTE" => Ok(Self::Minute),
            "SECOND" => Ok(Self::Second),
            "YEAR TO MONTH" => Ok(Self::YearToMonth),
            "DAY TO HOUR" => Ok(Self::DayToHour),
            "DAY TO MINUTE" => Ok(Self::DayToMinute),
            "DAY TO SECOND" => Ok(Self::DayToSecond),
            "HOUR TO MINUTE" => Ok(Self::HourToMinute),
            "HOUR TO SECOND" => Ok(Self::HourToSecond),
            "MINUTE TO SECOND" => Ok(Self::MinuteToSecond),
            field => Err(format!(
                "Cannot turn \"{field}\" into a Postgres interval field",
            )),
        }
    }
}
