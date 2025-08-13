use super::*;
use crate::{
    CaseStatement, CommonTableExpression, Condition, ConditionExpression, ConditionHolder,
    ConditionHolderContents, Cycle, Expr, FunctionCall, LogicalChainOper, Search, SelectStatement,
    SubQueryStatement, TableRef, WithClause, WithQuery,
};
use std::collections::HashSet;

impl AuditTrait for SelectStatement {
    fn audit(&self) -> Result<QueryAccessAudit, Error> {
        Ok(wrap_result(self.audit_impl()?))
    }
}

impl AuditTrait for WithQuery {
    fn audit(&self) -> Result<QueryAccessAudit, Error> {
        Ok(wrap_result(self.audit_impl()?))
    }
}

impl AuditTrait for WithClause {
    fn audit(&self) -> Result<QueryAccessAudit, Error> {
        Ok(wrap_result(self.audit_impl()?))
    }
}

impl SelectStatement {
    fn audit_impl(&self) -> Result<Vec<QueryAccessRequest>, Error> {
        let mut walker = Walker::default();
        walker.recurse_audit_select(self)?;
        Ok(walker.access)
    }
}

impl WithQuery {
    fn audit_impl(&self) -> Result<Vec<QueryAccessRequest>, Error> {
        let mut walker = Walker::default();
        walker.recurse_audit_with(self)?;
        Ok(walker.access)
    }
}

impl WithClause {
    fn audit_impl(&self) -> Result<Vec<QueryAccessRequest>, Error> {
        let mut walker = Walker::default();
        walker.recurse_audit_with_clause(self)?;
        walker.recurse_audit_with_clause_cleanup(self);
        Ok(walker.access)
    }
}

#[derive(Default)]
pub(super) struct Walker {
    pub(super) access: Vec<QueryAccessRequest>,
}

impl Walker {
    fn recurse_audit_select(&mut self, select: &SelectStatement) -> Result<(), Error> {
        for select in &select.selects {
            self.recurse_audit_expr(&select.expr)?;
        }
        for table_ref in &select.from {
            self.recurse_audit_table(table_ref)?;
        }
        for join in &select.join {
            self.recurse_audit_table(&join.table)?;
        }
        for (_, select) in &select.unions {
            self.recurse_audit_select(select)?;
        }
        self.recurse_audit_condition_holder(&select.r#where)?;
        if let Some(with) = &select.with {
            self.recurse_audit_with_clause(with)?;
            self.recurse_audit_with_clause_cleanup(with);
        }
        Ok(())
    }

    pub(super) fn recurse_audit_table(&mut self, table_ref: &TableRef) -> Result<(), Error> {
        match table_ref {
            TableRef::SubQuery(select, _) => self.recurse_audit_select(select)?,
            TableRef::FunctionCall(function, _) => self.recurse_audit_function(function)?,
            TableRef::Table(table_name, _) => {
                self.access.push(QueryAccessRequest {
                    access_type: AccessType::Select,
                    schema_table: table_name.clone(),
                });
            }
            TableRef::ValuesList(_, _) => (),
        }
        Ok(())
    }

    fn recurse_audit_with(&mut self, with: &WithQuery) -> Result<(), Error> {
        self.recurse_audit_with_clause(&with.with_clause)?;
        if let Some(subquery) = &with.query {
            self.recurse_audit_subquery(subquery)?;
        }
        self.recurse_audit_with_clause_cleanup(&with.with_clause);
        Ok(())
    }

    fn recurse_audit_function(&mut self, function: &FunctionCall) -> Result<(), Error> {
        for arg in &function.args {
            self.recurse_audit_expr(arg)?;
        }
        Ok(())
    }

    fn recurse_audit_expr(&mut self, expr: &Expr) -> Result<(), Error> {
        match expr {
            Expr::Unary(_, expr) | Expr::AsEnum(_, expr) => self.recurse_audit_expr(expr)?,
            Expr::FunctionCall(function) => self.recurse_audit_function(function)?,
            Expr::Binary(left, _, right) => {
                self.recurse_audit_expr(left)?;
                self.recurse_audit_expr(right)?;
            }
            Expr::SubQuery(_, subquery) => self.recurse_audit_subquery(subquery)?,
            Expr::CustomWithExpr(_, exprs) | Expr::Tuple(exprs) => {
                for expr in exprs {
                    self.recurse_audit_expr(expr)?;
                }
            }
            Expr::Case(case) => self.recurse_audit_case(case)?,
            Expr::Value(_)
            | Expr::Column(_)
            | Expr::Values(_)
            | Expr::Custom(_)
            | Expr::Keyword(_)
            | Expr::Constant(_)
            | Expr::TypeName(_) => (),
        }
        Ok(())
    }

    fn recurse_audit_subquery(&mut self, subquery: &SubQueryStatement) -> Result<(), Error> {
        match subquery {
            SubQueryStatement::SelectStatement(select) => self.recurse_audit_select(select)?,
            SubQueryStatement::InsertStatement(insert) => {
                self.access.append(&mut insert.audit()?.requests);
            }
            SubQueryStatement::UpdateStatement(update) => {
                self.access.append(&mut update.audit()?.requests);
            }
            SubQueryStatement::DeleteStatement(delete) => {
                self.access.append(&mut delete.audit()?.requests);
            }
            SubQueryStatement::WithStatement(with) => self.recurse_audit_with(with)?,
        }
        Ok(())
    }

    pub(super) fn recurse_audit_with_clause(
        &mut self,
        with_clause: &WithClause,
    ) -> Result<(), Error> {
        if let Some(search) = &with_clause.search {
            self.recurse_audit_cte_search(search)?;
        }
        if let Some(cycle) = &with_clause.cycle {
            self.recurse_audit_cte_cycle(cycle)?;
        }
        for cte in &with_clause.cte_expressions {
            self.recurse_audit_cte_expr(cte)?;
        }
        Ok(())
    }

    pub(super) fn recurse_audit_with_clause_cleanup(&mut self, with_clause: &WithClause) {
        // remove cte alias
        for cte in &with_clause.cte_expressions {
            if let Some(table_name) = &cte.table_name {
                self.remove_item(AccessType::Select, &TableName(None, table_name.clone()));
            }
        }
    }

    fn recurse_audit_cte_search(&mut self, search: &Search) -> Result<(), Error> {
        if let Some(expr) = &search.expr {
            self.recurse_audit_expr(&expr.expr)?;
        }
        Ok(())
    }

    fn recurse_audit_cte_cycle(&mut self, cycle: &Cycle) -> Result<(), Error> {
        if let Some(expr) = &cycle.expr {
            self.recurse_audit_expr(expr)?;
        }
        Ok(())
    }

    fn recurse_audit_cte_expr(&mut self, cte: &CommonTableExpression) -> Result<(), Error> {
        if let Some(query) = &cte.query {
            self.recurse_audit_subquery(query)?;
        }
        Ok(())
    }

    fn recurse_audit_case(&mut self, case: &CaseStatement) -> Result<(), Error> {
        for when in &case.when {
            self.recurse_audit_condition(&when.condition)?;
            self.recurse_audit_expr(&when.result)?;
        }
        if let Some(expr) = &case.r#else {
            self.recurse_audit_expr(expr)?;
        }
        Ok(())
    }

