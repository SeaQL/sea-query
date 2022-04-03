use crate::*;

pub trait ViewBuilder: QueryBuilder + QuotedBuilder {
    fn prepare_view_create_statement(&self, create: &ViewCreateStatement, sql: &mut SqlWriter) {}

    fn prepare_view_drop_statement(&self, drop: &ViewDropStatement, sql: &mut SqlWriter) {}

    fn prepare_view_rename_statement(&self, rename: &ViewRenameStatement, sql: &mut SqlWriter) {}
}
