use super::*;

impl QueryBuilder for SqliteQueryBuilder {
    fn char_length_function(&self) -> &str {
        "LENGTH"
    }

    fn prepare_select_lock(
        &self,
        _select_lock: &LockType,
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
        self.prepare_simple_expr(&order_expr.expr, sql, collector);
        write!(sql, " ").unwrap();
        self.prepare_order(&order_expr.order, sql, collector);
        match order_expr.nulls {
            None => (),
            Some(NullOrdering::Last) => write!(sql, " NULLS LAST").unwrap(),
            Some(NullOrdering::First) => write!(sql, " NULLS FIRST").unwrap(),
        }
    }

    fn prepare_query_statement(
        &self,
        query: &dyn QueryStatementBuilder,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        query.build_collect_any_into(self, sql, collector);
    }

    fn prepare_with_clause_recursive_options(
        &self,
        _: &WithClause,
        _: &mut SqlWriter,
        _: &mut dyn FnMut(Value),
    ) {
        // Sqlite doesn't support sql recursive with query 'SEARCH' and 'CYCLE' options.
    }
}
