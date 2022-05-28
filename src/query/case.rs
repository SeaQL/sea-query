use crate::{Condition, Expr, IntoCondition, SimpleExpr};

#[derive(Debug, Clone)]
pub(crate) struct CaseStatementCondition {
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
    ///             .case(Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![2, 4]), Expr::val(true))
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
    ///             Expr::case(
    ///                 Expr::tbl(Glyph::Table, Glyph::Aspect).gt(0),
    ///                 Expr::val("positive")
    ///              )
    ///             .case(
    ///                 Expr::tbl(Glyph::Table, Glyph::Aspect).lt(0),
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
    pub fn case<C, T>(mut self, cond: C, then: T) -> Self
    where
        C: IntoCondition,
        T: Into<Expr>,
    {
        self.when.push(CaseStatementCondition {
            condition: cond.into_condition(),
            result: then.into(),
        });
        self
    }

    /// Ends the case statement with the final `ELSE` result.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///         Expr::case(
    ///             Cond::any()
    ///                 .add(Expr::tbl(Character::Table, Character::FontSize).gt(48))
    ///                 .add(Expr::tbl(Character::Table, Character::SizeW).gt(500)),
    ///             Expr::val("large")
    ///         )
    ///         .case(
    ///             Cond::any()
    ///                 .add(Expr::tbl(Character::Table, Character::FontSize).between(24,48).into_condition())
    ///                 .add(Expr::tbl(Character::Table, Character::SizeW).between(300,500).into_condition()),
    ///             Expr::val("medium")
    ///         )
    ///         .finally(Expr::val("small")),
    ///         Alias::new("char_size"))
    ///     .from(Character::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"SELECT"#,
    ///         r#"(CASE WHEN ("character"."font_size" > 48 OR "character"."size_w" > 500) THEN 'large'"#,
    ///         r#"WHEN (("character"."font_size" BETWEEN 24 AND 48) OR ("character"."size_w" BETWEEN 300 AND 500)) THEN 'medium'"#,
    ///         r#"ELSE 'small' END) AS "char_size""#,
    ///         r#"FROM "character""#
    ///     ]
    ///     .join(" ")
    /// );    
    /// ```
    pub fn finally<E>(mut self, r#else: E) -> Self
    where
        E: Into<Expr>,
    {
        self.r#else = Some(r#else.into());
        self
    }
}

#[allow(clippy::from_over_into)]
impl Into<SimpleExpr> for CaseStatement {
    fn into(self) -> SimpleExpr {
        SimpleExpr::Case(Box::new(self))
    }
}
