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
    let out: Vec<i32> = v.unwrap();
    assert_eq!(out, vec![1, 2, 3, 4, 5]);
}

#[test]
#[cfg(feature = "postgres-array")]
fn test_option_array_value() {
    let v: Value = Value::Array(ArrayType::Int, None);
    let out: Option<Vec<i32>> = v.unwrap();
    assert_eq!(out, None);
}
