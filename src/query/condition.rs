use crate::{expr::SimpleExpr, types::LogicalChainOper};

pub trait ConditionalStatement {
    /// And where condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .and_where(Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![3, 4]))
    ///     .and_where(Expr::tbl(Glyph::Table, Glyph::Image).like("A%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%'"#
    /// );
    /// ```
    fn and_where(&mut self, other: SimpleExpr) -> &mut Self {
        self.and_or_where(LogicalChainOper::And(other))
    }

    /// And where condition, short hand for `if c.is_some() q.and_where(c)`.
    fn and_where_option(&mut self, other: Option<SimpleExpr>) -> &mut Self {
        if let Some(other) = other {
            self.and_or_where(LogicalChainOper::And(other));
        }
        self
    }

    /// Or where condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .or_where(Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![3, 4]))
    ///     .or_where(Expr::tbl(Glyph::Table, Glyph::Image).like("A%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) OR `glyph`.`image` LIKE 'A%'"#
    /// );
    /// ```
    fn or_where(&mut self, other: SimpleExpr) -> &mut Self {
        self.and_or_where(LogicalChainOper::Or(other))
    }

    #[doc(hidden)]
    // Trait implementation.
    fn and_or_where(&mut self, condition: LogicalChainOper) -> &mut Self;
}
