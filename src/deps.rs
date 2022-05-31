#[cfg(feature = "with-time-0_3")]
pub mod time {
    pub use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

    use time::format_description::FormatItem;
    use time::macros::format_description;

    pub static FORMAT_TIME: &'static [FormatItem<'static>] =
        format_description!("[year]-[month]-[day]");
    pub static FORMAT_DATE: &'static [FormatItem<'static>] =
        format_description!("[hour]:[minute]:[second]");
    pub static FORMAT_DATETIME: &'static [FormatItem<'static>] =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    pub static FORMAT_DATETIME_TZ: &'static [FormatItem<'static>] = format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"
    );

    pub fn offset(hours: i8) -> UtcOffset {
        UtcOffset::from_hms(hours, 0, 0).unwrap()
    }

    pub fn time(hour: u8, minute: u8, second: u8) -> Time {
        Time::from_hms(hour, minute, second).unwrap()
    }

    pub fn date(year: i32, month: u8, day: u8) -> time::Date {
        let month = time::Month::try_from(month).unwrap();
        Date::from_calendar_date(year, month, day).unwrap()
    }

    pub fn datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> PrimitiveDateTime {
        date(year, month, day).with_time(time(hour, minute, second))
    }

    pub fn datetimetz(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        offset_hours: i8,
    ) -> OffsetDateTime {
        datetime(year, month, day, hour, minute, second).assume_offset(offset(offset_hours))
    }
}

#[cfg(feature = "with-time-0_2")]
pub mod time {
    pub use time_0_2::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

    pub static FORMAT_TIME: &'static str = "%Y-%m-%d";
    pub static FORMAT_DATE: &'static str = "%H:%M:%S";
    pub static FORMAT_DATETIME: &'static str = "%Y-%m-%d %H:%M:%S";
    pub static FORMAT_DATETIME_TZ: &'static str = "%Y-%m-%d %H:%M:%S %z";

    pub fn offset(hours: i8) -> UtcOffset {
        UtcOffset::hours(hours)
    }

    pub fn time(hour: u8, minute: u8, second: u8) -> Time {
        Time::try_from_hms(hour, minute, second).unwrap()
    }

    pub fn date(year: i32, month: u8, day: u8) -> Date {
        Date::try_from_ymd(year, month, day).unwrap()
    }

    pub fn datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> PrimitiveDateTime {
        date(year, month, day).with_time(time(hour, minute, second))
    }

    pub fn datetimetz(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        offset_hours: i8,
    ) -> OffsetDateTime {
        datetime(year, month, day, hour, minute, second)
            .assume_utc()
            .to_offset(offset(offset_hours))
    }
}
