use quote::{quote, ToTokens};
use sea_query::*;

#[test]
fn with_quote_1() {
    for (interval_field, token_stream) in [
        (IntervalField::Year, quote! { IntervalField::Year }),
        (IntervalField::Month, quote! { IntervalField::Month }),
        (IntervalField::Day, quote! { IntervalField::Day }),
        (IntervalField::Hour, quote! { IntervalField::Hour }),
        (IntervalField::Minute, quote! { IntervalField::Minute }),
        (IntervalField::Second, quote! { IntervalField::Second }),
        (
            IntervalField::YearToMonth,
            quote! { IntervalField::YearToMonth },
        ),
        (
            IntervalField::DayToHour,
            quote! { IntervalField::DayToHour },
        ),
        (
            IntervalField::DayToMinute,
            quote! { IntervalField::DayToMinute },
        ),
        (
            IntervalField::DayToSecond,
            quote! { IntervalField::DayToSecond },
        ),
        (
            IntervalField::HourToMinute,
            quote! { IntervalField::HourToMinute },
        ),
        (
            IntervalField::HourToSecond,
            quote! { IntervalField::HourToSecond },
        ),
        (
            IntervalField::MinuteToSecond,
            quote! { IntervalField::MinuteToSecond },
        ),
    ] {
        assert_eq!(
            interval_field.into_token_stream().to_string(),
            token_stream.to_string()
        );
    }
}
