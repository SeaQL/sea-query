use super::*;
use crate::extension::mysql::*;

impl QueryBuilder for MysqlQueryBuilder {
    fn values_list_tuple_prefix(&self) -> &str {
        "ROW"
    }

    fn prepare_select_distinct(&self, select_distinct: &SelectDistinct, sql: &mut impl SqlWriter) {
        match select_distinct {
            SelectDistinct::All => sql.write_str("ALL").unwrap(),
            SelectDistinct::Distinct => sql.write_str("DISTINCT").unwrap(),
            SelectDistinct::DistinctRow => sql.write_str("DISTINCTROW").unwrap(),
            _ => {}
        };
    }

    fn prepare_index_hints(&self, select: &SelectStatement, sql: &mut impl SqlWriter) {
        if !select.index_hints.is_empty() {
            sql.write_str(" ").unwrap();
        }

        let mut hints = select.index_hints.iter();

        join_io!(
            hints,
            hint,
            join {
                sql.write_str(" ").unwrap();
            },
            do {
                match hint.r#type {
                    IndexHintType::Use => {
                        sql.write_str("USE INDEX ").unwrap();
                        self.prepare_index_hint_scope(&hint.scope, sql);
                        sql.write_str("(").unwrap();
                        self.prepare_iden(&hint.index, sql);
                    }
                    IndexHintType::Ignore => {
                        sql.write_str("IGNORE INDEX ").unwrap();
                        self.prepare_index_hint_scope(&hint.scope, sql);
                        sql.write_str("(").unwrap();
                        self.prepare_iden(&hint.index, sql);
                    }
                    IndexHintType::Force => {
                        sql.write_str("FORCE INDEX ").unwrap();
                        self.prepare_index_hint_scope(&hint.scope, sql);
                        sql.write_str("(").unwrap();
                        self.prepare_iden(&hint.index, sql);
                    }
                }
                sql.write_str(")").unwrap();
            }
        );
    }

    fn prepare_query_statement(&self, query: &SubQueryStatement, sql: &mut impl SqlWriter) {
        query.prepare_statement(self, sql);
    }

    fn prepare_with_clause_recursive_options(&self, _: &WithClause, _: &mut impl SqlWriter) {
        // MySQL doesn't support sql recursive with query 'SEARCH' and 'CYCLE' options.
    }

    fn prepare_with_query_clause_materialization(
        &self,
        _: &CommonTableExpression,
        _: &mut impl SqlWriter,
    ) {
        // MySQL doesn't support declaring materialization in SQL for with query.
    }

    fn prepare_update_join(
        &self,
        from: &[TableRef],
        condition: &ConditionHolder,
        sql: &mut impl SqlWriter,
    ) {
        if from.is_empty() {
            return;
        }

        sql.write_str(" JOIN ").unwrap();

        // TODO what if we have multiple from?
        self.prepare_table_ref(&from[0], sql);

        self.prepare_condition(condition, "ON", sql);
    }

    fn prepare_update_from(&self, _: &[TableRef], _: &mut impl SqlWriter) {}

    fn prepare_update_column(
        &self,
        table: &Option<Box<TableRef>>,
        from: &[TableRef],
        column: &DynIden,
        sql: &mut impl SqlWriter,
    ) {
        use std::ops::Deref;

        if !from.is_empty() {
            if let Some(table) = table {
                // Support only "naked" table names with no schema or alias.
                if let TableRef::Table(TableName(None, table), None) = table.deref() {
                    let column_name = ColumnName::from((table.clone(), column.clone()));
                    self.prepare_column_ref(&ColumnRef::Column(column_name), sql);
                    return;
                }
            }
        }
        self.prepare_iden(column, sql)
    }

    fn prepare_update_condition(
        &self,
        from: &[TableRef],
        condition: &ConditionHolder,
        sql: &mut impl SqlWriter,
    ) {
        if !from.is_empty() {
            return;
        }
        self.prepare_condition(condition, "WHERE", sql);
    }

    fn prepare_join_type(&self, join_type: &JoinType, sql: &mut impl SqlWriter) {
        match join_type {
            JoinType::FullOuterJoin => panic!("Mysql does not support FULL OUTER JOIN"),
            _ => self.prepare_join_type_common(join_type, sql),
        }
    }

    fn prepare_order_expr(&self, order_expr: &OrderExpr, sql: &mut impl SqlWriter) {
        match order_expr.nulls {
            None => (),
            Some(NullOrdering::Last) => {
                self.prepare_simple_expr(&order_expr.expr, sql);
                sql.write_str(" IS NULL ASC, ").unwrap()
            }
            Some(NullOrdering::First) => {
                self.prepare_simple_expr(&order_expr.expr, sql);
                sql.write_str(" IS NULL DESC, ").unwrap()
            }
        }
        if !matches!(order_expr.order, Order::Field(_)) {
            self.prepare_simple_expr(&order_expr.expr, sql);
        }
        self.prepare_order(order_expr, sql);
    }

    fn prepare_value(&self, value: Value, sql: &mut impl SqlWriter) {
        sql.push_param(value, self as _);
    }

    fn prepare_on_conflict_target(&self, _: &[OnConflictTarget], _: &mut impl SqlWriter) {
        // MySQL doesn't support declaring ON CONFLICT target.
    }

    fn prepare_on_conflict_action(
        &self,
        on_conflict_action: &Option<OnConflictAction>,
        sql: &mut impl SqlWriter,
    ) {
        match on_conflict_action {
            Some(OnConflictAction::DoNothing(pk_cols)) => {
                if !pk_cols.is_empty() {
                    self.prepare_on_conflict_do_update_keywords(sql);
                    let mut pk_cols_iter = pk_cols.iter();
                    join_io!(
                        pk_cols_iter,
                        pk_col,
                        join {
                            sql.write_str(", ").unwrap();
                        },
                        do {
                            self.prepare_iden(pk_col, sql);
                            sql.write_str(" = ").unwrap();
                            self.prepare_iden(pk_col, sql);
                        }
                    );
                } else {
                    sql.write_str(" IGNORE").unwrap();
                }
            }
            _ => self.prepare_on_conflict_action_common(on_conflict_action, sql),
        }
    }

    fn prepare_on_conflict_keywords(&self, sql: &mut impl SqlWriter) {
        sql.write_str(" ON DUPLICATE KEY").unwrap();
    }

    fn prepare_on_conflict_do_update_keywords(&self, sql: &mut impl SqlWriter) {
        sql.write_str(" UPDATE ").unwrap();
    }

    fn prepare_on_conflict_excluded_table(&self, col: &DynIden, sql: &mut impl SqlWriter) {
        sql.write_str("VALUES(").unwrap();
        self.prepare_iden(col, sql);
        sql.write_str(")").unwrap();
    }

    fn prepare_on_conflict_condition(&self, _: &ConditionHolder, _: &mut impl SqlWriter) {}

    fn prepare_returning(&self, _returning: &Option<ReturningClause>, _sql: &mut impl SqlWriter) {}

    fn random_function(&self) -> &str {
        "RAND"
    }

    fn insert_default_keyword(&self) -> &str {
        "()"
    }
}

impl MysqlQueryBuilder {
    fn prepare_index_hint_scope(&self, index_hint_scope: &IndexHintScope, sql: &mut impl SqlWriter) {
        match index_hint_scope {
            IndexHintScope::Join => {
                sql.write_str("FOR JOIN ").unwrap();
            }
            IndexHintScope::OrderBy => {
                sql.write_str("FOR ORDER BY ").unwrap();
            }
            IndexHintScope::GroupBy => {
                sql.write_str("FOR GROUP BY ").unwrap();
            }
            IndexHintScope::All => {}
        }
    }
}
