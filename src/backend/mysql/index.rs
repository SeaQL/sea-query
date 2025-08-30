use super::*;

impl IndexBuilder for MysqlQueryBuilder {
    fn prepare_table_index_expression(
        &self,
        create: &IndexCreateStatement,
        sql: &mut (impl SqlWriter + ?Sized),
    ) {
        self.prepare_index_prefix(create, sql);
        sql.write_str("KEY ").unwrap();

        if let Some(name) = &create.index.name {
            sql.write_char(self.quote().left()).unwrap();
            sql.write_str(name).unwrap();
            sql.write_char(self.quote().right()).unwrap();
            sql.write_str(" ").unwrap();
        }

        self.prepare_index_type(&create.index_type, sql);
        if matches!(create.index_type, Some(IndexType::FullText)) {
            sql.write_str(" ").unwrap();
        }

        self.prepare_index_columns(&create.index.columns, sql);
    }

    fn prepare_index_create_statement(
        &self,
        create: &IndexCreateStatement,
        sql: &mut (impl SqlWriter + ?Sized),
    ) {
        sql.write_str("CREATE ").unwrap();
        self.prepare_index_prefix(create, sql);
        sql.write_str("INDEX ").unwrap();

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

        self.prepare_index_type(&create.index_type, sql);
    }

    fn prepare_table_ref_index_stmt(
        &self,
        table_ref: &TableRef,
        sql: &mut (impl SqlWriter + ?Sized),
    ) {
        match table_ref {
            // Support only "naked" table names with no schema or alias.
            TableRef::Table(TableName(None, _), None) => {
                self.prepare_table_ref_iden(table_ref, sql)
            }
            _ => panic!("Not supported"),
        }
    }
    fn prepare_index_drop_statement(
        &self,
        drop: &IndexDropStatement,
        sql: &mut (impl SqlWriter + ?Sized),
    ) {
        sql.write_str("DROP INDEX ").unwrap();

        if drop.if_exists {
            panic!("Mysql does not support IF EXISTS for DROP INDEX")
        }

        if let Some(name) = &drop.index.name {
            sql.write_char(self.quote().left()).unwrap();
            sql.write_str(name).unwrap();
            sql.write_char(self.quote().right()).unwrap();
        }

        sql.write_str(" ON ").unwrap();
        if let Some(table) = &drop.table {
            self.prepare_table_ref_index_stmt(table, sql);
        }
    }

    fn prepare_index_type(
        &self,
        col_index_type: &Option<IndexType>,
        sql: &mut (impl SqlWriter + ?Sized),
    ) {
        if let Some(index_type) = col_index_type {
            if !matches!(index_type, IndexType::FullText) {
                sql.write_str(" USING ").unwrap();
                sql.write_str(match index_type {
                    IndexType::BTree => "BTREE",
                    IndexType::FullText => unreachable!(),
                    IndexType::Hash => "HASH",
                    IndexType::Custom(custom) => &custom.0,
                })
                .unwrap();
            }
        }
    }

    fn prepare_index_prefix(
        &self,
        create: &IndexCreateStatement,
        sql: &mut (impl SqlWriter + ?Sized),
    ) {
        if create.primary {
            sql.write_str("PRIMARY ").unwrap();
        }
        if create.unique {
            sql.write_str("UNIQUE ").unwrap();
        }
        if matches!(create.index_type, Some(IndexType::FullText)) {
            sql.write_str("FULLTEXT ").unwrap();
        }
    }

    fn prepare_index_columns(&self, columns: &[IndexColumn], sql: &mut (impl SqlWriter + ?Sized)) {
        macro_rules! prepare {
            ($i:ident) => {
                match $i {
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
            };
        }

        sql.write_str("(").unwrap();

        let mut cols = columns.iter();

        if let Some(col) = cols.next() {
            prepare!(col)
        }

        for col in cols {
            sql.write_str(", ").unwrap();
            prepare!(col)
        }

        sql.write_str(")").unwrap();
    }
}
