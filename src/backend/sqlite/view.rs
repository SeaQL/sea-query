use crate::*;

impl ViewBuilder for SqliteQueryBuilder {
    fn prepare_view_create_befor_view(&self, create: &ViewCreateStatement, sql: &mut SqlWriter) {
        if create.temporary {
            write!(sql, " TEMPORARY").unwrap();
        }
    }

    fn prepare_view_create_after_view(&self, create: &ViewCreateStatement, sql: &mut SqlWriter) {
        if create.if_not_exists {
            write!(sql, " IF NOT EXISTS").unwrap();
        }
    }

    fn prepare_view_create_opt_update(&self, _opt: &ViewCreateOpt, _sql: &mut SqlWriter) {
        // SQLite does not support view create options
    }

    fn prepare_view_rename_statement(&self, _rename: &ViewRenameStatement, _sql: &mut SqlWriter) {
        // SQLite does not support rename view
    }

    /// Translate [`ViewDropStatement`] into SQL statement.
    fn prepare_view_drop_statement(&self, drop: &ViewDropStatement, sql: &mut SqlWriter) {
        write!(sql, "DROP VIEW ").unwrap();

        if drop.if_exists {
            write!(sql, "IF EXISTS ").unwrap();
        }
        self.prepare_view_ref(&drop.views[0], sql);
    }

    fn prepare_view_drop_opt(&self, _drop_opt: &ViewDropOpt, _sql: &mut dyn std::fmt::Write) {
        // SQLite does not support view drop options
    }
}
