use crate::*;

impl ViewBuilder for SqliteQueryBuilder {
    fn prepare_view_rename_statement(&self, rename: &ViewRenameStatement, sql: &mut SqlWriter) {}
}
