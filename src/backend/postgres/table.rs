use super::*;

impl TableBuilder for PostgresQueryBuilder {
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut dyn SqlWriter) {
        let f = |column_def: &ColumnDef, sql: &mut dyn SqlWriter| {
            self.prepare_column_type_check_auto_increment(column_def, sql);
        };
        self.prepare_column_def_common(column_def, sql, f);
    }

    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut dyn SqlWriter) {
        match column_type {
            ColumnType::Char(length) => match length {
                Some(length) => {
                    sql.write_str("char(").unwrap();
                    write!(sql, "{length}").unwrap();
                    sql.write_char(')')
                }
                None => sql.write_str("char"),
            },
            ColumnType::String(length) => match length {
                StringLen::N(length) => {
                    sql.write_str("varchar(").unwrap();
                    write!(sql, "{length}").unwrap();
                    sql.write_char(')')
                }
                _ => sql.write_str("varchar"),
            },
            ColumnType::Text => sql.write_str("text"),
            ColumnType::TinyInteger | ColumnType::TinyUnsigned => sql.write_str("smallint"),
            ColumnType::SmallInteger | ColumnType::SmallUnsigned => sql.write_str("smallint"),
            ColumnType::Integer | ColumnType::Unsigned => sql.write_str("integer"),
            ColumnType::BigInteger | ColumnType::BigUnsigned => sql.write_str("bigint"),
            ColumnType::Float => sql.write_str("real"),
            ColumnType::Double => sql.write_str("double precision"),
            ColumnType::Decimal(precision) => match precision {
                Some((precision, scale)) => {
                    sql.write_str("decimal(").unwrap();
                    write!(sql, "{precision}").unwrap();
                    sql.write_str(", ").unwrap();
                    write!(sql, "{scale}").unwrap();
                    sql.write_char(')')
                }
                None => sql.write_str("decimal"),
            },
            ColumnType::DateTime => sql.write_str("timestamp without time zone"),
            ColumnType::Timestamp => sql.write_str("timestamp"),
            ColumnType::TimestampWithTimeZone => sql.write_str("timestamp with time zone"),
            ColumnType::Time => sql.write_str("time"),
            ColumnType::Date => sql.write_str("date"),
            ColumnType::Interval(fields, precision) => {
                sql.write_str("interval").unwrap();

                if let Some(fields) = fields {
                    write!(sql, " {fields}").unwrap();
                }

                if let Some(precision) = precision {
                    sql.write_char('(').unwrap();
                    write!(sql, "{precision}").unwrap();
                    sql.write_char(')').unwrap();
                }
                Ok(())
            }
            ColumnType::Binary(_) | ColumnType::VarBinary(_) | ColumnType::Blob => {
                sql.write_str("bytea")
            }
            ColumnType::Bit(length) => match length {
                Some(length) => {
                    sql.write_str("bit(").unwrap();
                    write!(sql, "{length}").unwrap();
                    sql.write_char(')')
                }
                None => sql.write_str("bit"),
            },
            ColumnType::VarBit(length) => {
                sql.write_str("varbit(").unwrap();
                write!(sql, "{length}").unwrap();
                sql.write_char(')')
            }
            ColumnType::Boolean => sql.write_str("bool"),
            ColumnType::Money(precision) => match precision {
                Some((precision, scale)) => {
                    sql.write_str("money(").unwrap();
                    write!(sql, "{precision}").unwrap();
                    sql.write_str(", ").unwrap();
                    write!(sql, "{scale}").unwrap();
                    sql.write_char(')')
                }
                None => sql.write_str("money"),
            },
            ColumnType::Json => sql.write_str("json"),
            ColumnType::JsonBinary => sql.write_str("jsonb"),
            ColumnType::Uuid => sql.write_str("uuid"),
            ColumnType::Array(elem_type) => {
                self.prepare_column_type(elem_type, sql);
                sql.write_str("[]")
            }
            ColumnType::Vector(size) => match size {
                Some(size) => {
                    sql.write_str("vector(").unwrap();
                    write!(sql, "{size}").unwrap();
                    sql.write_str(")")
                }
                None => sql.write_str("vector"),
            },
            ColumnType::Custom(iden) => sql.write_str(&iden.0),
            ColumnType::Enum { name, .. } => sql.write_str(&name.0),
            ColumnType::Cidr => sql.write_str("cidr"),
            ColumnType::Inet => sql.write_str("inet"),
            ColumnType::MacAddr => sql.write_str("macaddr"),
            ColumnType::Year => unimplemented!("Year is not available in Postgres."),
            ColumnType::LTree => sql.write_str("ltree"),
        }
        .unwrap()
    }

    fn column_spec_auto_increment_keyword(&self) -> &str {
        ""
    }

    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut dyn SqlWriter) {
        if alter.options.is_empty() {
            panic!("No alter option found")
        };
        sql.write_str("ALTER TABLE ").unwrap();
        if let Some(table) = &alter.table {
            self.prepare_table_ref_table_stmt(table, sql);
            sql.write_str(" ").unwrap();
        }

        let mut opts = alter.options.iter();

        intersperse_with!(
            opts,
            opt,
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                match opt {
                    TableAlterOption::AddColumn(AddColumnOption {
                        column,
                        if_not_exists,
                    }) => {
                        sql.write_str("ADD COLUMN ").unwrap();
                        if *if_not_exists {
                            sql.write_str("IF NOT EXISTS ").unwrap();
                        }
                        let f = |column_def: &ColumnDef, sql: &mut dyn SqlWriter| {
                            if let Some(column_type) = &column_def.types {
                                sql.write_str(" ").unwrap();
                                if column_def
                                    .spec
                                    .iter()
                                    .any(|v| matches!(v, ColumnSpec::AutoIncrement))
                                {
                                    self.prepare_column_auto_increment(column_type, sql);
                                } else {
                                    self.prepare_column_type(column_type, sql);
                                }
                            }
                        };
                        self.prepare_column_def_common(column, sql, f);
                    }
                    TableAlterOption::ModifyColumn(column_def) => {
                        if let Some(column_type) = &column_def.types {
                            sql.write_str("ALTER COLUMN ").unwrap();
                            self.prepare_iden(&column_def.name, sql);
                            sql.write_str(" TYPE ").unwrap();
                            self.prepare_column_type(column_type, sql);
                        }
                        let first = column_def.types.is_none();

                        column_def.spec.iter().fold(first, |first, column_spec| {
                            if !first
                                && !matches!(
                                    column_spec,
                                    ColumnSpec::AutoIncrement
                                        | ColumnSpec::Generated { .. }
                                        | ColumnSpec::Using(_)
                                )
                            {
                                sql.write_str(", ").unwrap();
                            }
                            match column_spec {
                                ColumnSpec::AutoIncrement => {}
                                ColumnSpec::Null => {
                                    sql.write_str("ALTER COLUMN ").unwrap();
                                    self.prepare_iden(&column_def.name, sql);
                                    sql.write_str(" DROP NOT NULL").unwrap();
                                }
                                ColumnSpec::NotNull => {
                                    sql.write_str("ALTER COLUMN ").unwrap();
                                    self.prepare_iden(&column_def.name, sql);
                                    sql.write_str(" SET NOT NULL").unwrap()
                                }
                                ColumnSpec::Default(v) => {
                                    sql.write_str("ALTER COLUMN ").unwrap();
                                    self.prepare_iden(&column_def.name, sql);
                                    sql.write_str(" SET DEFAULT ").unwrap();
                                    QueryBuilder::prepare_simple_expr(self, v, sql);
                                }
                                ColumnSpec::UniqueKey => {
                                    sql.write_str("ADD UNIQUE (").unwrap();
                                    self.prepare_iden(&column_def.name, sql);
                                    sql.write_str(")").unwrap();
                                }
                                ColumnSpec::PrimaryKey => {
                                    sql.write_str("ADD PRIMARY KEY (").unwrap();
                                    self.prepare_iden(&column_def.name, sql);
                                    sql.write_str(")").unwrap();
                                }
                                ColumnSpec::Check(check) => {
                                    self.prepare_check_constraint(check, sql)
                                }
                                ColumnSpec::Generated { .. } => {}
                                ColumnSpec::Extra(string) => sql.write_str(string).unwrap(),
                                ColumnSpec::Comment(_) => {}
                                ColumnSpec::Using(expr) => {
                                    sql.write_str(" USING ").unwrap();
                                    QueryBuilder::prepare_simple_expr(self, expr, sql);
                                }
                            }
                            false
                        });
                    }
                    TableAlterOption::RenameColumn(from_name, to_name) => {
                        sql.write_str("RENAME COLUMN ").unwrap();
                        self.prepare_iden(from_name, sql);
                        sql.write_str(" TO ").unwrap();
                        self.prepare_iden(to_name, sql);
                    }
                    TableAlterOption::DropColumn(column_name) => {
                        sql.write_str("DROP COLUMN ").unwrap();
                        self.prepare_iden(column_name, sql);
                    }
                    TableAlterOption::DropForeignKey(name) => {
                        let mut foreign_key = TableForeignKey::new();
                        foreign_key.name(name.to_string());
                        let drop = ForeignKeyDropStatement {
                            foreign_key,
                            table: None,
                        };
                        self.prepare_foreign_key_drop_statement_internal(
                            &drop,
                            sql,
                            Mode::TableAlter,
                        );
                    }
                    TableAlterOption::AddForeignKey(foreign_key) => {
                        let create = ForeignKeyCreateStatement {
                            foreign_key: foreign_key.to_owned(),
                        };
                        self.prepare_foreign_key_create_statement_internal(
                            &create,
                            sql,
                            Mode::TableAlter,
                        );
                    }
                }
            }
        );
    }

    fn prepare_table_rename_statement(
        &self,
        rename: &TableRenameStatement,
        sql: &mut dyn SqlWriter,
    ) {
        sql.write_str("ALTER TABLE ").unwrap();
        if let Some(from_name) = &rename.from_name {
            self.prepare_table_ref_table_stmt(from_name, sql);
        }
        sql.write_str(" RENAME TO ").unwrap();
        if let Some(to_name) = &rename.to_name {
            self.prepare_table_ref_table_stmt(to_name, sql);
        }
    }
}

