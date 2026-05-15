use std::borrow::Cow;

use crate::{
    ConditionHolder, ConditionalStatement, Expr, IntoColumnRef, IntoCondition, IntoTableRef,
    LogicalChainOper, NullOrdering, Order, OrderExpr, OrderedStatement, QueryBuilder,
    QueryStatement, QueryStatementBuilder, QueryStatementWriter, ReturningClause, SqlWriter,
    SubQueryStatement, TableRef, Value, Values, WithClause, WithQuery,
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
///     .cond_where(any![
///         Expr::col(Glyph::Id).lt(1),
///         Expr::col(Glyph::Id).gt(10),
///     ])
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"DELETE FROM `glyph` WHERE `id` < 1 OR `id` > 10"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder),
///     r#"DELETE FROM "glyph" WHERE "id" < 1 OR "id" > 10"#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"DELETE FROM "glyph" WHERE "id" < 1 OR "id" > 10"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteStatement {
    pub(crate) table: Option<Box<TableRef>>,
    pub(crate) r#where: ConditionHolder,
    pub(crate) orders: Vec<OrderExpr>,
    pub(crate) limit: Option<Value>,
    pub(crate) returning: Option<ReturningClause>,
    pub(crate) with: Option<WithClause>,
}

impl DeleteStatement {
    /// Construct a new [`DeleteStatement`]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn take(&mut self) -> Self {
        Self {
            table: self.table.take(),
            r#where: std::mem::take(&mut self.r#where),
            orders: std::mem::take(&mut self.orders),
            limit: self.limit.take(),
            returning: self.returning.take(),
            with: self.with.take(),
        }
    }

    /// Specify which table to delete from.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{audit::*, tests_cfg::*, *};
    ///
    /// let query = Query::delete()
    ///     .from_table(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Id).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1"#
    /// );
    /// assert_eq!(
    ///     query.audit_unwrap().deleted_tables(),
    ///     [Glyph::Table.into_iden()]
    /// );
    /// assert_eq!(query.audit_unwrap().selected_tables(), []);
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn from_table<T>(&mut self, tbl_ref: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.table = Some(Box::new(tbl_ref.into_table_ref()));
        self
    }

    /// Limit number of updated rows.
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit.into());
        self
    }

    /// RETURNING expressions.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{audit::*, tests_cfg::*, *};
    ///
    /// let query = Query::delete()
    ///     .from_table(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Id).eq(1))
    ///     .returning(Query::returning().columns([Glyph::Id]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.audit_unwrap().deleted_tables(),
    ///     [Glyph::Table.into_iden()]
    /// );
    /// assert_eq!(
    ///     query.audit_unwrap().selected_tables(),
    ///     [Glyph::Table.into_iden()]
    /// );
    /// ```
    pub fn returning(&mut self, returning_cols: ReturningClause) -> &mut Self {
        self.returning = Some(returning_cols);
        self
    }

    /// RETURNING expressions for a column.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::delete()
    ///     .from_table(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Id).eq(1))
    ///     .returning_col(Glyph::Id)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id""#
    /// );
    /// ```
    pub fn returning_col<C>(&mut self, col: C) -> &mut Self
    where
        C: IntoColumnRef,
    {
        self.returning(ReturningClause::Columns(vec![col.into_column_ref()]))
    }

    /// RETURNING expressions all columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::delete()
    ///     .from_table(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Id).eq(1))
    ///     .returning_all()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING *"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING *"#
    /// );
    /// ```
    pub fn returning_all(&mut self) -> &mut Self {
        self.returning(ReturningClause::All)
    }

    /// Create a [WithQuery] by specifying a [WithClause] to execute this query with.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{IntoCondition, IntoIden, audit::*, tests_cfg::*, *};
    ///
    /// let select = SelectStatement::new()
    ///         .columns([Glyph::Id])
    ///         .from(Glyph::Table)
    ///         .and_where(Expr::col(Glyph::Image).like("0%"))
    ///         .to_owned();
    ///     let cte = CommonTableExpression::new()
    ///         .query(select)
    ///         .column(Glyph::Id)
    ///         .table_name("cte")
    ///         .to_owned();
    ///     let with_clause = WithClause::new().cte(cte).to_owned();
    ///     let update = DeleteStatement::new()
    ///         .from_table(Glyph::Table)
    ///         .and_where(Expr::col(Glyph::Id).in_subquery(SelectStatement::new().column(Glyph::Id).from("cte").to_owned()))
    ///         .to_owned();
    ///     let query = update.with(with_clause);
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"WITH `cte` (`id`) AS (SELECT `id` FROM `glyph` WHERE `image` LIKE '0%') DELETE FROM `glyph` WHERE `id` IN (SELECT `id` FROM `cte`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"WITH "cte" ("id") AS (SELECT "id" FROM "glyph" WHERE "image" LIKE '0%') DELETE FROM "glyph" WHERE "id" IN (SELECT "id" FROM "cte")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"WITH "cte" ("id") AS (SELECT "id" FROM "glyph" WHERE "image" LIKE '0%') DELETE FROM "glyph" WHERE "id" IN (SELECT "id" FROM "cte")"#
    /// );
    /// assert_eq!(
    ///     query.audit_unwrap().deleted_tables(),
    ///     [Glyph::Table.into_iden()]
    /// );
    /// assert_eq!(
    ///     query.audit_unwrap().selected_tables(),
    ///     [Glyph::Table.into_iden()]
    /// );
    /// ```
    pub fn with(self, clause: WithClause) -> WithQuery {
        clause.query(self)
    }

    /// Create a Common Table Expression by specifying a [CommonTableExpression][crate::CommonTableExpression]
    /// or [WithClause] to execute this query with.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{IntoCondition, IntoIden, audit::*, tests_cfg::*, *};
    ///
    /// let select = SelectStatement::new()
    ///         .columns([Glyph::Id])
    ///         .from(Glyph::Table)
    ///         .and_where(Expr::col(Glyph::Image).like("0%"))
    ///         .to_owned();
    ///     let cte = CommonTableExpression::new()
    ///         .query(select)
    ///         .column(Glyph::Id)
    ///         .table_name("cte")
    ///         .to_owned();
    ///     let with_clause = WithClause::new().cte(cte).to_owned();
    ///     let query = DeleteStatement::new()
    ///         .with_cte(with_clause)
    ///         .from_table(Glyph::Table)
    ///         .and_where(Expr::col(Glyph::Id).in_subquery(SelectStatement::new().column(Glyph::Id).from("cte").to_owned()))
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"WITH `cte` (`id`) AS (SELECT `id` FROM `glyph` WHERE `image` LIKE '0%') DELETE FROM `glyph` WHERE `id` IN (SELECT `id` FROM `cte`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"WITH "cte" ("id") AS (SELECT "id" FROM "glyph" WHERE "image" LIKE '0%') DELETE FROM "glyph" WHERE "id" IN (SELECT "id" FROM "cte")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"WITH "cte" ("id") AS (SELECT "id" FROM "glyph" WHERE "image" LIKE '0%') DELETE FROM "glyph" WHERE "id" IN (SELECT "id" FROM "cte")"#
    /// );
    /// assert_eq!(
    ///     query.audit_unwrap().deleted_tables(),
    ///     [Glyph::Table.into_iden()]
    /// );
    /// assert_eq!(
    ///     query.audit_unwrap().selected_tables(),
    ///     [Glyph::Table.into_iden()]
    /// );
    /// ```
    pub fn with_cte<C: Into<WithClause>>(&mut self, clause: C) -> &mut Self {
        self.with = Some(clause.into());
        self
    }
}

