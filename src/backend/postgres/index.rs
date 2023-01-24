use super::*;

impl IndexBuilder for PostgresQueryBuilder {
    // Overriden due to different "NULLS NOT UNIQUE" position in table index expression
    // (as opposed to the regular index expression)
    fn prepare_table_index_expression(
        &self,
        create: &IndexCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        if let Some(name) = &create.index.name {
            write!(sql, "CONSTRAINT {}{}{} ", self.quote(), name, self.quote()).unwrap();
        }

        self.prepare_index_prefix(create, sql);

        if create.nulls_not_distinct {
            write!(sql, "NULLS NOT DISTINCT ").unwrap();
        }

        self.prepare_index_columns(&create.index.columns, sql);

        match create.deferrable {
            Some(Deferrable::NotDeferrable) => {
                write!(sql, " NOT DEFERRABLE").unwrap();
            }
            Some(Deferrable::DeferrableInitiallyImmediate) => {
                write!(sql, " DEFERRABLE INITIALLY IMMEDIATE").unwrap();
            }
            Some(Deferrable::DeferrableInitiallyDeferred) => {
                write!(sql, " DEFERRABLE INITIALLY DEFERRED").unwrap();
            }
            Some(_) => panic!("Not supported!"),
            None => {}
        }
    }

    fn prepare_index_create_statement(
        &self,
        create: &IndexCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        write!(sql, "CREATE ").unwrap();
        self.prepare_index_prefix(create, sql);
        write!(sql, "INDEX ").unwrap();

        if create.if_not_exists {
            write!(sql, "IF NOT EXISTS ").unwrap();
        }

        if let Some(name) = &create.index.name {
            write!(sql, "{}{}{}", self.quote(), name, self.quote()).unwrap();
        }

        write!(sql, " ON ").unwrap();
        if let Some(table) = &create.table {
            self.prepare_table_ref_index_stmt(table, sql);
        }

        self.prepare_index_type(&create.index_type, sql);
        write!(sql, " ").unwrap();
        self.prepare_index_columns(&create.index.columns, sql);

        if create.nulls_not_distinct {
            write!(sql, " NULLS NOT DISTINCT").unwrap();
        }

        match create.deferrable {
            Some(Deferrable::NotDeferrable) => {
                write!(sql, " NOT DEFERRABLE").unwrap();
            }
            Some(Deferrable::DeferrableInitiallyImmediate) => {
                write!(sql, " DEFERRABLE INITIALLY IMMEDIATE").unwrap();
            }
            Some(Deferrable::DeferrableInitiallyDeferred) => {
                write!(sql, " DEFERRABLE INITIALLY DEFERRED").unwrap();
            }
            Some(_) => panic!("Not supported!"),
            None => {}
        }
    }

    fn prepare_table_ref_index_stmt(&self, table_ref: &TableRef, sql: &mut dyn SqlWriter) {
        match table_ref {
            TableRef::Table(_) | TableRef::SchemaTable(_, _) => {
                self.prepare_table_ref_iden(table_ref, sql)
            }
            _ => panic!("Not supported"),
        }
    }

    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut dyn SqlWriter) {
        write!(sql, "DROP INDEX ").unwrap();
        if let Some(table) = &drop.table {
            match table {
                TableRef::Table(_) => {}
                TableRef::SchemaTable(schema, _) => {
                    schema.prepare(sql.as_writer(), self.quote());
                    write!(sql, ".").unwrap();
                }
                _ => panic!("Not supported"),
            }
        }
        if let Some(name) = &drop.index.name {
            write!(sql, "\"{}\"", name).unwrap();
        }
    }

    fn prepare_index_type(&self, col_index_type: &Option<IndexType>, sql: &mut dyn SqlWriter) {
        if let Some(index_type) = col_index_type {
            write!(
                sql,
                " USING {}",
                match index_type {
                    IndexType::BTree => "BTREE".to_owned(),
                    IndexType::FullText => "GIN".to_owned(),
                    IndexType::Hash => "HASH".to_owned(),
                    IndexType::Custom(custom) => custom.to_string(),
                }
            )
            .unwrap();
        }
    }

    fn prepare_index_prefix(&self, create: &IndexCreateStatement, sql: &mut dyn SqlWriter) {
        if create.primary {
            write!(sql, "PRIMARY KEY ").unwrap();
        }
        if create.unique {
            write!(sql, "UNIQUE ").unwrap();
        }
    }
}
