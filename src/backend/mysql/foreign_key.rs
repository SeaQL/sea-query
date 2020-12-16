use super::*;

impl ForeignKeyBuilder for MysqlQueryBuilder {
    fn prepare_foreign_key_create_statement(&mut self, create: &ForeignKeyCreateStatement, sql: &mut dyn FmtWrite) {
        if !create.inside_table_creation {
            write!(sql, "ALTER TABLE ").unwrap();
            if let Some(table) = &create.foreign_key.table {
                table.prepare(sql, '`');
            }
            write!(sql, " ADD ").unwrap();
        }

        write!(sql, "CONSTRAINT ").unwrap();
        if let Some(name) = &create.foreign_key.name {
            write!(sql, "`{}`", name).unwrap();
        }
        write!(sql, " FOREIGN KEY ").unwrap();
        if let Some(name) = &create.foreign_key.name {
            write!(sql, "`{}`", name).unwrap();
        }

        write!(sql, " (").unwrap();
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
        write!(sql, " ").unwrap();

        write!(sql, "(").unwrap();
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

        if let Some(foreign_key_action) = &create.foreign_key.on_delete {
            write!(sql, " ON UPDATE ").unwrap();
            self.prepare_foreign_key_action(&foreign_key_action, sql);
        }
    }

    fn prepare_foreign_key_action(&mut self, foreign_key_action: &ForeignKeyAction, sql: &mut dyn FmtWrite) {
        write!(sql, "{}", match foreign_key_action {
            ForeignKeyAction::Restrict => "RESTRICT",
            ForeignKeyAction::Cascade => "CASCADE",
            ForeignKeyAction::SetNull => "SET NULL",
            ForeignKeyAction::NoAction => "NO ACTION",
            ForeignKeyAction::SetDefault => "SET DEFAULT",
        }).unwrap()
    }

    fn prepare_foreign_key_drop_statement(&mut self, drop: &ForeignKeyDropStatement, sql: &mut dyn FmtWrite) {
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