use super::{Func, FunctionCall};
use crate::{Expr, WindowSelectType};

#[derive(Debug, Clone, PartialEq)]
pub struct WindowFunctionExpr {
    pub(crate) function: FunctionCall,
    pub(crate) window: WindowSelectType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowFunctionCall(FunctionCall);

impl From<FunctionCall> for WindowFunctionCall {
    fn from(call: FunctionCall) -> Self {
        Self(call)
    }
}

impl WindowFunctionCall {
    /// Set the window.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let window_name = "by_font_size";
    /// let query = Query::select()
    ///     .expr(Func::lag(Expr::col(Char::SizeW)).over(window_name))
    ///     .from(Char::Table)
    ///     .window(
    ///         window_name,
    ///         WindowStatement::partition_by(Char::FontSize)
    ///             .order_by(Char::Id, Order::Asc)
    ///             .take(),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT LAG(`size_w`) OVER `by_font_size` FROM `character` WINDOW `by_font_size` AS (PARTITION BY `font_size` ORDER BY `id` ASC)"#
    /// );
    /// ```
    pub fn over(self, window: impl Into<WindowSelectType>) -> Expr {
        Expr::WindowFunction(Box::new(WindowFunctionExpr {
            function: self.0,
            window: window.into(),
        }))
    }
}

impl Func {
    pub fn row_number() -> WindowFunctionCall {
        FunctionCall::new(Func::RowNumber).into()
    }

    pub fn rank() -> WindowFunctionCall {
        FunctionCall::new(Func::Rank).into()
    }

    pub fn dense_rank() -> WindowFunctionCall {
        FunctionCall::new(Func::DenseRank).into()
    }

    pub fn lead(expr: impl Into<Expr>) -> WindowFunctionCall {
        FunctionCall::new(Func::Lead).arg(expr).into()
    }

    pub fn lag(expr: impl Into<Expr>) -> WindowFunctionCall {
        FunctionCall::new(Func::Lag).arg(expr).into()
    }

    pub fn first_value(expr: impl Into<Expr>) -> WindowFunctionCall {
        FunctionCall::new(Func::FirstValue).arg(expr).into()
    }

    pub fn last_value(expr: impl Into<Expr>) -> WindowFunctionCall {
        FunctionCall::new(Func::LastValue).arg(expr).into()
    }
}
