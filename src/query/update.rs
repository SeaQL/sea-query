use crate::{
    backend::QueryBuilder,
    expr::*,
    prepare::*,
    query::{condition::*, OrderedStatement},
    types::*,
    value::*,
    QueryStatementBuilder,
};
#[cfg(feature = "with-json")]
use serde_json::Value as JsonValue;

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
    pub(crate) wherei: ConditionHolder,
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
            wherei: ConditionHolder::new(),
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
    where
        T: IntoTableRef,
    {
        self.table = Some(Box::new(tbl_ref.into_table_ref()));
        self
    }

    #[deprecated(
        since = "0.5.0",
        note = "Please use the UpdateStatement::table function instead"
    )]
    #[allow(clippy::wrong_self_convention)]
    pub fn into_table<T>(&mut self, table: T) -> &mut Self
    where
        T: IntoTableRef,
    {
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
    ///     .col_expr(Glyph::Aspect, Expr::cust("60 * 24 * 24"))
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
    pub fn col_expr<T>(&mut self, col: T, expr: SimpleExpr) -> &mut Self
    where
        T: IntoIden,
    {
        self.push_boxed_value(col.into_iden().to_string(), expr);
        self
    }

    /// Alias of [`UpdateStatement::col_expr`]
    pub fn value_expr<T>(&mut self, col: T, expr: SimpleExpr) -> &mut Self
    where
        T: IntoIden,
    {
        self.col_expr(col, expr)
    }

    /// Update multiple columns, taking a JSON Object as input.
    /// Will panic if `values` is not serde_json::Value::Object.
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
    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    pub fn json(&mut self, values: JsonValue) -> &mut Self {
        match values {
            JsonValue::Object(_) => (),
            _ => panic!("must be JsonValue::Object"),
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
    pub fn values<T, I>(&mut self, values: I) -> &mut Self
    where
        T: IntoIden,
        I: IntoIterator<Item = (T, Value)>,
    {
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
    where
        T: IntoIden,
    {
        self.push_boxed_value(col.into_iden().to_string(), SimpleExpr::Value(value));
        self
    }

    fn push_boxed_value(&mut self, k: String, v: SimpleExpr) -> &mut Self {
        self.values.push((k, Box::new(v)));
        self
    }

    /// Limit number of updated rows.
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(Value::BigUnsigned(limit));
        self
    }
}

impl QueryStatementBuilder for UpdateStatement {
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
    fn build_collect<T: QueryBuilder>(
        &self,
        query_builder: T,
        collector: &mut dyn FnMut(Value),
    ) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_update_statement(self, &mut sql, collector);
        sql.result()
    }

    fn build_collect_any(
        &self,
        query_builder: &dyn QueryBuilder,
        collector: &mut dyn FnMut(Value),
    ) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_update_statement(self, &mut sql, collector);
        sql.result()
    }
}

impl OrderedStatement for UpdateStatement {
    fn add_order_by(&mut self, order: OrderExpr) -> &mut Self {
        self.orders.push(order);
        self
    }
}

impl ConditionalStatement for UpdateStatement {
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
