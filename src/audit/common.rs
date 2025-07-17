use super::*;
use crate::TableRef;

pub(super) fn parse_audit_table(table_ref: &TableRef) -> Option<SchemaTable> {
    match table_ref {
        TableRef::SubQuery(_, _) => None,
        TableRef::FunctionCall(_, _) => None,
        TableRef::Table(tbl) | TableRef::TableAlias(tbl, _) => Some(SchemaTable(None, tbl.clone())),
        TableRef::SchemaTable(sch, tbl)
        | TableRef::DatabaseSchemaTable(_, sch, tbl)
        | TableRef::SchemaTableAlias(sch, tbl, _)
        | TableRef::DatabaseSchemaTableAlias(_, sch, tbl, _) => {
            Some(SchemaTable(Some(sch.clone()), tbl.clone()))
        }
        TableRef::ValuesList(_, _) => None,
    }
}
