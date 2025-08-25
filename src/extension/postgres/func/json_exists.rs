use std::{borrow::Cow, fmt::Write};

use crate::{Expr, Func, FunctionCall, PgFunc, PostgresQueryBuilder, QueryBuilder, Value, join_io};

#[derive(Debug, Clone)]
pub struct Builder {
    pub(crate) context_item: Expr,
    pub(crate) path_expression: Cow<'static, str>,
    pub(crate) passing: Vec<(Value, Cow<'static, str>)>,
    pub(crate) on_error: Option<OnClause>,
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
pub enum OnClause {
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

    /// Replace current PASSING parameters
    pub fn set_passing<V, A, I>(mut self, passing: I) -> Self
    where
        V: Into<Value>,
        A: Into<Cow<'static, str>>,
        I: IntoIterator<Item = (V, A)>,
    {
        self.passing = passing
            .into_iter()
            .map(|(v, a)| (v.into(), a.into()))
            .collect();
        self
    }

    /// Set ON ERROR clause
    pub fn on_error<E>(mut self, on_error: E) -> Self
    where
        E: Into<OnClause>,
    {
        self.on_error = Some(on_error.into());
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

        PostgresQueryBuilder.prepare_simple_expr(&self.context_item, &mut buf);
        buf.write_str(" '")?;
        buf.write_str(&self.path_expression)?;
        buf.write_str("'")?;

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

        if let Some(on_error) = self.on_error {
            buf.write_str(match on_error {
                OnClause::True => " TRUE",
                OnClause::False => " FALSE",
                OnClause::Unknown => " UNKNOWN",
                OnClause::Error => " ERROR",
            })?;
            buf.write_str(" ON ERROR")?;
        }

        Ok(FunctionCall::new(Func::Custom("JSON_EXISTS".into())).arg(Expr::Custom(buf)))
    }
}

impl PgFunc {
    /// Create a `JSON_EXISTS` function builder. Postgres only.
    ///
    /// The `JSON_EXISTS` function tests whether a JSON path expression returns any items for the specified JSON value.
    /// It returns a boolean value: `true` if the path expression returns one or more items, `false` otherwise.
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
    ///             Expr::val(r#"{"key1": [1,2,3]}"#),
    ///             "strict $.key1[*] ? (@ > $x)",
    ///         )
    ///         .passing(2, "x"),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_EXISTS(E'{\"key1\": [1,2,3]}' 'strict $.key1[*] ? (@ > $x)' PASSING 2 AS x)"#
    /// );
    /// ```
    ///
    /// With ERROR ON ERROR clause:
    /// ```
    /// use sea_query::extension::postgres::*;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::json_exists(Expr::val(r#"{"a": [1,2,3]}"#), "lax $.a[5]").error_on_error())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_EXISTS(E'{\"a\": [1,2,3]}' 'lax $.a[5]' ERROR ON ERROR)"#
    /// );
    /// ```
    ///
    /// With strict mode and ERROR ON ERROR:
    /// ```
    /// use sea_query::extension::postgres::*;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::json_exists(Expr::val(r#"{"a": [1,2,3]}"#), "strict $.a[5]").error_on_error())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_EXISTS(E'{\"a\": [1,2,3]}' 'strict $.a[5]' ERROR ON ERROR)"#
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
