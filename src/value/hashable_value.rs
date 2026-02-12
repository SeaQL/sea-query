use super::*;
use ordered_float::OrderedFloat;
use std::{
    hash::{Hash, Hasher},
    mem,
};

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l), Self::Bool(r)) => l == r,
            (Self::TinyInt(l), Self::TinyInt(r)) => l == r,
            (Self::SmallInt(l), Self::SmallInt(r)) => l == r,
            (Self::Int(l), Self::Int(r)) => l == r,
            (Self::BigInt(l), Self::BigInt(r)) => l == r,
            (Self::TinyUnsigned(l), Self::TinyUnsigned(r)) => l == r,
            (Self::SmallUnsigned(l), Self::SmallUnsigned(r)) => l == r,
            (Self::Unsigned(l), Self::Unsigned(r)) => l == r,
            (Self::BigUnsigned(l), Self::BigUnsigned(r)) => l == r,
            (Self::Float(l), Self::Float(r)) => cmp_f32(l, r),
            (Self::Double(l), Self::Double(r)) => cmp_f64(l, r),
            (Self::String(l), Self::String(r)) => l == r,
            (Self::Enum(l), Self::Enum(r)) => l == r,
            (Self::Char(l), Self::Char(r)) => l == r,
            (Self::Bytes(l), Self::Bytes(r)) => l == r,

            #[cfg(feature = "with-json")]
            (Self::Json(l), Self::Json(r)) => cmp_json(l, r),

            #[cfg(feature = "with-chrono")]
            (Self::ChronoDate(l), Self::ChronoDate(r)) => l == r,
            #[cfg(feature = "with-chrono")]
            (Self::ChronoTime(l), Self::ChronoTime(r)) => l == r,
            #[cfg(feature = "with-chrono")]
            (Self::ChronoDateTime(l), Self::ChronoDateTime(r)) => l == r,
            #[cfg(feature = "with-chrono")]
            (Self::ChronoDateTimeUtc(l), Self::ChronoDateTimeUtc(r)) => l == r,
            #[cfg(feature = "with-chrono")]
            (Self::ChronoDateTimeLocal(l), Self::ChronoDateTimeLocal(r)) => l == r,
            #[cfg(feature = "with-chrono")]
            (Self::ChronoDateTimeWithTimeZone(l), Self::ChronoDateTimeWithTimeZone(r)) => l == r,

            #[cfg(feature = "with-time")]
            (Self::TimeDate(l), Self::TimeDate(r)) => l == r,
            #[cfg(feature = "with-time")]
            (Self::TimeTime(l), Self::TimeTime(r)) => l == r,
            #[cfg(feature = "with-time")]
            (Self::TimeDateTime(l), Self::TimeDateTime(r)) => l == r,
            #[cfg(feature = "with-time")]
            (Self::TimeDateTimeWithTimeZone(l), Self::TimeDateTimeWithTimeZone(r)) => l == r,

            #[cfg(feature = "with-jiff")]
            (Self::JiffDate(l), Self::JiffDate(r)) => l == r,
            #[cfg(feature = "with-jiff")]
            (Self::JiffTime(l), Self::JiffTime(r)) => l == r,
            #[cfg(feature = "with-jiff")]
            (Self::JiffDateTime(l), Self::JiffDateTime(r)) => l == r,
            #[cfg(feature = "with-jiff")]
            (Self::JiffTimestamp(l), Self::JiffTimestamp(r)) => l == r,
            #[cfg(feature = "with-jiff")]
            (Self::JiffZoned(l), Self::JiffZoned(r)) => l == r,

            #[cfg(feature = "with-uuid")]
            (Self::Uuid(l), Self::Uuid(r)) => l == r,

            #[cfg(feature = "with-rust_decimal")]
            (Self::Decimal(l), Self::Decimal(r)) => l == r,

            #[cfg(feature = "with-bigdecimal")]
            (Self::BigDecimal(l), Self::BigDecimal(r)) => l == r,

            #[cfg(feature = "postgres-array")]
            (Self::Array(ty_l, values_l), Self::Array(ty_r, values_r)) => {
                ty_l == ty_r && values_l == values_r
            }

            #[cfg(feature = "postgres-vector")]
            (Self::Vector(l), Self::Vector(r)) => cmp_vector(l, r),

            #[cfg(feature = "with-ipnetwork")]
            (Self::IpNetwork(l), Self::IpNetwork(r)) => l == r,

            #[cfg(feature = "with-mac_address")]
            (Self::MacAddress(l), Self::MacAddress(r)) => l == r,

            _ => false,
        }
    }
}

