use super::*;
use super::query::pg_value_to_string;

impl TableBuilder for PostgresQueryBuilder {
    fn prepare_table_create_statement(&self, create: &TableCreateStatement, sql: &mut SqlWriter) {
        write!(sql, "CREATE TABLE ").unwrap();

        if create.if_not_exists {
            write!(sql, "IF NOT EXISTS ").unwrap();
        }

        if let Some(table) = &create.table {
            table.prepare(sql, '"');
        }

        write!(sql, " ( ").unwrap();
        let mut count = 0;

        for column_def in create.columns.iter() {
            if count > 0 {
                write!(sql, ", ").unwrap();
            }
            self.prepare_column_def(column_def, sql);
            count += 1;
        }

        for index in create.indexes.iter() {
            if count > 0 {
                write!(sql, ", ").unwrap();
            }
            self.prepare_table_index_expression(index, sql);
            count += 1;
        }

        for foreign_key in create.foreign_keys.iter() {
            if count > 0 {
                write!(sql, ", ").unwrap();
            }
            self.prepare_foreign_key_create_statement_internal(foreign_key, sql, true);
            count += 1;
        }

        write!(sql, " )").unwrap();

        for table_opt in create.options.iter() {
            write!(sql, " ").unwrap();
            self.prepare_table_opt(table_opt, sql);
        }
    }

    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut SqlWriter) {
        column_def.name.prepare(sql, '"');

        self.prepare_column_type_check_auto_increment(column_def, sql);

        for column_spec in column_def.spec.iter() {
            if let ColumnSpec::AutoIncrement = column_spec {
                continue;
            }
            write!(sql, " ").unwrap();
            self.prepare_column_spec(column_spec, sql);
        }
    }

    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut SqlWriter) {
        write!(sql, "{}", match column_type {
            ColumnType::Char(length) => match length {
                Some(length) => format!("char({})", length),
                None => "char".into(),
            },
            ColumnType::String(length) => match length {
                Some(length) => format!("varchar({})", length),
                None => "varchar".into(),
            },
            ColumnType::Text => "text".into(),
            ColumnType::TinyInteger(length) => match length {
                Some(length) => format!("tinyint({})", length),
                None => "tinyint".into(),
            },
            ColumnType::SmallInteger(length) => match length {
                Some(length) => format!("smallint({})", length),
                None => "smallint".into(),
            },
            ColumnType::Integer(length) => match length {
                Some(length) => format!("integer({})", length),
                None => "integer".into(),
            },
            ColumnType::BigInteger(length) => match length {
                Some(length) => format!("bigint({})", length),
                None => "bigint".into(),
            },
            ColumnType::Float(precision) => match precision {
                Some(precision) => format!("real({})", precision),
                None => "real".into(),
            },
            ColumnType::Double(precision) => match precision {
                Some(precision) => format!("double precision({})", precision),
                None => "double precision".into(),
            },
            ColumnType::Decimal(precision) => match precision {
                Some((precision, scale)) => format!("decimal({}, {})", precision, scale),
                None => "decimal".into(),
            },
            ColumnType::DateTime(precision) => match precision {
                Some(precision) => format!("datetime({})", precision),
                None => "datetime".into(),
            },
            ColumnType::Timestamp(precision) => match precision {
                Some(precision) => format!("timestamp({})", precision),
                None => "timestamp".into(),
            },
            ColumnType::Time(precision) => match precision {
                Some(precision) => format!("time({})", precision),
                None => "time".into(),
            },
            ColumnType::Date => "date".into(),
            ColumnType::Binary(length) => match length {
                Some(length) => format!("binary({})", length),
                None => "binary".into(),
            },
            ColumnType::Boolean => "bool".into(),
            ColumnType::Money(precision) => match precision {
                Some((precision, scale)) => format!("money({}, {})", precision, scale),
                None => "money".into(),
            },
            ColumnType::Json => "json".into(),
            ColumnType::JsonBinary => "jsonb".into(),
            ColumnType::Custom(iden) => iden.to_string(),
        }).unwrap()
    }

    fn prepare_column_spec(&self, column_spec: &ColumnSpec, sql: &mut SqlWriter) {
        match column_spec {
            ColumnSpec::Null => write!(sql, "NULL"),
            ColumnSpec::NotNull => write!(sql, "NOT NULL"),
            ColumnSpec::Default(value) => write!(sql, "DEFAULT {}", pg_value_to_string(value)),
            ColumnSpec::AutoIncrement => write!(sql, ""),
            ColumnSpec::UniqueKey => write!(sql, "UNIQUE"),
            ColumnSpec::PrimaryKey => write!(sql, "PRIMARY KEY"),
            ColumnSpec::Extra(string) => write!(sql, "{}", string),
        }.unwrap()
    }

    fn prepare_table_opt(&self, table_opt: &TableOpt, sql: &mut SqlWriter) {
        write!(sql, "{}", match table_opt {
            TableOpt::Engine(s) => format!("ENGINE={}", s),
            TableOpt::Collate(s) => format!("COLLATE={}", s),
            TableOpt::CharacterSet(s) => format!("DEFAULT CHARSET={}", s),
        }).unwrap()
    }

    fn prepare_table_partition(&self, _table_partition: &TablePartition, _sql: &mut SqlWriter) {

    }

    fn prepare_table_drop_statement(&self, drop: &TableDropStatement, sql: &mut SqlWriter) {
        write!(sql, "DROP TABLE ").unwrap();

        if drop.if_exists {
            write!(sql, "IF EXISTS ").unwrap();
        }

        drop.tables.iter().fold(true, |first, table| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            table.prepare(sql, '"');
            false
        });

        for drop_opt in drop.options.iter() {
            write!(sql, " ").unwrap();
            self.prepare_table_drop_opt(drop_opt, sql);
        }
    }

    fn prepare_table_drop_opt(&self, drop_opt: &TableDropOpt, sql: &mut SqlWriter) {
        write!(sql, "{}", match drop_opt {
            TableDropOpt::Restrict => "RESTRICT",
            TableDropOpt::Cascade => "CASCADE",
        }).unwrap();
    }

    fn prepare_table_truncate_statement(&self, truncate: &TableTruncateStatement, sql: &mut SqlWriter) {
        write!(sql, "TRUNCATE TABLE ").unwrap();

        if let Some(table) = &truncate.table {
            table.prepare(sql, '"');
        }
    }

    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut SqlWriter) {
        let alter_option = match &alter.alter_option {
            Some(alter_option) => alter_option,
            None => panic!("No alter option found"),
        };
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(table) = &alter.table {
            table.prepare(sql, '"');
            write!(sql, " ").unwrap();
        }
        match alter_option {
            TableAlterOption::AddColumn(column_def) => {
                write!(sql, "ADD COLUMN ").unwrap();
                self.prepare_column_def(column_def, sql);
            },
            TableAlterOption::ModifyColumn(column_def) => {
                write!(sql, "ALTER COLUMN ").unwrap();
                column_def.name.prepare(sql, '"');
                write!(sql, " TYPE").unwrap();
                self.prepare_column_type_check_auto_increment(column_def, sql);
                for column_spec in column_def.spec.iter() {
                    if let ColumnSpec::AutoIncrement = column_spec {
                        continue;
                    }
                    write!(sql, ", ").unwrap();
                    write!(sql, "ALTER COLUMN ").unwrap();
                    column_def.name.prepare(sql, '"');
                    write!(sql, " SET ").unwrap();
                    self.prepare_column_spec(column_spec, sql);
                }
            },
            TableAlterOption::RenameColumn(from_name, to_name) => {
                write!(sql, "RENAME COLUMN ").unwrap();
                from_name.prepare(sql, '"');
                write!(sql, " TO ").unwrap();
                to_name.prepare(sql, '"');
            },
            TableAlterOption::DropColumn(column_name) => {
                write!(sql, "DROP COLUMN ").unwrap();
                column_name.prepare(sql, '"');
            },
        }
    }

    fn prepare_table_rename_statement(&self, rename: &TableRenameStatement, sql: &mut SqlWriter) {
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(from_name) = &rename.from_name {
            from_name.prepare(sql, '"');
        }
        write!(sql, " RENAME TO ").unwrap();
        if let Some(to_name) = &rename.to_name {
            to_name.prepare(sql, '"');
        }
    }
}

impl PostgresQueryBuilder {
    fn prepare_column_type_check_auto_increment(&self, column_def: &ColumnDef, sql: &mut SqlWriter) {
        if let Some(column_type) = &column_def.types {
            write!(sql, " ").unwrap();
            let is_auto_increment = column_def.spec.iter().position(|s| matches!(s, ColumnSpec::AutoIncrement));
            if is_auto_increment.is_some() {
                match &column_type {
                    ColumnType::SmallInteger(_) => write!(sql, "smallserial").unwrap(),
                    ColumnType::Integer(_) =>  write!(sql, "serial").unwrap(),
                    ColumnType::BigInteger(_) =>  write!(sql, "bigserial").unwrap(),
                    _ => unimplemented!(),
                }
            } else {
                self.prepare_column_type(&column_type, sql);
            }
        }
    }
}