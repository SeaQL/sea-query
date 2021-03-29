use crate::{backend::QueryBuilder, types::*, expr::*, value::*, prepare::*};

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

    /// Order by column.
    pub fn order_by<T>(&mut self, col: T, order: Order) -> &mut Self 
        where T: IntoIden {
        self.orders.push(OrderExpr {
            expr: SimpleExpr::Column(col.into_iden()),
            order,
        });
        self
    }

    /// Order by column with table name prefix.
    pub fn order_by_tbl<T, C>
        (&mut self, table: T, col: C, order: Order) -> &mut Self 
        where T: IntoIden, C: IntoIden {
        self.orders.push(OrderExpr {
            expr: SimpleExpr::TableColumn(table.into_iden(), col.into_iden()),
            order,
        });
        self
    }

    /// Order by [`SimpleExpr`].
    pub fn order_by_expr(&mut self, expr: SimpleExpr, order: Order) -> &mut Self {
        self.orders.push(OrderExpr {
            expr,
            order,
        });
        self
    }

    /// Order by custom string.
    pub fn order_by_customs<T>(&mut self, cols: Vec<(T, Order)>) -> &mut Self 
        where T: ToString {
        let mut orders = cols.into_iter().map(
            |(c, order)| OrderExpr {
                expr: SimpleExpr::Custom(c.to_string()),
                order,
            }).collect();
        self.orders.append(&mut orders);
        self
    }

    /// Order by columns.
    pub fn order_by_columns<T>(&mut self, cols: Vec<(T, Order)>) -> &mut Self 
        where T: IntoIden {
        let mut orders = cols.into_iter().map(
            |(c, order)| OrderExpr {
                expr: SimpleExpr::Column(c.into_iden()),
                order,
            }).collect();
        self.orders.append(&mut orders);
        self
    }

    /// Order by columns with table prefix.
    pub fn order_by_table_columns<T, C>
        (&mut self, cols: Vec<(T, C, Order)>) -> &mut Self 
        where T: IntoIden, C: IntoIden {
        let mut orders = cols.into_iter().map(
            |(t, c, order)| OrderExpr {
                expr: SimpleExpr::TableColumn(t.into_iden(), c.into_iden()),
                order,
            }).collect();
        self.orders.append(&mut orders);
        self
    }

    /// Limit number of updated rows.
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(Value::BigUnsigned(limit));
        self
    }

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
    pub fn build_collect<T: QueryBuilder>(&self, query_builder: T, collector: &mut dyn FnMut(Value)) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_delete_statement(self, &mut sql, collector);
        sql.result()
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters    
    pub fn build_collect_any(&self, query_builder: &dyn QueryBuilder, collector: &mut dyn FnMut(Value)) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_delete_statement(self, &mut sql, collector);
        sql.result()
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters into a vector
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let (query, params) = Query::delete()
    ///     .from_table(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Id).eq(1))
    ///     .build(MysqlQueryBuilder);
    /// 
    /// assert_eq!(
    ///     query,
    ///     r#"DELETE FROM `glyph` WHERE `id` = ?"#
    /// );
    /// assert_eq!(
    ///     params,
    ///     Values(vec![
    ///         Value::Int(1),
    ///     ])
    /// );
    /// ```
    pub fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Values) {
        let mut values = Vec::new();
        let mut collector = |v| values.push(v);
        let sql = self.build_collect(query_builder, &mut collector);
        (sql, Values(values))
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters into a vector
    pub fn build_any(&self, query_builder: &dyn QueryBuilder) -> (String, Values) {
        let mut values = Vec::new();
        let mut collector = |v| values.push(v);
        let sql = self.build_collect_any(query_builder, &mut collector);
        (sql, Values(values))
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::delete()
    ///     .from_table(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Id).eq(1))
    ///     .to_string(MysqlQueryBuilder);
    /// 
    /// assert_eq!(
    ///     query,
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    /// ```
    pub fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String {
        let (sql, values) = self.build_any(&query_builder);
        inject_parameters(&sql, values.0, &query_builder)
    }
}