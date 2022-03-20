use crate::{expr::*, query::*, types::*};

/// frame_start or frame_end clause
#[derive(Debug, Clone)]
pub enum Frame {
    UboundedPreceding,
    Preceding(u32),
    CurrentRow,
    Following(u32),
    UnboundedFollowing,
}

/// Frame type
#[derive(Debug, Clone)]
pub enum FrameType {
    Range,
    Rows,
}

/// Frame clause
#[derive(Debug, Clone)]
pub struct FrameClause {
    pub(crate) r#type: FrameType,
    pub(crate) start: Frame,
    pub(crate) end: Option<Frame>,
}


/// Window expression
#[derive(Debug, Clone)]
pub struct WindowStatement {
    pub(crate) partition_by: (SimpleExpr, Vec<SimpleExpr>),
    pub(crate) order_by: Vec<OrderExpr>,
    pub(crate) frame: Option<FrameClause>,
}

impl OrderedStatement for WindowStatement {
    fn add_order_by(&mut self, order: OrderExpr) -> &mut Self {
        self.order_by.push(order);
        self
    }
}

impl WindowStatement {
    pub fn new(exp: SimpleExpr) -> Self
    {
        Self {
            partition_by: (exp, Vec::new()),
            order_by: Vec::new(),
            frame: None,
        }
    }

    pub fn partition_by<T>(col: T) -> Self
        where
            T: IntoColumnRef,
    {
        Self::new(SimpleExpr::Column(col.into_column_ref()))
    }

    pub fn partition_by_custom<T>(col: T) -> Self
        where
            T: ToString,
    {
        Self::new(SimpleExpr::Custom(col.to_string()))
    }

    pub fn add_partition_by(&mut self, partition: SimpleExpr) -> &mut Self {
        self.partition_by.1.push(partition);
        self
    }

}
