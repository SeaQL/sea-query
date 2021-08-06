//! Error types used in sea-query.

/// Result type for sea-query
pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// Column and value vector having different length
    #[error("Columns and values length mismatch: {col_len} != {val_len}")]
    ColValNumMismatch { col_len: usize, val_len: usize },
}
