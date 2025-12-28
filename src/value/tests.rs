use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_value() {
    macro_rules! test_value {
        ( $type: ty, $val: literal ) => {
            let val: $type = $val;
            let v: Value = val.into();
            let out: $type = v.unwrap();
            assert_eq!(out, val);
        };
    }

    test_value!(u8, 255);
    test_value!(u16, 65535);
    test_value!(i8, 127);
    test_value!(i16, 32767);
    test_value!(i32, 1073741824);
    test_value!(i64, 8589934592);
}

#[test]
fn test_option_value() {
    macro_rules! test_some_value {
        ( $type: ty, $val: literal ) => {
            let val: Option<$type> = Some($val);
            let v: Value = val.into();
            let out: $type = v.unwrap();
            assert_eq!(out, val.unwrap());
        };
    }

    macro_rules! test_none {
        ( $type: ty, $name: ident ) => {
            let val: Option<$type> = None;
            let v: Value = val.into();
            assert_eq!(v, Value::$name(None));
        };
    }

    test_some_value!(u8, 255);
    test_some_value!(u16, 65535);
    test_some_value!(i8, 127);
    test_some_value!(i16, 32767);
    test_some_value!(i32, 1073741824);
    test_some_value!(i64, 8589934592);

    test_none!(u8, TinyUnsigned);
    test_none!(u16, SmallUnsigned);
    test_none!(i8, TinyInt);
    test_none!(i16, SmallInt);
    test_none!(i32, Int);
    test_none!(i64, BigInt);
}

#[test]
fn test_cow_value() {
    let val: Cow<str> = "hello".into();
    let val2 = val.clone();
    let v: Value = val.into();
    let out: Cow<str> = v.unwrap();
    assert_eq!(out, val2);
}

#[test]
fn test_box_value() {
    let val: String = "hello".to_owned();
    let v: Value = val.clone().into();
    let out: String = v.unwrap();
    assert_eq!(out, val);
}

#[test]
fn test_value_tuple() {
    assert_eq!(
        1i32.into_value_tuple(),
        ValueTuple::One(Value::Int(Some(1)))
    );
    assert_eq!(
        "b".into_value_tuple(),
        ValueTuple::One(Value::String(Some("b".to_owned())))
    );
    assert_eq!(
        (1i32, "b").into_value_tuple(),
        ValueTuple::Two(Value::Int(Some(1)), Value::String(Some("b".to_owned())))
    );
    assert_eq!(
        (1i32, 2.4f64, "b").into_value_tuple(),
        ValueTuple::Three(
            Value::Int(Some(1)),
            Value::Double(Some(2.4)),
            Value::String(Some("b".to_owned()))
        )
    );
    assert_eq!(
        (1i32, 2.4f64, "b", 123u8).into_value_tuple(),
        ValueTuple::Many(vec![
            Value::Int(Some(1)),
            Value::Double(Some(2.4)),
            Value::String(Some("b".to_owned())),
            Value::TinyUnsigned(Some(123))
        ])
    );
    assert_eq!(
        (1i32, 2.4f64, "b", 123u8, 456u16).into_value_tuple(),
        ValueTuple::Many(vec![
            Value::Int(Some(1)),
            Value::Double(Some(2.4)),
            Value::String(Some("b".to_owned())),
            Value::TinyUnsigned(Some(123)),
            Value::SmallUnsigned(Some(456))
        ])
    );
    assert_eq!(
        (1i32, 2.4f64, "b", 123u8, 456u16, 789u32).into_value_tuple(),
        ValueTuple::Many(vec![
            Value::Int(Some(1)),
            Value::Double(Some(2.4)),
            Value::String(Some("b".to_owned())),
            Value::TinyUnsigned(Some(123)),
            Value::SmallUnsigned(Some(456)),
            Value::Unsigned(Some(789))
        ])
    );
}

