use super::*;

type_to_value!(Decimal, Decimal, Decimal(None));

impl NumericValue for Decimal {}
impl NumericValueNullable for Option<Decimal> {}

impl Value {
    pub fn is_decimal(&self) -> bool {
        matches!(self, Self::Decimal(_))
    }

    pub fn as_ref_decimal(&self) -> Option<&Decimal> {
        match self {
            Self::Decimal(v) => v.as_ref(),
            _ => panic!("not Value::Decimal"),
        }
    }

    pub fn decimal_to_f64(&self) -> Option<f64> {
        use rust_decimal::prelude::ToPrimitive;

        self.as_ref_decimal().map(|d| d.to_f64().unwrap())
    }
}
