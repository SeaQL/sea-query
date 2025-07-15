mod common;
mod insert;
mod select;
mod update;

use crate::DynIden;

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
    pub schema_table: SchemaTable,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemaTable(pub Option<DynIden>, pub DynIden);

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

    /// Warning: this discards the schema part of SchemaTable.
    /// Intended for testing only.
    pub fn selected_tables(&self) -> Vec<DynIden> {
        self.requests
            .iter()
            .filter_map(|item| {
                if item.access_type == AccessType::Select {
                    Some(item.schema_table.1.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Warning: this discards the schema part of SchemaTable.
    /// Intended for testing only.
    pub fn inserted_tables(&self) -> Vec<DynIden> {
        self.requests
            .iter()
            .filter_map(|item| {
                if item.access_type == AccessType::Insert {
                    Some(item.schema_table.1.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn updated_tables(&self) -> Vec<DynIden> {
        self.requests
            .iter()
            .filter_map(|item| {
                if item.access_type == AccessType::Update {
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
