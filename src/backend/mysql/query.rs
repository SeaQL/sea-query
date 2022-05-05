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
        }
        write!(sql, " ").unwrap();
        self.prepare_order(order_expr, sql, collector);
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

    fn prepare_on_conflict_target(
        &self,
        _: &Option<OnConflictTarget>,
        _: &mut SqlWriter,
        _: &mut dyn FnMut(Value),
    ) {
        // MySQL doesn't support declaring ON CONFLICT target.
    }

    fn prepare_on_conflict_keywords(&self, sql: &mut SqlWriter, _: &mut dyn FnMut(Value)) {
        write!(sql, " ON DUPLICATE KEY ").unwrap();
    }

    fn prepare_on_conflict_do_update_keywords(
        &self,
        sql: &mut SqlWriter,
        _: &mut dyn FnMut(Value),
    ) {
        write!(sql, " UPDATE ").unwrap();
    }

    fn prepare_on_conflict_excluded_table(
        &self,
        col: &DynIden,
        sql: &mut SqlWriter,
        _: &mut dyn FnMut(Value),
    ) {
        write!(sql, "VALUES(").unwrap();
        col.prepare(sql, self.quote());
        write!(sql, ")").unwrap();
    }

    fn insert_default_keyword(&self) -> &str {
        "()"
    }

    fn prepare_select_distinct(
        &self,
        select_distinct: &SelectDistinct,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(Value),
    ) {
        match select_distinct {
            SelectDistinct::All => write!(sql, "ALL").unwrap(),
            SelectDistinct::Distinct => write!(sql, "DISTINCT").unwrap(),
            SelectDistinct::DistinctRow => write!(sql, "DISTINCTROW").unwrap(),
            _ => {}
        };
    }
}
