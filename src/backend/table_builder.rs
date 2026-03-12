use crate::*;

pub trait TableBuilder:
    IndexBuilder + ForeignKeyBuilder + QuotedBuilder + TableRefBuilder + QueryBuilder
{
    /// Translate [`TableCreateStatement`] into SQL statement.
    fn prepare_table_create_statement(
        &self,
        create: &TableCreateStatement,
        sql: &mut impl SqlWriter,
    ) {
        sql.write_str("CREATE ").unwrap();

        self.prepare_create_temporary_table(create, sql);

        sql.write_str("TABLE ").unwrap();

        self.prepare_create_table_if_not_exists(create, sql);

        if let Some(table_ref) = &create.table {
            self.prepare_table_ref_table_stmt(table_ref, sql);
        }

        if let Some(partition_of) = &create.partition_of {
            sql.write_str(" PARTITION OF ").unwrap();
            self.prepare_table_ref_table_stmt(partition_of, sql);
        }

        if !create.columns.is_empty()
            || !create.indexes.is_empty()
            || !create.foreign_keys.is_empty()
            || !create.check.is_empty()
        {
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
                self.prepare_foreign_key_create_statement_internal(
                    foreign_key,
                    sql,
                    Mode::Creation,
                );
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
        }

        if let Some(partition_values) = &create.partition_values {
            sql.write_str(" ").unwrap();
            self.prepare_partition_values(partition_values, sql);
        }

        if let Some(partition_by) = &create.partition_by {
            sql.write_str(" PARTITION BY ").unwrap();
            self.prepare_partition_by(partition_by, sql);
        }

        if !create.partitions.is_empty() {
            sql.write_str(" ( ").unwrap();
            let mut first = true;
            for partition in create.partitions.iter() {
                if !first {
                    sql.write_str(", ").unwrap();
                }
                self.prepare_partition_definition(partition, sql);
                first = false;
            }
            sql.write_str(" )").unwrap();
        }

        self.prepare_table_opt(create, sql);

        if let Some(extra) = &create.extra {
            sql.write_str(" ").unwrap();
            sql.write_str(extra).unwrap();
        }
    }

    /// Translate [`TableRef`] into SQL statement.
    fn prepare_table_ref_table_stmt(&self, table_ref: &TableRef, sql: &mut impl SqlWriter) {
        match table_ref {
            // Support only unaliased (but potentialy qualified) table names.
            TableRef::Table(.., None) => self.prepare_table_ref_iden(table_ref, sql),
            _ => panic!("Not supported"),
        }
    }

    /// Translate [`ColumnDef`] into SQL statement.
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut impl SqlWriter);

    /// Translate [`ColumnDef`] into SQL statement.
    fn prepare_column_def_internal(
        &self,
        _is_alter_column: bool,
        column_def: &ColumnDef,
        sql: &mut impl SqlWriter,
    ) {
        self.prepare_column_def(column_def, sql);
    }

    /// Translate [`ColumnType`] into SQL statement.
    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut impl SqlWriter);

    /// Translate [`ColumnSpec`] into SQL statement.
    fn prepare_column_spec(&self, column_spec: &ColumnSpec, sql: &mut impl SqlWriter) {
        let ColumnSpec {
            nullable,
            default,
            auto_increment,
            unique,
            primary_key,
            check,
            generated,
            extra,
            comment,
            using: _,
        } = column_spec;

        if let Some(nullable) = nullable {
            sql.write_str(if *nullable { " NULL" } else { " NOT NULL" })
                .unwrap();
        }

        if let Some(default) = default {
            sql.write_str(" DEFAULT ").unwrap();
            // Wrap expressions in parentheses.
            // Most of database backends support this syntax.
            //
            // In MySQL 5.7, the DEFAULT clause doesn't accept any expressions,
            // so it will be invalid SQL in any case.
            //
            // References:
            // https://sqlite.org/lang_createtable.html
            match default {
                Expr::Value(_) | Expr::Constant(_) | Expr::Keyword(_) => {
                    self.prepare_expr(default, sql)
                }
                _ => {
                    sql.write_str("(").unwrap();
                    self.prepare_expr(default, sql);
                    sql.write_str(")").unwrap()
                }
            }
        }

        if let Some(generated) = generated {
            self.prepare_generated_column(&generated.expr, generated.stored, sql);
        }

        if *primary_key {
            sql.write_str(" PRIMARY KEY").unwrap();
        }

        if *auto_increment {
            sql.write_str(self.column_spec_auto_increment_keyword())
                .unwrap();
        }

        if *unique {
            sql.write_str(" UNIQUE").unwrap();
        }

        if let Some(check) = check {
            sql.write_str(" ").unwrap();
            self.prepare_check_constraint(check, sql);
        }

        if let Some(extra) = extra {
            sql.write_str(" ").unwrap();
            sql.write_str(extra).unwrap();
        }

        if let Some(comment) = comment {
            self.column_comment(comment, sql);
        }
    }

    /// column comment
    fn column_comment(&self, _comment: &str, _sql: &mut impl SqlWriter) {}

    /// The keyword for setting a column to be auto increment.
    fn column_spec_auto_increment_keyword(&self) -> &str;

    /// Translate [`TableOpt`] into SQL statement.
    fn prepare_table_opt(&self, create: &TableCreateStatement, sql: &mut impl SqlWriter) {
        self.prepare_table_opt_def(create, sql)
    }

    /// Default function
    fn prepare_table_opt_def(&self, create: &TableCreateStatement, sql: &mut impl SqlWriter) {
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

    /// Translate [`PartitionBy`] into SQL statement.
    fn prepare_partition_by(&self, _partition_by: &PartitionBy, _sql: &mut impl SqlWriter) {}

    /// Translate [`PartitionValues`] into SQL statement.
    fn prepare_partition_values(
        &self,
        _partition_values: &PartitionValues,
        _sql: &mut impl SqlWriter,
    ) {
    }

    /// Translate [`PartitionDefinition`] into SQL statement.
    fn prepare_partition_definition(
        &self,
        _partition_definition: &PartitionDefinition,
        _sql: &mut impl SqlWriter,
    ) {
    }

    /// Translate [`TableDropStatement`] into SQL statement.
    fn prepare_table_drop_statement(&self, drop: &TableDropStatement, sql: &mut impl SqlWriter) {
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
    fn prepare_table_drop_opt(&self, drop_opt: &TableDropOpt, sql: &mut impl SqlWriter) {
        match drop_opt {
            TableDropOpt::Restrict => sql.write_str(" RESTRICT").unwrap(),
            TableDropOpt::Cascade => sql.write_str(" CASCADE").unwrap(),
        }
    }

    /// Translate [`TableTruncateStatement`] into SQL statement.
    fn prepare_table_truncate_statement(
        &self,
        truncate: &TableTruncateStatement,
        sql: &mut impl SqlWriter,
    ) {
        sql.write_str("TRUNCATE TABLE ").unwrap();

        if let Some(table) = &truncate.table {
            self.prepare_table_ref_table_stmt(table, sql);
        }
    }

    /// Translate the check constraint into SQL statement
    fn prepare_check_constraint(&self, check: &Check, sql: &mut impl SqlWriter) {
        if let Some(name) = &check.name {
            sql.write_str("CONSTRAINT ").unwrap();
            self.prepare_iden(name, sql);
            sql.write_str(" ").unwrap();
        }

        sql.write_str("CHECK (").unwrap();
        QueryBuilder::prepare_expr(self, &check.expr, sql);
        sql.write_str(")").unwrap();
    }

    /// Translate the generated column into SQL statement
    fn prepare_generated_column(&self, r#gen: &Expr, stored: bool, sql: &mut impl SqlWriter) {
        sql.write_str(" GENERATED ALWAYS AS (").unwrap();
        QueryBuilder::prepare_expr(self, r#gen, sql);
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
        sql: &mut impl SqlWriter,
    ) {
        if create.if_not_exists {
            sql.write_str("IF NOT EXISTS ").unwrap();
        }
    }

    /// Translate TEMPORARY expression in [`TableCreateStatement`].
    fn prepare_create_temporary_table(
        &self,
        create: &TableCreateStatement,
        sql: &mut impl SqlWriter,
    ) {
        if create.temporary {
            sql.write_str("TEMPORARY ").unwrap();
        }
    }

    /// Translate [`TableAlterStatement`] into SQL statement.
    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut impl SqlWriter);

    /// Translate [`TableRenameStatement`] into SQL statement.
    fn prepare_table_rename_statement(
        &self,
        rename: &TableRenameStatement,
        sql: &mut impl SqlWriter,
    );
}
