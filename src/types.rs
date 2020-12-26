//! Common types used in the library.

use std::rc::Rc;
use std::fmt::Write;
use crate::{query::*, expr::*};

/// Identifier in query
pub trait Iden {
    fn prepare(&self, s: &mut dyn Write, q: char) {
        write!(s, "{}", q).unwrap();
        self.unquoted(s);
        write!(s, "{}", q).unwrap();
    }

    fn to_string(&self) -> String {
        let s = &mut String::new();
        self.unquoted(s);
        s.to_owned()
    }

    fn unquoted(&self, s: &mut dyn Write);
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
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "{}", self.0).unwrap();
    }
}
