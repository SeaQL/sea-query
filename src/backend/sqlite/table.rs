use super::*;

sqlite_impl! {
    TableBuilder {
        fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut dyn SqlWriter) {
            column_def.name.prepare(sql.as_writer(), self.quote());

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
                if let ColumnSpec::Comment(_) = column_spec {
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

        fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut dyn SqlWriter) {
            <
                <Self as backend::sqlite::SqliteBuilderVariant>::TypeMapper as StaticTypeMapper
            >::prepare_column_type(column_type, sql)
        }

        fn column_spec_auto_increment_keyword(&self) -> &str {
            "AUTOINCREMENT"
        }

        fn prepare_table_drop_opt(&self, _drop_opt: &TableDropOpt, _sql: &mut dyn SqlWriter) {
            // SQLite does not support table drop options
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
            write!(sql, "ALTER TABLE ").unwrap();
            if let Some(table) = &alter.table {
                self.prepare_table_ref_table_stmt(table, sql);
                write!(sql, " ").unwrap();
            }
            match &alter.options[0] {
                TableAlterOption::AddColumn(AddColumnOption {
                    column,
                    if_not_exists: _,
                }) => {
                    write!(sql, "ADD COLUMN ").unwrap();
                    self.prepare_column_def(column, sql);
                }
                TableAlterOption::ModifyColumn(_) => {
                    panic!("Sqlite not support modifying table column")
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
                    panic!("Sqlite does not support modification of foreign key constraints to existing tables");
                }
                TableAlterOption::AddForeignKey(_) => {
                    panic!("Sqlite does not support modification of foreign key constraints to existing tables");
                }
            }
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
}

#[derive(Default, Debug)]
pub struct DefaultTypeMapper;

#[derive(Default, Debug)]
pub struct ExactTypeMapper;

impl StaticTypeMapper for DefaultTypeMapper {
    fn prepare_column_type(column_type: &ColumnType, sql: &mut dyn SqlWriter) {
        write!(
            sql,
            "{}",
            match column_type {
                // ----------------------------------------------------------------------------------------------------
                // A NOTE REGRADING SQLITE TYPE AFFINITY: It's great that we try to be very on-point with our mappings,
                // but it turns out that SQLite pays absolutely no attention to any of this. SQLite's type affinity is
                // at the field level, and therefore a given column is not validated to contain a specific type, so
                // these types are really just "suggestions" rather than hard rules. It is because of this that
                // sea-query, by default, makes sacrifices on SQLite type correctness for the sake of schema discovery
                // ease. See, SQLite only has 5 proper field types: NULL, INTEGER, REAL, TEXT, and BLOB. This obviously
                // is not useful for schema discovery, but we can (ab)use the fact that these field types are not hard
                // requirements (and do not have to exactly match SQLite given that they're not used for column
                // validation) by pretending as if SQLite did actually have these stronger-typed columns. An example of
                // this is "BigInteger"/"BigUnsigned", which we map into BIGINT to aid schema discovery (i.e. so it
                // appears as a i64 instead of a i32) downstream. However, this is not appropriate for every use case,
                // as one example of where SQLite does actually care about the column type "suggestion" is for specific
                // constraints. To reuse the BIGINT example, AUTOINCREMENT is strictly only allowed on INTEGER column
                // types, meaning that any user hoping to use strong type hints but discard them where they do not
                // imply anything or only have negative implications are unable to do so. Therefore, the
                // StaticTypeMapper was introduced (alongside the SqliteTypedQueryBuilder) to allow the type mapping
                // to be customised. An example of this is the ExactTypeMapper.
                //
                // References:
                // https://www.sqlite.org/datatype3.html
                // https://www.sqlite.org/forum/info/2dfa968a702e1506e885cb06d92157d492108b22bf39459506ab9f7125bca7fd
                // https://github.com/SeaQL/sea-orm/issues/1832
                // https://github.com/SeaQL/sea-orm/issues/1067#issuecomment-1352302520
                // https://github.com/SeaQL/sea-query/pull/556
                // https://github.com/SeaQL/sea-query/issues/689
                // ----------------------------------------------------------------------------------------------------
                //
                // KEY TAKEAWAY: PLEASE UPDATE THE ExactTypeMapper FOR ANY FURTHER DEVIATIONS IN LINE WITH THE ABOVE!
                //
                // ----------------------------------------------------------------------------------------------------
                ColumnType::Char(length) => match length {
                    Some(length) => format!("text({length})"),
                    None => "text".into(),
                },
                ColumnType::String(length) => match length {
                    Some(length) => format!("text({length})"),
                    None => "text".into(),
                },
                ColumnType::Text => "text".into(),
                ColumnType::TinyInteger | ColumnType::TinyUnsigned => "integer".into(),
                ColumnType::SmallInteger | ColumnType::SmallUnsigned => "integer".into(),
                ColumnType::Integer | ColumnType::Unsigned => "integer".into(),
                // BIGINT is a deviation: this should be INTEGER, but has been made BIGINT for schema
                // discovery purposes. The exact type mapper preserves the INTEGER semantics.
                ColumnType::BigInteger | ColumnType::BigUnsigned => "bigint".into(),
                ColumnType::Float => "real".into(),
                ColumnType::Double => "real".into(),
                ColumnType::Decimal(precision) => match precision {
                    Some((precision, scale)) => format!("real({precision}, {scale})"),
                    None => "real".into(),
                },
                ColumnType::DateTime => "text".into(),
                ColumnType::Timestamp => "text".into(),
                ColumnType::TimestampWithTimeZone => "text".into(),
                ColumnType::Time => "text".into(),
                ColumnType::Date => "text".into(),
                ColumnType::Interval(_, _) => "unsupported".into(),
                ColumnType::Binary(blob_size) => match blob_size {
                    BlobSize::Blob(Some(length)) => format!("binary({length})"),
                    _ => "blob".into(),
                },
                ColumnType::VarBinary(length) => format!("binary({length})"),
                ColumnType::Boolean => "boolean".into(),
                ColumnType::Money(precision) => match precision {
                    Some((precision, scale)) => format!("integer({precision}, {scale})"),
                    None => "integer".into(),
                },
                ColumnType::Json => "text".into(),
                ColumnType::JsonBinary => "text".into(),
                ColumnType::Uuid => "text(36)".into(),
                ColumnType::Custom(iden) => iden.to_string(),
                ColumnType::Enum { .. } => "text".into(),
                ColumnType::Array(_) => unimplemented!("Array is not available in Sqlite."),
                ColumnType::Cidr => unimplemented!("Cidr is not available in Sqlite."),
                ColumnType::Inet => unimplemented!("Inet is not available in Sqlite."),
                ColumnType::MacAddr => unimplemented!("MacAddr is not available in Sqlite."),
                ColumnType::Year(_) => unimplemented!("Year is not available in Sqlite."),
                ColumnType::Bit(_) => unimplemented!("Bit is not available in Sqlite."),
                ColumnType::VarBit(_) => unimplemented!("VarBit is not available in Sqlite."),
            }
        )
        .unwrap()
    }
}

impl super::StaticTypeMapper for ExactTypeMapper {
    fn prepare_column_type(column_type: &ColumnType, sql: &mut dyn SqlWriter) {
        match column_type {
            ColumnType::BigInteger | ColumnType::BigUnsigned => write!(sql, "integer").unwrap(),
            _ => DefaultTypeMapper::prepare_column_type(column_type, sql),
        }
    }
}
