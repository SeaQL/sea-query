use crate::primitive_value::PrimitiveValue;
use crate::*;
use dyn_clonable::*;
use std::{any, borrow, fmt, ops};

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
    T: QueryValue + borrow::ToOwned,
    <T as std::borrow::ToOwned>::Owned: 'static + QueryValue,
{
    fn from(value: T) -> Self {
        Value(Box::new(value.to_owned()))
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

/// Indicates that a type is supported for use in SQL queries.
#[clonable]
pub trait QueryValue: QueryValuePartialEq + Clone + Send + Sync {
    /// Returns the value as an escaped string safe for use in SQL queries.
    fn query_value(&self, query_builder: &dyn QueryBuilder) -> String;

    /// Primitive value for use in Database.
    fn primitive_value(&self) -> PrimitiveValue;

    /// Cast type in queries.
    fn cast_as(&self) -> Option<&'static str> {
        None
    }
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
    Option<T>: Into<PrimitiveValue>,
{
    fn query_value(&self, query_builder: &dyn QueryBuilder) -> String {
        match self {
            Some(value) => value.query_value(query_builder),
            None => "NULL".to_string(),
        }
    }

    fn primitive_value(&self) -> PrimitiveValue {
        self.to_owned().into()
    }

    fn cast_as(&self) -> Option<&'static str> {
        match self {
            Some(value) => value.cast_as(),
            None => None,
        }
    }
}

macro_rules! impl_query_value {
    ($ty: ty) => {
        impl QueryValue for $ty {
            fn query_value(&self, _query_builder: &dyn QueryBuilder) -> String {
                format!("{}", self)
            }

            fn primitive_value(&self) -> PrimitiveValue {
                self.clone().into()
            }
        }
    };
}

macro_rules! impl_query_value_quoted {
    ($ty: ty) => {
        impl QueryValue for $ty {
            fn query_value(&self, query_builder: &dyn QueryBuilder) -> String {
                let mut buf = String::new();
                query_builder.write_string_quoted(self.to_string().as_ref(), &mut buf);
                buf
            }

            fn primitive_value(&self) -> PrimitiveValue {
                self.clone().into()
            }
        }
    };
}

impl_query_value_quoted!(&'static str);
impl_query_value_quoted!(String);
impl_query_value!(i8);
impl_query_value!(i16);
impl_query_value!(i32);
impl_query_value!(i64);
impl_query_value!(u8);
impl_query_value!(u16);
impl_query_value!(u32);
impl_query_value!(u64);
impl_query_value!(f32);
impl_query_value!(f64);

impl QueryValue for bool {
    fn query_value(&self, _query_builder: &dyn QueryBuilder) -> String {
        if *self {
            "TRUE".to_string()
        } else {
            "FALSE".to_string()
        }
    }

    fn primitive_value(&self) -> PrimitiveValue {
        (*self).into()
    }
}

// impl QueryValue for () {
//     fn query_value(&self, _query_builder: &dyn QueryBuilder) -> String {
//         "NULL".to_string()
//     }
// }

impl QueryValue for Vec<u8> {
    fn query_value(&self, _query_builder: &dyn QueryBuilder) -> String {
        format!(
            "x\'{}\'",
            self.iter()
                .map(|b| format!("{:02X}", b))
                .collect::<String>()
        )
    }

    fn primitive_value(&self) -> PrimitiveValue {
        self.clone().into()
    }
}

#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
impl_query_value_quoted!(serde_json::Value);

#[cfg(feature = "with-chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
impl QueryValue for chrono::NaiveDate {
    fn query_value(&self, _query_builder: &dyn QueryBuilder) -> String {
        format!("\'{}\'", self.format("%Y-%m-%d").to_string())
    }

    fn primitive_value(&self) -> PrimitiveValue {
        self.clone().into()
    }
}

#[cfg(feature = "with-chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
impl QueryValue for chrono::NaiveTime {
    fn query_value(&self, _query_builder: &dyn QueryBuilder) -> String {
        format!("\'{}\'", self.format("%H:%M:%S").to_string())
    }

    fn primitive_value(&self) -> PrimitiveValue {
        self.clone().into()
    }
}

#[cfg(feature = "with-chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
impl QueryValue for chrono::NaiveDateTime {
    fn query_value(&self, _query_builder: &dyn QueryBuilder) -> String {
        format!("\'{}\'", self.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    fn primitive_value(&self) -> PrimitiveValue {
        self.clone().into()
    }
}

#[cfg(feature = "with-chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
impl QueryValue for chrono::DateTime<chrono::FixedOffset> {
    fn query_value(&self, _query_builder: &dyn QueryBuilder) -> String {
        format!("\'{}\'", self.format("%Y-%m-%d %H:%M:%S %:z").to_string())
    }

    fn primitive_value(&self) -> PrimitiveValue {
        self.clone().into()
    }
}

#[cfg(feature = "with-uuid")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
impl_query_value_quoted!(uuid::Uuid);

#[cfg(feature = "with-rust_decimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
impl_query_value!(rust_decimal::Decimal);

#[cfg(feature = "with-bigdecimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
impl_query_value!(bigdecimal::BigDecimal);

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