#[test]
#[allow(clippy::clone_on_copy)]
fn test_from_value_tuple() {
    let mut val = 1i32;
    let original = val.clone();
    val = FromValueTuple::from_value_tuple(val);
    assert_eq!(val, original);

    let mut val = "b".to_owned();
    let original = val.clone();
    val = FromValueTuple::from_value_tuple(val);
    assert_eq!(val, original);

    let mut val = (1i32, "b".to_owned());
    let original = val.clone();
    val = FromValueTuple::from_value_tuple(val);
    assert_eq!(val, original);

    let mut val = (1i32, 2.4f64, "b".to_owned());
    let original = val.clone();
    val = FromValueTuple::from_value_tuple(val);
    assert_eq!(val, original);

    let mut val = (1i32, 2.4f64, "b".to_owned(), 123u8);
    let original = val.clone();
    val = FromValueTuple::from_value_tuple(val);
    assert_eq!(val, original);

    let mut val = (1i32, 2.4f64, "b".to_owned(), 123u8, 456u16);
    let original = val.clone();
    val = FromValueTuple::from_value_tuple(val);
    assert_eq!(val, original);

    let mut val = (1i32, 2.4f64, "b".to_owned(), 123u8, 456u16, 789u32);
    let original = val.clone();
    val = FromValueTuple::from_value_tuple(val);
    assert_eq!(val, original);
}

#[test]
fn test_value_tuple_iter() {
    let mut iter = (1i32).into_value_tuple().into_iter();
    assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
    assert_eq!(iter.next(), None);

    let mut iter = (1i32, 2.4f64).into_value_tuple().into_iter();
    assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
    assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
    assert_eq!(iter.next(), None);

    let mut iter = (1i32, 2.4f64, "b").into_value_tuple().into_iter();
    assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
    assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
    assert_eq!(iter.next().unwrap(), Value::String(Some("b".to_owned())));
    assert_eq!(iter.next(), None);

    let mut iter = (1i32, 2.4f64, "b", 123u8).into_value_tuple().into_iter();
    assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
    assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
    assert_eq!(iter.next().unwrap(), Value::String(Some("b".to_owned())));
    assert_eq!(iter.next().unwrap(), Value::TinyUnsigned(Some(123)));
    assert_eq!(iter.next(), None);

    let mut iter = (1i32, 2.4f64, "b", 123u8, 456u16)
        .into_value_tuple()
        .into_iter();
    assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
    assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
    assert_eq!(iter.next().unwrap(), Value::String(Some("b".to_owned())));
    assert_eq!(iter.next().unwrap(), Value::TinyUnsigned(Some(123)));
    assert_eq!(iter.next().unwrap(), Value::SmallUnsigned(Some(456)));
    assert_eq!(iter.next(), None);

    let mut iter = (1i32, 2.4f64, "b", 123u8, 456u16, 789u32)
        .into_value_tuple()
        .into_iter();
    assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
    assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
    assert_eq!(iter.next().unwrap(), Value::String(Some("b".to_owned())));
    assert_eq!(iter.next().unwrap(), Value::TinyUnsigned(Some(123)));
    assert_eq!(iter.next().unwrap(), Value::SmallUnsigned(Some(456)));
    assert_eq!(iter.next().unwrap(), Value::Unsigned(Some(789)));
    assert_eq!(iter.next(), None);
}

#[test]
#[cfg(feature = "with-json")]
fn test_json_value() {
    let json = serde_json::json! {{
        "a": 25.0,
        "b": "hello",
    }};
    let value: Value = json.clone().into();
    let out: Json = value.unwrap();
    assert_eq!(out, json);
}

#[test]
#[cfg(feature = "with-chrono")]
fn test_chrono_value() {
    let timestamp = NaiveDate::from_ymd_opt(2020, 1, 1)
        .unwrap()
        .and_hms_opt(2, 2, 2)
        .unwrap();
    let value: Value = timestamp.into();
    let out: NaiveDateTime = value.unwrap();
    assert_eq!(out, timestamp);
}

#[test]
#[cfg(feature = "with-chrono")]
fn test_chrono_utc_value() {
    let timestamp = DateTime::<Utc>::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(2022, 1, 2)
            .unwrap()
            .and_hms_opt(3, 4, 5)
            .unwrap(),
        Utc,
    );
    let value: Value = timestamp.into();
    let out: DateTime<Utc> = value.unwrap();
    assert_eq!(out, timestamp);
}

