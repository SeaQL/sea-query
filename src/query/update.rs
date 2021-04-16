#[cfg(feature="with-json")]
use serde_json::Value as JsonValue;
use crate::{backend::QueryBuilder, types::*, expr::*, value::*, prepare::*};

/// Update existing rows in the table
/// 
/// # Examples
/// 
/// ```
/// use sea_query::{*, tests_cfg::*};
/// 
/// let query = Query::update()
///     .table(Glyph::Table)
///     .values(vec![
///         (Glyph::Aspect, 1.23.into()),
///         (Glyph::Image, "123".into()),
///     ])
///     .and_where(Expr::col(Glyph::Id).eq(1))
///     .to_owned();
/// 
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"UPDATE `glyph` SET `aspect` = 1.23, `image` = '123' WHERE `id` = 1"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder),
///     r#"UPDATE "glyph" SET "aspect" = 1.23, "image" = '123' WHERE "id" = 1"#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"UPDATE `glyph` SET `aspect` = 1.23, `image` = '123' WHERE `id` = 1"#
/// );
/// ```
#[derive(Debug, Clone)]
pub struct UpdateStatement {
    pub(crate) table: Option<Box<TableRef>>,
    pub(crate) values: Vec<(String, Box<SimpleExpr>)>,
    pub(crate) wherei: Option<Box<SimpleExpr>>,
    pub(crate) orders: Vec<OrderExpr>,
    pub(crate) limit: Option<Value>,
}

impl Default for UpdateStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl UpdateStatement {
    /// Construct a new [`UpdateStatement`]
    pub fn new() -> Self {
        Self {
            table: None,
            values: Vec::new(),
            wherei: None,
            orders: Vec::new(),
            limit: None,
        }
    }

    /// Specify which table to update.
    /// 
    /// # Examples
    /// 
    /// See [`UpdateStatement::values`]
    #[allow(clippy::wrong_self_convention)]
    pub fn table<T>(&mut self, tbl_ref: T) -> &mut Self
        where T: IntoTableRef {
        self.table = Some(Box::new(tbl_ref.into_table_ref()));
        self
    }

