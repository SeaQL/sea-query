use super::*;
use crate::extension::postgres::types::*;

impl TypeBuilder for PostgresQueryBuilder {
    fn prepare_type_create_statement(&self, create: &TypeCreateStatement, sql: &mut SqlWriter, collector: &mut dyn FnMut(Value)) {
        write!(sql, "CREATE TYPE ").unwrap();

        if let Some(name) = &create.name {
            name.prepare(sql, '"');
        }

        if let Some(as_type) = &create.as_type {
            write!(sql, " AS ").unwrap();
            self.prepare_create_as_type(&as_type, sql);
        }

        if !create.values.is_empty() {
            write!(sql, " (").unwrap();

            for (count, val) in create.values.iter().enumerate() {
                if count > 0 {
                    write!(sql, ", ").unwrap();
                }
                self.prepare_value(&val.to_string().into(), sql, collector);
            }

            write!(sql, ")").unwrap();
        }
    }

    fn prepare_type_drop_statement(&self, drop: &TypeDropStatement, sql: &mut SqlWriter, _collector: &mut dyn FnMut(Value)) {
        write!(sql, "DROP TYPE ").unwrap();

        if drop.if_exists {
            write!(sql, "IF EXISTS ").unwrap();
        }

        for name in drop.names.iter() {
            name.prepare(sql, '"');
        }

        if let Some(option) = &drop.option {        
            write!(sql, " ").unwrap();
            self.prepare_drop_type_opt(&option, sql);
        }
    }
}

impl PostgresQueryBuilder {
    fn prepare_create_as_type(&self, as_type: &TypeAs, sql: &mut SqlWriter) {
        write!(sql, "{}", match as_type {
            TypeAs::Enum => "ENUM",
        }).unwrap()
    }

    fn prepare_drop_type_opt(&self, opt: &TypeDropOpt, sql: &mut SqlWriter) {
        write!(sql, "{}", match opt {
            TypeDropOpt::Cascade => "CASCADE",
            TypeDropOpt::Restrict => "RESTRICT",
        }).unwrap()
    }
}