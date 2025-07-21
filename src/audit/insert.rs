use super::*;
use crate::{InsertStatement, InsertValueSource};

impl AuditTrait for InsertStatement {
    fn audit(&self) -> Result<QueryAccessAudit, Error> {
        let mut requests = Vec::new();
        let Some(table) = &self.table else {
            return Err(Error::UnableToParseQuery);
        };
        let Some(schema_table) = common::parse_audit_table(table) else {
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
