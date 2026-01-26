#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PgExplainSerialize {
    None,
    Text,
    Binary,
}

impl PgExplainSerialize {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            PgExplainSerialize::None => "NONE",
            PgExplainSerialize::Text => "TEXT",
            PgExplainSerialize::Binary => "BINARY",
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct PgExplainOptions {
    pub(crate) verbose: Option<bool>,
    pub(crate) costs: Option<bool>,
    pub(crate) settings: Option<bool>,
    pub(crate) generic_plan: Option<bool>,
    pub(crate) buffers: Option<bool>,
    pub(crate) serialize: Option<PgExplainSerialize>,
    pub(crate) wal: Option<bool>,
    pub(crate) timing: Option<bool>,
    pub(crate) summary: Option<bool>,
    pub(crate) memory: Option<bool>,
}
