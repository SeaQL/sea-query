use core::fmt;
use std::borrow::Cow;
use std::fmt::Write;

use crate::extension::postgres::json_fn::WrapperKind;
use crate::extension::postgres::json_table::ExistsColumnBuilder;
use crate::*;

use super::column::ColumnBuilder;
use super::nested::NestedPathBuilder;
use super::types::*;

/// Builder for JSON_TABLE function
#[derive(Debug, Clone)]
pub struct Builder {
    pub(crate) context_item: Expr,
    pub(crate) path_expression: Cow<'static, str>,
    pub(crate) as_json_path_name: Option<Cow<'static, str>>,
    pub(crate) passing: Vec<(Value, Cow<'static, str>)>,
    pub(crate) columns: Vec<JsonTableColumn>,
    pub(crate) on_error: Option<OnErrorClause>,
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
    /// Set the JSON path name (AS clause)
    pub fn json_path_name<T>(mut self, name: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.as_json_path_name = Some(name.into());
        self
    }

    /// Add PASSING parameters
    pub fn passing<V, A>(mut self, value: V, alias: A) -> Self
    where
        V: Into<Value>,
        A: Into<Cow<'static, str>>,
    {
        self.passing.push((value.into(), alias.into()));
        self
    }

    /// Add multiple PASSING parameters at once
    pub fn passing_many<V, A, I>(mut self, passing: I) -> Self
    where
        V: Into<Value>,
        A: Into<Cow<'static, str>>,
        I: IntoIterator<Item = (V, A)>,
    {
        for (value, alias) in passing {
            self.passing.push((value.into(), alias.into()));
        }
        self
    }

    /// Add a FOR ORDINALITY column
    pub fn ordinality_column<N>(mut self, name: N) -> Self
    where
        N: Into<Cow<'static, str>>,
    {
        self.columns
            .push(JsonTableColumn::Ordinality { name: name.into() });
        self
    }

    /// Add a regular column
    pub fn column<N, T>(self, name: N, column_type: T) -> ColumnBuilder<Self>
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

    /// Add an EXISTS column
    pub fn exists_column<N, T>(self, name: N, column_type: T) -> ExistsColumnBuilder<Self>
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

    /// Add a NESTED PATH column
    pub fn nested<P>(self, path: P) -> NestedPathBuilder
    where
        P: Into<Cow<'static, str>>,
    {
        NestedPathBuilder {
            builder: self,
            path: path.into(),
            explicit: false,
            json_path_name: None,
            columns: Vec::new(),
        }
    }

    /// Set ON ERROR clause for the entire JSON_TABLE
    pub fn on_error(mut self, on_error: OnErrorClause) -> Self {
        self.on_error = Some(on_error);
        self
    }

    /// Convenience method for `ERROR ON ERROR`
    pub fn error_on_error(mut self) -> Self {
        self.on_error = Some(OnErrorClause::Error);
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

        PostgresQueryBuilder.prepare_simple_expr(&self.context_item, &mut buf);
        buf.write_str(", '")?;
        buf.write_str(&self.path_expression)?;
        buf.write_str("'")?;

        // AS json_path_name clause
        if let Some(ref json_path_name) = self.as_json_path_name {
            buf.write_str(" AS ")?;
            buf.write_str(json_path_name)?;
        }

        // PASSING clause
        let mut piter = self.passing.into_iter();
        join_io!(
            piter,
            value_as,
            first {
                buf.write_str(" PASSING ")?;
            },
            join {
                buf.write_str(", ")?;
            },
            do {
                PostgresQueryBuilder.prepare_value(value_as.0, &mut buf);
                buf.write_str(" AS ")?;
                buf.write_str(&value_as.1)?;
            }
        );

        // COLUMNS clause
        buf.write_str(" COLUMNS (")?;
        Self::write_columns(&self.columns, &mut buf)?;
        buf.write_str(")")?;

        // ON ERROR clause
        if let Some(on_error) = self.on_error {
            buf.write_str(match on_error {
                OnErrorClause::Error => " ERROR",
                OnErrorClause::Empty => " EMPTY",
                OnErrorClause::EmptyArray => " EMPTY ARRAY",
            })?;

            buf.write_str(" ON ERROR")?;
        }

        Ok(FunctionCall::new(Func::Custom("JSON_TABLE".into())).arg(Expr::Custom(buf)))
    }

    fn write_columns(columns: &[JsonTableColumn], buf: &mut String) -> fmt::Result {
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
                    match wrapper.kind {
                        WrapperKind::Without => buf.write_str(" WITHOUT")?,
                        WrapperKind::WithConditional => buf.write_str(" WITH CONDITIONAL")?,
                        WrapperKind::WithUnconditional => buf.write_str(" WITH UNCONDITIONAL")?,
                    }
                    if wrapper.array {
                        buf.write_str(" ARRAY")?;
                    }
                    buf.write_str(" WRAPPER")?;
                }

                if let Some(quotes) = quotes {
                    quotes.prepare(buf)?;
                }

                if let Some(on_empty) = on_empty {
                    match on_empty {
                        OnClause::Error => buf.write_str(" ERROR")?,
                        OnClause::Null => buf.write_str(" NULL")?,
                        OnClause::EmptyArray => buf.write_str(" EMPTY ARRAY")?,
                        OnClause::EmptyObject => buf.write_str(" EMPTY OBJECT")?,
                        OnClause::Default(expr) => {
                            buf.write_str(" DEFAULT ")?;
                            PostgresQueryBuilder.prepare_simple_expr(expr, buf);
                        }
                    }
                    buf.write_str(" ON EMPTY")?;
                }

                if let Some(on_error) = on_error {
                    match on_error {
                        OnClause::Error => buf.write_str(" ERROR")?,
                        OnClause::Null => buf.write_str(" NULL")?,
                        OnClause::EmptyArray => buf.write_str(" EMPTY ARRAY")?,
                        OnClause::EmptyObject => buf.write_str(" EMPTY OBJECT")?,
                        OnClause::Default(expr) => {
                            buf.write_str(" DEFAULT ")?;
                            PostgresQueryBuilder.prepare_simple_expr(expr, buf);
                        }
                    }
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
                    match on_error {
                        ExistsOnErrorClause::Error => buf.write_str(" ERROR")?,
                        ExistsOnErrorClause::True => buf.write_str(" TRUE")?,
                        ExistsOnErrorClause::False => buf.write_str(" FALSE")?,
                        ExistsOnErrorClause::Unknown => buf.write_str(" UNKNOWN")?,
                    }
                    buf.write_str(" ON ERROR")?;
                }
            }
            JsonTableColumn::Nested {
                path,
                as_json_path_name: json_path_name,
                columns,
                explicit_path,
            } => {
                buf.write_str("NESTED")?;
                if *explicit_path {
                    buf.write_str(" PATH")?;
                }
                buf.write_str(" '")?;
                buf.write_str(path)?;
                buf.write_str("'")?;

                if let Some(json_path_name) = json_path_name {
                    buf.write_str(" AS ")?;
                    buf.write_str(json_path_name)?;
                }

                buf.write_str(" COLUMNS (")?;
                Self::write_columns(columns, buf)?;
                buf.write_str(")")?;
            }
        }
        Ok(())
    }
}
