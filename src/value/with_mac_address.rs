use super::*;

type_to_value!(MacAddress, MacAddress, MacAddr);

impl Value {
    #[inline]
    pub fn mac_address<T: Into<Option<MacAddress>>>(value: T) -> Self {
        Self::MacAddress(value.into())
    }

    pub fn is_mac_address(&self) -> bool {
        matches!(self, Self::MacAddress(_))
    }

    pub fn as_ref_mac_address(&self) -> Option<&MacAddress> {
        match self {
            Self::MacAddress(v) => v.as_ref(),
            _ => panic!("not Value::MacAddress"),
        }
    }
}
