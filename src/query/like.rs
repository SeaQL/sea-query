use crate::{ConditionExpression, SimpleExpr, Value};

#[derive(Debug, Clone)]
pub struct LikeClause {
    pub(crate) pattern: Value,
    pub(crate) escape: Option<char>,
}

#[derive(Debug, Clone)]
pub struct LikeStatement {
    negative: bool,
    expr: SimpleExpr,
    clause: LikeClause,
}

impl LikeStatement {
    pub(crate) fn new(negative: bool, expr: SimpleExpr, pattern: &str) -> Self {
        Self {
            negative,
            expr,
            clause: LikeClause {
                pattern: Value::String(Some(Box::new(pattern.to_owned()))),
                escape: None,
            },
        }
    }

    pub fn escape(self, c: char) -> Self {
        Self {
            clause: LikeClause {
                pattern: self.clause.pattern,
                escape: Some(c),
            },
            ..self
        }
    }
}

impl Into<SimpleExpr> for LikeStatement {
    fn into(self) -> SimpleExpr {
        if self.negative {
            SimpleExpr::NotLike(Box::new(self.expr), self.clause)
        } else {
            SimpleExpr::Like(Box::new(self.expr), self.clause)
        }
    }
}

impl Into<ConditionExpression> for LikeStatement {
    fn into(self) -> ConditionExpression {
        ConditionExpression::SimpleExpr(self.into())
    }
}
