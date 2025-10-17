use crate::*;

pub trait IntoExpr: Into<Expr> {
    fn into_expr(self) -> Expr;
}

impl<T> IntoExpr for T
where
    T: Into<Expr>,
{
    fn into_expr(self) -> Expr {
        self.into()
    }
}

impl<T> From<T> for Expr
where
    T: Into<Value>,
{
    fn from(v: T) -> Self {
        Self::Value(v.into())
    }
}

impl From<Vec<Value>> for Expr {
    fn from(v: Vec<Value>) -> Self {
        Self::Values(v)
    }
}

impl From<SubQueryStatement> for Expr {
    fn from(v: SubQueryStatement) -> Self {
        Self::SubQuery(None, Box::new(v))
    }
}

macro_rules! from_into_subquery_expr {
    ($($ty:ty),+) => {
        $(
            impl From<$ty> for Expr {
                fn from(v: $ty) -> Self {
                    Self::SubQuery(None, Box::new(v.into()))
                }
            }
        )+
    };
}

from_into_subquery_expr!(
    WithQuery,
    DeleteStatement,
    UpdateStatement,
    InsertStatement,
    SelectStatement
);

impl From<FunctionCall> for Expr {
    fn from(func: FunctionCall) -> Self {
        Self::FunctionCall(func)
    }
}

impl From<ColumnRef> for Expr {
    fn from(col: ColumnRef) -> Self {
        Self::Column(col)
    }
}

impl From<Keyword> for Expr {
    fn from(k: Keyword) -> Self {
        Self::Keyword(k)
    }
}

impl From<LikeExpr> for Expr {
    fn from(like: LikeExpr) -> Self {
        match like.escape {
            Some(escape) => Self::Binary(
                Box::new(like.pattern.into()),
                BinOper::Escape,
                Box::new(Expr::Constant(escape.into())),
            ),
            None => like.pattern.into(),
        }
    }
}

impl From<TypeRef> for Expr {
    fn from(type_name: TypeRef) -> Self {
        Self::TypeName(type_name)
    }
}