    fn recurse_audit_condition_holder(&mut self, condition: &ConditionHolder) -> Result<(), Error> {
        match &condition.contents {
            ConditionHolderContents::Empty => (),
            ConditionHolderContents::Chain(chain) => {
                for oper in chain {
                    match oper {
                        LogicalChainOper::Or(expr) | LogicalChainOper::And(expr) => {
                            self.recurse_audit_expr(expr)?;
                        }
                    }
                }
            }
            ConditionHolderContents::Condition(condition) => {
                self.recurse_audit_condition(condition)?;
            }
        }
        Ok(())
    }

    fn recurse_audit_condition(&mut self, condition: &Condition) -> Result<(), Error> {
        for cond_expr in &condition.conditions {
            match cond_expr {
                ConditionExpression::Condition(condition) => {
                    self.recurse_audit_condition(condition)?;
                }
                ConditionExpression::Expr(expr) => self.recurse_audit_expr(expr)?,
            }
        }
        Ok(())
    }

    fn remove_item(&mut self, access_type: AccessType, target: &SchemaTable) {
        while let Some(pos) = self
            .access
            .iter()
            .position(|item| item.access_type == access_type && &item.schema_table == target)
        {
            self.access.remove(pos);
        }
    }
}

fn wrap_result(access: Vec<QueryAccessRequest>) -> QueryAccessAudit {
    let mut select_set = HashSet::new();
    let mut insert_set = HashSet::new();
    let mut update_set = HashSet::new();
    let mut delete_set = HashSet::new();
    QueryAccessAudit {
        requests: access
            .into_iter()
            .filter_map(|access| {
                let set = match access.access_type {
                    AccessType::Select => &mut select_set,
                    AccessType::Insert => &mut insert_set,
                    AccessType::Update => &mut update_set,
                    AccessType::Delete => &mut delete_set,
                    AccessType::Schema(_) => todo!(),
                };
                if set.contains(&access.schema_table) {
                    None
                } else {
                    set.insert(access.schema_table.clone());
                    Some(access)
                }
            })
            .collect(),
    }
}
