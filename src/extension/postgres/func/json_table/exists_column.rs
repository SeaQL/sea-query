use std::borrow::Cow;

use crate::{
    TypeRef,
    extension::postgres::json_table::{ExistsOnErrorClause, JsonTableColumn},
};

/// Builder for EXISTS columns in JSON_TABLE
#[derive(Debug)]
pub struct ExistsColumnBuilder<T> {
    pub(crate) builder: T,
    pub(crate) name: Cow<'static, str>,
    pub(crate) column_type: TypeRef,
    pub(crate) path: Option<Cow<'static, str>>,
    pub(crate) on_error: Option<ExistsOnErrorClause>,
}

impl<T> ExistsColumnBuilder<T> {
    /// Set PATH clause
    pub fn path<P>(mut self, path: P) -> Self
    where
        P: Into<Cow<'static, str>>,
    {
        self.path = Some(path.into());
        self
    }

    /// Set ON ERROR clause
    pub fn on_error(mut self, on_error: ExistsOnErrorClause) -> Self {
        self.on_error = Some(on_error);
        self
    }

    /// Convenience method for `ERROR ON ERROR`
    pub fn error_on_error(mut self) -> Self {
        self.on_error = Some(ExistsOnErrorClause::Error);
        self
    }

    /// Convenience method for `TRUE ON ERROR`
    pub fn true_on_error(mut self) -> Self {
        self.on_error = Some(ExistsOnErrorClause::True);
        self
    }

    /// Convenience method for `FALSE ON ERROR`
    pub fn false_on_error(mut self) -> Self {
        self.on_error = Some(ExistsOnErrorClause::False);
        self
    }

    /// Convenience method for `UNKNOWN ON ERROR`
    pub fn unknown_on_error(mut self) -> Self {
        self.on_error = Some(ExistsOnErrorClause::Unknown);
        self
    }
}

impl ExistsColumnBuilder<super::Builder> {
    /// Finish building this column and return to the main builder
    pub fn build_column(mut self) -> super::Builder {
        self.builder.columns.push(JsonTableColumn::Exists {
            name: self.name,
            column_type: self.column_type,
            path: self.path,
            on_error: self.on_error,
        });
        self.builder
    }
}
