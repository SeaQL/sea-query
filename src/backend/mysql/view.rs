use crate::*;

impl ViewBuilder for MysqlQueryBuilder {
    fn prepare_view_rename_statement(&self, rename: &ViewRenameStatement, sql: &mut dyn SqlWriter) {
        write!(sql, "RENAME TABLE ").unwrap();
        if let Some(from_name) = &rename.from_name {
            from_name.prepare(sql.as_writer(), self.quote());
        }
        write!(sql, " TO ").unwrap();
        if let Some(to_name) = &rename.to_name {
            to_name.prepare(sql.as_writer(), self.quote());
        }
    }
}
