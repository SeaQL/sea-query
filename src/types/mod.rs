//! Base types used throughout sea-query.

use crate::{Values, expr::*, query::*};
use std::fmt::Debug;

#[cfg(feature = "backend-postgres")]
use crate::extension::postgres::PgBinOper;
#[cfg(feature = "backend-sqlite")]
use crate::extension::sqlite::SqliteBinOper;

// Intentionally not `pub`, so that we're free to experiment with the internal structure.
mod iden;

pub use iden::*;

/// A reference counted pointer: either [`Rc`][std::rc::Rc] or [`Arc`][std::sync::Arc],
/// depending on the feature flags.
///
/// [`Arc`][std::sync::Arc] is used when `thread-safe` feature is activated.
#[cfg(not(feature = "thread-safe"))]
pub type RcOrArc<T> = std::rc::Rc<T>;
/// A reference counted pointer: either [`Rc`][std::rc::Rc] or [`Arc`][std::sync::Arc],
/// depending on the feature flags.
///
/// [`Arc`][std::sync::Arc] is used when `thread-safe` feature is activated.
#[cfg(feature = "thread-safe")]
pub type RcOrArc<T> = std::sync::Arc<T>;

/// A legacy namespace for compatibility.
///
/// It's needed, so that most existing [`SeaRc::new`][SeaRc::new] calls keep working.
///
/// This used to be an actual type
/// (a reference-counted pointer with special impls for `dyn Iden` contents).
/// It's not needed anymore.
#[derive(Debug)]
pub struct SeaRc;

impl SeaRc {
    /// A legacy method, kept for compatibility.
    ///
    /// Nowadays, instead of wrapping an `Iden` object,
    /// it eagerly "renders" it into a string and then drops the object.
    ///
    /// Note that most `Iden`s are statically known
    /// and their representations aren't actually "rendered" and allocated at runtime.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<I>(i: I) -> DynIden
    where
        I: Iden,
    {
        DynIden(i.quoted())
    }

    pub fn clone(iden: &DynIden) -> DynIden {
        iden.clone()
    }
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnOper {
    Not,
}

/// Binary operators.
///
/// If something is not supported here, you can use [`BinOper::Custom`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
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
    Mod,
    BitAnd,
    BitOr,
    LShift,
    RShift,
    As,
    Escape,
    Custom(&'static str),
    #[cfg(feature = "backend-postgres")]
    PgOperator(PgBinOper),
    #[cfg(feature = "backend-sqlite")]
    SqliteOperator(SqliteBinOper),
}

/// Logical chain operator: conjunction or disjunction.
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalChainOper {
    And(Expr),
    Or(Expr),
}

/// Join types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Join,
    CrossJoin,
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullOuterJoin,
    StraightJoin,
}

/// Nulls order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullOrdering {
    First,
    Last,
}

/// Order expression
#[derive(Debug, Clone, PartialEq)]
pub struct OrderExpr {
    pub(crate) expr: Expr,
    pub(crate) order: Order,
    pub(crate) nulls: Option<NullOrdering>,
}

/// Join on types
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum JoinOn {
    Condition(Box<ConditionHolder>),
    Columns(Vec<Expr>),
}

/// Ordering options
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Order {
    Asc,
    Desc,
    Field(Values),
}

/// Known SQL keywords that can be used as expressions.
///
/// If something is not supported here, you can use [`Keyword::Custom`].
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Keyword {
    Null,
    CurrentDate,
    CurrentTime,
    CurrentTimestamp,
    Default,
    Custom(DynIden),
}

/// Like Expression
#[derive(Debug, Clone)]
pub struct LikeExpr {
    pub(crate) pattern: String,
    pub(crate) escape: Option<char>,
}

impl LikeExpr {
    pub fn new<T>(pattern: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            pattern: pattern.into(),
            escape: None,
        }
    }

    #[deprecated(since = "0.29.0", note = "Please use the [`LikeExpr::new`] method")]
    pub fn str<T>(pattern: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            pattern: pattern.into(),
            escape: None,
        }
    }

    pub fn escape(self, c: char) -> Self {
        Self {
            pattern: self.pattern,
            escape: Some(c),
        }
    }
}

pub trait IntoLikeExpr: Into<LikeExpr> {
    fn into_like_expr(self) -> LikeExpr;
}

impl<T> IntoLikeExpr for T
where
    T: Into<LikeExpr>,
{
    fn into_like_expr(self) -> LikeExpr {
        self.into()
    }
}

impl<T> From<T> for LikeExpr
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        LikeExpr::new(value)
    }
}

/// SubQuery operators
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum SubQueryOper {
    Exists,
    Any,
    Some,
    All,
}
