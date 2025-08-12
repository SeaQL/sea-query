use super::*;

type_to_box_value!(BigDecimal, BigDecimal, Decimal(None));

impl Value {
    pub fn is_big_decimal(&self) -> bool {
        matches!(self, Self::BigDecimal(_))
    }

    /// # Panics
    ///
    /// Panics if self is not [`Value::BigDecimal`]
    pub fn as_ref_big_decimal(&self) -> Option<&BigDecimal> {
        match self {
            Self::BigDecimal(v) => v.as_ref().map(AsRef::as_ref),
            _ => panic!("not Value::BigDecimal"),
        }
    }

    /// # Panics
    ///
    /// Panics if the conversion to [`f64`] fails
    pub fn big_decimal_to_f64(&self) -> Option<f64> {
        use bigdecimal::ToPrimitive;
        self.as_ref_big_decimal().map(|d| d.to_f64().unwrap())
    }
}
