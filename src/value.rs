//! Universal value variants used in the library.

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