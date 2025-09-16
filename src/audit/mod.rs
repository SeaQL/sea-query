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

impl AccessType {
    pub fn as_str(at: &AccessType) -> &'static str {
        match at {
            AccessType::Select => "select",
            AccessType::Insert => "insert",
            AccessType::Update => "update",
            AccessType::Delete => "delete",
            AccessType::Schema(SchemaOper::Create) => "schema_create",
            AccessType::Schema(SchemaOper::Alter) => "schema_alter",
            AccessType::Schema(SchemaOper::Drop) => "schema_drop",
            AccessType::Schema(SchemaOper::Rename) => "schema_rename",
            AccessType::Schema(SchemaOper::Truncate) => "schema_truncate",
        }
    }
}

impl QueryAccessAudit {
    /// This filters the selects from access requests.
    pub fn selects(&self) -> Vec<TableName> {
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

    /// Warning: this discards the schema part of TableName.
    /// Intended for testing only.
    pub fn selected_tables(&self) -> Vec<DynIden> {
        self.filter_table_with_access_type(AccessType::Select)
    }

    /// Warning: this discards the schema part of TableName.
    /// Intended for testing only.
    pub fn inserted_tables(&self) -> Vec<DynIden> {
        self.filter_table_with_access_type(AccessType::Insert)
    }

    /// Warning: this discards the schema part of TableName.
    /// Intended for testing only.
    pub fn updated_tables(&self) -> Vec<DynIden> {
        self.filter_table_with_access_type(AccessType::Update)
    }

    /// Warning: this discards the schema part of TableName.
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
    /// Unsupported query
    UnsupportedQuery,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnableToParseQuery => f.write_str("Unable to parse query"),
            Self::UnsupportedQuery => f.write_str("Unsupported query"),
        }
    }
}
