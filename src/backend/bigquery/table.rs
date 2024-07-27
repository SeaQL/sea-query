use super::*;

impl TableBuilder for BigQueryQueryBuilder {
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut dyn SqlWriter) {
        column_def.name.prepare(sql.as_writer(), self.quote());

        if let Some(column_type) = &column_def.types {
            write!(sql, " ").unwrap();
            self.prepare_column_type(column_type, sql);
        }

        for column_spec in column_def.spec.iter() {
            if let ColumnSpec::PrimaryKey = column_spec {
                continue;
            }
            if let ColumnSpec::AutoIncrement = column_spec {
                continue;
            }
            if let ColumnSpec::Comment(_) = column_spec {
                continue;
            }
            write!(sql, " ").unwrap();
            self.prepare_column_spec(column_spec, sql);
        }
    }

    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut dyn SqlWriter) {
        write!(
            sql,
            "{}",
            match column_type {
                ColumnType::Char(length) => match length {
                    Some(length) => format!("STRING({length})"),
                    None => "STRING".into(),
                },
                ColumnType::String(length) => match length {
                    Some(length) => format!("STRING({length})"),
                    None => "STRING".into(),
                },
                ColumnType::Text => "STRING".into(),
                ColumnType::TinyInteger | ColumnType::TinyUnsigned => "INT64".into(),
                ColumnType::SmallInteger | ColumnType::SmallUnsigned => "INT64".into(),
                ColumnType::Integer | ColumnType::Unsigned => "INT64".into(),
                ColumnType::BigInteger | ColumnType::BigUnsigned => "INT64".into(),
                ColumnType::Float => "FLOAT64".into(),
                ColumnType::Double => "FLOAT64".into(),
                ColumnType::Decimal(precision) | ColumnType::Money(precision) => match precision {
                    Some((precision, scale)) => match scale {
                        0..=9 if precision.max(&1) <= precision && precision <= &(scale + 29) =>
                            format!("NUMERIC({precision}, {scale})"),
                        10..=38 if precision.max(&1) <= precision && precision <= &(scale + 38) =>
                            format!("BIGNUMERIC({precision}, {scale})"),
                        _ => panic!("Invalid precision and scale for NUMERIC type"),
                    },
                    None => "BIGNUMERIC".into(),
                },
                ColumnType::DateTime => "DATETIME".into(),
                ColumnType::Timestamp => "TIMESTAMP".into(),
                ColumnType::TimestampWithTimeZone => "TIMESTAMP".into(),
                ColumnType::Time => "TIME".into(),
                ColumnType::Date => "DATE".into(),
                ColumnType::Interval(_, _) => "INTERVAL".into(),
                ColumnType::Binary(blob_size) => match blob_size {
                    BlobSize::Blob(Some(length)) => format!("BYTES({length})"),
                    _ => "BYTES".into(),
                },
                ColumnType::VarBinary(length) => format!("BYTES({length})"),
                ColumnType::Boolean => "BOOL".into(),
                ColumnType::Json => "JSON".into(),
                ColumnType::JsonBinary => "JSON".into(),
                ColumnType::Uuid => "STRING(36)".into(),
                ColumnType::Custom(iden) => iden.to_string(),
                ColumnType::Enum { .. } => "STRING".into(),
                ColumnType::Array(col_type) => {
                    let mut sql = String::new();
                    self.prepare_column_type(col_type, &mut sql);
                    format!("ARRAY<{sql}>")
                }
                ColumnType::Cidr => unimplemented!("Cidr is not available in BigQuery."),
                ColumnType::Inet => unimplemented!("Inet is not available in BigQuery."),
                ColumnType::MacAddr => unimplemented!("MacAddr is not available in BigQuery."),
                ColumnType::Year(_) => unimplemented!("Year is not available in BigQuery."),
                ColumnType::Bit(_) => unimplemented!("Bit is not available in BigQuery."),
                ColumnType::VarBit(_) => unimplemented!("VarBit is not available in BigQuery."),
            }
        )
        .unwrap()
    }

    fn column_spec_auto_increment_keyword(&self) -> &str {
        panic!("BigQuery does not support auto increment");
    }

    fn prepare_table_drop_opt(&self, _drop_opt: &TableDropOpt, _sql: &mut dyn SqlWriter) {
        panic!("BigQuery does not support table drop option");
    }

    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut dyn SqlWriter) {
        if alter.options.is_empty() {
            panic!("No alter option found")
        }
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(table) = &alter.table {
            self.prepare_table_ref_table_stmt(table, sql);
            write!(sql, " ").unwrap();
        }
        alter.options.iter().fold(true, |first, option| {
            if !first {
                write!(sql, ", ").unwrap();
            };
            match option {
                TableAlterOption::AddColumn(AddColumnOption {
                    column,
                    if_not_exists: _,
                }) => {
                    write!(sql, "ADD COLUMN ").unwrap();
                    self.prepare_column_def(column, sql);
                }
                TableAlterOption::ModifyColumn(column_def) => {
                    if let Some(types) = &column_def.types {
                        write!(sql, "ALTER COLUMN ").unwrap();
                        column_def.name.prepare(sql.as_writer(), self.quote());
                        write!(sql, " SET DATA TYPE ").unwrap();
                        self.prepare_column_type(types, sql);
                    }
                    let first = column_def.types.is_none();

                    column_def.spec.iter().fold(first, |first, column_spec| {
                        if !first {
                            write!(sql, ", ").unwrap();
                        }
                        match column_spec {
                            ColumnSpec::AutoIncrement => {}
                            ColumnSpec::Null => {
                                write!(sql, "ALTER COLUMN ").unwrap();
                                column_def.name.prepare(sql.as_writer(), self.quote());
                                write!(sql, " DROP NOT NULL").unwrap();
                            }
                            ColumnSpec::NotNull => {
                                panic!("BigQuery doesn't support changing to REQUIRED")
                            }
                            ColumnSpec::Default(v) => {
                                write!(sql, "ALTER COLUMN ").unwrap();
                                column_def.name.prepare(sql.as_writer(), self.quote());
                                write!(sql, " SET DEFAULT ").unwrap();
                                QueryBuilder::prepare_simple_expr(self, v, sql);
                            }
                            ColumnSpec::UniqueKey => {
                                panic!("BigQuery doesn't support adding unique constraint")
                            }
                            ColumnSpec::PrimaryKey => {
                                panic!("BigQuery doesn't support adding primary key constraint")
                            }
                            ColumnSpec::Check(_check) => {
                                panic!("BigQuery doesn't support adding check constraint")
                            }
                            ColumnSpec::Generated { .. } => {}
                            ColumnSpec::Extra(string) => write!(sql, "{string}").unwrap(),
                            ColumnSpec::Comment(_) => {}
                        }
                        false
                    });
                }
                TableAlterOption::RenameColumn(from_name, to_name) => {
                    write!(sql, "RENAME COLUMN ").unwrap();
                    from_name.prepare(sql.as_writer(), self.quote());
                    write!(sql, " TO ").unwrap();
                    to_name.prepare(sql.as_writer(), self.quote());
                }
                TableAlterOption::DropColumn(col_name) => {
                    write!(sql, "DROP COLUMN ").unwrap();
                    col_name.prepare(sql.as_writer(), self.quote());
                }
                TableAlterOption::DropForeignKey(_) => {
                    panic!("BigQuery does not support modification of foreign key constraints to existing tables");
                }
                TableAlterOption::AddForeignKey(_) => {
                    panic!("BigQuery does not support modification of foreign key constraints to existing tables");
                }
            }
            false
        });
    }

    fn prepare_table_rename_statement(
        &self,
        rename: &TableRenameStatement,
        sql: &mut dyn SqlWriter,
    ) {
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(from_name) = &rename.from_name {
            self.prepare_table_ref_table_stmt(from_name, sql);
        }
        write!(sql, " RENAME TO ").unwrap();
        if let Some(to_name) = &rename.to_name {
            self.prepare_table_ref_table_stmt(to_name, sql);
        }
    }
}
