use super::*;
use crate::extension::postgres::*;

impl TypeBuilder for PostgresQueryBuilder {
    fn prepare_type_create_statement(&self, create: &TypeCreateStatement, sql: &mut dyn SqlWriter) {
        sql.write_str("CREATE TYPE ").unwrap();

        if let Some(name) = &create.name {
            self.prepare_type_ref(name, sql);
        }

        if let Some(as_type) = &create.as_type {
            sql.write_str(" AS ").unwrap();
            self.prepare_create_as_type(as_type, sql);
        }

        if !create.values.is_empty() {
            sql.write_str(" (").unwrap();

            let mut vals = create.values.iter();
            intersperse_with!(
                vals,
                val,
                join {
                    sql.write_str(", ").unwrap();
                },
                do {
                    self.prepare_value(val.to_string().into(), sql);
                }
            );

            sql.write_str(")").unwrap();
        }
    }

    fn prepare_type_drop_statement(&self, drop: &TypeDropStatement, sql: &mut dyn SqlWriter) {
        sql.write_str("DROP TYPE ").unwrap();

        if drop.if_exists {
            sql.write_str("IF EXISTS ").unwrap();
        }

        let mut names = drop.names.iter();

        intersperse_with!(
            names,
            name,
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                self.prepare_type_ref(name, sql);
            }
        );

        if let Some(option) = &drop.option {
            sql.write_str(" ").unwrap();
            self.prepare_drop_type_opt(option, sql);
        }
    }

    fn prepare_type_alter_statement(&self, alter: &TypeAlterStatement, sql: &mut dyn SqlWriter) {
        sql.write_str("ALTER TYPE ").unwrap();

        if let Some(name) = &alter.name {
            self.prepare_type_ref(name, sql);
        }

        if let Some(option) = &alter.option {
            self.prepare_alter_type_opt(option, sql)
        }
    }
}

impl PostgresQueryBuilder {
    fn prepare_create_as_type(&self, as_type: &TypeAs, sql: &mut dyn SqlWriter) {
        sql.write_str(match as_type {
            TypeAs::Enum => "ENUM",
        })
        .unwrap();
    }

    fn prepare_drop_type_opt(&self, opt: &TypeDropOpt, sql: &mut dyn SqlWriter) {
        sql.write_str(match opt {
            TypeDropOpt::Cascade => "CASCADE",
            TypeDropOpt::Restrict => "RESTRICT",
        })
        .unwrap();
    }

    fn prepare_alter_type_opt(&self, opt: &TypeAlterOpt, sql: &mut dyn SqlWriter) {
        match opt {
            TypeAlterOpt::Add {
                value,
                placement,
                if_not_exists,
            } => {
                sql.write_str(" ADD VALUE ").unwrap();
                if *if_not_exists {
                    sql.write_str("IF NOT EXISTS ").unwrap();
                }
                self.prepare_value(value.to_string().into(), sql);
                if let Some(add_option) = placement {
                    match add_option {
                        TypeAlterAddOpt::Before(before_value) => {
                            sql.write_str(" BEFORE ").unwrap();
                            self.prepare_value(before_value.to_string().into(), sql);
                        }
                        TypeAlterAddOpt::After(after_value) => {
                            sql.write_str(" AFTER ").unwrap();
                            self.prepare_value(after_value.to_string().into(), sql);
                        }
                    }
                }
            }
            TypeAlterOpt::Rename(new_name) => {
                sql.write_str(" RENAME TO ").unwrap();
                self.prepare_iden(new_name, sql);
            }
            TypeAlterOpt::RenameValue(existing, new_name) => {
                sql.write_str(" RENAME VALUE ").unwrap();
                self.prepare_value(existing.to_string().into(), sql);
                sql.write_str(" TO ").unwrap();
                self.prepare_value(new_name.to_string().into(), sql);
            }
        }
    }
}
