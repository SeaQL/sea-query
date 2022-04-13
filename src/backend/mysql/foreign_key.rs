use super::*;

impl ForeignKeyBuilder for MysqlQueryBuilder {
    fn prepare_foreign_key_drop_statement_internal(
        &self,
        drop: &ForeignKeyDropStatement,
        sql: &mut SqlWriter,
        mode: Mode,
    ) {
        if mode == Mode::Alter {
            write!(sql, "ALTER TABLE ").unwrap();
            if let Some(table) = &drop.table {
                table.prepare(sql, self.quote());
            }
            write!(sql, " ").unwrap();
        }

        write!(sql, "DROP FOREIGN KEY ").unwrap();
        if let Some(name) = &drop.foreign_key.name {
            write!(sql, "`{}`", name).unwrap();
        }
    }

    fn prepare_foreign_key_create_statement_internal(
        &self,
        create: &ForeignKeyCreateStatement,
        sql: &mut SqlWriter,
        mode: Mode,
    ) {
        if mode == Mode::Alter {
            write!(sql, "ALTER TABLE ").unwrap();
            if let Some(table) = &create.foreign_key.table {
                table.prepare(sql, self.quote());
            }
            write!(sql, " ").unwrap();
        }

        if mode != Mode::Creation {
            write!(sql, "ADD ").unwrap();
        }

        write!(sql, "CONSTRAINT ").unwrap();
        if let Some(name) = &create.foreign_key.name {
            write!(sql, "`{}`", name).unwrap();
        }
        write!(sql, " FOREIGN KEY ").unwrap();

        write!(sql, " (").unwrap();
        create.foreign_key.columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            col.prepare(sql, self.quote());
            false
        });
        write!(sql, ")").unwrap();

        write!(sql, " REFERENCES ").unwrap();
        if let Some(ref_table) = &create.foreign_key.ref_table {
            ref_table.prepare(sql, self.quote());
        }
        write!(sql, " ").unwrap();

        write!(sql, "(").unwrap();
        create
            .foreign_key
            .ref_columns
            .iter()
            .fold(true, |first, col| {
                if !first {
                    write!(sql, ", ").unwrap();
                }
                col.prepare(sql, self.quote());
                false
            });
        write!(sql, ")").unwrap();

        if let Some(foreign_key_action) = &create.foreign_key.on_delete {
            write!(sql, " ON DELETE ").unwrap();
            self.prepare_foreign_key_action(foreign_key_action, sql);
        }

        if let Some(foreign_key_action) = &create.foreign_key.on_update {
            write!(sql, " ON UPDATE ").unwrap();
            self.prepare_foreign_key_action(foreign_key_action, sql);
        }
    }
}
