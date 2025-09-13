use std::{borrow::Cow, fmt::Write};

use crate::{
    Expr, Func, FunctionCall, PgFunc, PostgresQueryBuilder, QueryBuilder, Value,
    extension::postgres::json_fn::{write_json_path_expr, write_passing},
};

#[derive(Debug, Clone)]
pub struct Builder {
    context_item: Expr,
    path_expression: Cow<'static, str>,
    passing: Vec<(Value, Cow<'static, str>)>,
    on_error: Option<OnClause>,
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

#[derive(Debug, Clone)]
pub(super) enum OnClause {
    True,
    False,
    Unknown,
    Error,
}

impl From<bool> for OnClause {
    fn from(value: bool) -> Self {
        if value {
            OnClause::True
        } else {
            OnClause::False
        }
    }
}

impl Builder {
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

    /// Convenience method for `TRUE ON ERROR`
    pub fn true_on_error(mut self) -> Self {
        self.on_error = Some(OnClause::True);
        self
    }

    /// Convenience method for `FALSE ON ERROR`
    pub fn false_on_error(mut self) -> Self {
        self.on_error = Some(OnClause::False);
        self
    }

    /// Convenience method for `UNKNOWN ON ERROR`
    pub fn unknown_on_error(mut self) -> Self {
        self.on_error = Some(OnClause::Unknown);
        self
    }

    /// Convenience method for `ERROR ON ERROR`
    pub fn error_on_error(mut self) -> Self {
        self.on_error = Some(OnClause::Error);
        self
    }

    pub fn build(self) -> FunctionCall {
        self.build_internal().unwrap()
    }

    fn build_internal(self) -> Result<FunctionCall, core::fmt::Error> {
        let mut buf = String::with_capacity(20);

        PostgresQueryBuilder.prepare_expr(&self.context_item, &mut buf);
        buf.write_str(" ")?;
        write_json_path_expr(&mut buf, &self.path_expression)?;

        write_passing(&mut buf, self.passing)?;

        if let Some(on_error) = self.on_error {
            buf.write_str(match on_error {
                OnClause::True => " TRUE",
                OnClause::False => " FALSE",
                OnClause::Unknown => " UNKNOWN",
                OnClause::Error => " ERROR",
            })?;
            buf.write_str(" ON ERROR")?;
        }

        Ok(FunctionCall::new(Func::Custom("JSON_EXISTS".into())).arg(Expr::Custom(buf.into())))
    }
}

impl PgFunc {
    /// Create a `JSON_EXISTS` function builder. Postgres only.
    ///
    /// # Examples
    ///
    /// Basic usage with PASSING parameters:
    /// ```
    /// use sea_query::extension::postgres::*;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(
    ///         PgFunc::json_exists(
    ///            Expr::cust(r#"jsonb '{"key1": [1,2,3]}'"#),
    ///             "strict $.key1[*] ? (@ > $x)",
    ///         )
    ///         .passing(2, "x"),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_EXISTS(jsonb '{"key1": [1,2,3]}' 'strict $.key1[*] ? (@ > $x)' PASSING 2 AS x)"#
    /// );
    /// ```
    ///
    /// With ERROR ON ERROR clause:
    /// ```
    /// use sea_query::extension::postgres::*;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(
    ///         PgFunc::json_exists(Expr::cust(r#"jsonb '{"a": [1,2,3]}'"#), "lax $.a[5]")
    ///             .error_on_error(),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_EXISTS(jsonb '{"a": [1,2,3]}' 'lax $.a[5]' ERROR ON ERROR)"#
    /// );
    /// ```
    ///
    /// With strict mode and ERROR ON ERROR:
    /// ```
    /// use sea_query::extension::postgres::*;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(
    ///         PgFunc::json_exists(Expr::cust(r#"jsonb '{"a": [1,2,3]}'"#), "strict $.a[5]")
    ///             .error_on_error(),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_EXISTS(jsonb '{"a": [1,2,3]}' 'strict $.a[5]' ERROR ON ERROR)"#
    /// );
    /// ```
    pub fn json_exists<C, P>(context_item: C, path_expression: P) -> Builder
    where
        C: Into<Expr>,
        P: Into<Cow<'static, str>>,
    {
        Builder {
            context_item: context_item.into(),
            path_expression: path_expression.into(),
            passing: Vec::new(),
            on_error: None,
        }
    }
}
