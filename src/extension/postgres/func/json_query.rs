use std::borrow::Cow;

use crate::{
    extension::postgres::{
        json_fn::{QuotesClause, WrapperClause, write_json_path_expr, write_passing},
        json_table::OnClause,
    },
    *,
};

/// Builder for JSON_QUERY function
#[derive(Debug, Clone)]
pub struct Builder {
    pub(crate) context_item: Expr,
    pub(crate) path_expression: Cow<'static, str>,
    pub(crate) passing: Vec<(Value, Cow<'static, str>)>,
    pub(crate) returning: Option<crate::TypeRef>,
    pub(crate) wrapper: Option<WrapperClause>,
    pub(crate) quotes: Option<QuotesClause>,
    pub(crate) on_empty: Option<OnClause>,
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

    /// Set WRAPPER clause
    pub fn wrapper<T>(mut self, wrapper: T) -> Self
    where
        T: Into<WrapperClause>,
    {
        self.wrapper = Some(wrapper.into());
        self
    }

    /// Set QUOTES clause
    pub fn quotes<T>(mut self, quotes: T) -> Self
    where
        T: Into<QuotesClause>,
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

    pub fn build(self) -> FunctionCall {
        self.build_internal().unwrap()
    }

    fn build_internal(self) -> Result<FunctionCall, core::fmt::Error> {
        let mut buf = String::with_capacity(50);

        PostgresQueryBuilder.prepare_simple_expr(&self.context_item, &mut buf);
        buf.write_str(" ")?;
        write_json_path_expr(&mut buf, &self.path_expression)?;

        write_passing(&mut buf, self.passing)?;

        // RETURNING clause
        if let Some(returning) = self.returning {
            buf.write_str(" RETURNING ")?;
            PostgresQueryBuilder.prepare_type_ref(&returning, &mut buf);
        }

        if let Some(wrapper) = self.wrapper {
            wrapper.write_to(&mut buf)?;
        }

        if let Some(quotes) = self.quotes {
            quotes.write_to(&mut buf)?;
        }

        if let Some(on_empty) = self.on_empty {
            buf.write_str(" ")?;
            on_empty.write_to(&mut buf)?;
            buf.write_str(" ON EMPTY")?;
        }

        if let Some(on_error) = self.on_error {
            buf.write_str(" ")?;
            on_error.write_to(&mut buf)?;
            buf.write_str(" ON ERROR")?;
        }

        Ok(FunctionCall::new(Func::Custom("JSON_QUERY".into())).arg(Expr::Custom(buf)))
    }
}

impl PgFunc {
    /// Create a `JSON_QUERY` function builder. Postgres only.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use sea_query::extension::postgres::*;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(
    ///         PgFunc::json_query(Expr::cust(r#"jsonb '[1,[2,3],null]'"#), "lax $[*][$off]")
    ///             .passing(1, "off")
    ///             .wrapper(json_fn::WrapperKind::WithConditional),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_QUERY(jsonb '[1,[2,3],null]' 'lax $[*][$off]' PASSING 1 AS off WITH CONDITIONAL WRAPPER)"#
    /// );
    /// ```
    ///
    /// With OMIT QUOTES:
    /// ```
    /// use sea_query::extension::postgres::*;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(
    ///         PgFunc::json_query(Expr::cust(r#"jsonb '{"a": "[1, 2]"}'"#), "lax $.a")
    ///             .quotes(json_fn::QuotesKind::Omit),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_QUERY(jsonb '{"a": "[1, 2]"}' 'lax $.a' OMIT QUOTES)"#
    /// );
    /// ```
    pub fn json_query<T, P>(context_item: T, path_expression: P) -> Builder
    where
        T: Into<Expr>,
        P: Into<Cow<'static, str>>,
    {
        Builder {
            context_item: context_item.into(),
            path_expression: path_expression.into(),
            passing: Vec::new(),
            returning: None,
            wrapper: None,
            quotes: None,
            on_empty: None,
            on_error: None,
        }
    }
}
