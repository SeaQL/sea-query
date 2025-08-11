use crate::*;

pub trait TableBuilder:
    IndexBuilder + ForeignKeyBuilder + QuotedBuilder + TableRefBuilder + QueryBuilder
{
    /// Translate [`TableCreateStatement`] into SQL statement.
    fn prepare_table_create_statement(
        &self,
        create: &TableCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        sql.write_str("CREATE ").unwrap();

        self.prepare_create_temporary_table(create, sql);

        sql.write_str("TABLE ").unwrap();

        self.prepare_create_table_if_not_exists(create, sql);

        if let Some(table_ref) = &create.table {
            self.prepare_table_ref_table_stmt(table_ref, sql);
        }

        sql.write_str(" ( ").unwrap();
        let mut first = true;

        create.columns.iter().for_each(|column_def| {
            if !first {
                sql.write_str(", ").unwrap();
            }
            self.prepare_column_def(column_def, sql);
            first = false;
        });

        create.indexes.iter().for_each(|index| {
            if !first {
                sql.write_str(", ").unwrap();
            }
            self.prepare_table_index_expression(index, sql);
            first = false;
        });

        create.foreign_keys.iter().for_each(|foreign_key| {
            if !first {
                sql.write_str(", ").unwrap();
            }
            self.prepare_foreign_key_create_statement_internal(foreign_key, sql, Mode::Creation);
            first = false;
        });

        create.check.iter().for_each(|check| {
            if !first {
                sql.write_str(", ").unwrap();
            }
            self.prepare_check_constraint(check, sql);
            first = false;
        });

        sql.write_str(" )").unwrap();

        self.prepare_table_opt(create, sql);

        if let Some(extra) = &create.extra {
            sql.write_str(" ").unwrap();
            sql.write_str(extra).unwrap();
        }
    }

    /// Translate [`TableRef`] into SQL statement.
    fn prepare_table_ref_table_stmt(&self, table_ref: &TableRef, sql: &mut dyn SqlWriter) {
        match table_ref {
            // Support only unaliased (but potentialy qualified) table names.
            TableRef::Table(.., None) => self.prepare_table_ref_iden(table_ref, sql),
            _ => panic!("Not supported"),
        }
    }

    /// Translate [`ColumnDef`] into SQL statement.
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut dyn SqlWriter);

    /// Translate [`ColumnDef`] into SQL statement.
    fn prepare_column_def_internal(
        &self,
        _is_alter_column: bool,
        column_def: &ColumnDef,
        sql: &mut dyn SqlWriter,
    ) {
        self.prepare_column_def(column_def, sql);
    }

    /// Translate [`ColumnType`] into SQL statement.
    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut dyn SqlWriter);

    /// Translate [`ColumnSpec`] into SQL statement.
    fn prepare_column_spec(&self, column_spec: &ColumnSpec, sql: &mut dyn SqlWriter) {
        match column_spec {
            ColumnSpec::Null => sql.write_str("NULL").unwrap(),
            ColumnSpec::NotNull => sql.write_str("NOT NULL").unwrap(),
            ColumnSpec::Default(value) => {
                sql.write_str("DEFAULT ").unwrap();
                QueryBuilder::prepare_simple_expr(self, value, sql);
            }
            ColumnSpec::AutoIncrement => sql
                .write_str(self.column_spec_auto_increment_keyword())
                .unwrap(),
            ColumnSpec::UniqueKey => sql.write_str("UNIQUE").unwrap(),
            ColumnSpec::PrimaryKey => sql.write_str("PRIMARY KEY").unwrap(),
            ColumnSpec::Check(check) => self.prepare_check_constraint(check, sql),
            ColumnSpec::Generated { expr, stored } => {
                self.prepare_generated_column(expr, *stored, sql)
            }
            ColumnSpec::Extra(string) => sql.write_str(string).unwrap(),
            ColumnSpec::Comment(comment) => self.column_comment(comment, sql),
            ColumnSpec::Using(_) => {}
        }
    }

    /// column comment
    fn column_comment(&self, _comment: &str, _sql: &mut dyn SqlWriter) {}

    /// The keyword for setting a column to be auto increment.
    fn column_spec_auto_increment_keyword(&self) -> &str;

    /// Translate [`TableOpt`] into SQL statement.
    fn prepare_table_opt(&self, create: &TableCreateStatement, sql: &mut dyn SqlWriter) {
        self.prepare_table_opt_def(create, sql)
    }

    /// Default function
    fn prepare_table_opt_def(&self, create: &TableCreateStatement, sql: &mut dyn SqlWriter) {
        for table_opt in create.options.iter() {
            sql.write_str(" ").unwrap();
            match table_opt {
                TableOpt::Engine(s) => {
                    sql.write_str("ENGINE=").unwrap();
                    sql.write_str(s).unwrap();
                }
                TableOpt::Collate(s) => {
                    sql.write_str("COLLATE=").unwrap();
                    sql.write_str(s).unwrap();
                }
                TableOpt::CharacterSet(s) => {
                    sql.write_str("DEFAULT CHARSET=").unwrap();
                    sql.write_str(s).unwrap();
                }
            }
        }
    }

    /// Translate [`TablePartition`] into SQL statement.
    fn prepare_table_partition(&self, _table_partition: &TablePartition, _sql: &mut dyn SqlWriter) {
    }

    /// Translate [`TableDropStatement`] into SQL statement.
    fn prepare_table_drop_statement(&self, drop: &TableDropStatement, sql: &mut dyn SqlWriter) {
        sql.write_str("DROP TABLE ").unwrap();

        if drop.if_exists {
            sql.write_str("IF EXISTS ").unwrap();
        }

        let mut tables = drop.tables.iter();
        join_io!(
            tables,
            table,
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                self.prepare_table_ref_table_stmt(table, sql);
            }
        );

        for drop_opt in drop.options.iter() {
            self.prepare_table_drop_opt(drop_opt, sql);
        }
    }

    /// Translate [`TableDropOpt`] into SQL statement.
    fn prepare_table_drop_opt(&self, drop_opt: &TableDropOpt, sql: &mut dyn SqlWriter) {
        match drop_opt {
            TableDropOpt::Restrict => sql.write_str(" RESTRICT").unwrap(),
            TableDropOpt::Cascade => sql.write_str(" CASCADE").unwrap(),
        }
    }

    /// Translate [`TableTruncateStatement`] into SQL statement.
    fn prepare_table_truncate_statement(
        &self,
        truncate: &TableTruncateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        sql.write_str("TRUNCATE TABLE ").unwrap();

        if let Some(table) = &truncate.table {
            self.prepare_table_ref_table_stmt(table, sql);
        }
    }

    /// Translate the check constraint into SQL statement
    fn prepare_check_constraint(&self, check: &Check, sql: &mut dyn SqlWriter) {
        if let Some(name) = &check.name {
            sql.write_str("CONSTRAINT ").unwrap();
            self.prepare_iden(name, sql);
            sql.write_str(" ").unwrap();
        }

        sql.write_str("CHECK (").unwrap();
        QueryBuilder::prepare_simple_expr(self, &check.expr, sql);
        sql.write_str(")").unwrap();
    }

    /// Translate the generated column into SQL statement
    fn prepare_generated_column(&self, r#gen: &Expr, stored: bool, sql: &mut dyn SqlWriter) {
        sql.write_str("GENERATED ALWAYS AS (").unwrap();
        QueryBuilder::prepare_simple_expr(self, r#gen, sql);
        sql.write_str(")").unwrap();
        if stored {
            sql.write_str(" STORED").unwrap();
        } else {
            sql.write_str(" VIRTUAL").unwrap();
        }
    }

    /// Translate IF NOT EXISTS expression in [`TableCreateStatement`].
    fn prepare_create_table_if_not_exists(
        &self,
        create: &TableCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        if create.if_not_exists {
            sql.write_str("IF NOT EXISTS ").unwrap();
        }
    }

    /// Translate TEMPORARY expression in [`TableCreateStatement`].
    fn prepare_create_temporary_table(
        &self,
        create: &TableCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        if create.temporary {
            sql.write_str("TEMPORARY ").unwrap();
        }
    }

    /// Translate [`TableAlterStatement`] into SQL statement.
    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut dyn SqlWriter);

    /// Translate [`TableRenameStatement`] into SQL statement.
    fn prepare_table_rename_statement(
        &self,
        rename: &TableRenameStatement,
        sql: &mut dyn SqlWriter,
    );
}
