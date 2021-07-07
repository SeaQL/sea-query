//! Container for all SQL value types.
use std::fmt::Write;

#[cfg(feature = "with-json")]
use serde_json::Value as Json;
#[cfg(feature = "with-json")]
use std::str::from_utf8;

#[cfg(feature = "with-chrono")]
use chrono::NaiveDateTime;

#[cfg(feature = "with-rust_decimal")]
use ::rust_decimal::Decimal;
#[cfg(feature = "with-rust_decimal")]
pub mod rust_decimal {
    pub use rust_decimal::prelude::ToPrimitive;
}

#[cfg(feature = "with-uuid")]
use uuid::Uuid;

/// Value variants
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    TinyInt(i8),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    TinyUnsigned(u8),
    SmallUnsigned(u16),
    Unsigned(u32),
    BigUnsigned(u64),
    Float(f32),
    Double(f64),
    // we want Value to be exactly 1 pointer sized, so anything larger should be boxed
    String(Box<String>),
    #[allow(clippy::box_vec)]
    Bytes(Box<Vec<u8>>),
    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json(Box<Json>),
    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    DateTime(Box<NaiveDateTime>),
    #[cfg(feature = "with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid(Box<Uuid>),
    #[cfg(feature = "with-rust_decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
    Decimal(Box<Decimal>),
}

pub trait ValueType {
    fn unwrap(v: Value) -> Self;

    fn type_name() -> &'static str;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Values(pub Vec<Value>);

#[derive(Debug, PartialEq)]
pub enum ValueTuple {
    One(Value),
    Two(Value, Value),
    Three(Value, Value, Value),
}

pub trait IntoValueTuple {
    fn into_value_tuple(self) -> ValueTuple;
}

impl Value {
    pub fn unwrap<T>(self) -> T
    where
        T: ValueType,
    {
        T::unwrap(self)
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

macro_rules! type_to_value {
    ( $type: ty, $name: ident ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::$name(x)
            }
        }

        impl From<Option<$type>> for Value {
            fn from(x: Option<$type>) -> Value {
                match x {
                    Some(v) => Value::$name(v),
                    None => Value::Null,
                }
            }
        }

        impl ValueType for $type {
            fn unwrap(v: Value) -> Self {
                match v {
                    Value::$name(x) => x,
                    _ => panic!("type error"),
                }
            }

            fn type_name() -> &'static str {
                stringify!($type)
            }
        }

        impl ValueType for Option<$type> {
            fn unwrap(v: Value) -> Self {
                match v {
                    Value::$name(x) => Some(x),
                    _ => panic!("type error"),
                }
            }

            fn type_name() -> &'static str {
                concat!("Option<", stringify!($type), ">")
            }
        }
    };
}

macro_rules! type_to_box_value {
    ( $type: ty, $name: ident ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::$name(Box::new(x))
            }
        }

        impl From<Option<$type>> for Value {
            fn from(x: Option<$type>) -> Value {
                match x {
                    Some(v) => Value::$name(Box::new(v)),
                    None => Value::Null,
                }
            }
        }

        impl ValueType for $type {
            fn unwrap(v: Value) -> Self {
                match v {
                    Value::$name(x) => *x,
                    _ => panic!("type error"),
                }
            }

            fn type_name() -> &'static str {
                stringify!($type)
            }
        }

        impl ValueType for Option<$type> {
            fn unwrap(v: Value) -> Self {
                match v {
                    Value::$name(x) => Some(*x),
                    _ => panic!("type error"),
                }
            }

            fn type_name() -> &'static str {
                concat!("Option<", stringify!($type), ">")
            }
        }
    };
}

type_to_value!(bool, Bool);
type_to_value!(i8, TinyInt);
type_to_value!(i16, SmallInt);
type_to_value!(i32, Int);
type_to_value!(i64, BigInt);
type_to_value!(u8, TinyUnsigned);
type_to_value!(u16, SmallUnsigned);
type_to_value!(u32, Unsigned);
type_to_value!(u64, BigUnsigned);
type_to_value!(f32, Float);
type_to_value!(f64, Double);

