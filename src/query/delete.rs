use crate::{
    backend::QueryBuilder,
    prepare::*,
    query::{condition::*, OrderedStatement},
    types::*,
    Query, QueryStatementBuilder, Queryable, SelectExpr, SelectStatement,
};

/// Delete existing rows from the table
///
/// # Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let query = Query::delete()
///     .from_table(Glyph::Table)
///     .or_where(Expr::col(Glyph::Id).lt(&1))
///     .or_where(Expr::col(Glyph::Id).gt(&10))
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"DELETE FROM `glyph` WHERE `id` < 1 OR `id` > 10"#
/// );
/// assert_eq!(
///     query.to_string(),
///     r#"DELETE FROM "glyph" WHERE "id" < 1 OR "id" > 10"#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"DELETE FROM `glyph` WHERE `id` < 1 OR `id` > 10"#
/// );
/// ```
#[derive(Debug, Clone)]
pub struct DeleteStatement<'a, DB> {
    pub(crate) table: Option<Box<TableRef<'a, DB>>>,
    pub(crate) wherei: ConditionHolder<'a, DB>,
    pub(crate) orders: Vec<OrderExpr<'a, DB>>,
    pub(crate) limit: Option<u64>,
    pub(crate) returning: Vec<SelectExpr<'a, DB>>,
}

impl<'a, DB> Default for DeleteStatement<'a, DB>
where
    DB: QueryBuilder<DB>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, DB> DeleteStatement<'a, DB>
where
    DB: QueryBuilder<DB>,
{
    /// Construct a new [`DeleteStatement`]
    pub fn new() -> Self {
        Self {
            table: None,
            wherei: ConditionHolder::new(),
            orders: Vec::new(),
            limit: None,
            returning: Vec::new(),
        }
    }

    /// Specify which table to delete from.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::delete()
    ///     .from_table(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Id).eq(&1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn from_table<T>(&mut self, tbl_ref: T) -> &mut Self
    where
        T: IntoTableRef<'a, DB>,
    {
        self.table = Some(Box::new(tbl_ref.into_table_ref()));
        self
    }

    /// Limit number of updated rows.
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// RETURNING expressions. Postgres only.
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::delete()
    ///     .from_table(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Id).eq(&1))
    ///     .returning(Query::select().column(Glyph::Id).take())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    /// ```
    pub fn returning(&mut self, select: SelectStatement<'a, DB>) -> &mut Self {
        self.returning = select.selects;
        self
    }

    /// RETURNING a column after delete. Postgres only.
    /// Wrapper over [`DeleteStatement::returning()`].
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::delete()
    ///     .from_table(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Id).eq(&1))
    ///     .returning_col(Glyph::Id)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    /// ```
    pub fn returning_col<C>(&mut self, col: C) -> &mut Self
    where
        DB: Default,
        Query: Queryable<DB>,
        C: IntoIden,
    {
        self.returning(Query::select().column(col.into_iden()).take())
    }
}

impl<'a, DB> QueryStatementBuilder<'a, DB> for DeleteStatement<'a, DB>
where
    DB: QueryBuilder<DB> + Default + 'a,
{
    /// Build corresponding SQL statement for certain database backend and collect query parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::delete()
    ///     .from_table(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Id).eq(&1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    ///
    /// let mut params = Vec::new();
    /// let mut collector = |v| params.push(v);
    ///
    /// assert_eq!(
    ///     query.build_collect(MysqlQueryBuilder, &mut collector),
    ///     r#"DELETE FROM `glyph` WHERE `id` = ?"#
    /// );
    /// assert_eq!(params, vec![Value::Int(Some(1)),]);
    /// ```
    fn build_collect(&'a self, collector: &mut dyn FnMut(&'a dyn QueryValue<DB>)) -> String {
        let mut sql = SqlWriter::new();
        DB::default().prepare_delete_statement(self, &mut sql, collector);
        sql.result()
    }
}

impl<'a, DB> OrderedStatement<'a, DB> for DeleteStatement<'a, DB> {
    fn add_order_by(&mut self, order: OrderExpr<'a, DB>) -> &mut Self {
        self.orders.push(order);
        self
    }
}

impl<'a, DB> ConditionalStatement<'a, DB> for DeleteStatement<'a, DB> {
    fn and_or_where(&mut self, condition: LogicalChainOper<'a, DB>) -> &mut Self {
        self.wherei.add_and_or(condition);
        self
    }

    fn cond_where<C>(&mut self, condition: C) -> &mut Self
    where
        C: IntoCondition<'a, DB>,
    {
        self.wherei.add_condition(condition.into_condition());
        self
    }
}
