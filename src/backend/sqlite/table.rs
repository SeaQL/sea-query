use super::*;

impl TableBuilder for SqliteQueryBuilder {
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut SqlWriter) {
        column_def.name.prepare(sql, '`');

        if let Some(column_type) = &column_def.types {
            write!(sql, " ").unwrap();
            self.prepare_column_type(column_type, sql);
        }

        let mut is_primary_key = false;
        let mut is_auto_increment = false;

        for column_spec in column_def.spec.iter() {
            if let ColumnSpec::PrimaryKey = column_spec {
                is_primary_key = true;
                continue;
            }
            if let ColumnSpec::AutoIncrement = column_spec {
                is_auto_increment = true;
                continue;
            }
            write!(sql, " ").unwrap();
            self.prepare_column_spec(column_spec, sql);
        }

        if is_primary_key {
            write!(sql, " ").unwrap();
            self.prepare_column_spec(&ColumnSpec::PrimaryKey, sql);
        }
        if is_auto_increment {
            write!(sql, " ").unwrap();
            self.prepare_column_spec(&ColumnSpec::AutoIncrement, sql);
        }
    }

    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut SqlWriter) {
        write!(
            sql,
            "{}",
            match column_type {
                ColumnType::Char(length) => match length {
                    Some(length) => format!("text({})", length),
                    None => "text".into(),
                },
                ColumnType::String(length) => match length {
                    Some(length) => format!("text({})", length),
                    None => "text".into(),
                },
                ColumnType::Text => "text".into(),
                ColumnType::TinyInteger(length) => match length {
                    Some(length) => format!("integer({})", length),
                    None => "integer".into(),
                },
                ColumnType::SmallInteger(length) => match length {
                    Some(length) => format!("integer({})", length),
                    None => "integer".into(),
                },
                ColumnType::Integer(length) => match length {
                    Some(length) => format!("integer({})", length),
                    None => "integer".into(),
                },
                ColumnType::BigInteger(length) => match length {
                    Some(length) => format!("integer({})", length),
                    None => "integer".into(),
                },
                ColumnType::Float(precision) => match precision {
                    Some(precision) => format!("real({})", precision),
                    None => "real".into(),
                },
                ColumnType::Double(precision) => match precision {
                    Some(precision) => format!("real({})", precision),
                    None => "real".into(),
                },
                ColumnType::Decimal(precision) => match precision {
                    Some((precision, scale)) => format!("real({}, {})", precision, scale),
                    None => "real".into(),
                },
                ColumnType::DateTime(precision) => match precision {
                    Some(precision) => format!("text({})", precision),
                    None => "text".into(),
                },
                ColumnType::Timestamp(precision) => match precision {
                    Some(precision) => format!("text({})", precision),
                    None => "text".into(),
                },
                ColumnType::TimestampWithTimeZone(precision) => match precision {
                    Some(precision) => format!("text({})", precision),
                    None => "text".into(),
                },
                ColumnType::Time(precision) => match precision {
                    Some(precision) => format!("text({})", precision),
                    None => "text".into(),
                },
                ColumnType::Date => "text".into(),
                ColumnType::Interval(_, _) => "unsupported".into(),
                ColumnType::Binary(length) => match length {
                    Some(length) => format!("binary({})", length),
                    None => "binary".into(),
                },
                ColumnType::Boolean => "integer".into(),
                ColumnType::Money(precision) => match precision {
                    Some((precision, scale)) => format!("integer({}, {})", precision, scale),
                    None => "integer".into(),
                },
                ColumnType::Json => "text".into(),
                ColumnType::JsonBinary => "text".into(),
                ColumnType::Uuid => "text(36)".into(),
                ColumnType::Custom(iden) => iden.to_string(),
                ColumnType::Enum(_, _) => "text".into(),
            }
        )
        .unwrap()
    }

    fn prepare_column_spec(&self, column_spec: &ColumnSpec, sql: &mut SqlWriter) {
        match column_spec {
            ColumnSpec::Null => write!(sql, "NULL"),
            ColumnSpec::NotNull => write!(sql, "NOT NULL"),
            ColumnSpec::Default(value) => write!(sql, "DEFAULT {}", self.value_to_string(value)),
            ColumnSpec::AutoIncrement => write!(sql, "AUTOINCREMENT"),
            ColumnSpec::UniqueKey => write!(sql, "UNIQUE"),
            ColumnSpec::PrimaryKey => write!(sql, "PRIMARY KEY"),
            ColumnSpec::Extra(string) => write!(sql, "{}", string),
            ColumnSpec::Unsigned => Ok(()),
        }
        .unwrap()
    }

    fn prepare_table_drop_opt(&self, _drop_opt: &TableDropOpt, _sql: &mut dyn std::fmt::Write) {
        // SQLite does not support table drop options
    }

    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut SqlWriter) {
        let alter_option = match &alter.alter_option {
            Some(alter_option) => alter_option,
            None => panic!("No alter option found"),
        };
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(table) = &alter.table {
            table.prepare(sql, '`');
            write!(sql, " ").unwrap();
        }
        match alter_option {
            TableAlterOption::AddColumn(column_def) => {
                write!(sql, "ADD COLUMN ").unwrap();
                self.prepare_column_def(column_def, sql);
            }
            TableAlterOption::ModifyColumn(_) => {
                panic!("Sqlite not support modifying table column")
            }
            TableAlterOption::RenameColumn(from_name, to_name) => {
                write!(sql, "RENAME COLUMN ").unwrap();
                from_name.prepare(sql, '`');
                write!(sql, " TO ").unwrap();
                to_name.prepare(sql, '`');
            }
            TableAlterOption::DropColumn(_) => {
                panic!("Sqlite not support dropping table column")
            }
        }
    }

    fn prepare_table_rename_statement(&self, rename: &TableRenameStatement, sql: &mut SqlWriter) {
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(from_name) = &rename.from_name {
            from_name.prepare(sql, '`');
        }
        write!(sql, " RENAME TO ").unwrap();
        if let Some(to_name) = &rename.to_name {
            to_name.prepare(sql, '`');
        }
    }
}