impl Eq for Value {}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        mem::discriminant(self).hash(state);
        match self {
            Value::Bool(v) => v.hash(state),
            Value::TinyInt(v) => v.hash(state),
            Value::SmallInt(v) => v.hash(state),
            Value::Int(v) => v.hash(state),
            Value::BigInt(v) => v.hash(state),
            Value::TinyUnsigned(v) => v.hash(state),
            Value::SmallUnsigned(v) => v.hash(state),
            Value::Unsigned(v) => v.hash(state),
            Value::BigUnsigned(v) => v.hash(state),
            Value::Float(v) => hash_f32(v, state),
            Value::Double(v) => hash_f64(v, state),
            Value::String(v) => v.hash(state),
            Value::Enum(v) => v.hash(state),
            Value::Char(v) => v.hash(state),
            Value::Bytes(v) => v.hash(state),

            #[cfg(feature = "with-json")]
            Value::Json(value) => hash_json(value, state),

            #[cfg(feature = "with-chrono")]
            Value::ChronoDate(naive_date) => naive_date.hash(state),
            #[cfg(feature = "with-chrono")]
            Value::ChronoTime(naive_time) => naive_time.hash(state),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(naive_date_time) => naive_date_time.hash(state),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeUtc(date_time) => date_time.hash(state),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeLocal(date_time) => date_time.hash(state),
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTimeWithTimeZone(date_time) => date_time.hash(state),

            #[cfg(feature = "with-time")]
            Value::TimeDate(date) => date.hash(state),
            #[cfg(feature = "with-time")]
            Value::TimeTime(time) => time.hash(state),
            #[cfg(feature = "with-time")]
            Value::TimeDateTime(primitive_date_time) => primitive_date_time.hash(state),
            #[cfg(feature = "with-time")]
            Value::TimeDateTimeWithTimeZone(offset_date_time) => offset_date_time.hash(state),

            #[cfg(feature = "with-jiff")]
            Value::JiffDate(date) => date.hash(state),
            #[cfg(feature = "with-jiff")]
            Value::JiffTime(time) => time.hash(state),
            #[cfg(feature = "with-jiff")]
            Value::JiffDateTime(datetime) => datetime.hash(state),
            #[cfg(feature = "with-jiff")]
            Value::JiffTimestamp(timestamp) => timestamp.hash(state),
            #[cfg(feature = "with-jiff")]
            Value::JiffZoned(zoned) => zoned.hash(state),

            #[cfg(feature = "with-uuid")]
            Value::Uuid(uuid) => uuid.hash(state),

            #[cfg(feature = "with-rust_decimal")]
            Value::Decimal(decimal) => decimal.hash(state),

            #[cfg(feature = "with-bigdecimal")]
            Value::BigDecimal(big_decimal) => big_decimal.hash(state),

            #[cfg(feature = "postgres-array")]
            Value::Array(array_type, vec) => {
                array_type.hash(state);
                vec.hash(state);
            }

            #[cfg(feature = "postgres-vector")]
            Value::Vector(vector) => hash_vector(vector, state),

            #[cfg(feature = "with-ipnetwork")]
            Value::IpNetwork(ip_network) => ip_network.hash(state),

            #[cfg(feature = "with-mac_address")]
            Value::MacAddress(mac_address) => mac_address.hash(state),

            #[cfg(feature = "postgres-range")]
            Value::Range(range) => range.hash(state),
        }
    }
}

fn hash_f32<H: Hasher>(v: &Option<f32>, state: &mut H) {
    match v {
        Some(v) => OrderedFloat(*v).hash(state),
        None => "null".hash(state),
    }
}

fn hash_f64<H: Hasher>(v: &Option<f64>, state: &mut H) {
    match v {
        Some(v) => OrderedFloat(*v).hash(state),
        None => "null".hash(state),
    }
}

fn cmp_f32(l: &Option<f32>, r: &Option<f32>) -> bool {
    match (l, r) {
        (Some(l), Some(r)) => OrderedFloat(*l).eq(&OrderedFloat(*r)),
        (None, None) => true,
        _ => false,
    }
}

