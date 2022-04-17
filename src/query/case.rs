use crate::{Condition, Expr, SimpleExpr};

#[derive(Debug, Clone)]
pub(crate) struct CaseStatementCondition {
    pub(crate) condition: Option<Condition>,
    pub(crate) result: Expr,
}

#[derive(Debug, Clone)]
pub struct CaseStatement {
    pub(crate) conditions: Vec<CaseStatementCondition>,
}

impl CaseStatement {
    /// Creates a new case statement expression
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///         CaseStatement::new()
    ///             .case(
    ///                 Cond::any()
    ///                     .add(Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![2, 4])),
    ///                 Expr::val(true)
    ///              )
    ///             .finally(Expr::val(false)),
    ///          Alias::new("is_even")
    ///     )
    ///     .from(Glyph::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT (CASE WHEN ("glyph"."aspect" IN (2, 4)) THEN TRUE ELSE FALSE END) AS "is_even" FROM "glyph""#
    /// );    
    /// ```
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    /// Adds new `CASE WHEN` to existing case statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///         CaseStatement::new()
    ///             .case(
    ///                 Cond::any()
    ///                     .add(Expr::tbl(Glyph::Table, Glyph::Aspect).gt(0)),
    ///                 Expr::val("positive")
    ///              )
    ///             .case(
    ///                 Cond::any()
    ///                     .add(Expr::tbl(Glyph::Table, Glyph::Aspect).lt(0)),
    ///                 Expr::val("negative")
    ///              )    
    ///             .finally(Expr::val("zero")),
    ///          Alias::new("polarity")
    ///     )
    ///     .from(Glyph::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT (CASE WHEN ("glyph"."aspect" > 0) THEN 'positive' WHEN ("glyph"."aspect" < 0) THEN 'negative' ELSE 'zero' END) AS "polarity" FROM "glyph""#
    /// );    
    /// ```

    pub fn case<C, E>(mut self, when: C, then: E) -> Self
    where
        C: Into<Condition>,
        E: Into<Expr>,
    {
        self.conditions.push(CaseStatementCondition {
            condition: Some(when.into()),
            result: then.into(),
        });
        self
    }

    /// Ends the case statement with the final `ELSE` result.
    pub fn finally<E>(mut self, r#else: E) -> Self
    where
        E: Into<Expr>,
    {
        self.conditions.push(CaseStatementCondition {
            condition: None,
            result: r#else.into(),
        });
        self
    }
}

impl Into<SimpleExpr> for CaseStatement {
    fn into(self) -> SimpleExpr {
        SimpleExpr::Case(Box::new(self))
    }
}