    #[deprecated(
        since = "0.5.0",
        note = "Please use the UpdateStatement::table function instead"
    )]
    #[allow(clippy::wrong_self_convention)]
    pub fn into_table<T>(&mut self, table: T) -> &mut Self
        where T: IntoTableRef {
        self.table(table)
    }

    /// Update column value by [`SimpleExpr`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::update()
    ///     .table(Glyph::Table)
    ///     .value_expr(Glyph::Aspect, Expr::cust("60 * 24 * 24"))
    ///     .values(vec![
    ///         (Glyph::Image, "24B0E11951B03B07F8300FD003983F03F0780060".into()),
    ///     ])
    ///     .and_where(Expr::col(Glyph::Id).eq(1))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"UPDATE `glyph` SET `aspect` = 60 * 24 * 24, `image` = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"UPDATE "glyph" SET "aspect" = 60 * 24 * 24, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"UPDATE `glyph` SET `aspect` = 60 * 24 * 24, `image` = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE `id` = 1"#
    /// );
    /// ```
    pub fn value_expr<T>(&mut self, col: T, exp: SimpleExpr) -> &mut Self
        where T: IntoIden {
        self.push_boxed_value(col.into_iden().to_string(), exp);
        self
    }

    /// Update column values by [`JsonValue`]. A convenience method if you have multiple column-value pairs to set at once.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::update()
    ///     .table(Glyph::Table)
    ///     .json(json!({
    ///         "aspect": 2.1345,
    ///         "image": "235m",
    ///     }))
    ///     .and_where(Expr::col(Glyph::Id).eq(1))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' WHERE `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '235m' WHERE "id" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' WHERE `id` = 1"#
    /// );
    /// ```
    #[cfg(feature="with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    pub fn json(&mut self, values: JsonValue) -> &mut Self {
        match values {
            JsonValue::Object(_) => (),
            _ => unimplemented!(),
        }
        for (k, v) in values.as_object().unwrap() {
            let v = json_value_to_sea_value(v);
            self.push_boxed_value(k.into(), SimpleExpr::Value(v));
        }
        self
    }

    /// Update column values.. A convenience method if you have multiple column-value pairs to set at once.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::update()
    ///     .table(Glyph::Table)
    ///     .values(vec![
    ///         (Glyph::Aspect, 2.1345.into()),
    ///         (Glyph::Image, "235m".into()),
    ///     ])
    ///     .and_where(Expr::col(Glyph::Id).eq(1))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' WHERE `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '235m' WHERE "id" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' WHERE `id` = 1"#
    /// );
    /// ```
    pub fn values<T>(&mut self, values: impl IntoIterator<Item = (T, Value)>) -> &mut Self
        where T: IntoIden {
        for (k, v) in values.into_iter() {
            self.push_boxed_value(k.into_iden().to_string(), SimpleExpr::Value(v));
        }
        self
    }

    /// Update column values.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::update()
    ///     .table(Glyph::Table)
    ///     .value(Glyph::Aspect, 2.1345.into())
    ///     .value(Glyph::Image, "235m".into())
    ///     .and_where(Expr::col(Glyph::Id).eq(1))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' WHERE `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '235m' WHERE "id" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' WHERE `id` = 1"#
    /// );
    /// ```
    pub fn value<T>(&mut self, col: T, value: Value) -> &mut Self
        where T: IntoIden {
        self.push_boxed_value(col.into_iden().to_string(), SimpleExpr::Value(value));
        self
    }

    fn push_boxed_value(&mut self, k: String, v: SimpleExpr) -> &mut Self {
        self.values.push((k, Box::new(v)));
        self
    }

    /// And where condition.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::update()
    ///     .table(Glyph::Table)
    ///     .values(vec![
    ///         (Glyph::Aspect, 2.1345.into()),
    ///         (Glyph::Image, "235m".into()),
    ///     ])
    ///     .and_where(Expr::col(Glyph::Id).gt(1))
    ///     .and_where(Expr::col(Glyph::Id).lt(3))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' WHERE (`id` > 1) AND (`id` < 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '235m' WHERE ("id" > 1) AND ("id" < 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' WHERE (`id` > 1) AND (`id` < 3)"#
    /// );
    /// ```
    pub fn and_where(&mut self, other: SimpleExpr) -> &mut Self {
        self.and_or_where(BinOper::And, other)
    }

    /// Or where condition.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::update()
    ///     .table(Glyph::Table)
    ///     .values(vec![
    ///         (Glyph::Aspect, 2.1345.into()),
    ///         (Glyph::Image, "235m".into()),
    ///     ])
    ///     .or_where(Expr::col(Glyph::Aspect).lt(1))
    ///     .or_where(Expr::col(Glyph::Aspect).gt(3))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' WHERE (`aspect` < 1) OR (`aspect` > 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '235m' WHERE ("aspect" < 1) OR ("aspect" > 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' WHERE (`aspect` < 1) OR (`aspect` > 3)"#
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
        where T: IntoColumnRef {
        self.orders.push(OrderExpr {
            expr: SimpleExpr::Column(col.into_column_ref()),
            order,
        });
        self
    }

    #[deprecated(
        since = "0.9.0",
        note = "Please use the [`UpdateStatement::order_by`] with a tuple as [`ColumnRef`]"
    )]
    pub fn order_by_tbl<T, C>
        (&mut self, table: T, col: C, order: Order) -> &mut Self 
        where T: IntoIden, C: IntoIden {
        self.order_by((table.into_iden(), col.into_iden()), order)
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
    pub fn order_by_customs<T>(&mut self, cols: impl IntoIterator<Item = (T, Order)>) -> &mut Self 
        where T: ToString {
        let mut orders = cols.into_iter().map(
            |(c, order)| OrderExpr {
                expr: SimpleExpr::Custom(c.to_string()),
                order,
            }).collect();
        self.orders.append(&mut orders);
        self
    }

    /// Order by vector of columns.
    pub fn order_by_columns<T>(&mut self, cols: impl IntoIterator<Item = (T, Order)>) -> &mut Self 
        where T: IntoColumnRef {
        let mut orders = cols.into_iter().map(
            |(c, order)| OrderExpr {
                expr: SimpleExpr::Column(c.into_column_ref()),
                order,
            }).collect();
        self.orders.append(&mut orders);
        self
    }

    #[deprecated(
        since = "0.9.0",
        note = "Please use the [`UpdateStatement::order_by_columns`] with a tuple as [`ColumnRef`]"
    )]
    pub fn order_by_table_columns<T, C>
        (&mut self, cols: Vec<(T, C, Order)>) -> &mut Self 
        where T: IntoIden, C: IntoIden {
        self.order_by_columns(cols.into_iter().map(|(t, c, o)| ((t.into_iden(), c.into_iden()), o)).collect())
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
    /// let query = Query::update()
    ///     .table(Glyph::Table)
    ///     .values(vec![
    ///         (Glyph::Aspect, 2.1345.into()),
    ///         (Glyph::Image, "235m".into()),
    ///     ])
    ///     .and_where(Expr::col(Glyph::Id).eq(1))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' WHERE `id` = 1"#
    /// );
    /// 
    /// let mut params = Vec::new();
    /// let mut collector = |v| params.push(v);
    /// 
    /// assert_eq!(
    ///     query.build_collect(MysqlQueryBuilder, &mut collector),
    ///     r#"UPDATE `glyph` SET `aspect` = ?, `image` = ? WHERE `id` = ?"#
    /// );
    /// assert_eq!(
    ///     params,
    ///     vec![
    ///         Value::Double(2.1345),
    ///         Value::String(Box::new(String::from("235m"))),
    ///         Value::Int(1),
    ///     ]
    /// );
    /// ```
    pub fn build_collect<T: QueryBuilder>(&self, query_builder: T, collector: &mut dyn FnMut(Value)) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_update_statement(self, &mut sql, collector);
        sql.result()
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters
    pub fn build_collect_any(&self, query_builder: &dyn QueryBuilder, collector: &mut dyn FnMut(Value)) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_update_statement(self, &mut sql, collector);
        sql.result()
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters into a vector
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let (query, params) = Query::update()
    ///     .table(Glyph::Table)
    ///     .values(vec![
    ///         (Glyph::Aspect, 2.1345.into()),
    ///         (Glyph::Image, "235m".into()),
    ///     ])
    ///     .and_where(Expr::col(Glyph::Id).eq(1))
    ///     .build(MysqlQueryBuilder);
    /// 
    /// assert_eq!(
    ///     query,
    ///     r#"UPDATE `glyph` SET `aspect` = ?, `image` = ? WHERE `id` = ?"#
    /// );
    /// assert_eq!(
    ///     params,
    ///     Values(vec![
    ///         Value::Double(2.1345),
    ///         Value::String(Box::new(String::from("235m"))),
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
    /// let query = Query::update()
    ///     .table(Glyph::Table)
    ///     .values(vec![
    ///         (Glyph::Aspect, 2.1345.into()),
    ///         (Glyph::Image, "235m".into()),
    ///     ])
    ///     .and_where(Expr::col(Glyph::Id).eq(1))
    ///     .to_string(MysqlQueryBuilder);
    /// 
    /// assert_eq!(
    ///     query,
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' WHERE `id` = 1"#
    /// );
    /// ```
    pub fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String {
        let (sql, values) = self.build_any(&query_builder);
        inject_parameters(&sql, values.0, &query_builder)
    }
}