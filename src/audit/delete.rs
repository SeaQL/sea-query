use super::*;
use crate::DeleteStatement;

impl AuditTrait for DeleteStatement {
    fn audit(&self) -> Result<QueryAccessAudit, Error> {
        let mut requests = Vec::new();
        let Some(table) = &self.table else {
            return Err(Error::UnableToParseQuery);
        };
        let Some(table_name) = common::parse_audit_table(table) else {
            return Err(Error::UnableToParseQuery);
        };
        if self.returning.is_some() {
            requests.push(QueryAccessRequest {
                access_type: AccessType::Select,
                table_name: table_name.clone(),
            });
        }
        requests.push(QueryAccessRequest {
            access_type: AccessType::Delete,
            table_name,
        });

        if let Some(with) = &self.with {
            let mut walker = select::Walker { access: requests };
            walker.recurse_audit_with_clause(with)?;
            walker.recurse_audit_with_clause_cleanup(with);
            requests = walker.access;
        }

        Ok(QueryAccessAudit { requests })
    }
}
