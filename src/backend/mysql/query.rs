use super::*;

impl QueryBuilder for MysqlQueryBuilder {
    fn prepare_returning(
        &self,
        _returning: &[SelectExpr],
        _sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(Value),
    ) {
    }

    fn prepare_order_expr(
        &self,
        order_expr: &OrderExpr,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        match order_expr.nulls {
            None => (),
            Some(Nulls::Last) => {
                self.prepare_simple_expr(&order_expr.expr, sql, collector);
                write!(sql, " IS NULL ASC, ").unwrap()
            }
            Some(Nulls::First) => {
                self.prepare_simple_expr(&order_expr.expr, sql, collector);
                write!(sql, " IS NULL DESC, ").unwrap()
            }
        }
        self.prepare_simple_expr(&order_expr.expr, sql, collector);
        write!(sql, " ").unwrap();
        self.prepare_order(&order_expr.order, sql, collector);
    }
}