impl QueryStatementBuilder for DeleteStatement {
    fn build_collect_any_into(&self, query_builder: &impl QueryBuilder, sql: &mut impl SqlWriter) {
        query_builder.prepare_delete_statement(self, sql);
    }
}

impl DeleteStatement {
    pub fn build_any(&self, query_builder: &impl QueryBuilder) -> (String, Values) {
        <Self as QueryStatementBuilder>::build_any(self, query_builder)
    }

    pub fn build_collect_any(
        &self,
        query_builder: &impl QueryBuilder,
        sql: &mut impl SqlWriter,
    ) -> String {
        <Self as QueryStatementBuilder>::build_collect_any(self, query_builder, sql)
    }

    pub fn build_collect_any_into(
        &self,
        query_builder: &impl QueryBuilder,
        sql: &mut impl SqlWriter,
    ) {
        <Self as QueryStatementBuilder>::build_collect_any_into(self, query_builder, sql)
    }
}

impl From<DeleteStatement> for QueryStatement {
    fn from(s: DeleteStatement) -> Self {
        Self::Delete(s)
    }
}

impl From<DeleteStatement> for SubQueryStatement {
    fn from(s: DeleteStatement) -> Self {
        Self::DeleteStatement(s)
    }
}

impl QueryStatementWriter for DeleteStatement {
    fn build_collect_into<T: QueryBuilder>(&self, query_builder: T, sql: &mut impl SqlWriter) {
        query_builder.prepare_delete_statement(self, sql);
    }
}

