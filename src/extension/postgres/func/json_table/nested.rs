use std::borrow::Cow;

use crate::extension::postgres::json_table::{Column, ExistsColumn};

use super::types::JsonTableColumn;

/// NESTED PATH column definition in a `JSON_TABLE` `COLUMNS` clause.
#[derive(Debug, Clone)]
pub struct NestedPath {
    // explicit_path: bool,
    path: Cow<'static, str>,
    json_path_name: Option<Cow<'static, str>>,
    columns: Vec<JsonTableColumn>,
}

impl NestedPath {
    pub fn new(path: impl Into<Cow<'static, str>>) -> Self {
        Self {
            // explicit_path: false,
            path: path.into(),
            json_path_name: None,
            columns: Vec::new(),
        }
    }

    pub fn path_name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.json_path_name = Some(name.into());
        self
    }

    pub fn for_ordinality(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.columns
            .push(JsonTableColumn::Ordinality { name: name.into() });
        self
    }

    pub fn column(mut self, column: Column) -> Self {
        self.columns.push(column.into_column());
        self
    }

    pub fn exists(mut self, column: ExistsColumn) -> Self {
        self.columns.push(column.into_column());
        self
    }

    pub fn nested(mut self, nested: NestedPath) -> Self {
        self.columns.push(nested.into_column());
        self
    }

    pub(super) fn into_column(self) -> JsonTableColumn {
        JsonTableColumn::Nested {
            // explicit_path: self.explicit_path,
            path: self.path,
            as_json_path_name: self.json_path_name,
            columns: self.columns,
        }
    }
}
