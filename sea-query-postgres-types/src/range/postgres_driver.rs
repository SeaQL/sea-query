use super::*;
use bytes::BytesMut;
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
                encode_range(lower, upper, buf, numeric_to_sql)?;
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

/// Encodes a `numrange` bound with `rust_decimal`'s `numeric` encoder.
///
/// The bound goes through the shortest decimal representation of the `f64`, so `0.1` is sent as
/// `0.1` (the same digits `Display` renders), like `sea-query-sqlx` does.
#[cfg(feature = "with-rust_decimal")]
fn numeric_to_sql(v: &f64, buf: &mut BytesMut) -> Result<(), BoxError> {
    let decimal: rust_decimal::Decimal = v
        .to_string()
        .parse()
        .map_err(|e| format!("invalid numrange bound {v}: {e}"))?;
    decimal.to_sql(&Type::NUMERIC, buf)?;
    Ok(())
}

#[cfg(not(feature = "with-rust_decimal"))]
fn numeric_to_sql(_: &f64, _: &mut BytesMut) -> Result<(), BoxError> {
    Err("numrange values require the with-rust_decimal feature".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(range: RangeType, ty: &Type) -> Result<Vec<u8>, BoxError> {
        let mut buf = BytesMut::new();
        range.to_sql_checked(ty, &mut buf)?;
        Ok(buf.to_vec())
    }

    #[test]
    fn unbounded_range_is_not_empty() {
        // flags 0x18 = lower and upper unbounded, 0x01 would be `empty`
        let range = RangeType::Int4Range(RangeBound::Unbounded, RangeBound::Unbounded);
        assert_eq!(encode(range, &Type::INT4_RANGE).unwrap(), vec![0x18]);
    }

    #[test]
    fn range_variant_must_match_type() {
        let int4 = RangeType::Int4Range(RangeBound::Inclusive(1), RangeBound::Exclusive(2));
        assert!(encode(int4.clone(), &Type::INT4_RANGE).is_ok());
        assert!(encode(int4, &Type::INT8_RANGE).is_err());

        let num = RangeType::NumRange(RangeBound::Inclusive(0.5), RangeBound::Unbounded);
        assert!(encode(num, &Type::INT8_RANGE).is_err());
    }

    #[test]
    fn numrange_bounds() {
        let num = |v| RangeType::NumRange(RangeBound::Inclusive(v), RangeBound::Unbounded);
        #[cfg(feature = "with-rust_decimal")]
        {
            assert!(encode(num(0.1), &Type::NUM_RANGE).is_ok());
            assert!(encode(num(f64::NAN), &Type::NUM_RANGE).is_err());
        }
        #[cfg(not(feature = "with-rust_decimal"))]
        assert!(encode(num(0.1), &Type::NUM_RANGE).is_err());
    }
}
