mod common;
mod delete;
mod insert;
mod select;
mod update;

use crate::{DynIden, TableName};

pub trait AuditTrait {
    fn audit(&self) -> Result<QueryAccessAudit, Error>;

    /// Shorthand for `audit().unwrap()`
    fn audit_unwrap(&self) -> QueryAccessAudit {
        self.audit().unwrap()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct QueryAccessAudit {
    pub requests: Vec<QueryAccessRequest>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct QueryAccessRequest {
    pub access_type: AccessType,
    /// Legacy naming, kept for compatibility. It should be `table_name`.
    ///
    /// The table name can be qualified as `(database.)(schema.)table`.
    pub schema_table: TableName,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AccessType {
    Select,
    Insert,
    Update,
    Delete,
    Schema(SchemaOper),
}

/// Schema Operation
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SchemaOper {
    Create,
    Alter,
    Drop,
    Rename,
    Truncate,
}

/// A table name, optionally qualified as `(database.)(schema.)table`.
///
/// This is a legacy type alias, to preserve some compatibility.
/// It's going to be deprecated in the future.
pub type SchemaTable = TableName;

impl QueryAccessAudit {
    /// This filters the selects from access requests.
    pub fn selects(&self) -> Vec<SchemaTable> {
        self.requests
            .iter()
            .filter_map(|item| {
                if item.access_type == AccessType::Select {
                    Some(item.schema_table.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Warning: this discards the schema part of [`SchemaTable`].
    /// Intended for testing only.
    pub fn selected_tables(&self) -> Vec<DynIden> {
        self.filter_table_with_access_type(AccessType::Select)
    }

    /// Warning: this discards the schema part of [`SchemaTable`].
    /// Intended for testing only.
    pub fn inserted_tables(&self) -> Vec<DynIden> {
        self.filter_table_with_access_type(AccessType::Insert)
    }

    /// Warning: this discards the schema part of [`SchemaTable`].
    /// Intended for testing only.
    pub fn updated_tables(&self) -> Vec<DynIden> {
        self.filter_table_with_access_type(AccessType::Update)
    }

    /// Warning: this discards the schema part of [`SchemaTable`].
    /// Intended for testing only.
    pub fn deleted_tables(&self) -> Vec<DynIden> {
        self.filter_table_with_access_type(AccessType::Delete)
    }

    fn filter_table_with_access_type(&self, access_type: AccessType) -> Vec<DynIden> {
        self.requests
            .iter()
            .filter_map(|item| {
                if item.access_type == access_type {
                    Some(item.schema_table.1.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Unable to parse query
    UnableToParseQuery,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnableToParseQuery => write!(f, "Unable to parse query"),
        }
    }
}
