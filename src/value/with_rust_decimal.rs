use super::*;

type_to_value!(Decimal, Decimal, Decimal(None));

impl Value {
    pub fn is_decimal(&self) -> bool {
        matches!(self, Self::Decimal(_))
    }

    /// # Panics
    ///
    /// Panics if self is not [`Value::Decimal`]
    pub fn as_ref_decimal(&self) -> Option<&Decimal> {
        match self {
            Self::Decimal(v) => v.as_ref(),
            _ => panic!("not Value::Decimal"),
        }
    }

    /// # Panics
    ///
    /// Panics if the conversion to [`f64`] fails
    pub fn decimal_to_f64(&self) -> Option<f64> {
        use rust_decimal::prelude::ToPrimitive;

        self.as_ref_decimal().map(|d| d.to_f64().unwrap())
    }
}
