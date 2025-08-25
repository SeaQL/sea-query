use std::borrow::Cow;

use crate::{
    extension::postgres::json_fn::{QuotesClause, WrapperClause, WrapperKind},
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

#[derive(Debug, Clone)]
pub enum OnClause {
    Error,
    Null,
    EmptyArray,
    EmptyObject,
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
        buf.write_str(" '")?;
        buf.write_str(&self.path_expression)?;
        buf.write_str("'")?;

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

        // RETURNING clause
        if let Some(returning) = self.returning {
            buf.write_str(" RETURNING ")?;
            PostgresQueryBuilder.prepare_type_ref(&returning, &mut buf);
        }

        // WRAPPER clause
        if let Some(wrapper) = self.wrapper {
            let cond = match wrapper.kind {
                WrapperKind::Without => " WITHOUT",
                WrapperKind::WithConditional => " WITH CONDITIONAL",
                WrapperKind::WithUnconditional => " WITH UNCONDITIONAL",
            };
            buf.write_str(cond)?;

            if wrapper.array {
                buf.write_str(" ARRAY")?;
            }

            buf.write_str(" WRAPPER")?;
        }

        // QUOTES clause
        if let Some(quotes) = self.quotes {
            quotes.prepare(&mut buf)?;
        }

        // ON EMPTY clause
        if let Some(on_empty) = self.on_empty {
            match on_empty {
                OnClause::Error => buf.write_str(" ERROR")?,
                OnClause::Null => buf.write_str(" NULL")?,
                OnClause::EmptyArray => buf.write_str(" EMPTY ARRAY")?,
                OnClause::EmptyObject => buf.write_str(" EMPTY OBJECT")?,
                OnClause::Default(expr) => {
                    buf.write_str(" DEFAULT ")?;
                    PostgresQueryBuilder.prepare_simple_expr(&expr, &mut buf);
                }
            };

            buf.write_str(" ON EMPTY")?;
        }

        // ON ERROR clause
        if let Some(on_error) = self.on_error {
            match on_error {
                OnClause::Error => buf.write_str(" ERROR ON ERROR")?,
                OnClause::Null => buf.write_str(" NULL ON ERROR")?,
                OnClause::EmptyArray => buf.write_str(" EMPTY ARRAY ON ERROR")?,
                OnClause::EmptyObject => buf.write_str(" EMPTY OBJECT ON ERROR")?,
                OnClause::Default(expr) => {
                    buf.write_str(" DEFAULT ")?;
                    PostgresQueryBuilder.prepare_simple_expr(&expr, &mut buf);
                    buf.write_str(" ON ERROR")?;
                }
            }
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
