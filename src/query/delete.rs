use crate::{backend::QueryBuilder, QueryStatementBuilder, query::OrderedStatement, types::*, expr::*, value::*, prepare::*};

/// Delete existing rows from the table
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let query = Query::delete()
///     .from_table(Glyph::Table)
///     .or_where(Expr::col(Glyph::Id).lt(1))
///     .or_where(Expr::col(Glyph::Id).gt(10))
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"DELETE FROM `glyph` WHERE (`id` < 1) OR (`id` > 10)"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder),
///     r#"DELETE FROM "glyph" WHERE ("id" < 1) OR ("id" > 10)"#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"DELETE FROM `glyph` WHERE (`id` < 1) OR (`id` > 10)"#
/// );
/// ```
#[derive(Debug, Clone)]
pub struct DeleteStatement {
    pub(crate) table: Option<Box<TableRef>>,
    pub(crate) wherei: Option<Box<SimpleExpr>>,
    pub(crate) orders: Vec<OrderExpr>,
    pub(crate) limit: Option<Value>,
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
            wherei: None,
            orders: Vec::new(),
            limit: None,
        }
    }

    /// Specify which table to delete from.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
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
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn from_table<T>(&mut self, tbl_ref: T) -> &mut Self
        where T: IntoTableRef {
        self.table = Some(Box::new(tbl_ref.into_table_ref()));
        self
    }

    /// And where condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::delete()
    ///     .from_table(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Id).gt(1))
    ///     .and_where(Expr::col(Glyph::Id).lt(10))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE (`id` > 1) AND (`id` < 10)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"DELETE FROM "glyph" WHERE ("id" > 1) AND ("id" < 10)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE (`id` > 1) AND (`id` < 10)"#
    /// );
    /// ```
    pub fn and_where(&mut self, other: SimpleExpr) -> &mut Self {
        self.and_or_where(BinOper::And, other)
    }

    /// And where condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::delete()
    ///     .from_table(Glyph::Table)
    ///     .or_where(Expr::col(Glyph::Id).lt(1))
    ///     .or_where(Expr::col(Glyph::Id).gt(10))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE (`id` < 1) OR (`id` > 10)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"DELETE FROM "glyph" WHERE ("id" < 1) OR ("id" > 10)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"DELETE FROM `glyph` WHERE (`id` < 1) OR (`id` > 10)"#
    /// );
    /// ```
    pub fn or_where(&mut self, other: SimpleExpr) -> &mut Self {
        self.and_or_where(BinOper::Or, other)
    }

    fn and_or_where(&mut self, bopr: BinOper, right: SimpleExpr) -> &mut Self {
        self.wherei = Self::merge_expr(
            self.wherei.take(),
            match bopr {
                BinOper::And => BinOper::And,
                BinOper::Or => BinOper::Or,
                _ => panic!("not allow"),
            },
            right
        );
        self
    }

    fn merge_expr(left: Option<Box<SimpleExpr>>, bopr: BinOper, right: SimpleExpr) -> Option<Box<SimpleExpr>> {
        Some(Box::new(match left {
            Some(left) => SimpleExpr::Binary(
                left,
                bopr,
                Box::new(right)
            ),
            None => right,
        }))
    }

    /// Limit number of updated rows.
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(Value::BigUnsigned(limit));
        self
    }

    pub fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String {
        <Self as QueryStatementBuilder>::to_string(self, query_builder)
    }

    pub fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Values) {
        <Self as QueryStatementBuilder>::build(self, query_builder)
    }

    pub fn build_any(&self, query_builder: &dyn QueryBuilder) -> (String, Values) {
        <Self as QueryStatementBuilder>::build_any(self, query_builder)
    }
}

impl QueryStatementBuilder for DeleteStatement {
    /// Build corresponding SQL statement for certain database backend and collect query parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
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
    /// assert_eq!(
    ///     params,
    ///     vec![
    ///         Value::Int(1),
    ///     ]
    /// );
    /// ```
    fn build_collect<T: QueryBuilder>(&self, query_builder: T, collector: &mut dyn FnMut(Value)) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_delete_statement(self, &mut sql, collector);
        sql.result()
    }

    fn build_collect_any(&self, query_builder: &dyn QueryBuilder, collector: &mut dyn FnMut(Value)) -> String {
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
