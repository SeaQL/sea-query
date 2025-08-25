use std::borrow::Cow;

use crate::extension::postgres::json_fn::{QuotesClause, WrapperClause};
use crate::*;

// Re-export QuotesKind for convenience
pub use crate::extension::postgres::json_fn::QuotesKind;

/// Represents a column definition in JSON_TABLE
#[derive(Debug, Clone)]
pub enum JsonTableColumn {
    /// FOR ORDINALITY column
    Ordinality { name: Cow<'static, str> },
    /// Regular column with type and optional clauses
    Regular {
        name: Cow<'static, str>,
        column_type: TypeRef,
        format_json: bool,
        encoding_utf8: bool,
        path: Option<Cow<'static, str>>,
        wrapper: Option<WrapperClause>,
        quotes: Option<QuotesClause>,
        on_empty: Option<OnClause>,
        on_error: Option<OnClause>,
    },
    /// EXISTS column
    Exists {
        name: Cow<'static, str>,
        column_type: TypeRef,
        path: Option<Cow<'static, str>>,
        on_error: Option<ExistsOnErrorClause>,
    },
    /// NESTED PATH column
    Nested {
        explicit_path: bool,
        path: Cow<'static, str>,
        as_json_path_name: Option<Cow<'static, str>>,
        columns: Vec<JsonTableColumn>,
    },
}

/// ON EMPTY/ON ERROR clause for regular columns
#[derive(Debug, Clone)]
pub enum OnClause {
    Error,
    Null,
    EmptyArray,
    EmptyObject,
    Default(Expr),
}

/// ON ERROR clause for EXISTS columns
#[derive(Debug, Clone)]
pub enum ExistsOnErrorClause {
    Error,
    True,
    False,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum OnErrorClause {
    Error,
    Empty,
    EmptyArray,
}
