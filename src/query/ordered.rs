use crate::{expr::*, types::*};

pub trait OrderedStatement {
    #[doc(hidden)]
    // Implementation for the trait.
    fn add_order_by(&mut self, order: OrderExpr) -> &mut Self;

    /// Order by column.
    ///
    /// # Examples
    ///
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
    fn order_by<T>(&mut self, col: T, order: Order) -> &mut Self
    where
        T: IntoColumnRef,
    {
        self.add_order_by(OrderExpr {
            expr: SimpleExpr::Column(col.into_column_ref()),
            order,
            nulls_last: None,
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
            nulls_last: None,
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
                nulls_last: None,
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
                nulls_last: None,
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
    ///     .order_by_nulls_last(Glyph::Image, Order::Desc, true)
    ///     .order_by_nulls_last((Glyph::Table, Glyph::Aspect), Order::Asc, false)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "aspect" FROM "glyph" ORDER BY "image" DESC NULLS LAST, "glyph"."aspect" ASC NULLS FISRT"#
    /// );
    /// ```
    fn order_by_nulls_last<T>(&mut self, col: T, order: Order, nulls_last: bool) -> &mut Self
    where
        T: IntoColumnRef,
    {
        self.add_order_by(OrderExpr {
            expr: SimpleExpr::Column(col.into_column_ref()),
            order,
            nulls_last: Some(nulls_last),
        })
    }

    /// Order by [`SimpleExpr`] with nulls order option.
    fn order_by_expr_nulls_last(
        &mut self,
        expr: SimpleExpr,
        order: Order,
        nulls_last: bool,
    ) -> &mut Self {
        self.add_order_by(OrderExpr {
            expr,
            order,
            nulls_last: Some(nulls_last),
        })
    }

    /// Order by custom string with nulls order option.
    fn order_by_customs_nulls_last<T>(
        &mut self,
        cols: Vec<(T, Order)>,
        nulls_last: bool,
    ) -> &mut Self
    where
        T: ToString,
    {
        cols.into_iter().for_each(|(c, order)| {
            self.add_order_by(OrderExpr {
                expr: SimpleExpr::Custom(c.to_string()),
                order,
                nulls_last: Some(nulls_last),
            });
        });
        self
    }

    /// Order by vector of columns with nulls order option.
    fn order_by_columns_nulls_last<T>(
        &mut self,
        cols: Vec<(T, Order)>,
        nulls_last: bool,
    ) -> &mut Self
    where
        T: IntoColumnRef,
    {
        cols.into_iter().for_each(|(c, order)| {
            self.add_order_by(OrderExpr {
                expr: SimpleExpr::Column(c.into_column_ref()),
                order,
                nulls_last: Some(nulls_last),
            });
        });
        self
    }
}
