pub use expr::*;
pub use func::*;
pub use types::*;

use crate::types::BinOper;

pub(crate) mod expr;
pub(crate) mod func;
pub(crate) mod interval;
pub(crate) mod types;

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgBinOper {
    ILike,
    NotILike,
    Matches,
    Contains,
    Contained,
    Concatenate,
    Similarity,
    WordSimilarity,
    StrictWordSimilarity,
    SimilarityDistance,
    WordSimilarityDistance,
    StrictWordSimilarityDistance,
}

impl From<PgBinOper> for BinOper {
    fn from(o: PgBinOper) -> Self {
        Self::PgOperator(o)
    }
}