impl<'a> From<&'a [u8]> for Value {
    fn from(x: &'a [u8]) -> Value {
        Value::Bytes(Box::<Vec<u8>>::new(x.into()))
    }
}

impl<'a> From<&'a str> for Value {
    fn from(x: &'a str) -> Value {
        let string: String = x.into();
        Value::String(Box::new(string))
    }
}

type_to_box_value!(Vec<u8>, Bytes);
type_to_box_value!(String, String);

#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
mod with_json {
    use super::*;

    type_to_box_value!(Json, Json);
}

#[cfg(feature = "with-chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
mod with_chrono {
    use super::*;

    type_to_box_value!(NaiveDateTime, DateTime);
}

#[cfg(feature = "with-rust_decimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
mod with_rust_decimal {
    use super::*;

    type_to_box_value!(Decimal, Decimal);
}

#[cfg(feature = "with-uuid")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
mod with_uuid {
    use super::*;

    type_to_box_value!(Uuid, Uuid);
}

impl Value {
    pub fn is_json(&self) -> bool {
        #[cfg(feature = "with-json")]
        return matches!(self, Self::Json(_));
        #[cfg(not(feature = "with-json"))]
        return false;
    }
    #[cfg(feature = "with-json")]
    pub fn as_ref_json(&self) -> &Json {
        match self {
            Self::Json(v) => v.as_ref(),
            _ => panic!("not Value::Json"),
        }
    }
    #[cfg(not(feature = "with-json"))]
    pub fn as_ref_json(&self) -> &bool {
        panic!("not Value::Json")
    }
}

impl Value {
    pub fn is_date_time(&self) -> bool {
        #[cfg(feature = "with-chrono")]
        return matches!(self, Self::DateTime(_));
        #[cfg(not(feature = "with-chrono"))]
        return false;
    }
    #[cfg(feature = "with-chrono")]
    pub fn as_ref_date_time(&self) -> &NaiveDateTime {
        match self {
            Self::DateTime(v) => v.as_ref(),
            _ => panic!("not Value::DateTime"),
        }
    }
    #[cfg(not(feature = "with-chrono"))]
    pub fn as_ref_date_time(&self) -> &bool {
        panic!("not Value::DateTime")
    }
}

impl Value {
    pub fn is_decimal(&self) -> bool {
        #[cfg(feature = "with-rust_decimal")]
        return matches!(self, Self::Decimal(_));
        #[cfg(not(feature = "with-rust_decimal"))]
        return false;
    }
    #[cfg(feature = "with-rust_decimal")]
    pub fn as_ref_decimal(&self) -> &Decimal {
        match self {
            Self::Decimal(v) => v.as_ref(),
            _ => panic!("not Value::Decimal"),
        }
    }
    #[cfg(not(feature = "with-rust_decimal"))]
    pub fn as_ref_decimal(&self) -> &bool {
        panic!("not Value::Decimal")
    }
}

impl Value {
    pub fn is_uuid(&self) -> bool {
        #[cfg(feature = "with-uuid")]
        return matches!(self, Self::Uuid(_));
        #[cfg(not(feature = "with-uuid"))]
        return false;
    }
    #[cfg(feature = "with-uuid")]
    pub fn as_ref_uuid(&self) -> &Uuid {
        match self {
            Self::Uuid(v) => v.as_ref(),
            _ => panic!("not Value::Uuid"),
        }
    }
    #[cfg(not(feature = "with-uuid"))]
    pub fn as_ref_uuid(&self) -> &bool {
        panic!("not Value::Uuid")
    }
}

impl IntoIterator for ValueTuple {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ValueTuple::One(v) => vec![v].into_iter(),
            ValueTuple::Two(v, w) => vec![v, w].into_iter(),
            ValueTuple::Three(u, v, w) => vec![u, v, w].into_iter(),
        }
    }
}

impl<V> IntoValueTuple for V
where
    V: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::One(self.into())
    }
}

impl<V, W> IntoValueTuple for (V, W)
where
    V: Into<Value>,
    W: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::Two(self.0.into(), self.1.into())
    }
}

