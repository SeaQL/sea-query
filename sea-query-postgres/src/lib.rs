#![forbid(unsafe_code)]

use std::error::Error;

use bytes::BytesMut;
use postgres_types::{IsNull, ToSql, Type, to_sql_checked};

use sea_query::{ArrayType, OptionEnum, QueryBuilder, Value, query::*};

#[derive(Clone, Debug, PartialEq)]
pub struct PostgresValue(pub Value);
#[derive(Clone, Debug, PartialEq)]
pub struct PostgresValues(pub Vec<PostgresValue>);

impl PostgresValues {
    pub fn as_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        self.0
            .iter()
            .map(|x| {
                let y: &(dyn ToSql + Sync) = x;
                y
            })
            .collect()
    }

    pub fn as_types(&self) -> Vec<Type> {
        self.0
            .iter()
            .map(|x| &x.0)
            .map(value_to_postgres_type)
            .collect()
    }
}

pub trait PostgresBinder {
    fn build_postgres<T: QueryBuilder>(&self, query_builder: T) -> (String, PostgresValues);
}

macro_rules! impl_postgres_binder {
    ($l:ident) => {
        impl PostgresBinder for $l {
            fn build_postgres<T: QueryBuilder>(
                &self,
                query_builder: T,
            ) -> (String, PostgresValues) {
                let (query, values) = self.build(query_builder);
                (
                    query,
                    PostgresValues(values.into_iter().map(PostgresValue).collect()),
                )
            }
        }
    };
}

impl_postgres_binder!(SelectStatement);
impl_postgres_binder!(UpdateStatement);
impl_postgres_binder!(InsertStatement);
impl_postgres_binder!(DeleteStatement);
impl_postgres_binder!(WithQuery);

