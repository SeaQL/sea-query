use super::*;

type_to_value!(IpNetwork, IpNetwork, Inet);

impl Value {
    pub fn is_ipnetwork(&self) -> bool {
        matches!(self, Self::IpNetwork(_))
    }

    /// # Panics
    ///
    /// Panics if self is not [`Value::IpNetwork`]
    pub fn as_ref_ipnetwork(&self) -> Option<&IpNetwork> {
        match self {
            Self::IpNetwork(v) => v.as_ref(),
            _ => panic!("not Value::IpNetwork"),
        }
    }

    /// # Panics
    ///
    /// Panics if self is not [`Value::IpNetwork`]
    pub fn as_ipaddr(&self) -> Option<IpAddr> {
        match self {
            Self::IpNetwork(v) => v.as_ref().map(ipnetwork::IpNetwork::network),
            _ => panic!("not Value::IpNetwork"),
        }
    }
}
