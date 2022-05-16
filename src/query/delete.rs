use crate::{
    backend::QueryBuilder,
    prepare::*,
    query::{condition::*, OrderedStatement},
    types::*,
    value::*,
    QueryStatementBuilder, QueryStatementWriter, ReturningClause, SubQueryStatement, WithClause,
    WithQuery,
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
///     .or_where(Expr::col(Glyph::Id).lt(1))
///     .or_where(Expr::col(Glyph::Id).gt(10))
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
#[derive(Debug, Clone)]
pub struct DeleteStatement {
    pub(crate) table: Option<Box<TableRef>>,
    pub(crate) wherei: ConditionHolder,
    pub(crate) orders: Vec<OrderExpr>,
    pub(crate) limit: Option<Value>,
    pub(crate) returning: Option<ReturningClause>,
}

impl Default for DeleteStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl DeleteStatement {
    /// Construct a new [`DeleteStatement`]
    pub fn new() -> Self {
        Self {
            table: None,
            wherei: ConditionHolder::new(),
            orders: Vec::new(),
            limit: None,
            returning: None,
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
        self.limit = Some(Value::BigUnsigned(Some(limit)));
        self
    }

    /// RETURNING expressions.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
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
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Image])
    ///     .values_panic(vec!["12A".into()])
    ///     .returning_all()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     "INSERT INTO `glyph` (`image`) VALUES ('12A')"
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING *"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING *"#
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
    /// use sea_query::{*, IntoCondition, IntoIden, tests_cfg::*};
    ///
    /// let select = SelectStatement::new()
    ///         .columns([Glyph::Id])
    ///         .from(Glyph::Table)
    ///         .and_where(Expr::col(Glyph::Image).like("0%"))
    ///         .to_owned();
    ///     let cte = CommonTableExpression::new()
    ///         .query(select)
    ///         .column(Glyph::Id)
    ///         .table_name(Alias::new("cte"))
    ///         .to_owned();
    ///     let with_clause = WithClause::new().cte(cte).to_owned();
    ///     let update = DeleteStatement::new()
    ///         .from_table(Glyph::Table)
    ///         .and_where(Expr::col(Glyph::Id).in_subquery(SelectStatement::new().column(Glyph::Id).from(Alias::new("cte")).to_owned()))
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
    /// ```
    pub fn with(self, clause: WithClause) -> WithQuery {
        clause.query(self)
    }
}

impl QueryStatementBuilder for DeleteStatement {
    fn build_collect_any_into(
        &self,
        query_builder: &dyn QueryBuilder,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        query_builder.prepare_delete_statement(self, sql, collector);
    }

    fn into_sub_query_statement(self) -> SubQueryStatement {
        SubQueryStatement::DeleteStatement(self)
    }
}

impl QueryStatementWriter for DeleteStatement {
    /// Build corresponding SQL statement for certain database backend and collect query parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
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
    fn build_collect<T: QueryBuilder>(
        &self,
        query_builder: T,
        collector: &mut dyn FnMut(Value),
    ) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_delete_statement(self, &mut sql, collector);
        sql.result()
    }
}

impl OrderedStatement for DeleteStatement {
    fn add_order_by(&mut self, order: OrderExpr) -> &mut Self {
        self.orders.push(order);
        self
    }
}

impl ConditionalStatement for DeleteStatement {
    fn and_or_where(&mut self, condition: LogicalChainOper) -> &mut Self {
        self.wherei.add_and_or(condition);
        self
    }

    fn cond_where<C>(&mut self, condition: C) -> &mut Self
    where
        C: IntoCondition,
    {
        self.wherei.add_condition(condition.into_condition());
        self
    }
}
