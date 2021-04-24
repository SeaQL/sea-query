use std::rc::Rc;
use super::*;

impl IndexBuilder for MysqlQueryBuilder {
    fn prepare_index_create_statement(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        write!(sql, "CREATE ").unwrap();
        if create.unique {
            write!(sql, "UNIQUE ").unwrap();
        }
        if matches!(create.index_type, Some(IndexType::FullText)) {
            write!(sql, "FULLTEXT ").unwrap();
        }
        write!(sql, "INDEX ").unwrap();

        if let Some(name) = &create.index.name {
            write!(sql, "`{}`", name).unwrap();
        }

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
    pub(crate) fn prepare_table_index_create_expression(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        if create.unique {
            write!(sql, "UNIQUE ").unwrap();
        }
        if matches!(create.index_type, Some(IndexType::FullText)) {
            write!(sql, "FULLTEXT ").unwrap();
        }
        write!(sql, "KEY ").unwrap();

        if let Some(name) = &create.index.name {
            write!(sql, "`{}`", name).unwrap();
        }

        self.prepare_index_type(&create.index_type, sql);

        self.prepare_index_columns(&create.index.columns, sql);
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

    fn prepare_index_columns(&self, columns: &[Rc<dyn Iden>], sql: &mut SqlWriter) {
        write!(sql, " (").unwrap();
        columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            col.prepare(sql, '`');
            false
        });
        write!(sql, ")").unwrap();
    }
}