impl PostgresQueryBuilder {
    fn prepare_column_auto_increment(&self, column_type: &ColumnType, sql: &mut dyn SqlWriter) {
        match &column_type {
            ColumnType::SmallInteger => sql.write_str("smallserial").unwrap(),
            ColumnType::Integer => sql.write_str("serial").unwrap(),
            ColumnType::BigInteger => sql.write_str("bigserial").unwrap(),
            _ => unimplemented!("{:?} doesn't support auto increment", column_type),
        }
    }

    fn prepare_column_type_check_auto_increment(
        &self,
        column_def: &ColumnDef,
        sql: &mut dyn SqlWriter,
    ) {
        if let Some(column_type) = &column_def.types {
            let is_auto_increment = column_def
                .spec
                .iter()
                .position(|s| matches!(s, ColumnSpec::AutoIncrement));
            sql.write_str(" ").unwrap();
            if is_auto_increment.is_some() {
                self.prepare_column_auto_increment(column_type, sql);
            } else {
                self.prepare_column_type(column_type, sql);
            }
        }
    }

    fn prepare_column_def_common<F>(&self, column_def: &ColumnDef, sql: &mut dyn SqlWriter, f: F)
    where
        F: Fn(&ColumnDef, &mut dyn SqlWriter),
    {
        self.prepare_iden(&column_def.name, sql);

        f(column_def, sql);

        for column_spec in column_def.spec.iter() {
            if let ColumnSpec::AutoIncrement = column_spec {
                continue;
            }
            if let ColumnSpec::Comment(_) = column_spec {
                continue;
            }
            sql.write_str(" ").unwrap();
            self.prepare_column_spec(column_spec, sql);
        }
    }
}
