use crate::{ConditionExpression, Expr, SimpleExpr};

#[derive(Debug, Clone)]
pub(crate) struct CaseStatementCondition {
    condition: Option<ConditionExpression>,
    result: Expr,
}

pub(crate) struct CaseStatement {
    pub(crate) conditions: Vec<CaseStatementCondition>,
}

impl CaseStatement {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    pub fn case<C, E>(mut self, when: C, then: E) -> Self
    where
        C: Into<ConditionExpression>,
        E: Into<Expr>,
    {
        self.conditions.push(CaseStatementCondition {
            condition: Some(when.into()),
            result: then.into(),
        });
        self
    }

    pub fn finally<E>(mut self, r#else: E) -> Self
    where
        E: Into<Expr>,
    {
        self.conditions.push(CaseStatementCondition {
            condition: None,
            result: r#else.into(),
        });
        self
    }

    pub fn then<E>(mut self, result: E) -> Self
    where
        E: Into<Expr>,
    {
        (*self
            .conditions
            .last_mut()
            .expect("`then` cannot be called before loading conditions on case statements."))
        .result = result.into();
        self
    }
}

impl From<CaseStatement> for SimpleExpr {
    fn from(val: CaseStatement) -> Self {
        let r = val.conditions.into_iter().map(|x| (x.condition.map_or(None, |x| Some(x.into())), x.result)).collect();
        SimpleExpr::Case()
    }
}