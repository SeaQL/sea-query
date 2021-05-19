//! Base types used throughout sea-query.

use crate::{expr::*, query::*};
use std::fmt;
use std::rc::Rc;

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

pub trait IntoIden {
    fn into_iden(self) -> Rc<dyn Iden>;
}

pub trait IdenList {
    type IntoIter: Iterator<Item = Rc<dyn Iden>>;

    fn into_iter(self) -> Self::IntoIter;
}

impl fmt::Debug for dyn Iden {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.unquoted(formatter);
        Ok(())
    }
}

/// Column references
#[derive(Debug, Clone)]
pub enum ColumnRef {
    Column(Rc<dyn Iden>),
    TableColumn(Rc<dyn Iden>, Rc<dyn Iden>),
}

pub trait IntoColumnRef {
    fn into_column_ref(self) -> ColumnRef;
}

/// Table references
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum TableRef {
    Table(Rc<dyn Iden>),
    SchemaTable(Rc<dyn Iden>, Rc<dyn Iden>),
    TableAlias(Rc<dyn Iden>, Rc<dyn Iden>),
    SchemaTableAlias(Rc<dyn Iden>, Rc<dyn Iden>, Rc<dyn Iden>),
    SubQuery(SelectStatement, Rc<dyn Iden>),
}

pub trait IntoTableRef {
    fn into_table_ref(self) -> TableRef;
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

// Impl begins

impl<T: 'static> IntoIden for T
where
    T: Iden,
{
    fn into_iden(self) -> Rc<dyn Iden> {
        Rc::new(self)
    }
}

impl IntoIden for Rc<dyn Iden> {
    fn into_iden(self) -> Rc<dyn Iden> {
        self
    }
}

impl<I> IdenList for I
where
    I: IntoIden,
{
    type IntoIter = std::iter::Once<Rc<dyn Iden>>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.into_iden())
    }
}

impl<A, B> IdenList for (A, B)
where
    A: IntoIden,
    B: IntoIden,
{
    type IntoIter = std::vec::IntoIter<Rc<dyn Iden>>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.0.into_iden(), self.1.into_iden()].into_iter()
    }
}

impl<A, B, C> IdenList for (A, B, C)
where
    A: IntoIden,
    B: IntoIden,
    C: IntoIden,
{
    type IntoIter = std::vec::IntoIter<Rc<dyn Iden>>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.0.into_iden(), self.1.into_iden(), self.2.into_iden()].into_iter()
    }
}

impl<T: 'static> IntoColumnRef for T
where
    T: IntoIden,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::Column(self.into_iden())
    }
}

impl<S: 'static, T: 'static> IntoColumnRef for (S, T)
where
    S: IntoIden,
    T: IntoIden,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::TableColumn(self.0.into_iden(), self.1.into_iden())
    }
}

impl<T: 'static> IntoTableRef for T
where
    T: IntoIden,
{
    fn into_table_ref(self) -> TableRef {
        TableRef::Table(self.into_iden())
    }
}

impl<S: 'static, T: 'static> IntoTableRef for (S, T)
where
    S: IntoIden,
    T: IntoIden,
{
    fn into_table_ref(self) -> TableRef {
        TableRef::SchemaTable(self.0.into_iden(), self.1.into_iden())
    }
}

impl TableRef {
    pub fn alias<A: 'static>(self, alias: A) -> Self
    where
        A: IntoIden,
    {
        match self {
            Self::Table(table) => Self::TableAlias(table, alias.into_iden()),
            Self::SchemaTable(schema, table) => {
                Self::SchemaTableAlias(schema, table, alias.into_iden())
            }
            _ => panic!("unexpected TableRef variant"),
        }
    }
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
