//! Universal value variants used in the library.
use std::str::from_utf8;
use std::fmt::Write;
use serde_json::Value as JsonValue;

/// Value variants
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Value {
    NULL,
    Bytes(Vec<u8>),
    Int(i64),
    UInt(u64),
    Float(f32),
    Double(f64),
    Date(u16, u8, u8, u8, u8, u8, u32),
    Time(bool, u32, u8, u8, u8, u32),
}

macro_rules! into_value_impl (
    (signed $t:ty) => (
        impl From<$t> for Value {
            fn from(x: $t) -> Value {
                Value::Int(x as i64)
            }
        }
    );
    (unsigned $t:ty) => (
        impl From<$t> for Value {
            fn from(x: $t) -> Value {
                Value::UInt(x as u64)
            }
        }
    );
);

into_value_impl!(signed i8);
into_value_impl!(signed i16);
into_value_impl!(signed i32);
into_value_impl!(signed i64);
into_value_impl!(signed isize);
into_value_impl!(unsigned u8);
into_value_impl!(unsigned u16);
into_value_impl!(unsigned u32);
into_value_impl!(unsigned u64);
into_value_impl!(unsigned usize);

impl From<f32> for Value {
    fn from(x: f32) -> Value {
        Value::Float(x)
    }
}

impl From<f64> for Value {
    fn from(x: f64) -> Value {
        Value::Double(x)
    }
}

impl From<bool> for Value {
    fn from(x: bool) -> Value {
        Value::Int(if x { 1 } else { 0 })
    }
}

impl<'a> From<&'a [u8]> for Value {
    fn from(x: &'a [u8]) -> Value {
        Value::Bytes(x.into())
    }
}

impl From<Vec<u8>> for Value {
    fn from(x: Vec<u8>) -> Value {
        Value::Bytes(x)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(x: &'a str) -> Value {
        let string: String = x.into();
        Value::Bytes(string.into_bytes())
    }
}

impl From<String> for Value {
    fn from(x: String) -> Value {
        Value::Bytes(x.into_bytes())
    }
}

macro_rules! from_array_impl {
    ($n:expr) => {
        impl From<[u8; $n]> for Value {
            fn from(x: [u8; $n]) -> Value {
                Value::from(&x[..])
            }
        }
    };
}

from_array_impl!(0);
from_array_impl!(1);
from_array_impl!(2);
from_array_impl!(3);
from_array_impl!(4);
from_array_impl!(5);
from_array_impl!(6);
from_array_impl!(7);
from_array_impl!(8);
from_array_impl!(9);
from_array_impl!(10);
from_array_impl!(11);
from_array_impl!(12);
from_array_impl!(13);
from_array_impl!(14);
from_array_impl!(15);
from_array_impl!(16);
from_array_impl!(17);
from_array_impl!(18);
from_array_impl!(19);
from_array_impl!(20);
from_array_impl!(21);
from_array_impl!(22);
from_array_impl!(23);
from_array_impl!(24);
from_array_impl!(25);
from_array_impl!(26);
from_array_impl!(27);
from_array_impl!(28);
from_array_impl!(29);
from_array_impl!(30);
from_array_impl!(31);
from_array_impl!(32);

/// Convert value to string
pub fn value_to_string(v: &Value) -> String {
    let mut s = String::new();
    match v {
        Value::NULL => write!(s, "NULL").unwrap(),
        Value::Bytes(v) => write!(s, "\'{}\'", std::str::from_utf8(v).unwrap()).unwrap(),
        Value::Int(v) => write!(s, "{}", v).unwrap(),
        Value::UInt(v) => write!(s, "{}", v).unwrap(),
        Value::Float(v) => write!(s, "{}", v).unwrap(),
        Value::Double(v) => write!(s, "{}", v).unwrap(),
        Value::Date(year, month, day, hour, minutes, seconds, _micro_seconds) => 
            write!(
                s, "{:04}{:02}{:02} {:02}{:02}{:02}",
                year, month, day, hour, minutes, seconds
            ).unwrap(),
        Value::Time(negative, days, hours, minutes, seconds, _micro_seconds) => 
            write!(
                s, "{}{:02}{:02} {:02}{:02}.{:03}",
                if *negative { "-" } else { "" }, days, hours, minutes, seconds, _micro_seconds / 1000
            ).unwrap(),
    };
    s
}

/// Convert json value to value
pub fn json_value_to_mysql_value(v: &JsonValue) -> Value {
    match v {
        JsonValue::Null => Value::NULL,
        JsonValue::Bool(v) => Value::Int(v.to_owned().into()),
        JsonValue::Number(v) =>
            if v.is_f64() {
                Value::Double(v.as_f64().unwrap())
            } else if v.is_u64() {
                Value::UInt(v.as_u64().unwrap())
            } else if v.is_i64() {
                Value::Int(v.as_i64().unwrap())
            } else {
                unimplemented!()
            },
        JsonValue::String(v) => Value::Bytes(v.as_bytes().to_vec()),
        JsonValue::Array(_) => unimplemented!(),
        JsonValue::Object(_) => unimplemented!(),
    }
}

/// Convert value to json value
#[allow(clippy::many_single_char_names)]
pub fn mysql_value_to_json_value(v: &Value) -> JsonValue {
    match v {
        Value::NULL => JsonValue::Null,
        Value::Bytes(v) => JsonValue::String(from_utf8(v).unwrap().to_string()),
        Value::Int(v) => (*v).into(),
        Value::UInt(v) => (*v).into(),
        Value::Float(v) => (*v).into(),
        Value::Double(v) => (*v).into(),
        Value::Date(y, m, d, 0, 0, 0, 0) => {
            JsonValue::String(format!("'{:04}-{:02}-{:02}'", y, m, d))
        },
        Value::Date(y, m, d, h, i, s, 0) => {
            JsonValue::String(format!("'{:04}-{:02}-{:02} {:02}:{:02}:{:02}'", y, m, d, h, i, s))
        },
        Value::Date(y, m, d, h, i, s, u) => {
            JsonValue::String(format!(
                "'{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}'",
                y, m, d, h, i, s, u
            ))
        },
        Value::Time(neg, d, h, i, s, 0) => {
            if *neg {
                JsonValue::String(format!("'-{:03}:{:02}:{:02}'", d * 24 + u32::from(*h), i, s))
            } else {
                JsonValue::String(format!("'{:03}:{:02}:{:02}'", d * 24 + u32::from(*h), i, s))
            }
        },
        Value::Time(neg, d, h, i, s, u) => {
            if *neg {
                JsonValue::String(format!("'-{:03}:{:02}:{:02}.{:06}'", d * 24 + u32::from(*h), i, s, u))
            } else {
                JsonValue::String(format!("'{:03}:{:02}:{:02}.{:06}'", d * 24 + u32::from(*h), i, s, u))
            }
        },
    }
}
