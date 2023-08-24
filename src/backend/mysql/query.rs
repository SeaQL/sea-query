use super::*;

impl QueryBuilder for MysqlQueryBuilder {
    fn values_list_tuple_prefix(&self) -> &str {
        "ROW"
    }

    fn prepare_select_distinct(&self, select_distinct: &SelectDistinct, sql: &mut dyn SqlWriter) {
        match select_distinct {
            SelectDistinct::All => write!(sql, "ALL").unwrap(),
            SelectDistinct::Distinct => write!(sql, "DISTINCT").unwrap(),
            SelectDistinct::DistinctRow => write!(sql, "DISTINCTROW").unwrap(),
            _ => {}
        };
    }

    fn prepare_query_statement(&self, query: &SubQueryStatement, sql: &mut dyn SqlWriter) {
        query.prepare_statement(self, sql);
    }

    fn prepare_with_clause_recursive_options(&self, _: &WithClause, _: &mut dyn SqlWriter) {
        // MySQL doesn't support sql recursive with query 'SEARCH' and 'CYCLE' options.
    }

    fn prepare_with_query_clause_materialization(
        &self,
        _: &CommonTableExpression,
        _: &mut dyn SqlWriter,
    ) {
        // MySQL doesn't support declaring materialization in SQL for with query.
    }

    fn prepare_join_type(&self, join_type: &JoinType, sql: &mut dyn SqlWriter) {
        match join_type {
            JoinType::FullOuterJoin => panic!("Mysql does not support FULL OUTER JOIN"),
            _ => self.prepare_join_type_common(join_type, sql),
        }
    }

    fn prepare_order_expr(&self, order_expr: &OrderExpr, sql: &mut dyn SqlWriter) {
        match order_expr.nulls {
            None => (),
            Some(NullOrdering::Last) => {
                self.prepare_simple_expr(&order_expr.expr, sql);
                write!(sql, " IS NULL ASC, ").unwrap()
            }
            Some(NullOrdering::First) => {
                self.prepare_simple_expr(&order_expr.expr, sql);
                write!(sql, " IS NULL DESC, ").unwrap()
            }
        }
        if !matches!(order_expr.order, Order::Field(_)) {
            self.prepare_simple_expr(&order_expr.expr, sql);
        }
        self.prepare_order(order_expr, sql);
    }

    fn prepare_value(&self, value: &Value, sql: &mut dyn SqlWriter) {
        sql.push_param(value.clone(), self as _);
    }

    #[doc(hidden)]
    /// Write ON CONFLICT expression
    fn prepare_on_conflict(&self, on_conflict: &Option<OnConflict>, sql: &mut dyn SqlWriter) {
        if let Some(on_conflict) = on_conflict {
            self.prepare_on_conflict_keywords(sql);
            self.prepare_on_conflict_target(&on_conflict.target, sql);
            self.prepare_on_conflict_condition(&on_conflict.target_where, sql);
            self.prepare_on_conflict_action(&on_conflict.action, sql);
            self.prepare_on_conflict_condition(&on_conflict.action_where, sql);
        }
    }

    fn prepare_on_conflict_action(
        &self,
        on_conflict_action: &Option<OnConflictAction>,
        sql: &mut dyn SqlWriter,
    ) {
        if let Some(action) = on_conflict_action {
            self.prepare_on_conflict_do_update_keywords(sql);
            match action {
                OnConflictAction::DoNothing(pk_cols) => {
                    pk_cols.iter().fold(true, |first, pk_col| {
                        if !first {
                            write!(sql, ", ").unwrap()
                        }
                        pk_col.prepare(sql.as_writer(), self.quote());
                        write!(sql, " = ").unwrap();
                        self.prepare_on_conflict_excluded_table(pk_col, sql);
                        false
                    });
                }
                OnConflictAction::Update(update_strats) => {
                    update_strats.iter().fold(true, |first, update_strat| {
                        if !first {
                            write!(sql, ", ").unwrap()
                        }
                        match update_strat {
                            OnConflictUpdate::Column(col) => {
                                col.prepare(sql.as_writer(), self.quote());
                                write!(sql, " = ").unwrap();
                                self.prepare_on_conflict_excluded_table(col, sql);
                            }
                            OnConflictUpdate::Expr(col, expr) => {
                                col.prepare(sql.as_writer(), self.quote());
                                write!(sql, " = ").unwrap();
                                self.prepare_simple_expr(expr, sql);
                            }
                        }
                        false
                    });
                }
            }
        }
    }

    fn prepare_on_conflict_target(&self, _: &Option<OnConflictTarget>, _: &mut dyn SqlWriter) {
        // MySQL doesn't support declaring ON CONFLICT target.
    }

    fn prepare_on_conflict_keywords(&self, sql: &mut dyn SqlWriter) {
        write!(sql, " ON DUPLICATE KEY").unwrap();
    }

    fn prepare_on_conflict_do_update_keywords(&self, sql: &mut dyn SqlWriter) {
        write!(sql, " UPDATE ").unwrap();
    }

    fn prepare_on_conflict_excluded_table(&self, col: &DynIden, sql: &mut dyn SqlWriter) {
        write!(sql, "VALUES(").unwrap();
        col.prepare(sql.as_writer(), self.quote());
        write!(sql, ")").unwrap();
    }

    fn prepare_on_conflict_condition(&self, _: &ConditionHolder, _: &mut dyn SqlWriter) {}

    fn prepare_returning(&self, _returning: &Option<ReturningClause>, _sql: &mut dyn SqlWriter) {}

    fn random_function(&self) -> &str {
        "RAND"
    }

    fn insert_default_keyword(&self) -> &str {
        "()"
    }
}
