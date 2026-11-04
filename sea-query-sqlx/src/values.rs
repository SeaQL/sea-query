#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct SqlxValues(pub sea_query::Values);

/// An error converting SeaQuery values into SQLx arguments.
#[derive(Debug)]
pub enum IntoArgumentsError {
    /// A value could not be encoded for the selected backend.
    Encode(Box<dyn std::error::Error + Send + Sync>),
    /// The selected backend does not support this value type.
    Unsupported(&'static str),
    /// An array contains a value of the wrong type.
    InvalidArray {
        element_type: &'static str,
        source: sea_query::ValueTypeErr,
    },
}

impl std::fmt::Display for IntoArgumentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encode(source) => write!(f, "error occurred while encoding a value: {source}"),
            Self::Unsupported(message) => f.write_str(message),
            Self::InvalidArray { element_type, .. } => {
                write!(
                    f,
                    "This Value::Array should consist of Value::{element_type}"
                )
            }
        }
    }
}

impl std::error::Error for IntoArgumentsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Encode(source) => Some(source.as_ref()),
            Self::Unsupported(_) => None,
            Self::InvalidArray { source, .. } => Some(source),
        }
    }
}
