use super::*;
use crate::{InsertStatement, InsertValueSource, TableRef};

impl AuditTrait for InsertStatement {
    fn audit(&self) -> Result<QueryAccessAudit, Error> {
        let mut requests = Vec::new();
        let Some(table) = &self.table else {
            return Err(Error::UnableToParseQuery);
        };
        let Some(schema_table) = parse_audit_table(table) else {
            return Err(Error::UnableToParseQuery);
        };
        if self.returning.is_some() {
            requests.push(QueryAccessRequest {
                access_type: AccessType::Select,
                schema_table: schema_table.clone(),
            });
        }
        requests.push(QueryAccessRequest {
            access_type: AccessType::Insert,
            schema_table,
        });
        if let Some(InsertValueSource::Select(select)) = &self.source {
            requests.append(&mut select.audit()?.requests)
        }
        if let Some(with) = &self.with {
            let mut walker = select::Walker { access: requests };
            walker.recurse_audit_with_clause(with)?;
            walker.recurse_audit_with_clause_cleanup(with);
            requests = walker.access;
        }
        Ok(QueryAccessAudit { requests })
    }
}

fn parse_audit_table(table_ref: &TableRef) -> Option<SchemaTable> {
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
