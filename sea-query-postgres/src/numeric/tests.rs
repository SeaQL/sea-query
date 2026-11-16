use std::str::FromStr;

use bigdecimal::num_bigint::BigInt;

use super::*;

fn encode(value: &BigDecimal) -> Vec<u16> {
    let mut out = BytesMut::new();
    to_sql(value, &mut out).unwrap();
    out.chunks(2)
        .map(|b| u16::from_be_bytes([b[0], b[1]]))
        .collect()
}

fn encode_str(text: &str) -> Vec<i16> {
    encode(&BigDecimal::from_str(text).unwrap())
        .into_iter()
        .map(|n| n as i16)
        .collect()
}

#[test]
fn numeric_send_layout() {
    // ndigits, weight, sign, dscale, digits
    assert_eq!(encode_str("0"), [0, 0, 0, 0]);
    assert_eq!(encode_str("0.00"), [0, 0, 0, 2]);
    assert_eq!(encode_str("-0.00"), [0, 0, 0, 2]);
    assert_eq!(encode_str("12.50"), [2, 0, 0, 2, 12, 5000]);
    assert_eq!(encode_str("-12.5"), [2, 0, 0x4000, 1, 12, 5000]);
    assert_eq!(encode_str("10000"), [1, 1, 0, 0, 1]);
    assert_eq!(encode_str("1E+3"), [1, 0, 0, 0, 1000]);
    assert_eq!(encode_str("12E+4"), [1, 1, 0, 0, 12]);
    assert_eq!(encode_str("0.00012"), [2, -1, 0, 5, 1, 2000]);
    assert_eq!(encode_str("0.000012"), [1, -2, 0, 6, 1200]);
    assert_eq!(
        encode_str("123456789012345678.0123456789"),
        [8, 4, 0, 10, 12, 3456, 7890, 1234, 5678, 123, 4567, 8900]
    );
}

#[test]
fn zero_with_any_exponent() {
    assert_eq!(encode_str("0E+200000"), [0, 0, 0, 0]);
    assert_eq!(encode_str("0E-20000"), [0, 0, 0, 0x3FFF]);
    for exponent in [i64::MIN, i64::MAX] {
        let zero = BigDecimal::new(BigInt::from(0), exponent);
        assert_eq!(encode(&zero)[..3], [0, 0, 0]);
    }
}

#[test]
fn numeric_range_limits() {
    for text in ["1E+131072", "1E-16384"] {
        let value = BigDecimal::from_str(text).unwrap();
        assert!(to_sql(&value, &mut BytesMut::new()).is_err(), "{text}");
    }
    for exponent in [i64::MIN, i64::MAX] {
        let value = BigDecimal::new(BigInt::from(1), exponent);
        assert!(to_sql(&value, &mut BytesMut::new()).is_err(), "{exponent}");
    }
    assert_eq!(encode_str("1E-16383"), [1, -4096, 0, 16383, 10]);
    assert_eq!(encode_str("1E+131071"), [1, 32767, 0, 0, 1000]);
}

#[test]
fn maximum_numeric_precision() {
    // PostgreSQL's integer and fractional digit limits.
    let digits = "1".repeat(131072 + 16383);
    let value = BigDecimal::new(BigInt::from_str(&digits).unwrap(), 16383);
    let encoded = encode(&value);
    assert_eq!(encoded[..4], [36864, 32767, 0, 16383]);
    assert_eq!(encoded.len(), 4 + 36864);
    assert_eq!(encoded[4..6], [1111, 1111]);
    assert_eq!(*encoded.last().unwrap(), 1110);
}

#[test]
fn parameter_type_selects_encoding() {
    use crate::PostgresValue;
    use postgres_types::{IsNull, ToSql, Type};
    use sea_query::Value;

    let value = BigDecimal::from_str("123456789012345678.0123456789").unwrap();
    let bind = |value: Value, ty: &Type| {
        let mut out = BytesMut::new();
        let is_null = PostgresValue(value).to_sql(ty, &mut out).unwrap();
        (matches!(is_null, IsNull::Yes), out.to_vec())
    };

    let mut expected = BytesMut::new();
    to_sql(&value, &mut expected).unwrap();
    assert_eq!(
        bind(value.clone().into(), &Type::NUMERIC),
        (false, expected.to_vec())
    );
    assert_eq!(
        bind(Value::BigDecimal(None), &Type::NUMERIC),
        (true, vec![])
    );
    let value_f64 = bigdecimal::ToPrimitive::to_f64(&value).unwrap();
    let (_, float) = bind(value.into(), &Type::FLOAT8);
    assert_eq!(float, value_f64.to_be_bytes());
}

#[test]
fn rejects_unsupported_parameter_types() {
    use crate::PostgresValue;
    use postgres_types::{ToSql, Type, WrongType};

    let value = PostgresValue("12.50".parse::<BigDecimal>().unwrap().into());
    for ty in [Type::FLOAT4, Type::INT8, Type::TEXT] {
        let error = value
            .to_sql_checked(&ty, &mut BytesMut::new())
            .err()
            .unwrap();
        assert!(error.is::<WrongType>(), "{ty}: {error}");
    }
}
