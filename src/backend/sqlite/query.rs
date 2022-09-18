use super::*;

impl QueryBuilder for SqliteQueryBuilder {
    fn prepare_select_lock(&self, _select_lock: &LockClause, _sql: &mut dyn SqlWriter) {
        // SQLite doesn't supports row locking
    }

    fn prepare_query_statement(&self, query: &SubQueryStatement, sql: &mut dyn SqlWriter) {
        query.prepare_statement(self, sql);
    }

    fn prepare_with_clause_recursive_options(&self, _: &WithClause, _: &mut dyn SqlWriter) {
        // Sqlite doesn't support sql recursive with query 'SEARCH' and 'CYCLE' options.
    }

    fn prepare_order_expr(&self, order_expr: &OrderExpr, sql: &mut dyn SqlWriter) {
        if !matches!(order_expr.order, Order::Field(_)) {
            self.prepare_simple_expr(&order_expr.expr, sql);
            write!(sql, " ").unwrap();
        }
        self.prepare_order(order_expr, sql);
        match order_expr.nulls {
            None => (),
            Some(NullOrdering::Last) => write!(sql, " NULLS LAST").unwrap(),
            Some(NullOrdering::First) => write!(sql, " NULLS FIRST").unwrap(),
        }
    }

    fn prepare_value(&self, value: &Value, sql: &mut dyn SqlWriter) {
        sql.push_param(value.clone(), self as _);
    }

    fn char_length_function(&self) -> &str {
        "LENGTH"
    }

    fn insert_default_values(&self, _: u32, sql: &mut dyn SqlWriter) {
        // SQLite doesn't support inserting multiple rows with default values
        write!(sql, "DEFAULT VALUES").unwrap()
    }

    fn prepare_sub_query_oper(&self, oper: &SubQueryOper, sql: &mut SqlWriter) {
        write!(
            sql,
            "{}",
            match oper {
                SubQueryOper::Exists => "EXISTS",
                SubQueryOper::Any => panic!("Operator 'ANY' doesnot support"),
                SubQueryOper::Some => panic!("Operator 'SOME' doesnot support"),
                SubQueryOper::All => panic!("Operator 'ALL' doesnot support"),
            }
        )
        .unwrap();
    }
}
