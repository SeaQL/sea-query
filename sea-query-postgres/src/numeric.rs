#[cfg(test)]
mod tests;

use std::error::Error;

use bigdecimal::BigDecimal;
use bigdecimal::num_bigint::Sign;
use bytes::{BufMut, BytesMut};

// Constants from PostgreSQL's numeric.c.
const NUMERIC_POS: u16 = 0x0000;
const NUMERIC_NEG: u16 = 0x4000;
const NUMERIC_DSCALE_MAX: i64 = 0x3FFF;

/// Encode a `BigDecimal` in PostgreSQL's binary `numeric` format.
pub(crate) fn to_sql(
    value: &BigDecimal,
    out: &mut BytesMut,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    // The value is integer * 10^-exponent.
    let (integer, exponent) = value.as_bigint_and_exponent();
    let (sign, decimal) = integer.to_radix_be(10);
    if sign == Sign::NoSign {
        // Clamping the display scale preserves zero's value.
        out.put_u16(0);
        out.put_i16(0);
        out.put_u16(NUMERIC_POS);
        out.put_u16(u16::try_from(exponent.clamp(0, NUMERIC_DSCALE_MAX))?);
        return Ok(());
    }
    let scale = exponent.max(0);
    if scale > NUMERIC_DSCALE_MAX {
        return Err(format!(
            "numeric scale {scale} is out of range (at most {NUMERIC_DSCALE_MAX})"
        )
        .into());
    }
    // Decimal exponent of the most significant digit.
    let top = i64::try_from(decimal.len())?
        .checked_sub(exponent)
        .and_then(|n| n.checked_sub(1))
        .ok_or("numeric value is out of range")?;
    let weight = i16::try_from(top.div_euclid(4)).map_err(|_| "numeric value is out of range")?;
    // Align base-10000 groups with the decimal point.
    let first = usize::try_from(top.rem_euclid(4))? + 1;
    let (head, tail) = decimal.split_at(first.min(decimal.len()));
    let group = |chunk: &[u8], width: usize| {
        chunk
            .iter()
            .copied()
            .chain(std::iter::repeat(0))
            .take(width)
            .fold(0i16, |n, d| n * 10 + i16::from(d))
    };
    let mut digits = Vec::with_capacity(decimal.len() / 4 + 2);
    digits.push(group(head, first));
    digits.extend(tail.chunks(4).map(|chunk| group(chunk, 4)));
    while digits.last() == Some(&0) {
        digits.pop();
    }
    out.put_u16(u16::try_from(digits.len()).map_err(|_| "numeric value has too many digits")?);
    out.put_i16(weight);
    out.put_u16(if sign == Sign::Minus {
        NUMERIC_NEG
    } else {
        NUMERIC_POS
    });
    out.put_u16(u16::try_from(scale)?);
    for digit in digits {
        out.put_i16(digit);
    }
    Ok(())
}