fn cmp_f64(l: &Option<f64>, r: &Option<f64>) -> bool {
    match (l, r) {
        (Some(l), Some(r)) => OrderedFloat(*l).eq(&OrderedFloat(*r)),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(feature = "with-json")]
fn hash_json<H: Hasher>(v: &Option<Box<Json>>, state: &mut H) {
    match v {
        Some(v) => serde_json::to_string(v).unwrap().hash(state),
        None => "null".hash(state),
    }
}

#[cfg(feature = "with-json")]
fn cmp_json(l: &Option<Box<Json>>, r: &Option<Box<Json>>) -> bool {
    match (l, r) {
        (Some(l), Some(r)) => serde_json::to_string(l)
            .unwrap()
            .eq(&serde_json::to_string(r).unwrap()),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(feature = "postgres-vector")]
fn hash_vector<H: Hasher>(v: &Option<pgvector::Vector>, state: &mut H) {
    match v {
        Some(v) => {
            for &value in v.as_slice().iter() {
                hash_f32(&Some(value), state);
            }
        }
        None => "null".hash(state),
    }
}

#[cfg(feature = "postgres-vector")]
fn cmp_vector(l: &Option<pgvector::Vector>, r: &Option<pgvector::Vector>) -> bool {
    match (l, r) {
        (Some(l), Some(r)) => {
            let (l, r) = (l.as_slice(), r.as_slice());
            if l.len() != r.len() {
                return false;
            }
            for (l, r) in l.iter().zip(r.iter()) {
                if !cmp_f32(&Some(*l), &Some(*r)) {
                    return false;
                }
            }
            true
        }
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;
    #[test]
    fn test_hash_value_0() {
        let hash_set: std::collections::HashSet<Value> = [
            Value::Int(None),
            Value::Int(None),
            Value::BigInt(None),
            Value::BigInt(None),
            Value::Float(None),
            Value::Float(None),           // Null is not NaN
            Value::Float(Some(f32::NAN)), // NaN considered equal
            Value::Float(Some(f32::NAN)),
            Value::Double(None),
            Value::Double(None),
            Value::Double(Some(f64::NAN)),
            Value::Double(Some(f64::NAN)),
        ]
        .into_iter()
        .collect();

        let unique: std::collections::HashSet<Value> = [
            Value::Int(None),
            Value::BigInt(None),
            Value::Float(None),
            Value::Double(None),
            Value::Float(Some(f32::NAN)),
            Value::Double(Some(f64::NAN)),
        ]
        .into_iter()
        .collect();

        assert_eq!(hash_set, unique);
    }

    #[test]
    fn test_hash_value_1() {
        let hash_set: std::collections::HashSet<Value> = [
            Value::Int(None),
            Value::Int(Some(1)),
            Value::Int(Some(1)),
            Value::BigInt(Some(2)),
            Value::BigInt(Some(2)),
            Value::Float(Some(3.0)),
            Value::Float(Some(3.0)),
            Value::Double(Some(3.0)),
            Value::Double(Some(3.0)),
            Value::BigInt(Some(5)),
        ]
        .into_iter()
        .collect();

        let unique: std::collections::HashSet<Value> = [
            Value::BigInt(Some(5)),
            Value::Double(Some(3.0)),
            Value::Float(Some(3.0)),
            Value::BigInt(Some(2)),
            Value::Int(Some(1)),
            Value::Int(None),
        ]
        .into_iter()
        .collect();

        assert_eq!(hash_set, unique);
    }

    #[cfg(feature = "postgres-array")]
    #[test]
    fn test_hash_value_array() {
        use crate::ArrayType;

        assert_eq!(
            Into::<Value>::into(vec![0i32, 1, 2]),
            Value::Array(
                ArrayType::Int,
                Some(Box::new(vec![
                    Value::Int(Some(0)),
                    Value::Int(Some(1)),
                    Value::Int(Some(2))
                ]))
            )
        );

        assert_eq!(
            Into::<Value>::into(vec![0f32, 1.0, 2.0]),
            Value::Array(
                ArrayType::Float,
                Some(Box::new(vec![
                    Value::Float(Some(0f32)),
                    Value::Float(Some(1.0)),
                    Value::Float(Some(2.0))
                ]))
            )
        );

        let hash_set: std::collections::HashSet<Value> = [
            Into::<Value>::into(vec![0i32, 1, 2]),
            Into::<Value>::into(vec![0i32, 1, 2]),
            Into::<Value>::into(vec![0f32, 1.0, 2.0]),
            Into::<Value>::into(vec![0f32, 1.0, 2.0]),
            Into::<Value>::into(vec![3f32, 2.0, 1.0]),
        ]
        .into_iter()
        .collect();

        let unique: std::collections::HashSet<Value> = [
            Into::<Value>::into(vec![0i32, 1, 2]),
            Into::<Value>::into(vec![0f32, 1.0, 2.0]),
            Into::<Value>::into(vec![3f32, 2.0, 1.0]),
        ]
        .into_iter()
        .collect();

        assert_eq!(hash_set, unique);
    }
}
