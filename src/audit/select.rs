use super::*;
use crate::{
    CaseStatement, CommonTableExpression, Condition, ConditionExpression, ConditionHolder,
    ConditionHolderContents, Cycle, Expr, FunctionCall, LogicalChainOper, Search, SelectStatement,
    SubQueryStatement, TableRef, WithClause, WithQuery,
};

impl AuditTrait for SelectStatement {
    fn audit(&self) -> QueryAccessAudit {
        wrap_result(self.audit_impl())
    }
}

impl AuditTrait for WithQuery {
    fn audit(&self) -> QueryAccessAudit {
        wrap_result(self.audit_impl())
    }
}

impl SelectStatement {
    fn audit_impl(&self) -> Vec<SchemaTable> {
        let mut walker = Walker::default();
        walker.recurse_audit_select(self);
        walker.access
    }
}

impl WithQuery {
    fn audit_impl(&self) -> Vec<SchemaTable> {
        let mut walker = Walker::default();
        walker.recurse_audit_with(self);
        walker.access
    }
}

#[derive(Default)]
struct Walker {
    access: Vec<SchemaTable>,
}

impl Walker {
    fn recurse_audit_select(&mut self, select: &SelectStatement) {
        for select in &select.selects {
            self.recurse_audit_expr(&select.expr);
        }
        for table_ref in &select.from {
            self.recurse_audit_table(table_ref);
        }
        for join in &select.join {
            self.recurse_audit_table(&join.table);
        }
        for (_, select) in &select.unions {
            self.recurse_audit_select(select);
        }
        self.recurse_audit_condition_holder(&select.r#where);
        if let Some(with) = &select.with {
            self.recurse_audit_with_clause(with);
            self.recurse_audit_with_clause_cleanup(with);
        }
    }

    fn recurse_audit_table(&mut self, table_ref: &TableRef) {
        match table_ref {
            TableRef::SubQuery(select, _) => self.recurse_audit_select(select),
            TableRef::FunctionCall(function, _) => self.recurse_audit_function(function),
            TableRef::Table(tbl) | TableRef::TableAlias(tbl, _) => {
                self.access.push(SchemaTable(None, tbl.clone()))
            }
            TableRef::SchemaTable(sch, tbl)
            | TableRef::DatabaseSchemaTable(_, sch, tbl)
            | TableRef::SchemaTableAlias(sch, tbl, _)
            | TableRef::DatabaseSchemaTableAlias(_, sch, tbl, _) => {
                self.access
                    .push(SchemaTable(Some(sch.clone()), tbl.clone()));
            }
            TableRef::ValuesList(_, _) => (),
        }
    }

    fn recurse_audit_with(&mut self, with: &WithQuery) {
        self.recurse_audit_with_clause(&with.with_clause);
        if let Some(subquery) = &with.query {
            self.recurse_audit_subquery(subquery);
        }
        self.recurse_audit_with_clause_cleanup(&with.with_clause);
    }

    fn recurse_audit_function(&mut self, function: &FunctionCall) {
        for arg in &function.args {
            self.recurse_audit_expr(arg);
        }
    }

    fn recurse_audit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Column(_) => (),
            Expr::Unary(_, expr) | Expr::AsEnum(_, expr) => self.recurse_audit_expr(expr),
            Expr::FunctionCall(function) => self.recurse_audit_function(function),
            Expr::Binary(left, _, right) => {
                self.recurse_audit_expr(left);
                self.recurse_audit_expr(right);
            }
            Expr::SubQuery(_, subquery) => self.recurse_audit_subquery(subquery),
            Expr::Value(_) => (),
            Expr::Values(_) => (),
            Expr::Custom(_) => (),
            Expr::CustomWithExpr(_, exprs) | Expr::Tuple(exprs) => {
                for expr in exprs {
                    self.recurse_audit_expr(expr);
                }
            }
            Expr::Keyword(_) => (),
            Expr::Case(case) => self.recurse_audit_case(case),
            Expr::Constant(_) => (),
        }
    }

    fn recurse_audit_subquery(&mut self, subquery: &SubQueryStatement) {
        match subquery {
            SubQueryStatement::SelectStatement(select) => self.recurse_audit_select(select),
            SubQueryStatement::WithStatement(with) => self.recurse_audit_with(with),
            _ => (),
        }
    }

    fn recurse_audit_with_clause(&mut self, with_clause: &WithClause) {
        if let Some(search) = &with_clause.search {
            self.recurse_audit_cte_search(search);
        }
        if let Some(cycle) = &with_clause.cycle {
            self.recurse_audit_cte_cycle(cycle);
        }
        for cte in &with_clause.cte_expressions {
            self.recurse_audit_cte_expr(cte);
        }
    }

    fn recurse_audit_with_clause_cleanup(&mut self, with_clause: &WithClause) {
        // remove cte alias
        for cte in &with_clause.cte_expressions {
            if let Some(table_name) = &cte.table_name {
                self.remove_item(&SchemaTable(None, table_name.clone()));
            }
        }
    }

    fn recurse_audit_cte_search(&mut self, search: &Search) {
        if let Some(expr) = &search.expr {
            self.recurse_audit_expr(&expr.expr);
        }
    }

    fn recurse_audit_cte_cycle(&mut self, cycle: &Cycle) {
        if let Some(expr) = &cycle.expr {
            self.recurse_audit_expr(expr);
        }
    }

    fn recurse_audit_cte_expr(&mut self, cte: &CommonTableExpression) {
        if let Some(query) = &cte.query {
            self.recurse_audit_subquery(query);
        }
    }

    fn recurse_audit_case(&mut self, case: &CaseStatement) {
        for when in &case.when {
            self.recurse_audit_condition(&when.condition);
            self.recurse_audit_expr(&when.result);
        }
        if let Some(expr) = &case.r#else {
            self.recurse_audit_expr(expr);
        }
    }

    fn recurse_audit_condition_holder(&mut self, condition: &ConditionHolder) {
        match &condition.contents {
            ConditionHolderContents::Empty => (),
            ConditionHolderContents::Chain(chain) => {
                for oper in chain {
                    match oper {
                        LogicalChainOper::And(expr) => self.recurse_audit_expr(expr),
                        LogicalChainOper::Or(expr) => self.recurse_audit_expr(expr),
                    }
                }
            }
            ConditionHolderContents::Condition(condition) => {
                self.recurse_audit_condition(condition)
            }
        }
    }

    fn recurse_audit_condition(&mut self, condition: &Condition) {
        for cond_expr in &condition.conditions {
            match cond_expr {
                ConditionExpression::Condition(condition) => {
                    self.recurse_audit_condition(condition)
                }
                ConditionExpression::Expr(expr) => self.recurse_audit_expr(expr),
            }
        }
    }

    fn remove_item(&mut self, target: &SchemaTable) {
        while let Some(pos) = self.access.iter().position(|item| item == target) {
            self.access.remove(pos);
        }
    }
}

fn wrap_result(access: Vec<SchemaTable>) -> QueryAccessAudit {
    QueryAccessAudit {
        requests: access
            .into_iter()
            .map(|schema_table| QueryAccessRequest {
                access_type: AccessType::Select,
                schema_table,
            })
            .collect(),
    }
}
