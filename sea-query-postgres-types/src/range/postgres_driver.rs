use super::*;
use bytes::{BufMut, BytesMut};
use postgres_protocol::types;
use postgres_types::{Kind, ToSql, Type, to_sql_checked};

type BoxError = Box<dyn std::error::Error + Sync + Send>;

impl ToSql for RangeType {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut BytesMut,
    ) -> Result<postgres_types::IsNull, BoxError>
    where
        Self: Sized,
    {
        let element_type = match *ty.kind() {
            Kind::Range(ref ty) => ty,
            _ => return Err(format!("unexpected type {:?}", ty).into()),
        };

        // The bounds are always sent as they are, PostgreSQL decides whether the range is
        // empty (e.g. `[1,1)`). `(,)` is the range of all values, as in `Display` and SQLx.
        match self {
            RangeType::Int4Range(lower, upper) => {
                expect_element_type(ty, element_type, &Type::INT4)?;
                encode_range(lower, upper, buf, |v, buf| {
                    types::int4_to_sql(*v, buf);
                    Ok(())
                })?;
            }
            RangeType::Int8Range(lower, upper) => {
                expect_element_type(ty, element_type, &Type::INT8)?;
                encode_range(lower, upper, buf, |v, buf| {
                    types::int8_to_sql(*v, buf);
                    Ok(())
                })?;
            }
            RangeType::NumRange(lower, upper) => {
                expect_element_type(ty, element_type, &Type::NUMERIC)?;
                encode_range(lower, upper, buf, |v, buf| numeric_to_sql(*v, buf))?;
            }
        }

        Ok(postgres_types::IsNull::No)
    }

    fn accepts(ty: &postgres_types::Type) -> bool
    where
        Self: Sized,
    {
        matches!(ty.kind(), &Kind::Range(_))
    }

    to_sql_checked!();
}

fn expect_element_type(ty: &Type, element_type: &Type, expected: &Type) -> Result<(), BoxError> {
    if element_type == expected {
        Ok(())
    } else {
        Err(format!("cannot encode a range of {expected} as {ty}").into())
    }
}

fn encode_range<T: Clone + Display>(
    lower: &RangeBound<T>,
    upper: &RangeBound<T>,
    buf: &mut BytesMut,
    encode: impl Fn(&T, &mut BytesMut) -> Result<(), BoxError>,
) -> Result<(), BoxError> {
    types::range_to_sql(
        |buf| bound_to_sql(lower, buf, &encode),
        |buf| bound_to_sql(upper, buf, &encode),
        buf,
    )
}

fn bound_to_sql<T: Clone + Display>(
    bound: &RangeBound<T>,
    buf: &mut BytesMut,
    encode: impl Fn(&T, &mut BytesMut) -> Result<(), BoxError>,
) -> Result<types::RangeBound<postgres_protocol::IsNull>, BoxError> {
    match bound {
        RangeBound::Exclusive(v) => {
            encode(v, buf)?;
            Ok(types::RangeBound::Exclusive(postgres_protocol::IsNull::No))
        }
        RangeBound::Inclusive(v) => {
            encode(v, buf)?;
            Ok(types::RangeBound::Inclusive(postgres_protocol::IsNull::No))
        }
        RangeBound::Unbounded => Ok(types::RangeBound::Unbounded),
    }
}

