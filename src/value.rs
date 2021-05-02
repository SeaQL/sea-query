//! Container for all SQL value types.
use std::fmt::Write;

#[cfg(feature="with-json")]
use std::str::from_utf8;
#[cfg(feature="with-json")]
use serde_json::Value as Json;

#[cfg(feature="with-chrono")]
use chrono::NaiveDateTime;

#[cfg(feature="with-uuid")]
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
    String(Box<String>),
    Bytes(Box<Vec<u8>>),
    #[cfg(feature="with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json(Box<Json>),
    #[cfg(feature="with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    DateTime(Box<NaiveDateTime>),
    #[cfg(feature="with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid(Box<Uuid>),
}

#[derive(Debug, PartialEq)]
pub struct Values(pub Vec<Value>);

impl From<bool> for Value {
    fn from(x: bool) -> Value {
        Value::Bool(x)
    }
}

impl From<i8> for Value {
    fn from(x: i8) -> Value {
        Value::TinyInt(x)
    }
}

impl From<i16> for Value {
    fn from(x: i16) -> Value {
        Value::SmallInt(x)
    }
}

impl From<i32> for Value {
    fn from(x: i32) -> Value {
        Value::Int(x)
    }
}

impl From<i64> for Value {
    fn from(x: i64) -> Value {
        Value::BigInt(x)
    }
}

impl From<u8> for Value {
    fn from(x: u8) -> Value {
        Value::TinyUnsigned(x)
    }
}

impl From<u16> for Value {
    fn from(x: u16) -> Value {
        Value::SmallUnsigned(x)
    }
}

impl From<u32> for Value {
    fn from(x: u32) -> Value {
        Value::Unsigned(x)
    }
}

impl From<u64> for Value {
    fn from(x: u64) -> Value {
        Value::BigUnsigned(x)
    }
}

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

impl<'a> From<&'a [u8]> for Value {
    fn from(x: &'a [u8]) -> Value {
        Value::Bytes(Box::<Vec<u8>>::new(x.into()))
    }
}

impl From<Vec<u8>> for Value {
    fn from(x: Vec<u8>) -> Value {
        Value::Bytes(Box::new(x))
    }
}

impl<'a> From<&'a str> for Value {
    fn from(x: &'a str) -> Value {
        let string: String = x.into();
        Value::String(Box::new(string))
    }
}

impl From<String> for Value {
    fn from(x: String) -> Value {
        Value::String(Box::new(x))
    }
}

#[cfg(feature="with-json")]
mod with_json {
    use super::*;

    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    impl From<Json> for Value {
        fn from(x: Json) -> Value {
            Value::Json(Box::new(x))
        }
    }
}

#[cfg(feature="with-chrono")]
mod with_chrono {
    use super::*;

    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    impl From<NaiveDateTime> for Value {
        fn from(x: NaiveDateTime) -> Value {
            Value::DateTime(Box::new(x))
        }
    }
}

#[cfg(feature="with-uuid")]
mod with_uuid {
    use super::*;

    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    impl From<Uuid> for Value {
        fn from(x: Uuid) -> Value {
            Value::Uuid(Box::new(x))
        }
    }
}

impl Value {
    pub fn is_json(&self) -> bool {
        #[cfg(feature="with-json")]
        return matches!(self, Self::Json(_));
        #[cfg(not(feature="with-json"))]
        return false;
    }
    #[cfg(feature="with-json")]
    pub fn as_ref_json(&self) -> &Json {
        match self {
            Self::Json(v) => v.as_ref(),
            _ => panic!("not Value::Json"),
        }
    }
    #[cfg(not(feature="with-json"))]
    pub fn as_ref_json(&self) -> &bool {
        panic!("not Value::Json")
    }

    pub fn is_date_time(&self) -> bool {
        #[cfg(feature="with-chrono")]
        return matches!(self, Self::DateTime(_));
        #[cfg(not(feature="with-chrono"))]
        return false;
    }
    #[cfg(feature="with-chrono")]
    pub fn as_ref_date_time(&self) -> &NaiveDateTime {
        match self {
            Self::DateTime(v) => v.as_ref(),
            _ => panic!("not Value::DateTime"),
        }
    }
    #[cfg(not(feature="with-chrono"))]
    pub fn as_ref_date_time(&self) -> &bool {
        panic!("not Value::DateTime")
    }

    pub fn is_uuid(&self) -> bool {
        #[cfg(feature="with-uuid")]
        return matches!(self, Self::Uuid(_));
        #[cfg(not(feature="with-uuid"))]
        return false;
    }
    #[cfg(feature="with-uuid")]
    pub fn as_ref_uuid(&self) -> &Uuid {
        match self {
            Self::Uuid(v) => v.as_ref(),
            _ => panic!("not Value::Uuid"),
        }
    }
    #[cfg(not(feature="with-uuid"))]
    pub fn as_ref_uuid(&self) -> &bool {
        panic!("not Value::Uuid")
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
            write!(output, "{}", match c {
                '0' => '\0',
                'b' => '\x08',
                't' => '\x09',
                'z' => '\x1a',
                'n' => '\n',
                'r' => '\r',
                c => c,
            }).unwrap();
            escape = false;
        } else {
            write!(output, "{}", c).unwrap();
        }
    }
    output
}

/// Convert json value to value
#[cfg(feature="with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
pub fn json_value_to_sea_value(v: &Json) -> Value {
    match v {
        Json::Null => Value::Null,
        Json::Bool(v) => Value::Int(v.to_owned().into()),
        Json::Number(v) =>
            if v.is_f64() {
                Value::Double(v.as_f64().unwrap())
            } else if v.is_i64() {
                Value::BigInt(v.as_i64().unwrap())
            } else if v.is_u64() {
                Value::BigUnsigned(v.as_u64().unwrap())
            } else {
                unimplemented!()
            },
        Json::String(v) => Value::String(Box::new(v.clone())),
        Json::Array(_) => unimplemented!(),
        Json::Object(v) => Value::Json(Box::new(Json::Object(v.clone()))),
    }
}

/// Convert value to json value
#[allow(clippy::many_single_char_names)]
#[cfg(feature="with-json")]
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
        #[cfg(feature="with-chrono")]
        Value::DateTime(v) => v.format("%Y-%m-%d %H:%M:%S").to_string().into(),
        #[cfg(feature="with-uuid")]
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
}