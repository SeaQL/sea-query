use super::*;

impl IndexBuilder for SqliteQueryBuilder {
    fn prepare_table_index_expression(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        if create.index.name.is_some() {
            write!(sql, "CONSTRAINT ").unwrap();
        }
        self.prepare_index_name(&create.index.name, sql);

        self.prepare_index_prefix(create, sql);

        self.prepare_index_columns(&create.index.columns, sql);
    }

    fn prepare_index_create_statement(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        write!(sql, "CREATE ").unwrap();
        self.prepare_index_prefix(create, sql);
        write!(sql, "INDEX ").unwrap();

        if create.if_not_exists {
            write!(sql, "IF NOT EXISTS ").unwrap();
        }

        self.prepare_index_name(&create.index.name, sql);

        write!(sql, " ON ").unwrap();
        if let Some(table) = &create.table {
            table.prepare(sql, self.quote());
        }

        self.prepare_index_columns(&create.index.columns, sql);

        self.prepare_filter(&create.r#where, sql);
    }

    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut SqlWriter) {
        write!(sql, "DROP INDEX ").unwrap();
        if let Some(name) = &drop.index.name {
            let quote = self.quote();
            write!(sql, "{}{}{}", quote, name, quote).unwrap();
        }

        write!(sql, " ON ").unwrap();
        if let Some(table) = &drop.table {
            table.prepare(sql, self.quote());
        }
    }

    fn write_column_index_prefix(&self, _col_prefix: &Option<u32>, _sql: &mut SqlWriter) {}

    fn prepare_filter(&self, condition: &ConditionHolder, sql: &mut SqlWriter) {
        let mut _params: Vec<Value> = Vec::new();
        let mut _collector = |v| _params.push(v);
        self.prepare_condition(condition, "WHERE", sql, &mut _collector);
    }

    fn prepare_index_prefix(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        if create.primary {
            write!(sql, "PRIMARY KEY ").unwrap();
        } else if create.unique {
            write!(sql, "UNIQUE ").unwrap();
        }
    }
}
