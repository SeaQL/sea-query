pub(crate) mod func;
pub(crate) mod interval;
pub(crate) mod types;

use crate::BinOper;
pub use func::*;
pub use interval::*;
pub use types::*;

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
        Self::PgOperators(o)
    }
}
