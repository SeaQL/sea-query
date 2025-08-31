use core::fmt;
use std::{borrow::Cow, fmt::Write};

use crate::{PostgresQueryBuilder, QueryBuilder, Value, join_io};

#[derive(Debug, Clone)]
pub struct QuotesClause {
    pub(crate) kind: QuotesKind,
    pub(crate) on_scalar_string: bool,
}

impl QuotesClause {
    pub(crate) fn write_to(&self, buf: &mut String) -> fmt::Result {
        match self.kind {
            QuotesKind::Keep => buf.write_str(" KEEP QUOTES")?,
            QuotesKind::Omit => buf.write_str(" OMIT QUOTES")?,
        }
        if self.on_scalar_string {
            buf.write_str(" ON SCALAR STRING")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum QuotesKind {
    Keep,
    Omit,
}

impl QuotesKind {
    pub fn on_scalar_string(self) -> QuotesClause {
        QuotesClause {
            kind: self,
            on_scalar_string: true,
        }
    }
}

impl From<QuotesKind> for QuotesClause {
    fn from(kind: QuotesKind) -> Self {
        QuotesClause {
            kind,
            on_scalar_string: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WrapperClause {
    pub(crate) kind: WrapperKind,
    pub(crate) array: bool,
}

impl WrapperClause {
    pub(crate) fn write_to(&self, buf: &mut String) -> fmt::Result {
        match self.kind {
            WrapperKind::Without => buf.write_str(" WITHOUT")?,
            WrapperKind::WithConditional => buf.write_str(" WITH CONDITIONAL")?,
            WrapperKind::WithUnconditional => buf.write_str(" WITH UNCONDITIONAL")?,
        }
        if self.array {
            buf.write_str(" ARRAY")?;
        }
        buf.write_str(" WRAPPER")
    }
}

#[derive(Debug, Clone)]
pub enum WrapperKind {
    Without,
    WithConditional,
    WithUnconditional,
}

impl WrapperKind {
    pub fn array(self) -> WrapperClause {
        WrapperClause {
            kind: self,
            array: true,
        }
    }
}

impl From<WrapperKind> for WrapperClause {
    fn from(kind: WrapperKind) -> Self {
        WrapperClause { kind, array: false }
    }
}

pub(super) fn write_json_path_expr(buf: &mut String, expr: &str) -> fmt::Result {
    buf.write_str("'")?;
    buf.write_str(expr)?;
    buf.write_str("'")?;
    Ok(())
}

pub(super) fn write_as_json_path_name(buf: &mut String, name: &str) -> fmt::Result {
    buf.write_str(" AS ")?;
    buf.write_str(name)?;
    Ok(())
}

pub(super) fn write_passing(
    buf: &mut String,
    passing: Vec<(Value, Cow<'static, str>)>,
) -> fmt::Result {
    let mut piter = passing.into_iter();
    join_io!(
        piter,
        value_as,
        first {
            buf.write_str(" PASSING ")?;
        },
        join {
            buf.write_str(", ")?;
        },
        do {
            PostgresQueryBuilder.prepare_value(value_as.0, buf);
            buf.write_str(" AS ")?;
            buf.write_str(&value_as.1)?;
        }
    );

    Ok(())
}
