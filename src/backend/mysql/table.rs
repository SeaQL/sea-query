use super::*;
use crate::write_int;

impl TableBuilder for MysqlQueryBuilder {
    fn prepare_table_opt(&self, create: &TableCreateStatement, sql: &mut impl SqlWriter) {
        // comment
        if let Some(comment) = &create.comment {
            sql.write_str(" COMMENT '").unwrap();
            self.write_escaped(sql, comment);
            sql.write_str("'").unwrap();
        }
        self.prepare_table_opt_def(create, sql)
    }

    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut impl SqlWriter) {
        self.prepare_iden(&column_def.name, sql);

        if let Some(column_type) = &column_def.types {
            sql.write_str(" ").unwrap();
            self.prepare_column_type(column_type, sql);
        }

        self.prepare_column_spec(&column_def.spec, sql);
    }

    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut impl SqlWriter) {
        match column_type {
            ColumnType::Char(length) => match length {
                Some(length) => {
                    sql.write_str("char(").unwrap();
                    write_int(sql, *length);
                    sql.write_str(")")
                }
                None => sql.write_str("char"),
            },
            ColumnType::String(length) => match length {
                StringLen::N(length) => {
                    sql.write_str("varchar(").unwrap();
                    write_int(sql, *length);
                    sql.write_char(')')
                }
                StringLen::None => sql.write_str("varchar(255)"),
                StringLen::Max => sql.write_str("varchar(65535)"),
            },
            ColumnType::Text => sql.write_str("text"),
            ColumnType::TinyInteger | ColumnType::TinyUnsigned => sql.write_str("tinyint"),
            ColumnType::SmallInteger | ColumnType::SmallUnsigned => sql.write_str("smallint"),
            ColumnType::Integer | ColumnType::Unsigned => sql.write_str("int"),
            ColumnType::BigInteger | ColumnType::BigUnsigned => sql.write_str("bigint"),
            ColumnType::Float => sql.write_str("float"),
            ColumnType::Double => sql.write_str("double"),
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
            ColumnType::DateTime => sql.write_str("datetime"),
            ColumnType::Timestamp => sql.write_str("timestamp"),
            ColumnType::TimestampWithTimeZone => sql.write_str("timestamp"),
            ColumnType::Time => sql.write_str("time"),
            ColumnType::Date => sql.write_str("date"),
            ColumnType::Year => sql.write_str("year"),
            ColumnType::Interval(_, _) => sql.write_str("unsupported"),
            ColumnType::Binary(length) => {
                sql.write_str("binary(").unwrap();
                write_int(sql, *length);
                sql.write_char(')')
            }
            ColumnType::VarBinary(length) => match length {
                StringLen::N(length) => {
                    sql.write_str("varbinary(").unwrap();
                    write_int(sql, *length);
                    sql.write_char(')')
                }
                StringLen::None => sql.write_str("varbinary(255)"),
                StringLen::Max => sql.write_str("varbinary(65535)"),
            },
            ColumnType::Blob => sql.write_str("blob"),
            ColumnType::Bit(length) => match length {
                Some(length) => {
                    sql.write_str("bit(").unwrap();
                    write_int(sql, *length);
                    sql.write_char(')')
                }
                None => sql.write_str("bit"),
            },
            ColumnType::VarBit(length) => {
                sql.write_str("bit(").unwrap();
                write_int(sql, *length);
                sql.write_char(')')
            }
            ColumnType::Boolean => sql.write_str("bool"),
            ColumnType::Money(precision) => match precision {
                Some((precision, scale)) => {
                    sql.write_str("decimal(").unwrap();
                    write_int(sql, *precision);
                    sql.write_str(", ").unwrap();
                    write_int(sql, *scale);
                    sql.write_char(')')
                }
                None => sql.write_str("decimal"),
            },
            ColumnType::Json => sql.write_str("json"),
            ColumnType::JsonBinary => sql.write_str("json"),
            ColumnType::Uuid => sql.write_str("binary(16)"),
            ColumnType::Custom(iden) => sql.write_str(&format!("{iden}")),
            ColumnType::Enum { variants, .. } => {
                sql.write_str("ENUM('").unwrap();

                let mut viter = variants.iter();
                join_io!(
                    viter,
                    variant,
                    join {
                        sql.write_str("', '").unwrap();
                    },
                    do {
                        sql.write_str(&variant.0).unwrap();
                    }
                );

                sql.write_str("')")
            }
            ColumnType::Array(_) => unimplemented!("Array is not available in MySQL."),
            ColumnType::Vector(_) => unimplemented!("Vector is not available in MySQL."),
            ColumnType::Cidr => unimplemented!("Cidr is not available in MySQL."),
            ColumnType::Inet => unimplemented!("Inet is not available in MySQL."),
            ColumnType::MacAddr => unimplemented!("MacAddr is not available in MySQL."),
            ColumnType::LTree => unimplemented!("LTree is not available in MySQL."),
        }
        .unwrap();

        if matches!(
            column_type,
            ColumnType::TinyUnsigned
                | ColumnType::SmallUnsigned
                | ColumnType::Unsigned
                | ColumnType::BigUnsigned
        ) {
            sql.write_str(" UNSIGNED").unwrap();
        }
    }

    fn column_spec_auto_increment_keyword(&self) -> &str {
        " AUTO_INCREMENT"
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
                        self.prepare_column_def(column, sql);
                    }
                    TableAlterOption::ModifyColumn(column_def) => {
                        sql.write_str("MODIFY COLUMN ").unwrap();
                        self.prepare_column_def(column_def, sql);
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
                };
            }
        );
    }

    fn prepare_table_rename_statement(
        &self,
        rename: &TableRenameStatement,
        sql: &mut impl SqlWriter,
    ) {
        sql.write_str("RENAME TABLE ").unwrap();
        if let Some(from_name) = &rename.from_name {
            self.prepare_table_ref_table_stmt(from_name, sql);
        }
        sql.write_str(" TO ").unwrap();
        if let Some(to_name) = &rename.to_name {
            self.prepare_table_ref_table_stmt(to_name, sql);
        }
    }

    /// column comment
    fn column_comment(&self, comment: &str, sql: &mut impl SqlWriter) {
        sql.write_str(" COMMENT '").unwrap();
        self.write_escaped(sql, comment);
        sql.write_str("'").unwrap();
    }
}
