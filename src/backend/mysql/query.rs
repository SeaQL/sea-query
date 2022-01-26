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
            Some(NullOrdering::Last) => {
                self.prepare_simple_expr(&order_expr.expr, sql, collector);
                write!(sql, " IS NULL ASC, ").unwrap()
            }
            Some(NullOrdering::First) => {
                self.prepare_simple_expr(&order_expr.expr, sql, collector);
                write!(sql, " IS NULL DESC, ").unwrap()
            }
        }
        if !matches!(order_expr.order, Order::Field(_)) {
            self.prepare_simple_expr(&order_expr.expr, sql, collector);
            write!(sql, " ").unwrap();
        }
        self.prepare_order(order_expr, sql, collector);
    }

    fn prepare_field_order(
        &self,
        order_expr: &OrderExpr,
        values: &Values,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        let len = values.0.len();
        let mut values_str = String::from("");
        for i in 0..len {
            let value = self.value_to_string(&values.0[i]);
            values_str.push_str(&*value);
            if i != len - 1 {
                values_str.push_str(", ");
            }
        }
        write!(sql, " IF(").unwrap();
        self.prepare_simple_expr(&order_expr.expr, sql, collector);
        write!(sql, " NOT IN ({}), {}, 0),", values_str, len).unwrap();
        write!(sql, " FIELD(").unwrap();
        self.prepare_simple_expr(&order_expr.expr, sql, collector);
        write!(sql, ", ").unwrap();
        write!(sql, "{}", values_str).unwrap();
        write!(sql, ")").unwrap();
    }

    fn prepare_query_statement(
        &self,
        query: &SubQueryStatement,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        query.prepare_statement(self, sql, collector);
    }

    fn prepare_with_clause_recursive_options(
        &self,
        _: &WithClause,
        _: &mut SqlWriter,
        _: &mut dyn FnMut(Value),
    ) {
        // MySQL doesn't support sql recursive with query 'SEARCH' and 'CYCLE' options.
    }

    fn prepare_with_query_clause_materialization(
        &self,
        _: &CommonTableExpression,
        _: &mut SqlWriter,
    ) {
        // MySQL doesn't support declaring materialization in SQL for with query.
    }
}