impl<U, V, W> IntoValueTuple for (U, V, W)
where
    U: Into<Value>,
    V: Into<Value>,
    W: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::Three(self.0.into(), self.1.into(), self.2.into())
    }
}

/// Escape a SQL string literal
pub fn escape_string(string: &str) -> String {
    string
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("'", "\\'")
        .replace("\0", "\\0")
        .replace("\x08", "\\b")
        .replace("\x09", "\\t")
        .replace("\x1a", "\\z")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
}

/// Unescape a SQL string literal
pub fn unescape_string(input: &str) -> String {
    let mut escape = false;
    let mut output = String::new();
    for c in input.chars() {
        if !escape && c == '\\' {
            escape = true;
        } else if escape {
            write!(
                output,
                "{}",
                match c {
                    '0' => '\0',
                    'b' => '\x08',
                    't' => '\x09',
                    'z' => '\x1a',
                    'n' => '\n',
                    'r' => '\r',
                    c => c,
                }
            )
            .unwrap();
            escape = false;
        } else {
            write!(output, "{}", c).unwrap();
        }
    }
    output
}

/// Convert JSON value to Sea Value.
/// `Json::Array` is not supported and will panic.
#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
pub fn json_value_to_sea_value(v: &Json) -> Value {
    match v {
        Json::Null => Value::Null,
        Json::Bool(v) => Value::Int(v.to_owned().into()),
        Json::Number(v) => {
            if v.is_f64() {
                Value::Double(v.as_f64().unwrap())
            } else if v.is_i64() {
                Value::BigInt(v.as_i64().unwrap())
            } else if v.is_u64() {
                Value::BigUnsigned(v.as_u64().unwrap())
            } else {
                unreachable!()
            }
        }
        Json::String(v) => Value::String(Box::new(v.clone())),
        Json::Array(_) => panic!("Json::Array is not supported"),
        Json::Object(v) => Value::Json(Box::new(Json::Object(v.clone()))),
    }
}

/// Convert value to json value
#[allow(clippy::many_single_char_names)]
#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
pub fn sea_value_to_json_value(v: &Value) -> Json {
    match v {
        Value::Null => Json::Null,
        Value::Bool(b) => Json::Bool(*b),
        Value::TinyInt(v) => (*v).into(),
        Value::SmallInt(v) => (*v).into(),
        Value::Int(v) => (*v).into(),
        Value::BigInt(v) => (*v).into(),
        Value::TinyUnsigned(v) => (*v).into(),
        Value::SmallUnsigned(v) => (*v).into(),
        Value::Unsigned(v) => (*v).into(),
        Value::BigUnsigned(v) => (*v).into(),
        Value::Float(v) => (*v).into(),
        Value::Double(v) => (*v).into(),
        Value::String(s) => Json::String(s.as_ref().clone()),
        Value::Bytes(s) => Json::String(from_utf8(s).unwrap().to_string()),
        Value::Json(v) => v.as_ref().clone(),
        #[cfg(feature = "with-chrono")]
        Value::DateTime(v) => v.format("%Y-%m-%d %H:%M:%S").to_string().into(),
        #[cfg(feature = "with-rust_decimal")]
        Value::Decimal(v) => {
            use self::rust_decimal::ToPrimitive;
            v.as_ref().to_f64().unwrap().into()
        },
        #[cfg(feature = "with-uuid")]
        Value::Uuid(v) => Json::String(v.to_string()),
    }
}

