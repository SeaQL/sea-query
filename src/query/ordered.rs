use crate::{expr::*, types::*};

pub trait OrderedStatement {
    #[doc(hidden)]
    // Implementation for the trait.
    fn add_order_by(&mut self, order: OrderExpr) -> &mut Self;

    /// Order by column.
    ///
    /// # Examples
    ///
    /// Order by ASC and DESC
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
    ///     .order_by(Glyph::Image, Order::Desc)
    ///     .order_by((Glyph::Table, Glyph::Aspect), Order::Asc)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, 0) > 2 ORDER BY `image` DESC, `glyph`.`aspect` ASC"#
    /// );
    /// ```
    ///
    /// Order by custom field ordering
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Glyph::Aspect])
    ///     .from(Glyph::Table)
    ///     .order_by(
    ///         Glyph::Id,
    ///         Order::Field(Values(vec![4.into(), 5.into(), 1.into(), 3.into()])),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     [
    ///         r#"SELECT `aspect`"#,
    ///         r#"FROM `glyph`"#,
    ///         r#"ORDER BY CASE"#,
    ///         r#"WHEN `id`=4 THEN 0"#,
    ///         r#"WHEN `id`=5 THEN 1"#,
    ///         r#"WHEN `id`=1 THEN 2"#,
    ///         r#"WHEN `id`=3 THEN 3"#,
    ///         r#"ELSE 4 END"#,
    ///     ]
    ///     .join(" ")
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"SELECT "aspect""#,
    ///         r#"FROM "glyph""#,
    ///         r#"ORDER BY CASE"#,
    ///         r#"WHEN "id"=4 THEN 0"#,
    ///         r#"WHEN "id"=5 THEN 1"#,
    ///         r#"WHEN "id"=1 THEN 2"#,
    ///         r#"WHEN "id"=3 THEN 3"#,
    ///         r#"ELSE 4 END"#,
    ///     ]
    ///     .join(" ")
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     [
    ///         r#"SELECT "aspect""#,
    ///         r#"FROM "glyph""#,
    ///         r#"ORDER BY CASE"#,
    ///         r#"WHEN "id"=4 THEN 0"#,
    ///         r#"WHEN "id"=5 THEN 1"#,
    ///         r#"WHEN "id"=1 THEN 2"#,
    ///         r#"WHEN "id"=3 THEN 3"#,
    ///         r#"ELSE 4 END"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    fn order_by<T>(&mut self, col: T, order: Order) -> &mut Self
    where
        T: IntoColumnRef,
    {
        self.add_order_by(OrderExpr {
            expr: SimpleExpr::Column(col.into_column_ref()),
            order,
            nulls: None,
        })
    }

    #[deprecated(
        since = "0.9.0",
        note = "Please use the [`OrderedStatement::order_by`] with a tuple as [`ColumnRef`]"
    )]
    fn order_by_tbl<T, C>(&mut self, table: T, col: C, order: Order) -> &mut Self
    where
        T: IntoIden,
        C: IntoIden,
    {
        self.order_by((table.into_iden(), col.into_iden()), order)
    }

    /// Order by [`SimpleExpr`].
    fn order_by_expr(&mut self, expr: SimpleExpr, order: Order) -> &mut Self {
        self.add_order_by(OrderExpr {
            expr,
            order,
            nulls: None,
        })
    }

    /// Order by custom string.
    fn order_by_customs<T>(&mut self, cols: Vec<(T, Order)>) -> &mut Self
    where
        T: ToString,
    {
        cols.into_iter().for_each(|(c, order)| {
            self.add_order_by(OrderExpr {
                expr: SimpleExpr::Custom(c.to_string()),
                order,
                nulls: None,
            });
        });
        self
    }

    /// Order by vector of columns.
    fn order_by_columns<T>(&mut self, cols: Vec<(T, Order)>) -> &mut Self
    where
        T: IntoColumnRef,
    {
        cols.into_iter().for_each(|(c, order)| {
            self.add_order_by(OrderExpr {
                expr: SimpleExpr::Column(c.into_column_ref()),
                order,
                nulls: None,
            });
        });
        self
    }

    #[deprecated(
        since = "0.9.0",
        note = "Please use the [`OrderedStatement::order_by_columns`] with a tuple as [`ColumnRef`]"
    )]
    fn order_by_table_columns<T, C>(&mut self, cols: Vec<(T, C, Order)>) -> &mut Self
    where
        T: IntoIden,
        C: IntoIden,
    {
        self.order_by_columns(
            cols.into_iter()
                .map(|(t, c, o)| ((t.into_iden(), c.into_iden()), o))
                .collect(),
        )
    }

    /// Order by column with nulls order option.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .order_by_with_nulls(Glyph::Image, Order::Desc, NullOrdering::Last)
    ///     .order_by_with_nulls((Glyph::Table, Glyph::Aspect), Order::Asc, NullOrdering::First)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "aspect" FROM "glyph" ORDER BY "image" DESC NULLS LAST, "glyph"."aspect" ASC NULLS FIRST"#
    /// );
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect` FROM `glyph` ORDER BY `image` IS NULL ASC, `image` DESC, `glyph`.`aspect` IS NULL DESC, `glyph`.`aspect` ASC"#
    /// );
    /// ```
    fn order_by_with_nulls<T>(&mut self, col: T, order: Order, nulls: NullOrdering) -> &mut Self
    where
        T: IntoColumnRef,
    {
        self.add_order_by(OrderExpr {
            expr: SimpleExpr::Column(col.into_column_ref()),
            order,
            nulls: Some(nulls),
        })
    }

    /// Order by [`SimpleExpr`] with nulls order option.
    fn order_by_expr_with_nulls(
        &mut self,
        expr: SimpleExpr,
        order: Order,
        nulls: NullOrdering,
    ) -> &mut Self {
        self.add_order_by(OrderExpr {
            expr,
            order,
            nulls: Some(nulls),
        })
    }

    /// Order by custom string with nulls order option.
    fn order_by_customs_with_nulls<T>(&mut self, cols: Vec<(T, Order, NullOrdering)>) -> &mut Self
    where
        T: ToString,
    {
        cols.into_iter().for_each(|(c, order, nulls)| {
            self.add_order_by(OrderExpr {
                expr: SimpleExpr::Custom(c.to_string()),
                order,
                nulls: Some(nulls),
            });
        });
        self
    }

    /// Order by vector of columns with nulls order option.
    fn order_by_columns_with_nulls<T>(&mut self, cols: Vec<(T, Order, NullOrdering)>) -> &mut Self
    where
        T: IntoColumnRef,
    {
        cols.into_iter().for_each(|(c, order, nulls)| {
            self.add_order_by(OrderExpr {
                expr: SimpleExpr::Column(c.into_column_ref()),
                order,
                nulls: Some(nulls),
            });
        });
        self
    }
}
