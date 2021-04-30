use super::*;

impl IndexBuilder for SqliteQueryBuilder {
    fn prepare_table_index_expression(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        self.prepare_index_prefix(create, sql);
        write!(sql, "KEY ").unwrap();

        self.prepare_index_name(&create.index.name, sql);

        // self.prepare_index_type(&create.index_type, sql);

        self.prepare_index_columns(&create.index.columns, sql);
    }

    fn prepare_index_create_statement(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        write!(sql, "CREATE ").unwrap();
        self.prepare_index_prefix(create, sql);
        write!(sql, "INDEX ").unwrap();

        self.prepare_index_name(&create.index.name, sql);

        write!(sql, " ON ").unwrap();
        if let Some(table) = &create.table {
            table.prepare(sql, '`');
        }

        // self.prepare_index_type(&create.index_type, sql);

        self.prepare_index_columns(&create.index.columns, sql);
    }

    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut SqlWriter) {
        write!(sql, "DROP INDEX ").unwrap();
        if let Some(name) = &drop.index.name {
            write!(sql, "`{}`", name).unwrap();
        }

        write!(sql, " ON ").unwrap();
        if let Some(table) = &drop.table {
            table.prepare(sql, '`');
        }
    }
}

impl SqliteQueryBuilder {
    fn prepare_index_prefix(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        if create.primary {
            write!(sql, "PRIMARY ").unwrap();
        }
        if create.unique {
            write!(sql, "UNIQUE ").unwrap();
        }
    }

    fn prepare_index_name(&self, name: &Option<String>, sql: &mut SqlWriter) {
        if let Some(name) = name {
            write!(sql, "`{}`", name).unwrap();
        }
    }

    fn prepare_index_columns(&self, columns: &[IndexColumn], sql: &mut SqlWriter) {
        write!(sql, " (").unwrap();
        columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            col.name.prepare(sql, '`');
            if let Some(order) = &col.order {
                match order {
                    IndexOrder::Asc => write!(sql, " ASC").unwrap(),
                    IndexOrder::Desc => write!(sql, " DESC").unwrap(),
                }
            }
            false
        });
        write!(sql, ")").unwrap();
    }
}