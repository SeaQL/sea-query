use super::*;

impl IndexBuilder for SqliteQueryBuilder {
    fn prepare_index_create_statement(
        &self,
        create: &IndexCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        sql.write_str("CREATE ").unwrap();
        self.prepare_index_prefix(create, sql);
        sql.write_str("INDEX ").unwrap();

        if create.if_not_exists {
            sql.write_str("IF NOT EXISTS ").unwrap();
        }

        if let Some(name) = &create.index.name {
            sql.write_char(self.quote().left()).unwrap();
            sql.write_str(name).unwrap();
            sql.write_char(self.quote().right()).unwrap();
        }

        sql.write_str(" ON ").unwrap();
        if let Some(table) = &create.table {
            self.prepare_table_ref_index_stmt(table, sql);
        }

        sql.write_str(" ").unwrap();
        self.prepare_index_columns(&create.index.columns, sql);
        self.prepare_filter(&create.r#where, sql);
    }

    fn prepare_table_ref_index_stmt(&self, table_ref: &TableRef, sql: &mut dyn SqlWriter) {
        match table_ref {
            // Support only "naked" table names with no schema or alias.
            TableRef::Table(TableName(None, _), None) => {
                self.prepare_table_ref_iden(table_ref, sql)
            }
            _ => panic!("Not supported"),
        }
    }

    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut dyn SqlWriter) {
        sql.write_str("DROP INDEX ").unwrap();

        if drop.if_exists {
            sql.write_str("IF EXISTS ").unwrap();
        }

        if let Some(name) = &drop.index.name {
            sql.write_char(self.quote().left()).unwrap();
            sql.write_str(name).unwrap();
            sql.write_char(self.quote().right()).unwrap();
        }
    }

    fn prepare_index_prefix(&self, create: &IndexCreateStatement, sql: &mut dyn SqlWriter) {
        if create.primary {
            sql.write_str("PRIMARY KEY ").unwrap();
        } else if create.unique {
            sql.write_str("UNIQUE ").unwrap();
        }
    }

    fn write_column_index_prefix(&self, _col_prefix: &Option<u32>, _sql: &mut dyn SqlWriter) {}

    fn prepare_filter(&self, condition: &ConditionHolder, sql: &mut dyn SqlWriter) {
        self.prepare_condition(condition, "WHERE", sql);
    }
}
