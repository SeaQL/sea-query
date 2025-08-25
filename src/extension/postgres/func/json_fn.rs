use core::fmt;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct QuotesClause {
    pub(crate) kind: QuotesKind,
    pub(crate) on_scalar_string: bool,
}

impl QuotesClause {
    pub(crate) fn prepare(&self, buf: &mut String) -> fmt::Result {
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
