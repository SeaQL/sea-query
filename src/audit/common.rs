use super::*;
use crate::TableRef;

pub(super) fn parse_audit_table(table_ref: &TableRef) -> Option<SchemaTable> {
    match table_ref {
        TableRef::SubQuery(_, _) => None,
        TableRef::FunctionCall(_, _) => None,
        TableRef::Table(tbl, _) => Some(tbl.clone()),
        TableRef::ValuesList(_, _) => None,
    }
}
