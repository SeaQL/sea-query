use super::*;

impl TableBuilder for DatabendQueryBuilder {
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut dyn SqlWriter) {
        MysqlQueryBuilder.prepare_column_def(column_def, sql)
    }

    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut dyn SqlWriter) {
        MysqlQueryBuilder.prepare_column_type(column_type, sql)
    }

    fn column_spec_auto_increment_keyword(&self) -> &str {
        ""
    }

    fn prepare_table_drop_opt(&self, _drop_opt: &TableDropOpt, _sql: &mut dyn SqlWriter) {
        // Sqlite does not support table drop options
    }

    fn prepare_table_truncate_statement(
        &self,
        _truncate: &TableTruncateStatement,
        _sql: &mut dyn SqlWriter,
    ) {
        panic!("Sqlite doesn't support TRUNCATE statement")
    }

    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut dyn SqlWriter) {
        if alter.options.is_empty() {
            panic!("No alter option found")
        }
        if alter.options.len() > 1 {
            panic!("Sqlite doesn't support multiple alter options")
        }
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(table) = &alter.table {
            self.prepare_table_ref_table_stmt(table, sql);
            write!(sql, " ").unwrap();
        }
        match &alter.options[0] {
            TableAlterOption::AddColumn(AddColumnOption {
                column,
                if_not_exists: _,
            }) => {
                write!(sql, "ADD COLUMN ").unwrap();
                self.prepare_column_def(column, sql);
            }
            TableAlterOption::ModifyColumn(_) => {
                panic!("Sqlite not support modifying table column")
            }
            TableAlterOption::RenameColumn(from_name, to_name) => {
                write!(sql, "RENAME COLUMN ").unwrap();
                from_name.prepare(sql.as_writer(), self.quote());
                write!(sql, " TO ").unwrap();
                to_name.prepare(sql.as_writer(), self.quote());
            }
            TableAlterOption::DropColumn(col_name) => {
                write!(sql, "DROP COLUMN ").unwrap();
                col_name.prepare(sql.as_writer(), self.quote());
            }
            TableAlterOption::DropForeignKey(_) => {
                panic!("Sqlite does not support modification of foreign key constraints to existing tables");
            }
            TableAlterOption::AddForeignKey(_) => {
                panic!("Sqlite does not support modification of foreign key constraints to existing tables");
            }
        }
    }

    fn prepare_table_rename_statement(
        &self,
        rename: &TableRenameStatement,
        sql: &mut dyn SqlWriter,
    ) {
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(from_name) = &rename.from_name {
            self.prepare_table_ref_table_stmt(from_name, sql);
        }
        write!(sql, " RENAME TO ").unwrap();
        if let Some(to_name) = &rename.to_name {
            self.prepare_table_ref_table_stmt(to_name, sql);
        }
    }
}
