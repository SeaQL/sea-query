use std::borrow::Cow;

use crate::extension::postgres::json_fn::{QuotesClause, WrapperClause};
use crate::extension::postgres::json_table::{ColumnBuilder, ExistsColumnBuilder};
use crate::*;

use super::builder::Builder;
use super::types::*;

/// Builder for NESTED PATH columns in JSON_TABLE
#[derive(Debug)]
pub struct NestedPathBuilder {
    pub(crate) builder: Builder,
    pub(crate) explicit: bool,
    pub(crate) path: Cow<'static, str>,
    pub(crate) json_path_name: Option<Cow<'static, str>>,
    pub(crate) columns: Vec<JsonTableColumn>,
}

impl NestedPathBuilder {
    /// Set the JSON path name (AS clause)
    pub fn json_path_name<T>(mut self, name: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.json_path_name = Some(name.into());
        self
    }

    /// Add a FOR ORDINALITY column to the nested path
    pub fn ordinality_column<N>(mut self, name: N) -> Self
    where
        N: Into<Cow<'static, str>>,
    {
        self.columns
            .push(JsonTableColumn::Ordinality { name: name.into() });
        self
    }

    /// Explicitly specify the path keyword
    pub fn explicit_path(mut self, value: bool) -> Self {
        self.explicit = value;
        self
    }

    /// Add a regular column to the nested path
    pub fn column<N, T>(self, name: N, column_type: T) -> ColumnBuilder<NestedPathBuilder>
    where
        N: Into<Cow<'static, str>>,
        T: Into<TypeRef>,
    {
        ColumnBuilder {
            builder: self,
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

    /// Add an EXISTS column to the nested path
    pub fn exists_column<N, T>(
        self,
        name: N,
        column_type: T,
    ) -> ExistsColumnBuilder<NestedPathBuilder>
    where
        N: Into<Cow<'static, str>>,
        T: Into<TypeRef>,
    {
        ExistsColumnBuilder {
            builder: self,
            name: name.into(),
            column_type: column_type.into(),
            path: None,
            on_error: None,
        }
    }

    /// Add another NESTED PATH column to the nested path
    pub fn nested<P>(self, path: P) -> NestedPathBuilder
    where
        P: Into<Cow<'static, str>>,
    {
        // Create a new nested builder that will add to this one's columns
        NestedPathBuilder {
            builder: self.builder,
            path: path.into(),
            json_path_name: None,
            columns: Vec::new(),
            explicit: false,
        }
    }

    /// Finish building this nested path and return to the main builder
    pub fn build_nested(mut self) -> Builder {
        self.builder.columns.push(JsonTableColumn::Nested {
            path: self.path,
            as_json_path_name: self.json_path_name,
            columns: self.columns,
            explicit_path: false,
        });
        self.builder
    }
}

impl ColumnBuilder<NestedPathBuilder> {
    /// Finish building this column and return to the nested path builder
    pub fn build_column(mut self) -> NestedPathBuilder {
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

impl ExistsColumnBuilder<NestedPathBuilder> {
    /// Finish building this column and return to the nested path builder
    pub fn build_column(mut self) -> NestedPathBuilder {
        self.builder.columns.push(JsonTableColumn::Exists {
            name: self.name,
            column_type: self.column_type,
            path: self.path,
            on_error: self.on_error,
        });
        self.builder
    }
}
