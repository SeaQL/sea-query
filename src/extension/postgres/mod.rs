pub use expr::*;
pub use extension::*;
pub use func::*;
pub use ltree::*;
pub use select::*;
pub use types::*;

use crate::types::BinOper;

pub(crate) mod expr;
pub(crate) mod extension;
pub(crate) mod func;
pub(crate) mod interval;
pub(crate) mod ltree;
pub(crate) mod select;
pub(crate) mod types;

/// Postgres-specific binary operators.
///
/// For all supported operators (including the standard ones), see [`BinOper`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PgBinOper {
    ILike,
    NotILike,
    /// `@@`. Full-text search match
    Matches,
    /// `@>`. Contains operator - checks if left operand contains right operand (arrays, JSON)
    Contains,
    /// `<@`. Contained operator - checks if left operand is contained by right operand (arrays, JSON)
    Contained,
    /// `||`. String/Array concatenation operator
    Concatenate,
    /// `&&`. Overlap operator - checks if arrays have any elements in common
    Overlap,
    /// `%`. Text similarity operator,
    /// requires `pg_trgm` extension
    Similarity,
    /// `<%`. Word similarity operator,
    /// requires `pg_trgm` extension
    WordSimilarity,
    /// `<<%`. Strict word similarity operator,
    /// requires `pg_trgm` extension
    StrictWordSimilarity,
    /// `<->`. Similarity distance operato
    /// requires `pg_trgm` extension
    SimilarityDistance,
    /// `<<->`. Word similarity distance operator
    /// requires `pg_trgm` extension
    WordSimilarityDistance,
    /// `<<<->`. Strict word similarity distance operator
    /// requires `pg_trgm` extension
    StrictWordSimilarityDistance,
    /// `->`. Retrieves JSON field as JSON value
    GetJsonField,
    /// `->>`. Retrieves JSON field and casts it to text
    CastJsonField,
    /// `~`. Regex operator, case sensitively
    Regex,
    /// `~*`. Regex operator, case-insensitively
    RegexCaseInsensitive,
    #[cfg(feature = "postgres-vector")]
    /// `<->`. L2 (Euclidean) distance operator
    EuclideanDistance,
    #[cfg(feature = "postgres-vector")]
    /// `<#>`. Negative inner product operator
    NegativeInnerProduct,
    #[cfg(feature = "postgres-vector")]
    /// `<=>`. Cosine distance operator
    CosineDistance,
}

impl From<PgBinOper> for BinOper {
    fn from(o: PgBinOper) -> Self {
        Self::PgOperator(o)
    }
}
