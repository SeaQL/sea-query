use ordered_float::{FloatCore, OrderedFloat};

use super::Array;

#[inline]
fn map_option_ordered_float_vec<T>(
    vec: &[Option<T>],
) -> impl Iterator<Item = Option<OrderedFloat<T>>> + '_
where
    T: FloatCore,
{
    vec.iter().copied().map(|x| x.map(OrderedFloat))
}

#[inline]
fn cmp_option_ordered_float_vec<T>(left: &[Option<T>], right: &[Option<T>]) -> bool
where
    T: FloatCore,
{
    map_option_ordered_float_vec(left).eq(map_option_ordered_float_vec(right))
}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::TinyInt(l0), Self::TinyInt(r0)) => l0 == r0,
            (Self::SmallInt(l0), Self::SmallInt(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::BigInt(l0), Self::BigInt(r0)) => l0 == r0,
            (Self::TinyUnsigned(l0), Self::TinyUnsigned(r0)) => l0 == r0,
            (Self::SmallUnsigned(l0), Self::SmallUnsigned(r0)) => l0 == r0,
            (Self::Unsigned(l0), Self::Unsigned(r0)) => l0 == r0,
            (Self::BigUnsigned(l0), Self::BigUnsigned(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => cmp_option_ordered_float_vec(l0, r0),
            (Self::Double(l0), Self::Double(r0)) => cmp_option_ordered_float_vec(l0, r0),
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::Bytes(l0), Self::Bytes(r0)) => l0 == r0,
            (Self::Enum(l0), Self::Enum(r0)) => l0 == r0,
            #[cfg(feature = "with-json")]
            (Self::Json(l0), Self::Json(r0)) => l0 == r0,
            #[cfg(feature = "with-chrono")]
            (Self::ChronoDate(l0), Self::ChronoDate(r0)) => l0 == r0,
            #[cfg(feature = "with-chrono")]
            (Self::ChronoTime(l0), Self::ChronoTime(r0)) => l0 == r0,
            #[cfg(feature = "with-chrono")]
            (Self::ChronoDateTime(l0), Self::ChronoDateTime(r0)) => l0 == r0,
            #[cfg(feature = "with-chrono")]
            (Self::ChronoDateTimeUtc(l0), Self::ChronoDateTimeUtc(r0)) => l0 == r0,
            #[cfg(feature = "with-chrono")]
            (Self::ChronoDateTimeLocal(l0), Self::ChronoDateTimeLocal(r0)) => l0 == r0,
            #[cfg(feature = "with-chrono")]
            (Self::ChronoDateTimeWithTimeZone(l0), Self::ChronoDateTimeWithTimeZone(r0)) => {
                l0 == r0
            }
            #[cfg(feature = "with-time")]
            (Self::TimeDate(l0), Self::TimeDate(r0)) => l0 == r0,
            #[cfg(feature = "with-time")]
            (Self::TimeTime(l0), Self::TimeTime(r0)) => l0 == r0,
            #[cfg(feature = "with-time")]
            (Self::TimeDateTime(l0), Self::TimeDateTime(r0)) => l0 == r0,
            #[cfg(feature = "with-time")]
            (Self::TimeDateTimeWithTimeZone(l0), Self::TimeDateTimeWithTimeZone(r0)) => l0 == r0,
            #[cfg(feature = "with-jiff")]
            (Self::JiffDate(l0), Self::JiffDate(r0)) => l0 == r0,
            #[cfg(feature = "with-jiff")]
            (Self::JiffTime(l0), Self::JiffTime(r0)) => l0 == r0,
            #[cfg(feature = "with-jiff")]
            (Self::JiffDateTime(l0), Self::JiffDateTime(r0)) => l0 == r0,
            #[cfg(feature = "with-jiff")]
            (Self::JiffTimestamp(l0), Self::JiffTimestamp(r0)) => l0 == r0,
            #[cfg(feature = "with-jiff")]
            (Self::JiffZoned(l0), Self::JiffZoned(r0)) => l0 == r0,
            #[cfg(feature = "with-uuid")]
            (Self::Uuid(l0), Self::Uuid(r0)) => l0 == r0,
            #[cfg(feature = "with-rust_decimal")]
            (Self::Decimal(l0), Self::Decimal(r0)) => l0 == r0,
            #[cfg(feature = "with-bigdecimal")]
            (Self::BigDecimal(l0), Self::BigDecimal(r0)) => l0 == r0,
            #[cfg(feature = "with-ipnetwork")]
            (Self::IpNetwork(l0), Self::IpNetwork(r0)) => l0 == r0,
            #[cfg(feature = "with-mac_address")]
            (Self::MacAddress(l0), Self::MacAddress(r0)) => l0 == r0,
            (Self::Null(l0), Self::Null(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Array {}

impl std::hash::Hash for Array {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        use ordered_float::OrderedFloat;

        std::mem::discriminant(self).hash(state);
        match self {
            Array::Bool(items) => items.hash(state),
            Array::TinyInt(items) => items.hash(state),
            Array::SmallInt(items) => items.hash(state),
            Array::Int(items) => items.hash(state),
            Array::BigInt(items) => items.hash(state),
            Array::TinyUnsigned(items) => items.hash(state),
            Array::SmallUnsigned(items) => items.hash(state),
            Array::Unsigned(items) => items.hash(state),
            Array::BigUnsigned(items) => items.hash(state),
            Array::Float(items) => {
                for x in items.iter() {
                    x.map(OrderedFloat).hash(state)
                }
            }
            Array::Double(items) => {
                for x in items.iter() {
                    x.map(OrderedFloat).hash(state)
                }
            }
            Array::String(items) => items.hash(state),
            Array::Char(items) => items.hash(state),
            Array::Bytes(items) => items.hash(state),
            Array::Enum(items) => items.hash(state),
            #[cfg(feature = "with-json")]
            Array::Json(items) => items.hash(state),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDate(items) => items.hash(state),
            #[cfg(feature = "with-chrono")]
            Array::ChronoTime(items) => items.hash(state),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTime(items) => items.hash(state),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeUtc(items) => items.hash(state),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeLocal(items) => items.hash(state),
            #[cfg(feature = "with-chrono")]
            Array::ChronoDateTimeWithTimeZone(items) => items.hash(state),
            #[cfg(feature = "with-time")]
            Array::TimeDate(items) => items.hash(state),
            #[cfg(feature = "with-time")]
            Array::TimeTime(items) => items.hash(state),
            #[cfg(feature = "with-time")]
            Array::TimeDateTime(items) => items.hash(state),
            #[cfg(feature = "with-time")]
            Array::TimeDateTimeWithTimeZone(items) => items.hash(state),
            #[cfg(feature = "with-jiff")]
            Array::JiffDate(items) => items.hash(state),
            #[cfg(feature = "with-jiff")]
            Array::JiffTime(items) => items.hash(state),
            #[cfg(feature = "with-jiff")]
            Array::JiffDateTime(items) => items.hash(state),
            #[cfg(feature = "with-jiff")]
            Array::JiffTimestamp(items) => items.hash(state),
            #[cfg(feature = "with-jiff")]
            Array::JiffZoned(items) => items.hash(state),
            #[cfg(feature = "with-uuid")]
            Array::Uuid(items) => items.hash(state),
            #[cfg(feature = "with-rust_decimal")]
            Array::Decimal(items) => items.hash(state),
            #[cfg(feature = "with-bigdecimal")]
            Array::BigDecimal(items) => items.hash(state),
            #[cfg(feature = "with-ipnetwork")]
            Array::IpNetwork(items) => items.hash(state),
            #[cfg(feature = "with-mac_address")]
            Array::MacAddress(items) => items.hash(state),
            Array::Null(ty) => ty.hash(state),
        }
    }
}
