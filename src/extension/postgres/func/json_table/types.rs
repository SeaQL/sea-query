use core::fmt;
use std::borrow::Cow;

use crate::extension::postgres::json_fn::{QuotesClause, WrapperClause};
use crate::*;

// Re-export QuotesKind for convenience
pub use crate::extension::postgres::json_fn::QuotesKind;

/// Represents a column definition in JSON_TABLE
#[derive(Debug, Clone)]
pub(super) enum JsonTableColumn {
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
pub(in super::super) enum OnClause {
    Error,
    Null,
    EmptyArray,
    EmptyObject,
    Default(Expr),
}

impl OnClause {
    pub(in super::super) fn write_to(&self, buf: &mut String) -> fmt::Result {
        match self {
            OnClause::Error => buf.write_str("ERROR")?,
            OnClause::Null => buf.write_str("NULL")?,
            OnClause::EmptyArray => buf.write_str("EMPTY ARRAY")?,
            OnClause::EmptyObject => buf.write_str("EMPTY OBJECT")?,
            OnClause::Default(expr) => {
                buf.write_str("DEFAULT ")?;
                PostgresQueryBuilder.prepare_simple_expr(expr, buf);
            }
        };
        Ok(())
    }
}

/// ON ERROR clause for EXISTS columns
#[derive(Debug, Clone)]
pub(super) enum ExistsOnErrorClause {
    Error,
    True,
    False,
    Unknown,
}

impl ExistsOnErrorClause {
    pub(super) fn write_to(&self, buf: &mut String) -> fmt::Result {
        match self {
            ExistsOnErrorClause::Error => buf.write_str("ERROR")?,
            ExistsOnErrorClause::True => buf.write_str("TRUE")?,
            ExistsOnErrorClause::False => buf.write_str("FALSE")?,
            ExistsOnErrorClause::Unknown => buf.write_str("UNKNOWN")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(super) enum OnErrorClause {
    Error,
    Empty,
    EmptyArray,
}

impl OnErrorClause {
    pub(super) fn write_to(&self, buf: &mut String) -> fmt::Result {
        buf.write_str(match self {
            OnErrorClause::Error => "ERROR",
            OnErrorClause::Empty => "EMPTY",
            OnErrorClause::EmptyArray => "EMPTY ARRAY",
        })?;

        buf.write_str(" ON ERROR")
    }
}
