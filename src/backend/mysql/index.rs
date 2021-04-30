use super::*;

impl IndexBuilder for MysqlQueryBuilder {
    fn prepare_table_index_expression(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        self.prepare_index_prefix(create, sql);
        write!(sql, "KEY ").unwrap();

        self.prepare_index_name(&create.index.name, sql);

        self.prepare_index_type(&create.index_type, sql);

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

        self.prepare_index_type(&create.index_type, sql);

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

impl MysqlQueryBuilder {
    fn prepare_index_prefix(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        if create.primary {
            write!(sql, "PRIMARY ").unwrap();
        }
        if create.unique {
            write!(sql, "UNIQUE ").unwrap();
        }
        if matches!(create.index_type, Some(IndexType::FullText)) {
            write!(sql, "FULLTEXT ").unwrap();
        }
    }

    fn prepare_index_name(&self, name: &Option<String>, sql: &mut SqlWriter) {
        if let Some(name) = name {
            write!(sql, "`{}`", name).unwrap();
        }
    }

    fn prepare_index_type(&self, col_index_type: &Option<IndexType>, sql: &mut SqlWriter) {
        if let Some(index_type) = col_index_type {
            if !matches!(index_type, IndexType::FullText) {
                write!(sql, " USING {}", match index_type {
                    IndexType::BTree => "BTREE".to_owned(),
                    IndexType::FullText => unreachable!(),
                    IndexType::Hash => "HASH".to_owned(),
                    IndexType::Custom(custom) => custom.to_string(),
                }).unwrap();
            }
        }
    }

    fn prepare_index_columns(&self, columns: &[IndexColumn], sql: &mut SqlWriter) {
        write!(sql, " (").unwrap();
        columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            col.name.prepare(sql, '`');
            if let Some(prefix) = col.prefix {
                write!(sql, " (").unwrap();
                write!(sql, "{}", prefix).unwrap();
                write!(sql, ")").unwrap();
            }
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