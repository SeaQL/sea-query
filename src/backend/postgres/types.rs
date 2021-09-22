use super::*;
use crate::extension::postgres::*;

impl TypeBuilder for PostgresQueryBuilder {
    fn prepare_type_create_statement<'a>(
        &'a self,
        create: &'a TypeCreateStatement,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<PostgresQueryBuilder>),
    ) {
        write!(sql, "CREATE TYPE ").unwrap();

        if let Some(name) = &create.name {
            name.prepare(sql, '"');
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
                self.prepare_value(val, sql, collector);
            }

            write!(sql, ")").unwrap();
        }
    }

    fn prepare_type_drop_statement<'a>(
        &self,
        drop: &'a TypeDropStatement,
        sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(&'a dyn QueryValue<PostgresQueryBuilder>),
    ) {
        write!(sql, "DROP TYPE ").unwrap();

        if drop.if_exists {
            write!(sql, "IF EXISTS ").unwrap();
        }

        for name in drop.names.iter() {
            name.prepare(sql, '"');
        }

        if let Some(option) = &drop.option {
            write!(sql, " ").unwrap();
            self.prepare_drop_type_opt(option, sql);
        }
    }

    fn prepare_type_alter_statement<'a>(
        &self,
        alter: &'a TypeAlterStatement,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<PostgresQueryBuilder>),
    ) {
        write!(sql, "ALTER TYPE ").unwrap();

        if let Some(name) = &alter.name {
            name.prepare(sql, '"');
        }

        if let Some(option) = &alter.option {
            self.prepare_alter_type_opt(option, sql, collector)
        }
    }
}

impl PostgresQueryBuilder {
    fn prepare_create_as_type(&self, as_type: &TypeAs, sql: &mut SqlWriter) {
        write!(
            sql,
            "{}",
            match as_type {
                TypeAs::Enum => "ENUM",
            }
        )
        .unwrap()
    }

    fn prepare_drop_type_opt(&self, opt: &TypeDropOpt, sql: &mut SqlWriter) {
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

    fn prepare_alter_type_opt<'a>(
        &self,
        opt: &'a TypeAlterOpt,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<PostgresQueryBuilder>),
    ) {
        match opt {
            TypeAlterOpt::Add(value, placement) => {
                write!(sql, " ADD VALUE ").unwrap();
                match placement {
                    Some(add_option) => match add_option {
                        TypeAlterAddOpt::Before(before_value) => {
                            self.prepare_value(value, sql, collector);
                            write!(sql, " BEFORE ").unwrap();
                            self.prepare_value(before_value, sql, collector);
                        }
                        TypeAlterAddOpt::After(after_value) => {
                            self.prepare_value(value, sql, collector);
                            write!(sql, " AFTER ").unwrap();
                            self.prepare_value(after_value, sql, collector);
                        }
                    },
                    None => self.prepare_value(value, sql, collector),
                }
            }
            TypeAlterOpt::Rename(new_name) => {
                write!(sql, " RENAME TO ").unwrap();
                self.prepare_value(new_name, sql, collector);
            }
            TypeAlterOpt::RenameValue(existing, new_name) => {
                write!(sql, " RENAME VALUE ").unwrap();
                self.prepare_value(existing, sql, collector);
                write!(sql, " TO ").unwrap();
                self.prepare_value(new_name, sql, collector);
            }
        }
    }
}
