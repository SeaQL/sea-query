use std::borrow::Cow;

use crate::*;

#[derive(Debug, Clone)]
pub struct Builder {
    pub(crate) context_item: Expr,
    pub(crate) path_expression: Cow<'static, str>,
    pub(crate) passing: Vec<(Value, Cow<'static, str>)>,
    pub(crate) returning: Option<crate::TypeRef>,
    pub(crate) on_empty: Option<OnClause>,
    pub(crate) on_error: Option<OnClause>,
}

#[derive(Debug, Clone)]
pub enum OnClause {
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

    /// Replace current PASSING parameters
    pub fn set_passing<V, A, I>(mut self, passing: I) -> Self
    where
        V: Into<Value>,
        A: Into<Cow<'static, str>>,
        I: IntoIterator<Item = (V, A)>,
    {
        self.passing = passing
            .into_iter()
            .map(|(a, b)| (a.into(), b.into()))
            .collect();
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

    /// Set ON EMPTY clause
    pub fn on_empty(mut self, on_empty: OnClause) -> Self {
        self.on_empty = Some(on_empty);
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

    /// Set ON ERROR clause
    pub fn on_error(mut self, on_error: OnClause) -> Self {
        self.on_error = Some(on_error);
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

        // ON EMPTY clause
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

        // ON ERROR clause
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
    /// Returns the result of applying the SQL/JSON path_expression to the context_item.
    /// Only use JSON_VALUE() if the extracted value is expected to be a single SQL/JSON scalar item.
    /// Supports RETURNING, ON EMPTY, and ON ERROR clauses.
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
    ///         PgFunc::json_value(Expr::val(r#""123.45""#), "$")
    ///             .returning("float".into())
    ///             .build(),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_VALUE(E'\"123.45\"' '$' RETURNING float)"#
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
    ///         PgFunc::json_value(Expr::val(r#"[1,2]"#), "strict $[$off]")
    ///             .passing(1, "off")
    ///             .build(),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT JSON_VALUE('[1,2]' 'strict $[$off]' PASSING 1 AS off)"#
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
    ///         PgFunc::json_value(Expr::val(r#"[1,2]"#), "strict $[*]")
    ///             .on_error(json_value::OnError::Default(Expr::val(9))),
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
