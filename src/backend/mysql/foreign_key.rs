use super::*;

impl ForeignKeyBuilder for MysqlQueryBuilder {
    fn prepare_table_ref_fk_stmt(&self, table_ref: &TableRef, sql: &mut impl SqlWriter) {
        match table_ref {
            // Support only "naked" table names with no schema or alias.
            TableRef::Table(TableName(None, _), None) => {
                self.prepare_table_ref_iden(table_ref, sql)
            }
            _ => panic!("Not supported"),
        }
    }

    fn prepare_foreign_key_drop_statement_internal(
        &self,
        drop: &ForeignKeyDropStatement,
        sql: &mut impl SqlWriter,
        mode: Mode,
    ) {
        if mode == Mode::Alter {
            sql.write_str("ALTER TABLE ").unwrap();
            if let Some(table) = &drop.table {
                self.prepare_table_ref_fk_stmt(table, sql);
            }
            sql.write_str(" ").unwrap();
        }

        sql.write_str("DROP FOREIGN KEY ").unwrap();
        if let Some(name) = &drop.foreign_key.name {
            sql.write_char(self.quote().left()).unwrap();
            sql.write_str(name).unwrap();
            sql.write_char(self.quote().right()).unwrap();
        }
    }

    fn prepare_foreign_key_create_statement_internal(
        &self,
        create: &ForeignKeyCreateStatement,
        sql: &mut impl SqlWriter,
        mode: Mode,
    ) {
        if mode == Mode::Alter {
            sql.write_str("ALTER TABLE ").unwrap();
            if let Some(table) = &create.foreign_key.table {
                self.prepare_table_ref_fk_stmt(table, sql);
            }
            sql.write_str(" ").unwrap();
        }

        if mode != Mode::Creation {
            sql.write_str("ADD ").unwrap();
        }

        sql.write_str("CONSTRAINT ").unwrap();
        if let Some(name) = &create.foreign_key.name {
            sql.write_char(self.quote().left()).unwrap();
            sql.write_str(name).unwrap();
            sql.write_char(self.quote().right()).unwrap();
        }
        sql.write_str(" FOREIGN KEY (").unwrap();

        let mut fk_cols = create.foreign_key.columns.iter();

        if let Some(col) = fk_cols.next() {
            self.prepare_iden(col, sql);
        }

        for col in fk_cols {
            sql.write_str(", ").unwrap();
            self.prepare_iden(col, sql);
        }

        sql.write_str(") REFERENCES ").unwrap();
        if let Some(ref_table) = &create.foreign_key.ref_table {
            self.prepare_table_ref_fk_stmt(ref_table, sql);
        }
        sql.write_str(" (").unwrap();

        let mut fk_ref_cols = create.foreign_key.ref_columns.iter();
        if let Some(col) = fk_ref_cols.next() {
            self.prepare_iden(col, sql);
        }

        for col in fk_ref_cols {
            sql.write_str(", ").unwrap();
            self.prepare_iden(col, sql);
        }

        sql.write_str(")").unwrap();

        if let Some(foreign_key_action) = &create.foreign_key.on_delete {
            sql.write_str(" ON DELETE ").unwrap();
            self.prepare_foreign_key_action(foreign_key_action, sql);
        }

        if let Some(foreign_key_action) = &create.foreign_key.on_update {
            sql.write_str(" ON UPDATE ").unwrap();
            self.prepare_foreign_key_action(foreign_key_action, sql);
        }
    }
}