impl ToSql for PostgresValue {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        macro_rules! to_sql {
            ( $v: expr, $ty: ty ) => {
                $v.map(|v| v as $ty).as_ref().to_sql(ty, out)
            };
        }
        match &self.0 {
            Value::Bool(v) => to_sql!(v, bool),
            Value::TinyInt(v) => to_sql!(v, i8),
            Value::SmallInt(v) => to_sql!(v, i16),
            Value::Int(v) => to_sql!(v, i32),
            Value::BigInt(v) => to_sql!(v, i64),
            Value::TinyUnsigned(v) => to_sql!(v, u32),
            Value::SmallUnsigned(v) => to_sql!(v, u32),
            Value::Unsigned(v) => to_sql!(v, u32),
            Value::BigUnsigned(v) => to_sql!(v, i64),
            Value::Float(v) => to_sql!(v, f32),
            Value::Double(v) => to_sql!(v, f64),
            Value::String(v) => v.as_deref().to_sql(ty, out),
            Value::Enum(v) => match v {
                OptionEnum::Some(v) => Some(v.value.as_ref()).to_sql(ty, out),
                OptionEnum::None(_) => Option::<&str>::None.to_sql(ty, out),
            },
            Value::Char(v) => v.map(|v| v.to_string()).to_sql(ty, out),
            Value::Bytes(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-json")]
            Value::Json(v) => v.as_deref().to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDate(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-time")]
            Value::TimeDate(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-time")]
            Value::TimeTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-time")]
            Value::TimeDateTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-jiff")]
            Value::JiffDate(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-jiff")]
            Value::JiffTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-jiff")]
            Value::JiffDateTime(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-jiff")]
            Value::JiffTimestamp(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-rust_decimal")]
            Value::Decimal(v) => v.to_sql(ty, out),
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(None) => Ok(IsNull::Yes),
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(Some(v)) if *ty == Type::NUMERIC => {
                bigdecimal_numeric::to_sql(v, out)?;
                Ok(IsNull::No)
            }
            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(Some(v)) => {
                use bigdecimal::ToPrimitive;
                v.to_f64()
                    .expect("Fail to convert bigdecimal as f64")
                    .to_sql(ty, out)
            }
            #[cfg(feature = "with-uuid")]
            Value::Uuid(v) => v.to_sql(ty, out),
            #[cfg(feature = "postgres-array")]
            Value::Array(_, Some(v)) => v
                .iter()
                .map(|v| PostgresValue(v.clone()))
                .collect::<Vec<PostgresValue>>()
                .to_sql(ty, out),
            #[cfg(feature = "postgres-array")]
            Value::Array(_, None) => Ok(IsNull::Yes),
            #[cfg(feature = "postgres-vector")]
            Value::Vector(Some(v)) => v.to_sql(ty, out),
            #[cfg(feature = "postgres-vector")]
            Value::Vector(None) => Ok(IsNull::Yes),
            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(v) => {
                use cidr::IpCidr;
                v.map(|v| {
                    IpCidr::new(v.network(), v.prefix())
                        .expect("Fail to convert IpNetwork to IpCidr")
                })
                .to_sql(ty, out)
            }
            #[cfg(feature = "with-mac_address")]
            Value::MacAddress(v) => {
                use eui48::MacAddress;
                v.map(|v| MacAddress::new(v.bytes())).to_sql(ty, out)
            }
            #[cfg(feature = "postgres-range")]
            Value::Range(None) => Ok(IsNull::Yes),
            #[cfg(feature = "postgres-range")]
            Value::Range(Some(v)) => v.to_sql(ty, out),
        }
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }

    to_sql_checked!();
}

/// Binary encoding of `BigDecimal` for `NUMERIC` parameters, so the value reaches the server
/// exactly: all digits and the scale are kept (`1.2300` stays `1.2300`), which an `f64` cannot do.
#[cfg(feature = "with-bigdecimal")]
mod bigdecimal_numeric {
    use std::error::Error;

    use bigdecimal::BigDecimal;
    use bigdecimal::num_bigint::Sign;
    use bytes::{BufMut, BytesMut};

    /// `NUMERIC_POS` and `NUMERIC_NEG` of PostgreSQL's `numeric.c`.
    const POSITIVE: u16 = 0x0000;
    const NEGATIVE: u16 = 0x4000;
    /// `NUMERIC_DSCALE_MAX`: the most fraction digits a `numeric` keeps.
    const DSCALE_MAX: i64 = 0x3FFF;
    /// The most digits a `numeric` has before the decimal point.
    const INTEGER_DIGITS_MAX: i64 = 131072;

    /// Writes `value` in the layout of PostgreSQL's `numeric_send`: number of base-10000 digits,
    /// weight of the first digit, sign, display scale, then the digits. A negative scale (`1E+3`)
    /// is written as the equivalent integer with scale 0.
    pub(crate) fn to_sql(
        value: &BigDecimal,
        out: &mut BytesMut,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let (integer, scale) = value.as_bigint_and_exponent();
        let (sign, magnitude) = integer.into_parts();
        let mut decimal = magnitude.to_str_radix(10);
        if scale < -INTEGER_DIGITS_MAX {
            return Err(format!("numeric exponent {} is out of range", -scale).into());
        }
        let scale = if scale < 0 {
            decimal.extend(std::iter::repeat_n('0', usize::try_from(-scale)?));
            0
        } else {
            scale
        };
        if scale > DSCALE_MAX {
            return Err(
                format!("numeric scale {scale} is out of range (at most {DSCALE_MAX})").into(),
            );
        }
        let fraction = usize::try_from(scale)?;
        // whole groups of four on both sides of the point
        let mut digits = String::new();
        let integer_len = decimal.len().saturating_sub(fraction);
        digits.extend(std::iter::repeat_n('0', (4 - integer_len % 4) % 4));
        digits.extend(std::iter::repeat_n(
            '0',
            fraction.saturating_sub(decimal.len()),
        ));
        digits.push_str(&decimal);
        digits.extend(std::iter::repeat_n('0', (4 - fraction % 4) % 4));
        let groups: Vec<i16> = digits
            .as_bytes()
            .chunks(4)
            .map(|group| group.iter().fold(0i16, |n, d| n * 10 + i16::from(d - b'0')))
            .collect();
        let integer_groups = i64::try_from(integer_len.div_ceil(4))?;
        let leading = groups.iter().take_while(|g| **g == 0).count();
        let trailing = groups[leading..]
            .iter()
            .rev()
            .take_while(|g| **g == 0)
            .count();
        let groups = &groups[leading..groups.len() - trailing];
        let (weight, sign) = if groups.is_empty() {
            (0, POSITIVE)
        } else {
            let weight = integer_groups - 1 - i64::try_from(leading)?;
            (
                weight,
                if sign == Sign::Minus {
                    NEGATIVE
                } else {
                    POSITIVE
                },
            )
        };
        out.put_i16(i16::try_from(groups.len()).map_err(|_| "numeric value has too many digits")?);
        out.put_i16(i16::try_from(weight).map_err(|_| "numeric value is out of range")?);
        out.put_u16(sign);
        out.put_u16(u16::try_from(scale)?);
        for group in groups {
            out.put_i16(*group);
        }
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use std::str::FromStr;

        use super::*;

        fn encode(text: &str) -> Vec<i16> {
            let mut out = BytesMut::new();
            to_sql(&BigDecimal::from_str(text).unwrap(), &mut out).unwrap();
            out.chunks(2)
                .map(|b| i16::from_be_bytes([b[0], b[1]]))
                .collect()
        }

        #[test]
        fn numeric_send_layout() {
            // ndigits, weight, sign, dscale, digits
            assert_eq!(encode("0"), [0, 0, 0, 0]);
            assert_eq!(encode("0.00"), [0, 0, 0, 2]);
            assert_eq!(encode("12.50"), [2, 0, 0, 2, 12, 5000]);
            assert_eq!(encode("-12.5"), [2, 0, 0x4000, 1, 12, 5000]);
            assert_eq!(encode("10000"), [1, 1, 0, 0, 1]);
            assert_eq!(encode("1E+3"), [1, 0, 0, 0, 1000]);
            assert_eq!(encode("0.00012"), [2, -1, 0, 5, 1, 2000]);
            assert_eq!(
                encode("123456789012345678.0123456789"),
                [8, 4, 0, 10, 12, 3456, 7890, 1234, 5678, 123, 4567, 8900]
            );
        }

        #[test]
        fn out_of_range() {
            for text in ["1E+131073", "1E-16384"] {
                let value = BigDecimal::from_str(text).unwrap();
                assert!(to_sql(&value, &mut BytesMut::new()).is_err(), "{text}");
            }
            let mut out = BytesMut::new();
            to_sql(&BigDecimal::from_str("1E-16383").unwrap(), &mut out).unwrap();
            to_sql(&BigDecimal::from_str("1E+131071").unwrap(), &mut out).unwrap();
        }

        #[test]
        fn bound_by_type() {
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
            // other parameter types keep the `f64` conversion
            let value_f64 = bigdecimal::ToPrimitive::to_f64(&value).unwrap();
            let (_, float) = bind(value.into(), &Type::FLOAT8);
            assert_eq!(float, value_f64.to_be_bytes());
        }
    }
}

fn value_to_postgres_type(value: &Value) -> Type {
    match value {
        Value::Bool(_) => Type::BOOL,
        Value::TinyInt(_) => Type::INT2,
        Value::TinyUnsigned(_) => Type::INT2,
        Value::SmallInt(_) => Type::INT2,
        Value::SmallUnsigned(_) => Type::INT4,
        Value::Int(_) => Type::INT4,
        Value::BigInt(_) => Type::INT8,
        Value::Unsigned(_) => Type::INT8,
        Value::BigUnsigned(_) => Type::NUMERIC,
        Value::Float(_) => Type::FLOAT4,
        Value::Double(_) => Type::FLOAT8,
        Value::String(_) => Type::TEXT,
        Value::Enum(_) => Type::TEXT,
        #[cfg(feature = "postgres-range")]
        Value::Range(_) => Type::INT8_RANGE,
        Value::Char(_) => Type::CHAR,
        Value::Bytes(_) => Type::BYTEA,
        #[cfg(feature = "with-json")]
        Value::Json(_) => Type::JSON,
        #[cfg(feature = "with-chrono")]
        Value::ChronoDate(_) => Type::DATE,
        #[cfg(feature = "with-chrono")]
        Value::ChronoTime(_) => Type::TIME,
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTime(_) => Type::TIMESTAMP,
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeUtc(_) => Type::TIMESTAMP,
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeLocal(_) => Type::TIMESTAMP,
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeWithTimeZone(_) => Type::TIMESTAMPTZ,
        #[cfg(feature = "with-time")]
        Value::TimeDate(_) => Type::DATE,
        #[cfg(feature = "with-time")]
        Value::TimeTime(_) => Type::TIME,
        #[cfg(feature = "with-time")]
        Value::TimeDateTime(_) => Type::TIMESTAMP,
        #[cfg(feature = "with-time")]
        Value::TimeDateTimeWithTimeZone(_) => Type::TIMESTAMPTZ,
        #[cfg(feature = "with-jiff")]
        Value::JiffDate(_) => Type::DATE,
        #[cfg(feature = "with-jiff")]
        Value::JiffTime(_) => Type::TIME,
        #[cfg(feature = "with-jiff")]
        Value::JiffDateTime(_) => Type::TIMESTAMP,
        #[cfg(feature = "with-jiff")]
        Value::JiffTimestamp(_) => Type::TIMESTAMPTZ,
        #[cfg(feature = "with-uuid")]
        Value::Uuid(_) => Type::UUID,
        #[cfg(feature = "with-rust_decimal")]
        Value::Decimal(_) => Type::NUMERIC,
        #[cfg(feature = "with-bigdecimal")]
        Value::BigDecimal(_) => Type::NUMERIC,
        #[cfg(feature = "postgres-array")]
        Value::Array(ty, _) => array_type_to_pg_type(ty),
        #[cfg(feature = "postgres-vector")]
        Value::Vector(_) => Type::FLOAT4_ARRAY,
        #[cfg(feature = "with-ipnetwork")]
        Value::IpNetwork(_) => Type::INET,
        #[cfg(feature = "with-mac_address")]
        Value::MacAddress(_) => Type::MACADDR,
    }
}

fn array_type_to_pg_type(ty: &ArrayType) -> Type {
    match ty {
        ArrayType::Bool => Type::BOOL_ARRAY,
        ArrayType::TinyInt => Type::INT2_ARRAY,
        ArrayType::TinyUnsigned => Type::INT2_ARRAY,
        ArrayType::SmallInt => Type::INT2_ARRAY,
        ArrayType::SmallUnsigned => Type::INT4_ARRAY,
        ArrayType::Int => Type::INT4_ARRAY,
        ArrayType::Unsigned => Type::INT8_ARRAY,
        ArrayType::BigInt => Type::INT8_ARRAY,
        ArrayType::BigUnsigned => Type::NUMERIC_ARRAY,
        ArrayType::Float => Type::FLOAT4_ARRAY,
        ArrayType::Double => Type::FLOAT8_ARRAY,
        ArrayType::String => Type::TEXT_ARRAY,
        ArrayType::Char => Type::CHAR_ARRAY,
        ArrayType::Bytes => Type::BYTEA_ARRAY,
        #[cfg(feature = "with-json")]
        ArrayType::Json => Type::JSON_ARRAY,
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDate => Type::DATE_ARRAY,
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoTime => Type::TIME_ARRAY,
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTime => Type::TIMESTAMP_ARRAY,
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTimeUtc => Type::TIMESTAMP_ARRAY,
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTimeLocal => Type::TIMESTAMP_ARRAY,
        #[cfg(feature = "with-chrono")]
        ArrayType::ChronoDateTimeWithTimeZone => Type::TIMESTAMPTZ_ARRAY,
        #[cfg(feature = "with-time")]
        ArrayType::TimeDate => Type::DATE_ARRAY,
        #[cfg(feature = "with-time")]
        ArrayType::TimeTime => Type::TIME_ARRAY,
        #[cfg(feature = "with-time")]
        ArrayType::TimeDateTime => Type::TIMESTAMP_ARRAY,
        #[cfg(feature = "with-time")]
        ArrayType::TimeDateTimeWithTimeZone => Type::TIMESTAMPTZ_ARRAY,
        #[cfg(feature = "with-jiff")]
        ArrayType::JiffDate => Type::DATE_ARRAY,
        #[cfg(feature = "with-jiff")]
        ArrayType::JiffTime => Type::TIME_ARRAY,
        #[cfg(feature = "with-jiff")]
        ArrayType::JiffDateTime => Type::TIMESTAMP_ARRAY,
        #[cfg(feature = "with-jiff")]
        ArrayType::JiffTimestamp => Type::TIMESTAMPTZ_ARRAY,
        #[cfg(feature = "with-uuid")]
        ArrayType::Uuid => Type::UUID_ARRAY,
        #[cfg(feature = "with-rust_decimal")]
        ArrayType::Decimal => Type::NUMERIC_ARRAY,
        #[cfg(feature = "with-bigdecimal")]
        ArrayType::BigDecimal => Type::NUMERIC_ARRAY,
        #[cfg(feature = "with-ipnetwork")]
        ArrayType::IpNetwork => Type::INET_ARRAY,
        #[cfg(feature = "with-mac_address")]
        ArrayType::MacAddress => Type::MACADDR_ARRAY,
        ArrayType::Enum(_) => Type::TEXT_ARRAY,
        #[cfg(feature = "postgres-range")]
        ArrayType::Range => Type::INT8_RANGE_ARRAY,
    }
}
