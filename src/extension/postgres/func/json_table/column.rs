use std::borrow::Cow;

use crate::extension::postgres::json_fn::{QuotesClause, WrapperClause};
use crate::*;

use super::builder::Builder;
use super::types::*;

/// Builder for regular columns in JSON_TABLE
#[derive(Debug)]
pub struct ColumnBuilder<T> {
    pub(crate) builder: T,
    pub(crate) name: Cow<'static, str>,
    pub(crate) column_type: TypeRef,
    pub(crate) format_json: bool,
    pub(crate) encoding_utf8: bool,
    pub(crate) path: Option<Cow<'static, str>>,
    pub(crate) wrapper: Option<WrapperClause>,
    pub(crate) quotes: Option<QuotesClause>,
    pub(crate) on_empty: Option<OnClause>,
    pub(crate) on_error: Option<OnClause>,
}

impl<T> ColumnBuilder<T> {
    /// Set FORMAT JSON
    pub fn format_json(mut self) -> Self {
        self.format_json = true;
        self
    }

    /// Set ENCODING UTF8 (requires FORMAT JSON)
    pub fn encoding_utf8(mut self) -> Self {
        self.encoding_utf8 = true;
        self
    }

    /// Set PATH clause
    pub fn path<P>(mut self, path: P) -> Self
    where
        P: Into<Cow<'static, str>>,
    {
        self.path = Some(path.into());
        self
    }

    /// Set WRAPPER clause
    pub fn wrapper<W>(mut self, wrapper: W) -> Self
    where
        W: Into<WrapperClause>,
    {
        self.wrapper = Some(wrapper.into());
        self
    }

    /// Set QUOTES clause
    pub fn quotes<Q>(mut self, quotes: Q) -> Self
    where
        Q: Into<QuotesClause>,
    {
        self.quotes = Some(quotes.into());
        self
    }

    /// Set ON EMPTY clause
    pub fn on_empty(mut self, on_empty: OnClause) -> Self {
        self.on_empty = Some(on_empty);
        self
    }

    /// Set ON ERROR clause
    pub fn on_error(mut self, on_error: OnClause) -> Self {
        self.on_error = Some(on_error);
        self
    }

    /// Convenience method for `ERROR ON EMPTY`
    pub fn error_on_empty(mut self) -> Self {
        self.on_empty = Some(OnClause::Error);
        self
    }

    /// Convenience method for `NULL ON EMPTY`
    pub fn null_on_empty(mut self) -> Self {
        self.on_empty = Some(OnClause::Null);
        self
    }

    /// Convenience method for `EMPTY ARRAY ON EMPTY`
    pub fn empty_array_on_empty(mut self) -> Self {
        self.on_empty = Some(OnClause::EmptyArray);
        self
    }

    /// Convenience method for `EMPTY OBJECT ON EMPTY`
    pub fn empty_object_on_empty(mut self) -> Self {
        self.on_empty = Some(OnClause::EmptyObject);
        self
    }

    /// Convenience method for `DEFAULT <expr> ON EMPTY`
    pub fn default_on_empty<E>(mut self, expr: E) -> Self
    where
        E: Into<Expr>,
    {
        self.on_empty = Some(OnClause::Default(expr.into()));
        self
    }

    /// Convenience method for `ERROR ON ERROR`
    pub fn error_on_error(mut self) -> Self {
        self.on_error = Some(OnClause::Error);
        self
    }

    /// Convenience method for `NULL ON ERROR`
    pub fn null_on_error(mut self) -> Self {
        self.on_error = Some(OnClause::Null);
        self
    }

    /// Convenience method for `EMPTY ARRAY ON ERROR`
    pub fn empty_array_on_error(mut self) -> Self {
        self.on_error = Some(OnClause::EmptyArray);
        self
    }

    /// Convenience method for `EMPTY OBJECT ON ERROR`
    pub fn empty_object_on_error(mut self) -> Self {
        self.on_error = Some(OnClause::EmptyObject);
        self
    }

    /// Convenience method for `DEFAULT <expr> ON ERROR`
    pub fn default_on_error<E>(mut self, expr: E) -> Self
    where
        E: Into<Expr>,
    {
        self.on_error = Some(OnClause::Default(expr.into()));
        self
    }
}

impl ColumnBuilder<Builder> {
    /// Finish building this column and return to the main builder
    pub fn build_column(mut self) -> Builder {
        self.builder.columns.push(JsonTableColumn::Regular {
            name: self.name,
            column_type: self.column_type,
            format_json: self.format_json,
            encoding_utf8: self.encoding_utf8,
            path: self.path,
            wrapper: self.wrapper,
            quotes: self.quotes,
            on_empty: self.on_empty,
            on_error: self.on_error,
        });
        self.builder
    }
}
