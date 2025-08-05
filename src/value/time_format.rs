use time::format_description::FormatItem;
use time::macros::format_description;

pub static FORMAT_DATE: &[FormatItem<'static>] = format_description!("[year]-[month]-[day]");
pub static FORMAT_TIME: &[FormatItem<'static>] =
    format_description!("[hour]:[minute]:[second].[subsecond digits:6]");
pub static FORMAT_DATETIME: &[FormatItem<'static>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]");
pub static FORMAT_DATETIME_TZ: &[FormatItem<'static>] = format_description!(
    "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6] [offset_hour sign:mandatory]:[offset_minute]"
);
