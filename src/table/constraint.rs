use crate::{DynIden, Expr};

#[derive(Debug, Clone)]
pub struct Check {
    pub(crate) name: Option<DynIden>,
    pub(crate) expr: Expr,
}

impl Check {
    pub fn named<N, E>(name: N, expr: E) -> Self
    where
        N: Into<DynIden>,
        E: Into<Expr>,
    {
        Self {
            name: Some(name.into()),
            expr: expr.into(),
        }
    }

    pub fn unnamed<E>(expr: E) -> Self
    where
        E: Into<Expr>,
    {
        Self {
            name: None,
            expr: expr.into(),
        }
    }
}

impl<E> From<E> for Check
where
    E: Into<Expr>,
{
    fn from(expr: E) -> Self {
        Self::unnamed(expr)
    }
}

impl<I, E> From<(I, E)> for Check
where
    I: Into<DynIden>,
    E: Into<Expr>,
{
    fn from((name, expr): (I, E)) -> Self {
        Self::named(name, expr)
    }
}
