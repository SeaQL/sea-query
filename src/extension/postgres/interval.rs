use std::convert::TryFrom;
use std::fmt;

use crate::IntervalField;

impl fmt::Display for IntervalField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fields = match self {
            IntervalField::Year => "YEAR",
            IntervalField::Month => "MONTH",
            IntervalField::Day => "DAY",
            IntervalField::Hour => "HOUR",
            IntervalField::Minute => "MINUTE",
            IntervalField::Second => "SECOND",
            IntervalField::YearToMonth => "YEAR TO MONTH",
            IntervalField::DayToHour => "DAY TO HOUR",
            IntervalField::DayToMinute => "DAY TO MINUTE",
            IntervalField::DayToSecond => "DAY TO SECOND",
            IntervalField::HourToMinute => "HOUR TO MINUTE",
            IntervalField::HourToSecond => "HOUR TO SECOND",
            IntervalField::MinuteToSecond => "MINUTE TO SECOND",
        };
        write!(f, "{}", fields)
    }
}

impl TryFrom<String> for IntervalField {
    type Error = String;

    fn try_from(field: String) -> Result<Self, Self::Error> {
        IntervalField::try_from(field.as_str())
    }
}

impl TryFrom<&String> for IntervalField {
    type Error = String;

    fn try_from(field: &String) -> Result<Self, Self::Error> {
        IntervalField::try_from(field.as_str())
    }
}

impl TryFrom<&str> for IntervalField {
    type Error = String;

    fn try_from(field: &str) -> Result<Self, Self::Error> {
        match field.trim_start().trim_end().to_uppercase().as_ref() {
            "YEAR" => Ok(IntervalField::Year),
            "MONTH" => Ok(IntervalField::Month),
            "DAY" => Ok(IntervalField::Day),
            "HOUR" => Ok(IntervalField::Hour),
            "MINUTE" => Ok(IntervalField::Minute),
            "SECOND" => Ok(IntervalField::Second),
            "YEAR TO MONTH" => Ok(IntervalField::YearToMonth),
            "DAY TO HOUR" => Ok(IntervalField::DayToHour),
            "DAY TO MINUTE" => Ok(IntervalField::DayToMinute),
            "DAY TO SECOND" => Ok(IntervalField::DayToSecond),
            "HOUR TO MINUTE" => Ok(IntervalField::HourToMinute),
            "HOUR TO SECOND" => Ok(IntervalField::HourToSecond),
            "MINUTE TO SECOND" => Ok(IntervalField::MinuteToSecond),
            field => Err(format!(
                "Cannot turn \"{}\" into a Postgres interval field",
                field,
            )),
        }
    }
}
