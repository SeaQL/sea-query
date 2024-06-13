use super::*;

impl QueryBuilder for BigQueryQueryBuilder {
    fn placeholder(&self) -> (&str, bool) {
        ("$", true)
    }

    fn prepare_select_lock(&self, _select_lock: &LockClause, _sql: &mut dyn SqlWriter) {
        // SQLite doesn't supports row locking
    }

    fn if_null_function(&self) -> &str {
        "COALESCE"
    }

    fn prepare_sub_query_oper(&self, oper: &SubQueryOper, sql: &mut dyn SqlWriter) {
        write!(
            sql,
            "{}",
            match oper {
                SubQueryOper::Exists => "EXISTS",
                SubQueryOper::Any => panic!("Operator 'ANY' doesnot support"),
                SubQueryOper::Some => panic!("Operator 'SOME' doesnot support"),
                SubQueryOper::All => panic!("Operator 'ALL' doesnot support"),
                // Should add custom operator options. In the case of BigQuery, ARRAY, Scalar subquery
            }
        )
        .unwrap();
    }

    fn prepare_union_statement(
        &self,
        union_type: UnionType,
        select_statement: &SelectStatement,
        sql: &mut dyn SqlWriter,
    ) {
        match union_type {
            UnionType::Intersect => write!(sql, " INTERSECT ").unwrap(),
            UnionType::Distinct => write!(sql, " UNION DISTINCT ").unwrap(),
            UnionType::Except => write!(sql, " EXCEPT ").unwrap(),
            UnionType::All => write!(sql, " UNION ALL ").unwrap(),
        }
        self.prepare_select_statement(select_statement, sql);
    }

    fn prepare_query_statement(&self, query: &SubQueryStatement, sql: &mut dyn SqlWriter) {
        query.prepare_statement(self, sql);
    }

    fn prepare_order_expr(&self, order_expr: &OrderExpr, sql: &mut dyn SqlWriter) {
        if !matches!(order_expr.order, Order::Field(_)) {
            self.prepare_simple_expr(&order_expr.expr, sql);
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
        "CHAR_LENGTH"
    }

    fn insert_default_values(&self, _: u32, _sql: &mut dyn SqlWriter) {
        panic!("BigQuery does not support default values");
    }
}
