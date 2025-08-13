use super::*;
use crate::TableRef;

pub(super) fn parse_audit_table(table_ref: &TableRef) -> Option<SchemaTable> {
    match table_ref {
        TableRef::Table(tbl, _) => Some(tbl.clone()),
        TableRef::ValuesList(_, _) | TableRef::FunctionCall(_, _) | TableRef::SubQuery(_, _) => {
            None
        }
    }
}
