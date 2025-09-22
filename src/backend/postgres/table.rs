use super::*;
use crate::write_int;

impl TableBuilder for PostgresQueryBuilder {
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut impl SqlWriter) {
        fn f(this: &PostgresQueryBuilder, column_def: &ColumnDef, sql: &mut impl SqlWriter) {
            this.prepare_column_type_check_auto_increment(column_def, sql);
        }

        self.prepare_column_def_common(column_def, sql, |column_def, sql| f(self, column_def, sql));
    }

    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut impl SqlWriter) {
        match column_type {
            ColumnType::Char(length) => match length {
                Some(length) => {
                    sql.write_str("char(").unwrap();
                    write_int(sql, *length);
                    sql.write_char(')')
                }
                None => sql.write_str("char"),
            },
            ColumnType::String(length) => match length {
                StringLen::N(length) => {
                    sql.write_str("varchar(").unwrap();
                    write_int(sql, *length);
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
                    write_int(sql, *precision);
                    sql.write_str(", ").unwrap();
                    write_int(sql, *scale);
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
                    write_int(sql, *precision);
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
                    write_int(sql, *length);
                    sql.write_char(')')
                }
                None => sql.write_str("bit"),
            },
            ColumnType::VarBit(length) => {
                sql.write_str("varbit(").unwrap();
                write_int(sql, *length);
                sql.write_char(')')
            }
            ColumnType::Boolean => sql.write_str("bool"),
            ColumnType::Money(precision) => match precision {
                Some((precision, scale)) => {
                    sql.write_str("money(").unwrap();
                    write_int(sql, *precision);
                    sql.write_str(", ").unwrap();
                    write_int(sql, *scale);
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
                    write_int(sql, *size);
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

    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut impl SqlWriter) {
        if alter.options.is_empty() {
            panic!("No alter option found")
        };
        sql.write_str("ALTER TABLE ").unwrap();
        if let Some(table) = &alter.table {
            self.prepare_table_ref_table_stmt(table, sql);
            sql.write_str(" ").unwrap();
        }

        let mut opts = alter.options.iter();

        join_io!(
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

                        self.prepare_column_def_common(column, sql, |column_def, sql| {
                            if let Some(column_type) = &column_def.types {
                                write!(sql, " ").unwrap();
                                if column_def.spec.auto_increment {
                                    self.prepare_column_auto_increment(column_type, sql);
                                } else {
                                    self.prepare_column_type(column_type, sql);
                                }
                            }
                        });
                    }
                    TableAlterOption::ModifyColumn(column_def) => {
                        self.prepare_modify_column(sql, column_def);
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
        sql: &mut impl SqlWriter,
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
    fn prepare_column_auto_increment(&self, column_type: &ColumnType, sql: &mut impl SqlWriter) {
        let num_ty = match column_type {
            ColumnType::SmallInteger => "smallint GENERATED BY DEFAULT AS IDENTITY",
            ColumnType::Integer => "integer GENERATED BY DEFAULT AS IDENTITY",
            ColumnType::BigInteger => "bigint GENERATED BY DEFAULT AS IDENTITY",
            _ => unimplemented!("{:?} doesn't support auto increment", column_type),
        };

        sql.write_str(num_ty).unwrap();
    }

    fn prepare_column_type_check_auto_increment(
        &self,
        column_def: &ColumnDef,
        sql: &mut impl SqlWriter,
    ) {
        if let Some(column_type) = &column_def.types {
            let is_auto_increment = column_def.spec.auto_increment;

            sql.write_str(" ").unwrap();

            if is_auto_increment {
                self.prepare_column_auto_increment(column_type, sql);
            } else {
                self.prepare_column_type(column_type, sql);
            }
        }
    }

    fn prepare_column_def_common<F, W>(&self, column_def: &ColumnDef, sql: &mut W, f: F)
    where
        F: Fn(&ColumnDef, &mut W),
        W: SqlWriter,
    {
        self.prepare_iden(&column_def.name, sql);

        f(column_def, sql);

        self.prepare_column_spec(&column_def.spec, sql);
    }

    fn prepare_modify_column(&self, sql: &mut impl SqlWriter, column_def: &ColumnDef) {
        let mut is_first = true;

        macro_rules! write_comma_if_not_first {
            () => {
                if !is_first {
                    write!(sql, ", ").unwrap();
                } else {
                    is_first = false
                }
            };
        }

        if let Some(column_type) = &column_def.types {
            write!(sql, "ALTER COLUMN ").unwrap();
            self.prepare_iden(&column_def.name, sql);
            write!(sql, " TYPE ").unwrap();
            self.prepare_column_type(column_type, sql);
            is_first = false;
        }

        if column_def.spec.auto_increment {
            //
        }

        if let Some(nullable) = column_def.spec.nullable {
            write_comma_if_not_first!();
            write!(sql, "ALTER COLUMN ").unwrap();
            self.prepare_iden(&column_def.name, sql);
            if nullable {
                write!(sql, " DROP NOT NULL").unwrap();
            } else {
                write!(sql, " SET NOT NULL").unwrap();
            }
        }

        if let Some(default) = &column_def.spec.default {
            write_comma_if_not_first!();
            write!(sql, "ALTER COLUMN ").unwrap();
            self.prepare_iden(&column_def.name, sql);
            write!(sql, " SET DEFAULT ").unwrap();
            QueryBuilder::prepare_expr(self, default, sql);
        }
        if column_def.spec.unique {
            write_comma_if_not_first!();
            write!(sql, "ADD UNIQUE (").unwrap();
            self.prepare_iden(&column_def.name, sql);
            write!(sql, ")").unwrap();
        }
        if column_def.spec.primary_key {
            write_comma_if_not_first!();
            write!(sql, "ADD PRIMARY KEY (").unwrap();
            self.prepare_iden(&column_def.name, sql);
            write!(sql, ")").unwrap();
        }
        if let Some(check) = &column_def.spec.check {
            write_comma_if_not_first!();
            self.prepare_check_constraint(check, sql);
        }

        if let Some(x) = &column_def.spec.generated {
            let _ = x;
        }

        if let Some(x) = &column_def.spec.comment {
            let _ = x;
        }

        if let Some(expr) = &column_def.spec.using {
            write!(sql, " USING ").unwrap();
            QueryBuilder::prepare_expr(self, expr, sql);
        }

        if let Some(extra) = &column_def.spec.extra {
            write!(sql, "{extra}").unwrap()
        }

        let _ = is_first;
    }
}
