use super::*;

type_to_value!(IpNetwork, IpNetwork, Inet);

impl Value {
    #[inline]
    pub fn ip_network<T: Into<Option<IpNetwork>>>(value: T) -> Self {
        Self::IpNetwork(value.into())
    }

    pub fn is_ipnetwork(&self) -> bool {
        matches!(self, Self::IpNetwork(_))
    }

    pub fn as_ref_ipnetwork(&self) -> Option<&IpNetwork> {
        match self {
            Self::IpNetwork(v) => v.as_ref(),
            _ => panic!("not Value::IpNetwork"),
        }
    }

    pub fn as_ipaddr(&self) -> Option<IpAddr> {
        match self {
            Self::IpNetwork(v) => v.as_ref().map(|v| v.network()),
            _ => panic!("not Value::IpNetwork"),
        }
    }
}
