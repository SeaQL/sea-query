use crate::{Condition, Expr, IntoCondition, SimpleExpr};

#[derive(Debug, Clone)]
pub struct CaseStatementCondition {
    pub(crate) condition: Condition,
    pub(crate) result: Expr,
}

#[derive(Debug, Clone, Default)]
pub struct CaseStatement {
    pub(crate) when: Vec<CaseStatementCondition>,
    pub(crate) r#else: Option<Expr>,
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
    ///             .case((Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![2, 4]), Expr::val(true)))
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
        Self::default()
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
    ///             Expr::case((
    ///                 Expr::tbl(Glyph::Table, Glyph::Aspect).gt(0),
    ///                 Expr::val("positive")
    ///              ))
    ///             .case((
    ///                 Expr::tbl(Glyph::Table, Glyph::Aspect).lt(0),
    ///                 Expr::val("negative")
    ///              ))
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
    pub fn case<C>(mut self, case: C) -> Self
    where
        C: IntoCaseStatement,
    {
        self.when.push(case.into_case_statement());
        self
    }

    /// Ends the case statement with the final `ELSE` result.
    pub fn finally<E>(mut self, r#else: E) -> Self
    where
        E: Into<Expr>,
    {
        self.r#else = Some(r#else.into());
        self
    }
}

impl<C> From<C> for CaseStatement
where
    C: IntoCaseStatement,
{
    fn from(c: C) -> CaseStatement {
        CaseStatement::new().case(c.into_case_statement())
    }
}

impl Into<SimpleExpr> for CaseStatement {
    fn into(self) -> SimpleExpr {
        SimpleExpr::Case(Box::new(self))
    }
}

pub trait IntoCaseStatement {
    fn into_case_statement(self) -> CaseStatementCondition;
}

impl IntoCaseStatement for CaseStatementCondition {
    fn into_case_statement(self) -> CaseStatementCondition {
        self
    }
}

impl<C: 'static, T: 'static> IntoCaseStatement for (C, T)
where
    C: IntoCondition,
    T: Into<Expr>,
{
    fn into_case_statement(self) -> CaseStatementCondition {
        let (c, t) = self;
        CaseStatementCondition {
            condition: c.into_condition(),
            result: t.into(),
        }
    }
}