#[test]
#[cfg(feature = "with-chrono")]
fn test_chrono_local_value() {
    let timestamp_utc = DateTime::<Utc>::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(2022, 1, 2)
            .unwrap()
            .and_hms_opt(3, 4, 5)
            .unwrap(),
        Utc,
    );
    let timestamp_local: DateTime<Local> = timestamp_utc.into();
    let value: Value = timestamp_local.into();
    let out: DateTime<Local> = value.unwrap();
    assert_eq!(out, timestamp_local);
}

#[test]
#[cfg(feature = "with-chrono")]
fn test_chrono_timezone_value() {
    let timestamp = DateTime::parse_from_rfc3339("2020-01-01T02:02:02+08:00").unwrap();
    let value: Value = timestamp.into();
    let out: DateTime<FixedOffset> = value.unwrap();
    assert_eq!(out, timestamp);
}

#[test]
#[cfg(feature = "with-chrono")]
fn test_chrono_query() {
    use crate::*;

    let string = "2020-01-01T02:02:02+08:00";
    let timestamp = DateTime::parse_from_rfc3339(string).unwrap();

    let query = Query::select().expr(timestamp).to_owned();

    let formatted = "2020-01-01 02:02:02.000000 +08:00";

    assert_eq!(
        query.to_string(MysqlQueryBuilder),
        format!("SELECT '{formatted}'")
    );
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        format!("SELECT '{formatted}'")
    );
    assert_eq!(
        query.to_string(SqliteQueryBuilder),
        format!("SELECT '{formatted}'")
    );
}

#[test]
#[cfg(feature = "with-time")]
fn test_time_value() {
    use time::macros::{date, time};
    let timestamp = date!(2020 - 01 - 01).with_time(time!(2:2:2));
    let value: Value = timestamp.into();
    let out: PrimitiveDateTime = value.unwrap();
    assert_eq!(out, timestamp);
}

#[test]
#[cfg(feature = "with-time")]
fn test_time_utc_value() {
    use time::macros::{date, time};
    let timestamp = date!(2022 - 01 - 02).with_time(time!(3:04:05)).assume_utc();
    let value: Value = timestamp.into();
    let out: OffsetDateTime = value.unwrap();
    assert_eq!(out, timestamp);
}

#[test]
#[cfg(feature = "with-time")]
fn test_time_local_value() {
    use time::macros::{date, offset, time};
    let timestamp_utc = date!(2022 - 01 - 02).with_time(time!(3:04:05)).assume_utc();
    let timestamp_local: OffsetDateTime = timestamp_utc.to_offset(offset!(+3));
    let value: Value = timestamp_local.into();
    let out: OffsetDateTime = value.unwrap();
    assert_eq!(out, timestamp_local);
}

#[test]
#[cfg(feature = "with-time")]
fn test_time_timezone_value() {
    use time::macros::{date, offset, time};
    let timestamp = date!(2022 - 01 - 02)
        .with_time(time!(3:04:05))
        .assume_offset(offset!(+8));
    let value: Value = timestamp.into();
    let out: OffsetDateTime = value.unwrap();
    assert_eq!(out, timestamp);
}

#[test]
#[cfg(feature = "with-time")]
fn test_time_query() {
    use crate::*;
    use time::macros::datetime;

    let timestamp = datetime!(2020-01-01 02:02:02 +8);
    let query = Query::select().expr(timestamp).to_owned();
    let formatted = "2020-01-01 02:02:02.000000 +08:00";

    assert_eq!(
        query.to_string(MysqlQueryBuilder),
        format!("SELECT '{formatted}'")
    );
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        format!("SELECT '{formatted}'")
    );
    assert_eq!(
        query.to_string(SqliteQueryBuilder),
        format!("SELECT '{formatted}'")
    );
}

#[test]
#[cfg(feature = "with-uuid")]
fn test_uuid_value() {
    let uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
    let value: Value = uuid.into();
    let out: Uuid = value.unwrap();
    assert_eq!(out, uuid);

    let uuid_braced = uuid.braced();
    let value: Value = uuid_braced.into();
    let out: Uuid = value.unwrap();
    assert_eq!(out, uuid);

    let uuid_hyphenated = uuid.hyphenated();
    let value: Value = uuid_hyphenated.into();
    let out: Uuid = value.unwrap();
    assert_eq!(out, uuid);

    let uuid_simple = uuid.simple();
    let value: Value = uuid_simple.into();
    let out: Uuid = value.unwrap();
    assert_eq!(out, uuid);

    let uuid_urn = uuid.urn();
    let value: Value = uuid_urn.into();
    let out: Uuid = value.unwrap();
    assert_eq!(out, uuid);
}

