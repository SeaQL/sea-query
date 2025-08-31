use super::*;

impl TableBuilder for SqliteQueryBuilder {
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut dyn SqlWriter) {
        self.prepare_iden(&column_def.name, sql);

        if let Some(column_type) = &column_def.types {
            sql.write_str(" ").unwrap();
            self.prepare_column_type(&column_def.spec, column_type, sql);
        }

        self.prepare_column_spec(&column_def.spec, sql);
    }

    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut dyn SqlWriter) {
        self.prepare_column_type(&ColumnSpec::default(), column_type, sql)
    }

    fn column_spec_auto_increment_keyword(&self) -> &str {
        " AUTOINCREMENT"
    }

    fn prepare_table_drop_opt(&self, _drop_opt: &TableDropOpt, _sql: &mut dyn SqlWriter) {
        // Sqlite does not support table drop options
    }

    fn prepare_table_truncate_statement(
        &self,
        _truncate: &TableTruncateStatement,
        _sql: &mut dyn SqlWriter,
    ) {
        panic!("Sqlite doesn't support TRUNCATE statement")
    }

    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut dyn SqlWriter) {
        if alter.options.is_empty() {
            panic!("No alter option found")
        }
        if alter.options.len() > 1 {
            panic!("Sqlite doesn't support multiple alter options")
        }
        sql.write_str("ALTER TABLE ").unwrap();
        if let Some(table) = &alter.table {
            self.prepare_table_ref_table_stmt(table, sql);
            sql.write_str(" ").unwrap();
        }
        match &alter.options[0] {
            TableAlterOption::AddColumn(AddColumnOption {
                column,
                if_not_exists: _,
            }) => {
                sql.write_str("ADD COLUMN ").unwrap();
                self.prepare_column_def(column, sql);
            }
            TableAlterOption::ModifyColumn(_) => {
                panic!("Sqlite not support modifying table column")
            }
            TableAlterOption::RenameColumn(from_name, to_name) => {
                sql.write_str("RENAME COLUMN ").unwrap();
                self.prepare_iden(from_name, sql);
                sql.write_str(" TO ").unwrap();
                self.prepare_iden(to_name, sql);
            }
            TableAlterOption::DropColumn(col_name) => {
                sql.write_str("DROP COLUMN ").unwrap();
                self.prepare_iden(col_name, sql);
            }
            TableAlterOption::DropForeignKey(_) => {
                panic!(
                    "Sqlite does not support modification of foreign key constraints to existing tables"
                );
            }
            TableAlterOption::AddForeignKey(_) => {
                panic!(
                    "Sqlite does not support modification of foreign key constraints to existing tables"
                );
            }
        }
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

impl SqliteQueryBuilder {
    fn prepare_column_type(
        &self,
        column_specs: &ColumnSpec,
        column_type: &ColumnType,
        sql: &mut dyn SqlWriter,
    ) {
        let is_auto_increment = column_specs.auto_increment;

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
            ColumnType::TinyInteger | ColumnType::TinyUnsigned => sql.write_str(integer("tinyint")),
            ColumnType::SmallInteger | ColumnType::SmallUnsigned => {
                sql.write_str(integer("smallint"))
            }
            ColumnType::Integer | ColumnType::Unsigned => sql.write_str("integer"),
            #[allow(clippy::if_same_then_else)]
            ColumnType::BigInteger | ColumnType::BigUnsigned => {
                if is_auto_increment {
                    sql.write_str("integer")
                } else {
                    sql.write_str(integer("bigint"))
                }
            }
            ColumnType::Float => sql.write_str("float"),
            ColumnType::Double => sql.write_str("double"),
            ColumnType::Decimal(precision) => match precision {
                Some((precision, scale)) => {
                    if precision > &16 {
                        panic!("precision cannot be larger than 16");
                    }
                    sql.write_str("real(").unwrap();
                    write!(sql, "{precision}").unwrap();
                    sql.write_str(", ").unwrap();
                    write!(sql, "{scale}").unwrap();
                    sql.write_char(')')
                }
                None => sql.write_str("real"),
            },
            ColumnType::DateTime => sql.write_str("datetime_text"),
            ColumnType::Timestamp => sql.write_str("timestamp_text"),
            ColumnType::TimestampWithTimeZone => sql.write_str("timestamp_with_timezone_text"),
            ColumnType::Time => sql.write_str("time_text"),
            ColumnType::Date => sql.write_str("date_text"),
            ColumnType::Interval(_, _) => unimplemented!("Interval is not available in Sqlite."),
            ColumnType::Binary(length) => {
                sql.write_str("blob(").unwrap();
                write!(sql, "{length}").unwrap();
                sql.write_char(')')
            }
            ColumnType::VarBinary(length) => match length {
                StringLen::N(length) => {
                    sql.write_str("varbinary_blob(").unwrap();
                    write!(sql, "{length}").unwrap();
                    sql.write_char(')')
                }
                _ => sql.write_str("varbinary_blob"),
            },
            ColumnType::Blob => sql.write_str("blob"),
            ColumnType::Boolean => sql.write_str("boolean"),
            ColumnType::Money(precision) => match precision {
                Some((precision, scale)) => {
                    sql.write_str("real_money(").unwrap();
                    write!(sql, "{precision}").unwrap();
                    sql.write_str(", ").unwrap();
                    write!(sql, "{scale}").unwrap();
                    sql.write_char(')')
                }
                None => sql.write_str("real_money"),
            },
            ColumnType::Json => sql.write_str("json_text"),
            ColumnType::JsonBinary => sql.write_str("jsonb_text"),
            ColumnType::Uuid => sql.write_str("uuid_text"),
            ColumnType::Custom(iden) => sql.write_str(&iden.0),
            ColumnType::Enum { .. } => sql.write_str("enum_text"),
            ColumnType::Array(_) => unimplemented!("Array is not available in Sqlite."),
            ColumnType::Vector(_) => unimplemented!("Vector is not available in Sqlite."),
            ColumnType::Cidr => unimplemented!("Cidr is not available in Sqlite."),
            ColumnType::Inet => unimplemented!("Inet is not available in Sqlite."),
            ColumnType::MacAddr => unimplemented!("MacAddr is not available in Sqlite."),
            ColumnType::Year => unimplemented!("Year is not available in Sqlite."),
            ColumnType::Bit(_) => unimplemented!("Bit is not available in Sqlite."),
            ColumnType::VarBit(_) => unimplemented!("VarBit is not available in Sqlite."),
            ColumnType::LTree => unimplemented!("LTree is not available in Sqlite."),
        }
        .unwrap()
    }
}

fn integer(ty: &str) -> &str {
    if cfg!(feature = "option-sqlite-exact-column-type") {
        "integer"
    } else {
        ty
    }
}
