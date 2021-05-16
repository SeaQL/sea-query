use crate::{expr::SimpleExpr, types::LogicalChainOper};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionWhereType {
    Any,
    All,
}

/// Represents the value of an [`any()`] or [`all()`]: a set of disjunctive or conjunctive conditions.
#[derive(Debug, Clone)]
pub struct ConditionWhere {
    pub(crate) condition_type: ConditionWhereType,
    pub(crate) conditions: Vec<ConditionExpression>,
}

/// Represents anything that can be passed to an [`any()`] or [`all()`]'s [`ConditionWhere::add`] method.
///
/// The arguments are automatically converted to the right enum.
#[derive(Debug, Clone)]
pub enum ConditionExpression {
    ConditionWhere(ConditionWhere),
    SimpleExpr(SimpleExpr),
}

impl ConditionWhere {
    /// Add a condition to the set.
    ///
    /// If it's an [`any()`], it will be separated from the others by an `" OR "` in the query. If it's
    /// an [`all()`], it will be separated by an `" AND "`.
    pub fn add<C: Into<ConditionExpression>>(mut self, condition: C) -> Self {
        let expr = condition.into();
        // Don't add empty `any()` and `all()`.
        if let ConditionExpression::ConditionWhere(c) = &expr {
            if c.conditions.is_empty() {
                return self;
            }
        }
        self.conditions.push(expr);
        self
    }
}

impl std::convert::From<ConditionWhere> for ConditionExpression {
    fn from(condition: ConditionWhere) -> Self {
        ConditionExpression::ConditionWhere(condition)
    }
}

impl std::convert::From<SimpleExpr> for ConditionExpression {
    fn from(condition: SimpleExpr) -> Self {
        ConditionExpression::SimpleExpr(condition)
    }
}

/// Create a condition that is true if any of the conditions is true.
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let query = Query::select()
///     .column(Glyph::Image)
///     .from(Glyph::Table)
///     .cond_where(
///       any()
///       .add(Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![3, 4]))
///       .add(Expr::tbl(Glyph::Table, Glyph::Image).like("A%")))
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) OR `glyph`.`image` LIKE 'A%'"#
/// );
/// ```
pub fn any() -> ConditionWhere {
    ConditionWhere {
        condition_type: ConditionWhereType::Any,
        conditions: Vec::new(),
    }
}

/// Create a condition that is false if any of the conditions is false.
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let query = Query::select()
///     .column(Glyph::Image)
///     .from(Glyph::Table)
///     .cond_where(
///       all()
///       .add(Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![3, 4]))
///       .add(Expr::tbl(Glyph::Table, Glyph::Image).like("A%")))
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%'"#
/// );
/// ```
pub fn all() -> ConditionWhere {
    ConditionWhere {
        condition_type: ConditionWhereType::All,
        conditions: Vec::new(),
    }
}

/// Macro to easily create an [`any()`].
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let query = Query::select()
///     .column(Glyph::Image)
///     .from(Glyph::Table)
///     .cond_where(
///       any![
///         Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![3, 4]),
///         Expr::tbl(Glyph::Table, Glyph::Image).like("A%")])
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) OR `glyph`.`image` LIKE 'A%'"#
/// );
/// ```
#[macro_export]
macro_rules! any {
    ( $( $x:expr ),* ) => {
        {
            let mut tmp = any();
            $(
                tmp = tmp.add($x);
            )*
            tmp
        }
    };
}

/// Macro to easily create an [`all()`].
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let query = Query::select()
///     .column(Glyph::Image)
///     .from(Glyph::Table)
///     .cond_where(
///       all![
///         Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![3, 4]),
///         Expr::tbl(Glyph::Table, Glyph::Image).like("A%")])
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%'"#
/// );
#[macro_export]
macro_rules! all {
    ( $( $x:expr ),* ) => {
        {
            let mut tmp = all();
            $(
                tmp = tmp.add($x);
            )*
            tmp
        }
    };
}

pub trait ConditionalStatement {
    /// And where condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .and_where(Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![3, 4]))
    ///     .and_where(Expr::tbl(Glyph::Table, Glyph::Image).like("A%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%'"#
    /// );
    /// ```
    fn and_where(&mut self, other: SimpleExpr) -> &mut Self {
        self.and_or_where(LogicalChainOper::And(other))
    }

    /// And where condition, short hand for `if c.is_some() q.and_where(c)`.
    fn and_where_option(&mut self, other: Option<SimpleExpr>) -> &mut Self {
        if let Some(other) = other {
            self.and_or_where(LogicalChainOper::And(other));
        }
        self
    }

    #[deprecated(
        since = "0.10.7",
        note = "Please use [`ConditionalStatement::cond_where`] or only [`ConditionalStatement::and_where`]. The evaluation of mixed `and_where` and `or_where` can be surprising."
    )]
    fn or_where(&mut self, other: SimpleExpr) -> &mut Self {
        self.and_or_where(LogicalChainOper::Or(other))
    }

    #[doc(hidden)]
    // Trait implementation.
    fn and_or_where(&mut self, condition: LogicalChainOper) -> &mut Self;

    /// Where condition, expressed with [`any!`] and [`all!`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .cond_where(
    ///       all![
    ///         Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![3, 4]),
    ///         any![
    ///           Expr::tbl(Glyph::Table, Glyph::Image).like("A%"),
    ///           Expr::tbl(Glyph::Table, Glyph::Image).like("B%")]])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) AND (`glyph`.`image` LIKE 'A%' OR `glyph`.`image` LIKE 'B%')"#
    /// );
    /// ```
    fn cond_where(&mut self, condition: ConditionWhere) -> &mut Self;
}

#[derive(Debug, Clone)]
pub(crate) enum ConditionHolderContents {
    Empty,
    And(Vec<LogicalChainOper>),
    Where(ConditionWhere),
}

#[derive(Debug, Clone)]
pub(crate) struct ConditionHolder {
    pub contents: ConditionHolderContents,
}

impl ConditionHolder {

    pub fn new() -> Self {
        Self { contents: ConditionHolderContents::Empty }
    }

    pub fn is_empty(&self) -> bool {
        match &self.contents {
            ConditionHolderContents::Empty => true,
            ConditionHolderContents::And(c) => c.is_empty(),
            ConditionHolderContents::Where(c) => c.conditions.is_empty(),
        }
    }

    pub fn add_and_or(&mut self, condition: LogicalChainOper) {
        match &mut self.contents {
            ConditionHolderContents::Empty => self.contents = ConditionHolderContents::And(vec![condition]),
            ConditionHolderContents::And(c) => c.push(condition),
            ConditionHolderContents::Where(_) => panic!("Cannot mix `and_where`/`or_where` and `cond_where` in statements")
        }
    }

    pub fn set_where(&mut self, condition: ConditionWhere) {
        match &mut self.contents {
            ConditionHolderContents::Empty => self.contents = ConditionHolderContents::Where(condition),
            ConditionHolderContents::Where(_) => panic!("Multiple `cond_where` are not supported"),
            ConditionHolderContents::And(_) => panic!("Cannot mix `and_where`/`or_where` and `cond_where` in statements")
        }
    }
}
