use super::*;
use crate::UpdateStatement;

impl AuditTrait for UpdateStatement {
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
            access_type: AccessType::Update,
            schema_table,
        });

        for from in &self.from {
            let mut walker = select::Walker::default();
            walker.recurse_audit_table(from)?;
            requests.append(&mut walker.access);
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
