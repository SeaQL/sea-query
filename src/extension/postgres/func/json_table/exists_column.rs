use std::borrow::Cow;

use crate::{TypeRef, extension::postgres::json_table::ExistsOnErrorClause};

use super::types::JsonTableColumn;

/// EXISTS column definition in a `JSON_TABLE` `COLUMNS` clause.
#[derive(Debug, Clone)]
pub struct ExistsColumn {
    name: Cow<'static, str>,
    column_type: TypeRef,
    path: Option<Cow<'static, str>>,
    on_error: Option<ExistsOnErrorClause>,
}

impl ExistsColumn {
    pub fn new(name: impl Into<Cow<'static, str>>, column_type: impl Into<TypeRef>) -> Self {
        Self {
            name: name.into(),
            column_type: column_type.into(),
            path: None,
            on_error: None,
        }
    }

    pub fn path(mut self, path: impl Into<Cow<'static, str>>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn error_on_error(mut self) -> Self {
        self.on_error = Some(ExistsOnErrorClause::Error);
        self
    }

    pub fn true_on_error(mut self) -> Self {
        self.on_error = Some(ExistsOnErrorClause::True);
        self
    }

    pub fn false_on_error(mut self) -> Self {
        self.on_error = Some(ExistsOnErrorClause::False);
        self
    }

    pub fn unknown_on_error(mut self) -> Self {
        self.on_error = Some(ExistsOnErrorClause::Unknown);
        self
    }

    pub(super) fn into_column(self) -> JsonTableColumn {
        JsonTableColumn::Exists {
            name: self.name,
            column_type: self.column_type,
            path: self.path,
            on_error: self.on_error,
        }
    }
}
