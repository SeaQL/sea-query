//! Common types used in the library.

use std::{rc::Rc, str::from_utf8};
use std::fmt::Write as FmtWrite;
use serde_json::Value as JsonValue;
use crate::{query::*, expr::*, value::*};

/// Identifier in query
pub trait Iden {
    fn prepare(&self, s: &mut dyn FmtWrite, q: char) {
        write!(s, "{}", q).unwrap();
        self.unquoted(s);
        write!(s, "{}", q).unwrap();
    }

    fn to_string(&self) -> String {
        let s = &mut String::new();
        self.unquoted(s);
        s.to_owned()
    }

    fn unquoted(&self, s: &mut dyn FmtWrite);
}

/// All table references
#[derive(Clone)]
pub enum TableRef {
    Table(Rc<dyn Iden>),
    TableAlias(Rc<dyn Iden>, Rc<dyn Iden>),
    SubQuery(SelectStatement, Rc<dyn Iden>),
}

/// Unary operator
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnOper {
    Not,
}

/// Binary operator
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinOper {
    And,
    Or,
    Like,
    NotLike,
    Is,
    IsNot,
    In,
    NotIn,
    Between,
    NotBetween,
    Equal,
    NotEqual,
    SmallerThan,
    GreaterThan,
    SmallerThanOrEqual,
    GreaterThanOrEqual,
    Add,
    Sub,
    Mul,
    Div,
}

/// Logical chain operator
#[derive(Clone)]
pub enum LogicalChainOper {
    And(SimpleExpr),
    Or(SimpleExpr),
}

/// Query functions
#[derive(Clone, PartialEq, Eq)]
pub enum Function {
    Max,
    Min,
    Sum,
    Count,
    IfNull,
    Custom(String),
}

/// Join types
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Join,
    InnerJoin,
    LeftJoin,
    RightJoin,
}

/// Order expression
#[derive(Clone)]
pub struct OrderExpr {
    pub(crate) expr: SimpleExpr,
    pub(crate) order: Order,
}

/// Join on types
#[derive(Clone)]
pub enum JoinOn {
    Condition(Box<SimpleExpr>),
    Columns(Vec<SimpleExpr>),
}

/// Ordering options
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Asc,
    Desc,
}

/// Shorthand to create name alias
#[derive(Clone)]
pub struct Alias(String);

impl Alias {
    pub fn new(n: &str) -> Self {
        Self(n.to_owned())
    }
}

impl Iden for Alias {
    fn unquoted(&self, s: &mut dyn FmtWrite) {
        write!(s, "{}", self.0).unwrap();
    }
}

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
