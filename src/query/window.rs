use crate::{expr::*, query::*, types::*};

pub trait OverStatement {
    #[doc(hidden)]
    // Implementation for the trait.
    fn add_partition_by(&mut self, partition: SimpleExpr) -> &mut Self;

    /// Partition by column.
    fn partition_by<T>(&mut self, col: T) -> &mut Self
    where
        T: IntoColumnRef,
    {
        self.add_partition_by(SimpleExpr::Column(col.into_column_ref()))
    }

    /// Partition by custom string.
    fn partition_by_customs<T>(&mut self, cols: Vec<T>) -> &mut Self
    where
        T: ToString,
    {
        cols.into_iter().for_each(|c| {
            self.add_partition_by(SimpleExpr::Custom(c.to_string()));
        });
        self
    }

    /// Partition by vector of columns.
    fn partition_by_columns<T>(&mut self, cols: Vec<T>) -> &mut Self
    where
        T: IntoColumnRef,
    {
        cols.into_iter().for_each(|c| {
            self.add_partition_by(SimpleExpr::Column(c.into_column_ref()));
        });
        self
    }
}

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

impl WindowStatement {
    /// Construct a new [`WindowStatement`] by [`SimpleExpr`]
    pub fn expr(exp: SimpleExpr) -> Self {
        Self {
            partition_by: (exp, Vec::new()),
            order_by: Vec::new(),
            frame: None,
        }
    }

    /// Construct a new [`WindowStatement`] by column name
    pub fn column<T>(col: T) -> Self
    where
        T: IntoColumnRef,
    {
        Self::expr(SimpleExpr::Column(col.into_column_ref()))
    }

    /// Construct a new [`WindowStatement`] by custom
    pub fn custom<T>(col: T) -> Self
    where
        T: ToString,
    {
        Self::expr(SimpleExpr::Custom(col.to_string()))
    }

    /// frame_start
    pub fn frame_start(&mut self, r#type: FrameType, start: Frame) -> &mut Self {
        self.frame(r#type, start, None)
    }

    /// BETWEEN frame_start AND frame_end
    pub fn frame_between(&mut self, r#type: FrameType, start: Frame, end: Frame) -> &mut Self {
        self.frame(r#type, start, Some(end))
    }

    ///
    pub fn frame(&mut self, r#type: FrameType, start: Frame, end: Option<Frame>) -> &mut Self {
        let frame_clause = FrameClause { r#type, start, end };
        self.frame = Some(frame_clause);
        self
    }
}

impl OverStatement for WindowStatement {
    fn add_partition_by(&mut self, partition: SimpleExpr) -> &mut Self {
        self.partition_by.1.push(partition);
        self
    }
}

impl OrderedStatement for WindowStatement {
    fn add_order_by(&mut self, order: OrderExpr) -> &mut Self {
        self.order_by.push(order);
        self
    }
}