/// Writes `v` in the binary format of PostgreSQL's `numeric` type.
///
/// The value is taken from the shortest decimal representation of the `f64`, so `0.1` is sent
/// as `0.1` (the same digits `Display` renders) rather than the exact binary value.
fn numeric_to_sql(v: f64, buf: &mut BytesMut) -> Result<(), BoxError> {
    if !v.is_finite() {
        return Err(format!("invalid numrange bound {v}: bounds must be finite").into());
    }

    // `Display` for f64 never uses exponent notation
    let repr = v.abs().to_string();
    let (int_part, frac_part) = repr.split_once('.').unwrap_or((&repr, ""));

    // numeric stores base 10000 digits, so align both parts to groups of 4 decimal digits
    let int_pad = (4 - int_part.len() % 4) % 4;
    let frac_pad = (4 - frac_part.len() % 4) % 4;
    let decimal_digits: Vec<i16> = std::iter::repeat_n(0, int_pad)
        .chain(int_part.bytes().map(|b| (b - b'0') as i16))
        .chain(frac_part.bytes().map(|b| (b - b'0') as i16))
        .chain(std::iter::repeat_n(0, frac_pad))
        .collect();
    let groups: Vec<i16> = decimal_digits
        .chunks(4)
        .map(|c| c.iter().fold(0, |acc, d| acc * 10 + d))
        .collect();

    let mut weight = ((int_pad + int_part.len()) / 4) as i32 - 1;
    let mut digits = groups.as_slice();
    while let [0, rest @ ..] = digits {
        digits = rest;
        weight -= 1;
    }
    while let [rest @ .., 0] = digits {
        digits = rest;
    }
    if digits.is_empty() {
        weight = 0;
    }

    let sign: u16 = if v.is_sign_negative() && !digits.is_empty() {
        0x4000
    } else {
        0x0000
    };

    buf.put_i16(i16::try_from(digits.len())?);
    buf.put_i16(i16::try_from(weight)?);
    buf.put_u16(sign);
    buf.put_u16(u16::try_from(frac_part.len())?);
    for digit in digits {
        buf.put_i16(*digit);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn numeric(v: f64) -> Vec<u8> {
        let mut buf = BytesMut::new();
        numeric_to_sql(v, &mut buf).unwrap();
        buf.to_vec()
    }

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn numeric_matches_postgres_numeric_send() {
        // expected bytes from `SELECT encode(numeric_send(<v>::numeric), 'hex')`
        assert_eq!(numeric(0.1), hex("0001ffff0000000103e8"));
        assert_eq!(numeric(-1234.5678), hex("000200004000000404d2162e"));
        assert_eq!(numeric(1e20), hex("00010005000000000001"));
        assert_eq!(numeric(0.0), hex("0000000000000000"));
        assert_eq!(numeric(-0.0), hex("0000000000000000"));
        assert_eq!(numeric(1.5), hex("000200000000000100011388"));
        assert_eq!(numeric(12345.0), hex("000200010000000000010929"));
        assert_eq!(numeric(0.00012), hex("0002ffff00000005000107d0"));
    }

    #[test]
    fn numeric_rejects_non_finite() {
        let mut buf = BytesMut::new();
        assert!(numeric_to_sql(f64::NAN, &mut buf).is_err());
        assert!(numeric_to_sql(f64::INFINITY, &mut buf).is_err());
        assert!(numeric_to_sql(f64::NEG_INFINITY, &mut buf).is_err());
    }

    fn encode(range: RangeType, ty: &Type) -> Result<Vec<u8>, BoxError> {
        let mut buf = BytesMut::new();
        range.to_sql_checked(ty, &mut buf)?;
        Ok(buf.to_vec())
    }

    #[test]
    fn unbounded_range_is_not_empty() {
        // flags 0x18 = lower and upper unbounded, 0x01 would be `empty`
        assert_eq!(
            encode(
                RangeType::Int4Range(RangeBound::Unbounded, RangeBound::Unbounded),
                &Type::INT4_RANGE
            )
            .unwrap(),
            vec![0x18]
        );
    }

    #[test]
    fn range_variant_must_match_type() {
        let int4 = RangeType::Int4Range(RangeBound::Inclusive(1), RangeBound::Exclusive(2));
        assert!(encode(int4.clone(), &Type::INT4_RANGE).is_ok());
        assert!(encode(int4.clone(), &Type::INT8_RANGE).is_err());
        assert!(encode(int4, &Type::NUM_RANGE).is_err());

        let num = RangeType::NumRange(RangeBound::Inclusive(0.5), RangeBound::Unbounded);
        assert!(encode(num.clone(), &Type::NUM_RANGE).is_ok());
        assert!(encode(num, &Type::INT8_RANGE).is_err());
    }
}
