use std::borrow::Cow;

use crate::extension::postgres::json_fn::{QuotesClause, WrapperClause};
use crate::*;

use super::types::{JsonTableColumn, OnClause};

/// Column definition in a `JSON_TABLE` `COLUMNS` clause.
#[derive(Debug, Clone)]
pub struct Column {
    name: Cow<'static, str>,
    column_type: TypeRef,
    format_json: bool,
    encoding_utf8: bool,
    path: Option<Cow<'static, str>>,
    wrapper: Option<WrapperClause>,
    quotes: Option<QuotesClause>,
    on_empty: Option<OnClause>,
    on_error: Option<OnClause>,
}

impl Column {
    pub fn new(name: impl Into<Cow<'static, str>>, column_type: impl Into<TypeRef>) -> Self {
        Self {
            name: name.into(),
            column_type: column_type.into(),
            format_json: false,
            encoding_utf8: false,
            path: None,
            wrapper: None,
            quotes: None,
            on_empty: None,
            on_error: None,
        }
    }

    /// Set `FORMAT JSON`.
    pub fn format_json(mut self) -> Self {
        self.format_json = true;
        self
    }

    /// Set `ENCODING UTF8`.
    pub fn encoding_utf8(mut self) -> Self {
        self.format_json = true;
        self.encoding_utf8 = true;
        self
    }

    /// Set `PATH`.
    pub fn path(mut self, path: impl Into<Cow<'static, str>>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Set `WRAPPER`.
    pub fn wrapper(mut self, wrapper: impl Into<WrapperClause>) -> Self {
        self.wrapper = Some(wrapper.into());
        self
    }

    /// Set `QUOTES`.
    pub fn quotes(mut self, quotes: impl Into<QuotesClause>) -> Self {
        self.quotes = Some(quotes.into());
        self
    }

    pub fn error_on_empty(mut self) -> Self {
        self.on_empty = Some(OnClause::Error);
        self
    }

    pub fn null_on_empty(mut self) -> Self {
        self.on_empty = Some(OnClause::Null);
        self
    }

    pub fn empty_array_on_empty(mut self) -> Self {
        self.on_empty = Some(OnClause::EmptyArray);
        self
    }

    pub fn empty_object_on_empty(mut self) -> Self {
        self.on_empty = Some(OnClause::EmptyObject);
        self
    }

    pub fn default_on_empty(mut self, expr: impl Into<Expr>) -> Self {
        self.on_empty = Some(OnClause::Default(expr.into()));
        self
    }

    pub fn error_on_error(mut self) -> Self {
        self.on_error = Some(OnClause::Error);
        self
    }

    pub fn null_on_error(mut self) -> Self {
        self.on_error = Some(OnClause::Null);
        self
    }

    pub fn empty_array_on_error(mut self) -> Self {
        self.on_error = Some(OnClause::EmptyArray);
        self
    }

    pub fn empty_object_on_error(mut self) -> Self {
        self.on_error = Some(OnClause::EmptyObject);
        self
    }

    pub fn default_on_error(mut self, expr: impl Into<Expr>) -> Self {
        self.on_error = Some(OnClause::Default(expr.into()));
        self
    }

    pub(super) fn into_column(self) -> JsonTableColumn {
        JsonTableColumn::Regular {
            name: self.name,
            column_type: self.column_type,
            format_json: self.format_json,
            encoding_utf8: self.encoding_utf8,
            path: self.path,
            wrapper: self.wrapper,
            quotes: self.quotes,
            on_empty: self.on_empty,
            on_error: self.on_error,
        }
    }
}