impl DeleteStatement {
    pub fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String {
        <Self as QueryStatementWriter>::to_string(self, query_builder)
    }

    pub fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Values) {
        <Self as QueryStatementWriter>::build(self, query_builder)
    }

    pub fn build_collect<T: QueryBuilder>(
        &self,
        query_builder: T,
        sql: &mut impl SqlWriter,
    ) -> String {
        <Self as QueryStatementWriter>::build_collect(self, query_builder, sql)
    }

    pub fn build_collect_into<T: QueryBuilder>(&self, query_builder: T, sql: &mut impl SqlWriter) {
        <Self as QueryStatementWriter>::build_collect_into(self, query_builder, sql);
    }
}

impl OrderedStatement for DeleteStatement {
    fn add_order_by(&mut self, order: OrderExpr) -> &mut Self {
        self.orders.push(order);
        self
    }

    fn clear_order_by(&mut self) -> &mut Self {
        self.orders = Vec::new();
        self
    }
}

impl DeleteStatement {
    pub fn add_order_by(&mut self, order: OrderExpr) -> &mut Self {
        <Self as OrderedStatement>::add_order_by(self, order)
    }

    pub fn clear_order_by(&mut self) -> &mut Self {
        <Self as OrderedStatement>::clear_order_by(self)
    }

    pub fn order_by<T: IntoColumnRef>(&mut self, col: T, order: Order) -> &mut Self {
        <Self as OrderedStatement>::order_by(self, col, order)
    }

    pub fn order_by_expr(&mut self, expr: Expr, order: Order) -> &mut Self {
        <Self as OrderedStatement>::order_by_expr(self, expr, order)
    }

    pub fn order_by_customs<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
        I: IntoIterator<Item = (T, Order)>,
    {
        <Self as OrderedStatement>::order_by_customs(self, cols)
    }

    pub fn order_by_columns<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = (T, Order)>,
    {
        <Self as OrderedStatement>::order_by_columns(self, cols)
    }

    pub fn order_by_with_nulls<T: IntoColumnRef>(
        &mut self,
        col: T,
        order: Order,
        nulls: NullOrdering,
    ) -> &mut Self {
        <Self as OrderedStatement>::order_by_with_nulls(self, col, order, nulls)
    }

    pub fn order_by_expr_with_nulls(
        &mut self,
        expr: Expr,
        order: Order,
        nulls: NullOrdering,
    ) -> &mut Self {
        <Self as OrderedStatement>::order_by_expr_with_nulls(self, expr, order, nulls)
    }

    pub fn order_by_customs_with_nulls<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
        I: IntoIterator<Item = (T, Order, NullOrdering)>,
    {
        <Self as OrderedStatement>::order_by_customs_with_nulls(self, cols)
    }

    pub fn order_by_columns_with_nulls<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = (T, Order, NullOrdering)>,
    {
        <Self as OrderedStatement>::order_by_columns_with_nulls(self, cols)
    }
}

impl ConditionalStatement for DeleteStatement {
    fn and_or_where(&mut self, condition: LogicalChainOper) -> &mut Self {
        self.r#where.add_and_or(condition);
        self
    }

    fn cond_where<C>(&mut self, condition: C) -> &mut Self
    where
        C: IntoCondition,
    {
        self.r#where.add_condition(condition.into_condition());
        self
    }
}

impl DeleteStatement {
    pub fn and_where(&mut self, other: Expr) -> &mut Self {
        <Self as ConditionalStatement>::and_where(self, other)
    }

    pub fn and_where_option(&mut self, other: Option<Expr>) -> &mut Self {
        <Self as ConditionalStatement>::and_where_option(self, other)
    }

    pub fn and_or_where(&mut self, condition: LogicalChainOper) -> &mut Self {
        <Self as ConditionalStatement>::and_or_where(self, condition)
    }

    pub fn cond_where<C: IntoCondition>(&mut self, condition: C) -> &mut Self {
        <Self as ConditionalStatement>::cond_where(self, condition)
    }
}
