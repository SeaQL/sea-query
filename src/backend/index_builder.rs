use crate::*;

pub trait IndexBuilder: QuotedBuilder + TableRefBuilder {
    /// Translate [`IndexCreateStatement`] into SQL expression.
    /// This is the default implementation for `PostgresQueryBuilder` and `SqliteQueryBuilder`.
    /// `MysqlQueryBuilder` overrides this default implementation.
    fn prepare_table_index_expression(
        &self,
        create: &IndexCreateStatement,
        sql: &mut impl SqlWriter,
    ) {
        if let Some(name) = &create.index.name {
            sql.write_str("CONSTRAINT ").unwrap();
            sql.write_char(self.quote().left()).unwrap();
            sql.write_str(name).unwrap();
            sql.write_char(self.quote().right()).unwrap();
            sql.write_str(" ").unwrap();
        }

        self.prepare_index_prefix(create, sql);

        self.prepare_index_columns(&create.index.columns, sql);

        self.prepare_filter(&create.r#where, sql);
    }

    /// Translate [`IndexCreateStatement`] into SQL statement.
    fn prepare_index_create_statement(
        &self,
        create: &IndexCreateStatement,
        sql: &mut impl SqlWriter,
    );

    /// Translate [`TableRef`] into SQL statement.
    fn prepare_table_ref_index_stmt(&self, table_ref: &TableRef, sql: &mut impl SqlWriter);

    /// Translate [`IndexDropStatement`] into SQL statement.
    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut impl SqlWriter);

    #[doc(hidden)]
    /// Write the index type (Btree, hash, ...).
    fn prepare_index_type(&self, _col_index_type: &Option<IndexType>, _sql: &mut impl SqlWriter) {}

    #[doc(hidden)]
    /// Write the index prefix (primary, unique, ...).
    fn prepare_index_prefix(&self, create: &IndexCreateStatement, sql: &mut impl SqlWriter);

    #[doc(hidden)]
    /// Write the column index prefix.
    fn write_column_index_prefix(&self, col_prefix: &Option<u32>, sql: &mut impl SqlWriter) {
        if let Some(prefix) = col_prefix {
            sql.write_str(" (").unwrap();
            write_int(sql, *prefix);
            sql.write_str(")").unwrap();
        }
    }

    #[doc(hidden)]
    /// Write the index column with table column.
    fn prepare_index_column_with_table_column(
        &self,
        column: &IndexColumnTableColumn,
        sql: &mut impl SqlWriter,
    ) {
        self.prepare_iden(&column.name, sql);
        self.write_column_index_prefix(&column.prefix, sql);
        if let Some(order) = &column.order {
            match order {
                IndexOrder::Asc => sql.write_str(" ASC").unwrap(),
                IndexOrder::Desc => sql.write_str(" DESC").unwrap(),
            }
        }
    }

    #[doc(hidden)]
    /// Write the column index prefix.
    fn prepare_index_columns(&self, columns: &[IndexColumn], sql: &mut impl SqlWriter) {
        macro_rules! prepare {
            ($i:ident) => {
                match $i {
                    IndexColumn::TableColumn(column) => {
                        self.prepare_index_column_with_table_column(column, sql);
                    }
                    IndexColumn::Expr(_) => panic!("Not supported"),
                }
            };
        }

        sql.write_str("(").unwrap();

        let mut citer = columns.iter();

        if let Some(col) = citer.next() {
            prepare!(col)
        }

        for col in citer {
            sql.write_str(", ").unwrap();
            prepare!(col)
        }

        sql.write_str(")").unwrap();
    }

    #[doc(hidden)]
    // Write WHERE clause for partial index. This function is not available in MySQL.
    fn prepare_filter(&self, _condition: &ConditionHolder, _sql: &mut impl SqlWriter) {}
}
