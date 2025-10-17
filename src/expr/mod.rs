//! Building blocks of SQL statements.
//!
//! [`Expr`] is an arbitrary, dynamically-typed SQL expression.
//! It can be used in select fields, where clauses and many other places.
//!
//! [`ExprTrait`] provides "operator" methods for building expressions.

// Intentionally not `pub`. They only export a single item each.
// It's a mechanical split to manage LoC.
mod conv;
mod r#enum;
mod r#trait;

pub use conv::IntoExpr;
pub use r#enum::Expr;
pub use r#trait::ExprTrait;

/// A legacy compatibility alias for [`Expr`].
///
/// These used to be two separate (but very similar) types.
pub type SimpleExpr = Expr;
