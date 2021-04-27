use super::*;

impl ForeignKeyBuilder for SqliteQueryBuilder {
    fn prepare_foreign_key_create_statement(&self, create: &ForeignKeyCreateStatement, sql: &mut SqlWriter) {
        self.prepare_foreign_key_create_statement_internal(create, sql, false)
    }

    fn prepare_foreign_key_action(&self, foreign_key_action: &ForeignKeyAction, sql: &mut SqlWriter) {
        write!(sql, "{}", match foreign_key_action {
            ForeignKeyAction::Restrict => "RESTRICT",
            ForeignKeyAction::Cascade => "CASCADE",
            ForeignKeyAction::SetNull => "SET NULL",
            ForeignKeyAction::NoAction => "NO ACTION",
            ForeignKeyAction::SetDefault => "SET DEFAULT",
        }).unwrap()
    }

    fn prepare_foreign_key_drop_statement(&self, drop: &ForeignKeyDropStatement, sql: &mut SqlWriter) {
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(table) = &drop.table {
            table.prepare(sql, '`');
        }

        write!(sql, " DROP FOREIGN KEY ").unwrap();
        if let Some(name) = &drop.foreign_key.name {
            write!(sql, "`{}`", name).unwrap();
        }
    }
}

impl SqliteQueryBuilder {
    pub(crate) fn prepare_foreign_key_create_statement_internal(&self, create: &ForeignKeyCreateStatement, sql: &mut SqlWriter, inside_table_creation: bool) {
        if !inside_table_creation {
            panic!("Sqlite does not support modification of foreign key constraints to existing tables");
        }

        write!(sql, "FOREIGN KEY (").unwrap();
        create.foreign_key.columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            col.prepare(sql, '`');
            false
        });
        write!(sql, ")").unwrap();

        write!(sql, " REFERENCES ").unwrap();
        if let Some(ref_table) = &create.foreign_key.ref_table {
            ref_table.prepare(sql, '`');
        }
        write!(sql, " (").unwrap();
        create.foreign_key.ref_columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            col.prepare(sql, '`');
            false
        });
        write!(sql, ")").unwrap();

        if let Some(foreign_key_action) = &create.foreign_key.on_delete {
            write!(sql, " ON DELETE ").unwrap();
            self.prepare_foreign_key_action(&foreign_key_action, sql);
        }

        if let Some(foreign_key_action) = &create.foreign_key.on_update {
            write!(sql, " ON UPDATE ").unwrap();
            self.prepare_foreign_key_action(&foreign_key_action, sql);
        }
    }
}