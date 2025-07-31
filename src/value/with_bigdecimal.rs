use super::*;

type_to_box_value!(BigDecimal, BigDecimal, Decimal(None));

impl Value {
    pub fn is_big_decimal(&self) -> bool {
        matches!(self, Self::BigDecimal(_))
    }

    pub fn as_ref_big_decimal(&self) -> Option<&BigDecimal> {
        match self {
            Self::BigDecimal(v) => v.as_ref().map(|x| x.as_ref()),
            _ => panic!("not Value::BigDecimal"),
        }
    }

    pub fn big_decimal_to_f64(&self) -> Option<f64> {
        use bigdecimal::ToPrimitive;
        self.as_ref_big_decimal().map(|d| d.to_f64().unwrap())
    }
}
