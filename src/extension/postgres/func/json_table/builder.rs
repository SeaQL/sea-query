use core::fmt;
use std::borrow::Cow;
use std::fmt::Write;

use crate::extension::postgres::json_fn::{
    write_as_json_path_name, write_json_path_expr, write_passing,
};
use crate::*;

use super::column::Column;
use super::exists_column::ExistsColumn;
use super::nested::NestedPath;
use super::types::*;

/// Builder for JSON_TABLE function
#[derive(Debug, Clone)]
pub struct Builder {
    pub(super) context_item: Expr,
    pub(super) path_expression: Cow<'static, str>,
    pub(super) as_json_path_name: Option<Cow<'static, str>>,
    pub(super) passing: Vec<(Value, Cow<'static, str>)>,
    pub(super) columns: Vec<JsonTableColumn>,
    pub(super) on_error: Option<OnErrorClause>,
}

impl From<Builder> for FunctionCall {
    fn from(builder: Builder) -> Self {
        builder.build()
    }
}

impl From<Builder> for Expr {
    fn from(value: Builder) -> Self {
        Expr::FunctionCall(FunctionCall::from(value))
    }
}

impl Builder {
    /// Set the JSON path name (AS clause).
    pub fn path_name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.as_json_path_name = Some(name.into());
        self
    }

    /// Add PASSING parameters
    pub fn passing(mut self, value: impl Into<Value>, alias: impl Into<Cow<'static, str>>) -> Self {
        self.passing.push((value.into(), alias.into()));
        self
    }

    /// Add multiple PASSING parameters at once
    pub fn passing_many(
        mut self,
        passing: impl IntoIterator<Item = (impl Into<Value>, impl Into<Cow<'static, str>>)>,
    ) -> Self {
        for (value, alias) in passing {
            self.passing.push((value.into(), alias.into()));
        }
        self
    }

    /// Add a `FOR ORDINALITY` column.
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

    /// Convenience method for `ERROR ON ERROR`
    pub fn error_on_error(mut self) -> Self {
        self.on_error = Some(OnErrorClause::Error);
        self
    }

    /// Convenience method for `EMPTY ON ERROR`
    pub fn empty_on_error(mut self) -> Self {
        self.on_error = Some(OnErrorClause::Empty);
        self
    }

    /// Convenience method for `EMPTY ARRAY ON ERROR`
    pub fn empty_array_on_error(mut self) -> Self {
        self.on_error = Some(OnErrorClause::EmptyArray);
        self
    }

    pub fn build(self) -> FunctionCall {
        self.build_internal().unwrap()
    }

    fn build_internal(self) -> Result<FunctionCall, core::fmt::Error> {
        let mut buf = String::with_capacity(200);

        PostgresQueryBuilder.prepare_expr(&self.context_item, &mut buf);
        buf.write_str(", ")?;
        write_json_path_expr(&mut buf, &self.path_expression)?;

        if let Some(name) = &self.as_json_path_name {
            write_as_json_path_name(&mut buf, name)?;
        }

        write_passing(&mut buf, self.passing)?;

        Self::write_columns(&self.columns, &mut buf)?;

        if let Some(on_error) = &self.on_error {
            buf.write_str(" ")?;
            on_error.write_to(&mut buf)?;
        }

        Ok(FunctionCall::new(Func::Custom("JSON_TABLE".into())).arg(Expr::Custom(buf.into())))
    }

    fn write_columns(columns: &[JsonTableColumn], buf: &mut String) -> fmt::Result {
        buf.write_str(" COLUMNS (")?;
        let mut citer = columns.iter();
        join_io!(
            citer,
            col,
            join {
                buf.write_str(", ")?;
            },
            do {
                Self::write_column_static(col, buf)?;
            }
        );
        buf.write_str(")")?;

        Ok(())
    }

    fn write_column_static(
        column: &JsonTableColumn,
        buf: &mut String,
    ) -> Result<(), core::fmt::Error> {
        match column {
            JsonTableColumn::Ordinality { name } => {
                buf.write_str(name)?;
                buf.write_str(" FOR ORDINALITY")?;
            }
            JsonTableColumn::Regular {
                name,
                column_type,
                format_json,
                encoding_utf8,
                path,
                wrapper,
                quotes,
                on_empty,
                on_error,
            } => {
                buf.write_str(name)?;
                buf.write_str(" ")?;
                PostgresQueryBuilder.prepare_type_ref(column_type, buf);

                if *format_json {
                    buf.write_str(" FORMAT JSON")?;
                    if *encoding_utf8 {
                        buf.write_str(" ENCODING UTF8")?;
                    }
                }

                if let Some(path) = path {
                    buf.write_str(" PATH '")?;
                    buf.write_str(path)?;
                    buf.write_str("'")?;
                }

                if let Some(wrapper) = wrapper {
                    wrapper.write_to(buf)?;
                }

                if let Some(quotes) = quotes {
                    quotes.write_to(buf)?;
                }

                if let Some(on_empty) = on_empty {
                    buf.write_str(" ")?;
                    on_empty.write_to(buf)?;
                    buf.write_str(" ON EMPTY")?;
                }

                if let Some(on_error) = on_error {
                    buf.write_str(" ")?;
                    on_error.write_to(buf)?;
                    buf.write_str(" ON ERROR")?;
                }
            }
            JsonTableColumn::Exists {
                name,
                column_type,
                path,
                on_error,
            } => {
                buf.write_str(name)?;
                buf.write_str(" ")?;
                PostgresQueryBuilder.prepare_type_ref(column_type, buf);
                buf.write_str(" EXISTS")?;

                if let Some(path) = path {
                    buf.write_str(" PATH '")?;
                    buf.write_str(path)?;
                    buf.write_str("'")?;
                }

                if let Some(on_error) = on_error {
                    buf.write_str(" ")?;
                    on_error.write_to(buf)?;
                }
            }
            JsonTableColumn::Nested {
                path,
                as_json_path_name: json_path_name,
                columns,
                // explicit_path,
            } => {
                buf.write_str("NESTED PATH")?;
                buf.write_str(" ")?;
                write_json_path_expr(buf, path)?;

                if let Some(name) = json_path_name {
                    write_as_json_path_name(buf, name)?;
                }

                Self::write_columns(columns, buf)?;
            }
        }
        Ok(())
    }
}
