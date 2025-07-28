use crate::Expr;

#[derive(Debug, Clone)]
pub enum Check {
    Named(&'static str, Expr),
    Unnamed(Expr),
}

impl Check {
    pub fn expr(&self) -> &Expr {
        match self {
            Check::Named(_, expr) => expr,
            Check::Unnamed(expr) => expr,
        }
    }
}
