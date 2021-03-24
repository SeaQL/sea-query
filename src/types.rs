//! Common types used in the library.

use std::fmt;
use std::rc::Rc;
use crate::{query::*, expr::*};

/// Identifier in query
pub trait Iden {
    fn prepare(&self, s: &mut dyn fmt::Write, q: char) {
        write!(s, "{}", q).unwrap();
        self.unquoted(s);
        write!(s, "{}", q).unwrap();
    }

    fn to_string(&self) -> String {
        let s = &mut String::new();
        self.unquoted(s);
        s.to_owned()
    }

    fn unquoted(&self, s: &mut dyn fmt::Write);
}

impl fmt::Debug for dyn Iden {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.unquoted(formatter);
        Ok(())
    }
}

/// All table references
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum TableRef {
    Table(Rc<dyn Iden>),
    SchemaTable(Rc<dyn Iden>, Rc<dyn Iden>),
    TableAlias(Rc<dyn Iden>, Rc<dyn Iden>),
    SchemaTableAlias(Rc<dyn Iden>, Rc<dyn Iden>, Rc<dyn Iden>),
    SubQuery(SelectStatement, Rc<dyn Iden>),
}

/// Unary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOper {
    Not,
}

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone)]
pub enum LogicalChainOper {
    And(SimpleExpr),
    Or(SimpleExpr),
}

/// Join types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Join,
    InnerJoin,
    LeftJoin,
    RightJoin,
}

/// Order expression
#[derive(Debug, Clone)]
pub struct OrderExpr {
    pub(crate) expr: SimpleExpr,
    pub(crate) order: Order,
}

/// Join on types
#[derive(Debug, Clone)]
pub enum JoinOn {
    Condition(Box<SimpleExpr>),
    Columns(Vec<SimpleExpr>),
}

/// Ordering options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Asc,
    Desc,
}

/// Helper for create name alias
#[derive(Debug, Clone)]
pub struct Alias(String);

/// Common SQL Keywords
#[derive(Debug, Clone)]
pub enum Keyword {
    Null,
    Custom(Rc<dyn Iden>),
}

impl Alias {
    pub fn new(n: &str) -> Self {
        Self(n.to_owned())
    }
}

impl Iden for Alias {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        write!(s, "{}", self.0).unwrap();
    }
}
