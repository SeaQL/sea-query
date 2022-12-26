use crate::*;

impl ViewBuilder for PostgresQueryBuilder {
    fn prepare_view_create_befor_view(
        &self,
        create: &ViewCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        if create.or_replace {
            write!(sql, " OR REPLACE").unwrap();
        }

        if create.temporary {
            write!(sql, " TEMPORARY").unwrap();
        }

        if create.recursive {
            write!(sql, " RECURSIVE").unwrap();
        }
    }

    fn prepare_view_rename_statement(&self, rename: &ViewRenameStatement, sql: &mut dyn SqlWriter) {
        write!(sql, "ALTER VIEW ").unwrap();
        if let Some(from_name) = &rename.from_name {
            from_name.prepare(sql.as_writer(), self.quote());
        }
        write!(sql, " TO ").unwrap();
        if let Some(to_name) = &rename.to_name {
            to_name.prepare(sql.as_writer(), self.quote());
        }
    }
}
