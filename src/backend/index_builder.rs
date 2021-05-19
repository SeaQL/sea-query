use crate::*;

pub trait IndexBuilder: QuotedBuilder {
    /// Translate [`IndexCreateStatement`] into SQL expression.
    fn prepare_table_index_expression(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        self.prepare_index_prefix(create, sql);
        write!(sql, "KEY ").unwrap();

        self.prepare_index_name(&create.index.name, sql);

        self.prepare_index_type(&create.index_type, sql);

        self.prepare_index_columns(&create.index.columns, sql);
    }

    /// Translate [`IndexCreateStatement`] into SQL statement.
    fn prepare_index_create_statement(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        write!(sql, "CREATE ").unwrap();
        self.prepare_index_prefix(create, sql);
        write!(sql, "INDEX ").unwrap();

        self.prepare_index_name(&create.index.name, sql);

        write!(sql, " ON ").unwrap();
        if let Some(table) = &create.table {
            table.prepare(sql, self.quote());
        }

        self.prepare_index_type(&create.index_type, sql);

        self.prepare_index_columns(&create.index.columns, sql);
    }

    /// Translate [`IndexDropStatement`] into SQL statement.
    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut SqlWriter);

    #[doc(hidden)]
    /// Write the index type (Btree, hash, ...).
    fn prepare_index_type(&self, _col_index_type: &Option<IndexType>, _sql: &mut SqlWriter) {}

    #[doc(hidden)]
    /// Write the index prefix (primary, unique, ...).
    fn prepare_index_prefix(&self, create: &IndexCreateStatement, sql: &mut SqlWriter);

    #[doc(hidden)]
    /// Write the column index prefix.
    fn write_column_index_prefix(&self, col_prefix: &Option<u32>, sql: &mut SqlWriter) {
        if let Some(prefix) = col_prefix {
            write!(sql, " ({})", prefix).unwrap();
        }
    }

    #[doc(hidden)]
    /// Write the column index prefix.
    fn prepare_index_columns(&self, columns: &[IndexColumn], sql: &mut SqlWriter) {
        write!(sql, " (").unwrap();
        columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            col.name.prepare(sql, self.quote());
            self.write_column_index_prefix(&col.prefix, sql);
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

    #[doc(hidden)]
    /// Write index name.
    fn prepare_index_name(&self, name: &Option<String>, sql: &mut SqlWriter) {
        if let Some(name) = name {
            write!(sql, "{}{}{}", self.quote(), name, self.quote()).unwrap();
        }
    }
}
