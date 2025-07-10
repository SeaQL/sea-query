mod select;

use crate::DynIden;

pub trait AuditTrait {
    fn audit(&self) -> QueryAccessAudit;
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
}

#[derive(Debug, Clone, PartialEq)]
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
}
