use super::*;

impl IndexBuilder for PostgresQueryBuilder {
    // Overriden due to different "NULLS NOT UNIQUE" position in table index expression
    // (as opposed to the regular index expression)
    fn prepare_table_index_expression(
        &self,
        create: &IndexCreateStatement,
        sql: &mut (impl SqlWriter + ?Sized),
    ) {
        if let Some(name) = &create.index.name {
            sql.write_str("CONSTRAINT ").unwrap();
            sql.write_char(self.quote().left()).unwrap();
            sql.write_str(name).unwrap();
            sql.write_char(self.quote().right()).unwrap();
            sql.write_str(" ").unwrap();
        }

        self.prepare_index_prefix(create, sql);

        if create.nulls_not_distinct {
            sql.write_str("NULLS NOT DISTINCT ").unwrap();
        }

        self.prepare_index_columns(&create.index.columns, sql);

        if !create.include_columns.is_empty() {
            sql.write_str(" ").unwrap();
            self.prepare_include_columns(&create.include_columns, sql);
        }
    }

    fn prepare_index_create_statement(
        &self,
        create: &IndexCreateStatement,
        sql: &mut (impl SqlWriter + ?Sized),
    ) {
        sql.write_str("CREATE ").unwrap();
        self.prepare_index_prefix(create, sql);
        sql.write_str("INDEX ").unwrap();

        if create.concurrently {
            write!(sql, "CONCURRENTLY ").unwrap();
        }

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

        self.prepare_index_type(&create.index_type, sql);
        sql.write_str(" ").unwrap();
        self.prepare_index_columns(&create.index.columns, sql);

        if !create.include_columns.is_empty() {
            sql.write_str(" ").unwrap();
            self.prepare_include_columns(&create.include_columns, sql);
        }

        if create.nulls_not_distinct {
            sql.write_str(" NULLS NOT DISTINCT").unwrap();
        }
        self.prepare_filter(&create.r#where, sql);
    }

    fn prepare_table_ref_index_stmt(
        &self,
        table_ref: &TableRef,
        sql: &mut (impl SqlWriter + ?Sized),
    ) {
        // Support only `table` and `schema.table` forms.
        // No `database.schema.table` or aliases.
        let TableRef::Table(table_name, None) = table_ref else {
            panic!("Not supported");
        };
        match table_name.as_iden_tuple() {
            (Some(_db), _schema, _table) => panic!("Not supported"),
            (None, _schema, _table) => self.prepare_table_ref_iden(table_ref, sql),
        }
    }

    fn prepare_index_drop_statement(
        &self,
        drop: &IndexDropStatement,
        sql: &mut (impl SqlWriter + ?Sized),
    ) {
        sql.write_str("DROP INDEX ").unwrap();

        if drop.concurrently {
            write!(sql, "CONCURRENTLY ").unwrap();
        }

        if drop.if_exists {
            sql.write_str("IF EXISTS ").unwrap();
        }

        if let Some(table) = &drop.table {
            // Support only `table` and `schema.table` forms.
            // No `database.schema.table` or aliases.
            let TableRef::Table(table_name, None) = table else {
                panic!("Not supported");
            };
            match table_name.as_iden_tuple() {
                (None, None, _table) => {}
                (None, Some(schema), _table) => {
                    self.prepare_iden(schema, sql);
                    sql.write_str(".").unwrap();
                }
                (Some(_db), _schema, _table) => panic!("Not supported"),
            }
        }
        if let Some(name) = &drop.index.name {
            sql.write_char(self.quote().left()).unwrap();
            sql.write_str(name).unwrap();
            sql.write_char(self.quote().right()).unwrap();
        }
    }

    fn prepare_index_type(
        &self,
        col_index_type: &Option<IndexType>,
        sql: &mut (impl SqlWriter + ?Sized),
    ) {
        if let Some(index_type) = col_index_type {
            sql.write_str(" USING ").unwrap();
            match index_type {
                IndexType::BTree => sql.write_str("BTREE"),
                IndexType::FullText => sql.write_str("GIN"),
                IndexType::Hash => sql.write_str("HASH"),
                IndexType::Custom(custom) => sql.write_str(&custom.0),
            }
            .unwrap()
        }
    }

    fn prepare_index_prefix(
        &self,
        create: &IndexCreateStatement,
        sql: &mut (impl SqlWriter + ?Sized),
    ) {
        if create.primary {
            sql.write_str("PRIMARY KEY ").unwrap();
        }
        if create.unique {
            sql.write_str("UNIQUE ").unwrap();
        }
    }

    fn prepare_index_columns(&self, columns: &[IndexColumn], sql: &mut (impl SqlWriter + ?Sized)) {
        sql.write_str("(").unwrap();

        let mut cols = columns.iter();
        join_io!(
            cols,
            col,
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                match col {
                    IndexColumn::TableColumn(column) => {
                        self.prepare_index_column_with_table_column(column, sql);
                    }
                    IndexColumn::Expr(column) => {
                        sql.write_str("(").unwrap();
                        self.prepare_simple_expr(&column.expr, sql);
                        sql.write_str(")").unwrap();
                        if let Some(order) = &column.order {
                            match order {
                                IndexOrder::Asc => sql.write_str(" ASC").unwrap(),
                                IndexOrder::Desc => sql.write_str(" DESC").unwrap(),
                            }
                        }
                    }
                }
            }
        );

        sql.write_str(")").unwrap();
    }

    fn prepare_filter(&self, condition: &ConditionHolder, sql: &mut (impl SqlWriter + ?Sized)) {
        self.prepare_condition(condition, "WHERE", sql);
    }
}

impl PostgresQueryBuilder {
    fn prepare_include_columns(&self, columns: &[DynIden], sql: &mut (impl SqlWriter + ?Sized)) {
        sql.write_str("INCLUDE (").unwrap();

        let mut cols = columns.iter();
        join_io!(
            cols,
            col,
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                self.prepare_iden(col, sql);
            }
        );

        sql.write_str(")").unwrap();
    }
}