impl Values {
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_1() {
        let test = r#" "abc" "#;
        assert_eq!(escape_string(test), r#" \"abc\" "#.to_owned());
        assert_eq!(unescape_string(escape_string(test).as_str()), test);
    }

    #[test]
    fn test_escape_2() {
        let test = "a\nb\tc";
        assert_eq!(escape_string(test), "a\\nb\\tc".to_owned());
        assert_eq!(unescape_string(escape_string(test).as_str()), test);
    }

    #[test]
    fn test_escape_3() {
        let test = "a\\b";
        assert_eq!(escape_string(test), "a\\\\b".to_owned());
        assert_eq!(unescape_string(escape_string(test).as_str()), test);
    }

    #[test]
    fn test_escape_4() {
        let test = "a\"b";
        assert_eq!(escape_string(test), "a\\\"b".to_owned());
        assert_eq!(unescape_string(escape_string(test).as_str()), test);
    }

    #[test]
    fn test_value() {
        macro_rules! test_value {
            ( $type: ty, $val: literal ) => {
                let val: $type = $val;
                let v: Value = val.into();
                let out: $type = v.unwrap();
                assert_eq!(out, val);
            };
        }

        test_value!(u8, 255);
        test_value!(u16, 65535);
        test_value!(i8, 127);
        test_value!(i16, 32767);
        test_value!(i32, 1073741824);
        test_value!(i64, 8589934592);
    }

    #[test]
    fn test_option_value() {
        macro_rules! test_some_value {
            ( $type: ty, $val: literal ) => {
                let val: Option<$type> = Some($val);
                let v: Value = val.into();
                let out: $type = v.unwrap();
                assert_eq!(out, val.unwrap());
            };
        }

        macro_rules! test_none {
            ( $type: ty ) => {
                let val: Option<$type> = None;
                let v: Value = val.into();
                assert_eq!(v, Value::Null);
            };
        }

        test_some_value!(u8, 255);
        test_some_value!(u16, 65535);
        test_some_value!(i8, 127);
        test_some_value!(i16, 32767);
        test_some_value!(i32, 1073741824);
        test_some_value!(i64, 8589934592);

        test_none!(u8);
        test_none!(u16);
        test_none!(i8);
        test_none!(i16);
        test_none!(i32);
        test_none!(i64);
    }

    #[test]
    fn test_box_value() {
        let val: String = "hello".to_owned();
        let v: Value = val.clone().into();
        let out: String = v.unwrap();
        assert_eq!(out, val);
    }

    #[test]
    fn test_value_tuple() {
        assert_eq!(1i32.into_value_tuple(), ValueTuple::One(Value::Int(1)));
        assert_eq!(
            "b".into_value_tuple(),
            ValueTuple::One(Value::String(Box::new("b".to_owned())))
        );
        assert_eq!(
            (1i32, "b").into_value_tuple(),
            ValueTuple::Two(Value::Int(1), Value::String(Box::new("b".to_owned())))
        );
        assert_eq!(
            (1i32, 2.4f64, "b").into_value_tuple(),
            ValueTuple::Three(
                Value::Int(1),
                Value::Double(2.4),
                Value::String(Box::new("b".to_owned()))
            )
        );
    }

    #[test]
    fn test_value_tuple_iter() {
        let mut iter = (1i32).into_value_tuple().into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(1));
        assert_eq!(iter.next(), None);

        let mut iter = (1i32, 2.4f64).into_value_tuple().into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(1));
        assert_eq!(iter.next().unwrap(), Value::Double(2.4));
        assert_eq!(iter.next(), None);

        let mut iter = (1i32, 2.4f64, "b").into_value_tuple().into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(1));
        assert_eq!(iter.next().unwrap(), Value::Double(2.4));
        assert_eq!(
            iter.next().unwrap(),
            Value::String(Box::new("b".to_owned()))
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[cfg(feature = "with-json")]
    fn test_json_value() {
        let json = serde_json::json! {{
            "a": 25.0,
            "b": "hello",
        }};
        let value: Value = json.clone().into();
        let out: Json = value.unwrap();
        assert_eq!(out, json);
    }

    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_chrono_value() {
        let timestamp = chrono::NaiveDate::from_ymd(2020, 1, 1).and_hms(2, 2, 2);
        let value: Value = timestamp.into();
        let out: NaiveDateTime = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-uuid")]
    fn test_uuid_value() {
        let uuid = uuid::Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
        let value: Value = uuid.into();
        let out: uuid::Uuid = value.unwrap();
        assert_eq!(out, uuid);
    }

    #[test]
    #[cfg(feature = "with-rust_decimal")]
    fn test_decimal_value() {
        use std::str::FromStr;

        let num = "2.02";
        let val = Decimal::from_str(num).unwrap();
        let v: Value = val.into();
        let out: Decimal = v.unwrap();
        assert_eq!(out.to_string(), num);
    }
}
