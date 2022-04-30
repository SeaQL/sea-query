use super::*;

impl QueryBuilder for SqliteQueryBuilder {
    fn char_length_function(&self) -> &str {
        "LENGTH"
    }

    fn prepare_select_lock(
        &self,
        _select_lock: &LockClause,
        _sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(Value),
    ) {
        // SQLite doesn't supports row locking
    }

    fn prepare_order_expr(
        &self,
        order_expr: &OrderExpr,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        if !matches!(order_expr.order, Order::Field(_)) {
            self.prepare_simple_expr(&order_expr.expr, sql, collector);
        }
        write!(sql, " ").unwrap();
        self.prepare_order(order_expr, sql, collector);
        match order_expr.nulls {
            None => (),
            Some(NullOrdering::Last) => write!(sql, " NULLS LAST").unwrap(),
            Some(NullOrdering::First) => write!(sql, " NULLS FIRST").unwrap(),
        }
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
        // Sqlite doesn't support sql recursive with query 'SEARCH' and 'CYCLE' options.
    }

    fn insert_default_values(&self, _: u32, sql: &mut SqlWriter) {
        // SQLite doesn't support inserting multiple rows with default values
        write!(sql, "DEFAULT VALUES").unwrap()
    }
}
