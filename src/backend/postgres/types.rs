use super::*;
use crate::extension::postgres::*;

impl TypeBuilder for PostgresQueryBuilder {
    fn prepare_type_create_statement(&self, create: &TypeCreateStatement, sql: &mut dyn SqlWriter) {
        write!(sql, "CREATE TYPE ").unwrap();

        if let Some(name) = &create.name {
            self.prepare_type_ref(name, sql);
        }

        if let Some(as_type) = &create.as_type {
            write!(sql, " AS ").unwrap();
            self.prepare_create_as_type(as_type, sql);
        }

        if !create.values.is_empty() {
            write!(sql, " (").unwrap();

            for (count, val) in create.values.iter().enumerate() {
                if count > 0 {
                    write!(sql, ", ").unwrap();
                }
                self.prepare_value(&val.to_string().into(), sql);
            }

            write!(sql, ")").unwrap();
        }
    }

    fn prepare_type_drop_statement(&self, drop: &TypeDropStatement, sql: &mut dyn SqlWriter) {
        write!(sql, "DROP TYPE ").unwrap();

        if drop.if_exists {
            write!(sql, "IF EXISTS ").unwrap();
        }

        drop.names.iter().fold(true, |first, name| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            self.prepare_type_ref(name, sql);
            false
        });

        if let Some(option) = &drop.option {
            write!(sql, " ").unwrap();
            self.prepare_drop_type_opt(option, sql);
        }
    }

    fn prepare_type_alter_statement(&self, alter: &TypeAlterStatement, sql: &mut dyn SqlWriter) {
        write!(sql, "ALTER TYPE ").unwrap();

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
        write!(
            sql,
            "{}",
            match as_type {
                TypeAs::Enum => "ENUM",
            }
        )
        .unwrap()
    }

    fn prepare_drop_type_opt(&self, opt: &TypeDropOpt, sql: &mut dyn SqlWriter) {
        write!(
            sql,
            "{}",
            match opt {
                TypeDropOpt::Cascade => "CASCADE",
                TypeDropOpt::Restrict => "RESTRICT",
            }
        )
        .unwrap()
    }

    fn prepare_alter_type_opt(&self, opt: &TypeAlterOpt, sql: &mut dyn SqlWriter) {
        match opt {
            TypeAlterOpt::Add(value, placement) => {
                write!(sql, " ADD VALUE ").unwrap();
                match placement {
                    Some(add_option) => match add_option {
                        TypeAlterAddOpt::Before(before_value) => {
                            self.prepare_value(&value.to_string().into(), sql);
                            write!(sql, " BEFORE ").unwrap();
                            self.prepare_value(&before_value.to_string().into(), sql);
                        }
                        TypeAlterAddOpt::After(after_value) => {
                            self.prepare_value(&value.to_string().into(), sql);
                            write!(sql, " AFTER ").unwrap();
                            self.prepare_value(&after_value.to_string().into(), sql);
                        }
                    },
                    None => self.prepare_value(&value.to_string().into(), sql),
                }
            }
            TypeAlterOpt::Rename(new_name) => {
                write!(sql, " RENAME TO ").unwrap();
                self.prepare_value(&new_name.to_string().into(), sql);
            }
            TypeAlterOpt::RenameValue(existing, new_name) => {
                write!(sql, " RENAME VALUE ").unwrap();
                self.prepare_value(&existing.to_string().into(), sql);
                write!(sql, " TO ").unwrap();
                self.prepare_value(&new_name.to_string().into(), sql);
            }
        }
    }
}