#[test]
#[cfg(feature = "with-rust_decimal")]
fn test_decimal_value() {
    use std::str::FromStr;

    let num = "2.02";
    let val = Decimal::from_str(num).unwrap();
    let v: Value = val.into();
    let out: Decimal = v.unwrap();
    assert_eq!(out.to_string(), num);
}

#[test]
#[cfg(feature = "postgres-array")]
fn test_array_value() {
    let array = vec![1, 2, 3, 4, 5];
    let v: Value = array.into();
    let out: Vec<Option<i32>> = v.unwrap();
    assert_eq!(out, vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
}

#[test]
#[cfg(feature = "postgres-array")]
fn test_option_array_value() {
    let v: Value = Value::Array(Array::Null(ArrayType::Int));
    let out: Option<Vec<Option<i32>>> = v.unwrap();
    assert_eq!(out, None);
}

#[test]
#[cfg(feature = "postgres-array")]
fn test_try_from_value_array() {
    fn assert_try_from_value_array<T>(v1: T, v2: T)
    where
        T: ArrayElement + Clone + PartialEq + std::fmt::Debug,
    {
        let expected = vec![Some(v1.clone()), None, Some(v2.clone())];
        let value: Value = expected.clone().into();
        let out = T::try_from_value(value).unwrap();
        assert_eq!(out, expected);

        let value = Value::Array(Array::Null(T::ArrayValueType::array_type()));
        let out = T::try_from_value(value).unwrap();
        assert_eq!(out, Vec::<Option<T>>::new());
    }

    assert_try_from_value_array(true, false);
    assert_try_from_value_array(-1i8, 2);
    assert_try_from_value_array(-2i16, 3);
    assert_try_from_value_array(-3i32, 4);
    assert_try_from_value_array(-4i64, 5);
    assert_try_from_value_array(6u16, 7);
    assert_try_from_value_array(8u32, 9);
    assert_try_from_value_array(10u64, 11);
    assert_try_from_value_array(1.5f32, 2.5);
    assert_try_from_value_array(3.25f64, 4.75);
    assert_try_from_value_array(String::from("a"), String::from("b"));
    assert_try_from_value_array('a', 'b');
    assert_try_from_value_array(vec![1, 2, 3], vec![4, 5, 6]);

    #[cfg(feature = "with-json")]
    {
        assert_try_from_value_array(serde_json::json!({"a": 1}), serde_json::json!({"b": 2}));
    }

    #[cfg(feature = "with-chrono")]
    {
        assert_try_from_value_array(
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2021, 2, 2).unwrap(),
        );
        assert_try_from_value_array(
            NaiveTime::from_hms_opt(1, 2, 3).unwrap(),
            NaiveTime::from_hms_opt(4, 5, 6).unwrap(),
        );
        assert_try_from_value_array(
            NaiveDate::from_ymd_opt(2020, 1, 1)
                .unwrap()
                .and_hms_opt(1, 2, 3)
                .unwrap(),
            NaiveDate::from_ymd_opt(2021, 2, 2)
                .unwrap()
                .and_hms_opt(4, 5, 6)
                .unwrap(),
        );
        assert_try_from_value_array(
            DateTime::<Utc>::from_utc(
                NaiveDate::from_ymd_opt(2020, 1, 1)
                    .unwrap()
                    .and_hms_opt(1, 2, 3)
                    .unwrap(),
                Utc,
            ),
            DateTime::<Utc>::from_utc(
                NaiveDate::from_ymd_opt(2021, 2, 2)
                    .unwrap()
                    .and_hms_opt(4, 5, 6)
                    .unwrap(),
                Utc,
            ),
        );
        assert_try_from_value_array(
            chrono::TimeZone::from_utc_datetime(
                &Local,
                &NaiveDate::from_ymd_opt(2020, 1, 1)
                    .unwrap()
                    .and_hms_opt(1, 2, 3)
                    .unwrap(),
            ),
            chrono::TimeZone::from_utc_datetime(
                &Local,
                &NaiveDate::from_ymd_opt(2021, 2, 2)
                    .unwrap()
                    .and_hms_opt(4, 5, 6)
                    .unwrap(),
            ),
        );
        assert_try_from_value_array(
            DateTime::<FixedOffset>::from_utc(
                NaiveDate::from_ymd_opt(2020, 1, 1)
                    .unwrap()
                    .and_hms_opt(1, 2, 3)
                    .unwrap(),
                FixedOffset::east_opt(0).unwrap(),
            ),
            DateTime::<FixedOffset>::from_utc(
                NaiveDate::from_ymd_opt(2021, 2, 2)
                    .unwrap()
                    .and_hms_opt(4, 5, 6)
                    .unwrap(),
                FixedOffset::east_opt(0).unwrap(),
            ),
        );
    }

    #[cfg(feature = "with-time")]
    {
        assert_try_from_value_array(
            time::Date::from_calendar_date(2020, time::Month::January, 1).unwrap(),
            time::Date::from_calendar_date(2021, time::Month::February, 2).unwrap(),
        );
        assert_try_from_value_array(
            time::Time::from_hms(1, 2, 3).unwrap(),
            time::Time::from_hms(4, 5, 6).unwrap(),
        );
        assert_try_from_value_array(
            PrimitiveDateTime::new(
                time::Date::from_calendar_date(2020, time::Month::January, 1).unwrap(),
                time::Time::from_hms(1, 2, 3).unwrap(),
            ),
            PrimitiveDateTime::new(
                time::Date::from_calendar_date(2021, time::Month::February, 2).unwrap(),
                time::Time::from_hms(4, 5, 6).unwrap(),
            ),
        );
        assert_try_from_value_array(
            OffsetDateTime::from_unix_timestamp(0).unwrap(),
            OffsetDateTime::from_unix_timestamp(60).unwrap(),
        );
    }

    #[cfg(feature = "with-jiff")]
    {
        assert_try_from_value_array(jiff::civil::date(2020, 1, 1), jiff::civil::date(2021, 2, 2));
        assert_try_from_value_array(
            jiff::civil::time(1, 2, 3, 123456 * 1000),
            jiff::civil::time(4, 5, 6, 234567 * 1000),
        );
        assert_try_from_value_array(
            jiff::civil::date(2020, 1, 1).at(1, 2, 3, 123456 * 1000),
            jiff::civil::date(2021, 2, 2).at(4, 5, 6, 234567 * 1000),
        );
        assert_try_from_value_array(
            jiff::Timestamp::constant(0, 123456 * 1000),
            jiff::Timestamp::constant(1, 234567 * 1000),
        );
        assert_try_from_value_array(
            jiff::fmt::strtime::parse(
                "%Y-%m-%d %H:%M:%S%.6f %:z",
                "1970-01-01 00:00:00.123456 +00:00",
            )
            .unwrap()
            .to_zoned()
            .unwrap(),
            jiff::fmt::strtime::parse(
                "%Y-%m-%d %H:%M:%S%.6f %:z",
                "1970-01-01 00:00:00.234567 +00:00",
            )
            .unwrap()
            .to_zoned()
            .unwrap(),
        );
    }

    #[cfg(feature = "with-uuid")]
    {
        assert_try_from_value_array(Uuid::from_bytes([1; 16]), Uuid::from_bytes([2; 16]));
    }

    #[cfg(feature = "with-rust_decimal")]
    {
        assert_try_from_value_array(Decimal::new(123, 2), Decimal::new(456, 2));
    }

    #[cfg(feature = "with-bigdecimal")]
    {
        assert_try_from_value_array(BigDecimal::from(123), BigDecimal::from(456));
    }

    #[cfg(feature = "with-ipnetwork")]
    {
        assert_try_from_value_array(
            IpNetwork::new(
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 0, 1)),
                24,
            )
            .unwrap(),
            IpNetwork::new(
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(10, 0, 0, 1)),
                16,
            )
            .unwrap(),
        );
    }

    #[cfg(feature = "with-mac_address")]
    {
        assert_try_from_value_array(
            MacAddress::new([0, 1, 2, 3, 4, 5]),
            MacAddress::new([6, 7, 8, 9, 10, 11]),
        );
    }
}
