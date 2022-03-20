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

/// Over expression
#[derive(Debug, Clone)]
pub struct FrameStatement {
    pub(crate) r#type: FrameType,
    pub(crate) start: Frame,
    pub(crate) end: Option<Frame>,
}

pub trait PartitionStatement {
    fn add_partition_by(&mut self, col: SimpleExpr) -> &mut Self;

    fn partition_by<T>(&mut self, col: T) -> &mut Self
    where
        T: IntoColumnRef,
    {
        self.add_partition_by(SimpleExpr::Column(col.into_column_ref()));
        self
    }

    fn partition_by_columns<T>(&mut self, cols: Vec<T>) -> &mut Self
    where
        T: IntoColumnRef,
    {
        cols.into_iter().for_each(|c| {
            self.add_partition_by(SimpleExpr::Column(c.into_column_ref()));
        });
        self
    }

    fn partition_by_customs<T>(&mut self, cols: Vec<T>) -> &mut Self
    where
        T: ToString,
    {
        cols.into_iter().for_each(|c| {
            self.add_partition_by(SimpleExpr::Custom(c.to_string()));
        });
        self
    }
}

#[derive(Debug, Clone)]
pub struct WindowStatement {
    pub(crate) partition_by: Vec<SimpleExpr>,
    pub(crate) order_by: Vec<OrderExpr>,
    pub(crate) frame: Option<FrameStatement>,
}

impl OrderedStatement for WindowStatement {
    fn add_order_by(&mut self, order: OrderExpr) -> &mut Self {
        self.order_by.push(order);
        self
    }
}

impl PartitionStatement for WindowStatement {
    fn add_partition_by(&mut self, partition: SimpleExpr) -> &mut Self {
        self.partition_by.push(partition);
        self
    }
}
