use std::rc::Rc;
use crate::{backend::QueryBuilder, types::*, expr::*, value::*};

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
#[derive(Clone)]
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
    pub fn from_table<T: 'static>(&mut self, table: T) -> &mut Self
        where T: Iden {
        self.from_table_dyn(Rc::new(table))
    }

    /// Specify which table to delete from, variation of [`DeleteStatement::from_table`].
    #[allow(clippy::wrong_self_convention)]
    pub fn from_table_dyn(&mut self, table: Rc<dyn Iden>) -> &mut Self {
        self.table = Some(Box::new(TableRef::Table(table)));
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
    pub fn order_by<T: 'static>(&mut self, col: T, order: Order) -> &mut Self 
        where T: Iden {
        self.order_by_dyn(Rc::new(col), order)
    }

    /// Order by column, variation of [`DeleteStatement::order_by`].
    pub fn order_by_dyn(&mut self, col: Rc<dyn Iden>, order: Order) -> &mut Self {
        self.orders.push(OrderExpr {
            expr: SimpleExpr::Column(col),
            order,
        });
        self
    }

    /// Order by column with table name prefix.
    pub fn order_by_tbl<T: 'static, C: 'static>
        (&mut self, table: T, col: C, order: Order) -> &mut Self 
        where T: Iden, C: Iden {
        self.order_by_tbl_dyn(Rc::new(table), Rc::new(col), order)
    }

    /// Order by column with table name prefix, variation of [`DeleteStatement::order_by_tbl`].
    pub fn order_by_tbl_dyn(&mut self, table: Rc<dyn Iden>, col: Rc<dyn Iden>, order: Order) -> &mut Self {
        self.orders.push(OrderExpr {
            expr: SimpleExpr::TableColumn(table, col),
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
    pub fn order_by_customs<T: 'static>(&mut self, cols: Vec<(T, Order)>) -> &mut Self 
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
    pub fn order_by_columns<T: 'static>(&mut self, cols: Vec<(T, Order)>) -> &mut Self 
        where T: Iden {
        self.order_by_columns_dyn(cols.into_iter().map(
            |(c, order)| (Rc::new(c) as Rc<dyn Iden>, order)
        ).collect())
    }

    /// Order by columns, variation of [`DeleteStatement::order_by_columns`].
    pub fn order_by_columns_dyn(&mut self, cols: Vec<(Rc<dyn Iden>, Order)>) -> &mut Self {
        let mut orders = cols.into_iter().map(
            |(c, order)| OrderExpr {
                expr: SimpleExpr::Column(c),
                order,
            }).collect();
        self.orders.append(&mut orders);
        self
    }

    /// Order by columns with table prefix.
    pub fn order_by_table_columns<T: 'static, C: 'static>
        (&mut self, cols: Vec<(T, C, Order)>) -> &mut Self 
        where T: Iden, C: Iden {
        self.order_by_table_columns_dyn(cols.into_iter().map(
            |(t, c, order)| (Rc::new(t) as Rc<dyn Iden>, Rc::new(c) as Rc<dyn Iden>, order)
        ).collect())
    }

    /// Order by columns with table prefix, variation of [`DeleteStatement::order_by_columns`].
    #[allow(clippy::type_complexity)]
    pub fn order_by_table_columns_dyn(&mut self, cols: Vec<(Rc<dyn Iden>, Rc<dyn Iden>, Order)>) -> &mut Self {
        let mut orders = cols.into_iter().map(
            |(t, c, order)| OrderExpr {
                expr: SimpleExpr::TableColumn(t, c),
                order,
            }).collect();
        self.orders.append(&mut orders);
        self
    }

    /// Limit number of updated rows.
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(Value::UInt(limit));
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
        let mut sql = String::new();
        query_builder.prepare_delete_statement(self, &mut sql, collector);
        sql
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters    
    pub fn build_collect_any(&self, query_builder: Box<dyn QueryBuilder>, collector: &mut dyn FnMut(Value)) -> String {
        let mut sql = String::new();
        query_builder.prepare_delete_statement(self, &mut sql, collector);
        sql
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
    ///     vec![
    ///         Value::Int(1),
    ///     ]
    /// );
    /// ```
    pub fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Vec<Value>) {
        let mut params = Vec::new();
        let mut collector = |v| params.push(v);
        let sql = self.build_collect(query_builder, &mut collector);
        (sql, params)
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters into a vector
    pub fn build_any(&self, query_builder: Box<dyn QueryBuilder>) -> (String, Vec<Value>) {
        let mut params = Vec::new();
        let mut collector = |v| params.push(v);
        let sql = self.build_collect_any(query_builder, &mut collector);
        (sql, params)
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
        let (mut string, values) = self.build(query_builder);
        for v in values.iter() {
            string = string.replacen("?", value_to_string(v).as_ref(), 1);
        }
        string
    }
}