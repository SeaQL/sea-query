use crate::*;
use dyn_clonable::*;
use std::{any, fmt, ops};

/// A value which is safe for use in queries.
#[derive(Clone, Debug)]
pub struct Value(Box<dyn QueryValue>);

impl Value {
    pub fn as_ref(&self) -> &dyn QueryValue {
        self.0.as_ref()
    }
}

impl<T> From<T> for Value
where
    T: 'static + QueryValue,
{
    fn from(value: T) -> Self {
        Value(Box::new(value))
    }
}

impl From<Value> for Box<dyn QueryValue> {
    fn from(value: Value) -> Self {
        value.0
    }
}

impl ops::Deref for Value {
    type Target = dyn QueryValue;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        self.0.box_eq(other.0.as_any())
    }
}

/// Indicates that a SQL type is supported for use in queries.
/// Convert a value to a String for use in queries.
#[clonable]
pub trait QueryValue: QueryValuePartialEq + Clone {
    /// Returns the value as an escaped string safe for use in queries.
    fn query_value(&self, query_builder: &dyn QueryBuilder) -> String;
}

impl std::fmt::Debug for dyn QueryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.query_value(&CommonSqlQueryBuilder))
    }
}

impl std::fmt::Display for dyn QueryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.query_value(&CommonSqlQueryBuilder))
    }
}

pub trait QueryValuePartialEq {
    fn as_any(&self) -> &dyn any::Any;
    fn box_eq(&self, other: &dyn any::Any) -> bool;
}

impl<T> QueryValuePartialEq for T
where
    T: 'static + PartialEq,
{
    fn as_any(&self) -> &dyn any::Any {
        self
    }

    fn box_eq(&self, other: &dyn any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
}

impl PartialEq for dyn QueryValue {
    fn eq(&self, other: &dyn QueryValue) -> bool {
        self.box_eq(other.as_any())
    }
}

impl<T> QueryValue for Option<T>
where
    T: 'static + QueryValue + QueryValuePartialEq + Clone + PartialEq,
{
    fn query_value(&self, query_builder: &dyn QueryBuilder) -> String {
        match self {
            Some(value) => value.query_value(query_builder),
            None => "NULL".to_string(),
        }
    }
}

macro_rules! into_box_query_value {
    ($ty: ty) => {
        impl From<$ty> for Box<dyn QueryValue> {
            fn from(value: $ty) -> Self {
                Box::new(value)
            }
        }

        impl From<Option<$ty>> for Box<dyn QueryValue> {
            fn from(value: Option<$ty>) -> Self {
                Box::new(value)
            }
        }
    };
}

macro_rules! into_box_query_value_owned {
    ($ty: ty) => {
        impl From<$ty> for Box<dyn QueryValue> {
            fn from(value: $ty) -> Self {
                Box::new(value.to_owned())
            }
        }

        impl From<Option<$ty>> for Box<dyn QueryValue> {
            fn from(value: Option<$ty>) -> Self {
                Box::new(value.map(ToOwned::to_owned))
            }
        }
    };
}

macro_rules! impl_query_value {
    ($ty: ty) => {
        impl QueryValue for $ty {
            fn query_value(&self, _query_builder: &dyn QueryBuilder) -> String {
                format!("{}", self)
            }
        }
    };
}

macro_rules! impl_query_value_quoted {
    ($ty: ty) => {
        impl QueryValue for $ty {
            fn query_value(&self, query_builder: &dyn QueryBuilder) -> String {
                let mut buf = String::new();
                query_builder.write_string_quoted(self.as_ref(), &mut buf);
                buf
            }
        }
    };
}

into_box_query_value!(());
into_box_query_value_owned!(&str);
into_box_query_value!(String);
into_box_query_value!(i32);
into_box_query_value!(i64);
into_box_query_value!(u32);
into_box_query_value!(u64);
into_box_query_value!(f32);
into_box_query_value!(f64);
into_box_query_value!(u8);
into_box_query_value!(Vec<u8>);
#[cfg(feature = "with-chrono")]
into_box_query_value!(chrono::DateTime<chrono::FixedOffset>);
#[cfg(feature = "with-chrono")]
into_box_query_value!(chrono::NaiveDateTime);

impl_query_value_quoted!(&'static str);
impl_query_value_quoted!(String);
impl_query_value!(i32);
impl_query_value!(i64);
impl_query_value!(u32);
impl_query_value!(u64);
impl_query_value!(f32);
impl_query_value!(f64);
impl_query_value!(u8);

impl QueryValue for () {
    fn query_value(&self, _query_builder: &dyn QueryBuilder) -> String {
        "NULL".to_string()
    }
}

impl QueryValue for Vec<u8> {
    fn query_value(&self, _query_builder: &dyn QueryBuilder) -> String {
        format!(
            "x\'{}\'",
            self.iter()
                .map(|b| format!("{:02X}", b))
                .collect::<String>()
        )
    }
}

#[cfg(feature = "with-chrono")]
impl QueryValue for chrono::DateTime<chrono::FixedOffset> {
    fn query_value(&self, _query_builder: &dyn QueryBuilder) -> String {
        format!("\'{}\'", self.format("%Y-%m-%d %H:%M:%S %:z").to_string())
    }
}

#[cfg(feature = "with-chrono")]
impl QueryValue for chrono::NaiveDateTime {
    fn query_value(&self, _query_builder: &dyn QueryBuilder) -> String {
        format!("\'{}\'", self.format("%Y-%m-%d %H:%M:%S").to_string())
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
