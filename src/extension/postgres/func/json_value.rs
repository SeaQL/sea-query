use std::borrow::Cow;

use crate::{
    extension::postgres::json_fn::{write_json_path_expr, write_passing},
    *,
};

#[derive(Debug, Clone)]
pub struct Builder {
    context_item: Expr,
    path_expression: Cow<'static, str>,
    passing: Vec<(Value, Cow<'static, str>)>,
    returning: Option<crate::TypeRef>,
    on_empty: Option<OnClause>,
    on_error: Option<OnClause>,
}

#[derive(Debug, Clone)]
enum OnClause {
    Error,
    Null,
    Default(Expr),
}

impl Builder {
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

    /// Set RETURNING clause
    pub fn returning<T>(mut self, returning: T) -> Self
    where
        T: Into<TypeRef>,
    {
        self.returning = Some(returning.into());
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

    /// Convenience method for `DEFAULT <expr> ON EMPTY`
    pub fn default_on_empty<T>(mut self, expr: T) -> Self
    where
        T: Into<Expr>,
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

    /// Convenience method for `DEFAULT <expr> ON ERROR`
    pub fn default_on_error<T>(mut self, expr: T) -> Self
    where
        T: Into<Expr>,
    {
        self.on_error = Some(OnClause::Default(expr.into()));
        self
    }

    pub fn build(self) -> FunctionCall {
        self.build_internal().unwrap()
    }

    fn build_internal(self) -> Result<FunctionCall, core::fmt::Error> {
        let mut buf = String::with_capacity(50);

        PostgresQueryBuilder.prepare_simple_expr(&self.context_item, &mut buf);
        buf.write_str(" ")?;
        write_json_path_expr(&mut buf, &self.path_expression)?;

        write_passing(&mut buf, self.passing)?;

        if let Some(returning) = self.returning {
            buf.write_str(" RETURNING ")?;

            PostgresQueryBuilder.prepare_type_ref(&returning, &mut buf);
        }

        if let Some(on_empty) = self.on_empty {
            match on_empty {
                OnClause::Error => buf.write_str(" ERROR")?,
                OnClause::Null => buf.write_str(" NULL")?,
                OnClause::Default(expr) => {
                    buf.write_str(" DEFAULT ")?;
                    PostgresQueryBuilder.prepare_simple_expr(&expr, &mut buf);
                }
            }
            buf.write_str(" ON EMPTY")?;
        }

        if let Some(on_error) = self.on_error {
            match on_error {
                OnClause::Error => buf.write_str(" ERROR")?,
                OnClause::Null => buf.write_str(" NULL")?,
                OnClause::Default(expr) => {
                    buf.write_str(" DEFAULT ")?;
                    PostgresQueryBuilder.prepare_simple_expr(&expr, &mut buf);
                }
            };
            buf.write_str(" ON ERROR")?;
        }

        Ok(FunctionCall::new(Func::Custom("JSON_VALUE".into())).arg(Expr::Custom(buf)))
    }
}

impl From<Builder> for FunctionCall {
    fn from(builder: Builder) -> Self {
        builder.build()
    }
}

impl From<Builder> for Expr {
    fn from(builder: Builder) -> Self {
        Expr::FunctionCall(builder.build())
    }
}

impl PgFunc {
    /// Create a `JSON_VALUE` function builder. Postgres only.
    ///
    /// # Examples
    ///
    /// Basic usage with RETURNING:
    /// ```
    /// use sea_query::extension::postgres::*;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(
    ///         PgFunc::json_value(Expr::cust(r#"jsonb '"123.45"'"#), "$")
    ///             .returning("float")
    ///             .build(),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_VALUE(jsonb '"123.45"' '$' RETURNING "float")"#
    /// );
    /// ```
    ///
    /// With PASSING parameters:
    /// ```
    /// use sea_query::extension::postgres::*;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(
    ///         PgFunc::json_value(Expr::cust(r#"jsonb '[1,2]'"#), "strict $[$off]")
    ///             .passing(1, "off")
    ///             .build(),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_VALUE(jsonb '[1,2]' 'strict $[$off]' PASSING 1 AS off)"#
    /// );
    /// ```
    ///
    /// With DEFAULT ON ERROR:
    /// ```
    /// use sea_query::extension::postgres::*;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(
    ///         PgFunc::json_value(Expr::val(r#"[1,2]"#), "strict $[*]").default_on_error(Expr::val(9)),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_VALUE('[1,2]' 'strict $[*]' DEFAULT 9 ON ERROR)"#
    /// );
    /// ```
    pub fn json_value<T, P>(context_item: T, path_expression: P) -> Builder
    where
        T: Into<Expr>,
        P: Into<Cow<'static, str>>,
    {
        Builder {
            context_item: context_item.into(),
            path_expression: path_expression.into(),
            passing: Vec::new(),
            returning: None,
            on_empty: None,
            on_error: None,
        }
    }
}
