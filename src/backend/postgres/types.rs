use super::*;
use crate::{extension::postgres::*, utils::JoinWrite};

impl TypeBuilder for PostgresQueryBuilder {
    fn prepare_type_create_statement(
        &self,
        create: &TypeCreateStatement,
        sql: &mut impl SqlWriter,
    ) {
        match &create.as_type {
            Some(TypeAs::Enum) => {
                self.prepare_enum_type_create_statement(create.name.as_ref(), &create.values, sql);
            }
            Some(TypeAs::Composite) => {
                self.prepare_composite_type_create_statement(
                    create.name.as_ref(),
                    &create.fields,
                    sql,
                );
            }
            None => {
                sql.write_str("CREATE TYPE ").unwrap();
                if let Some(name) = &create.name {
                    self.prepare_type_ref(name, sql);
                }
            }
        }
    }

    fn prepare_type_drop_statement(&self, drop: &TypeDropStatement, sql: &mut impl SqlWriter) {
        sql.write_str("DROP TYPE ").unwrap();

        if drop.if_exists {
            sql.write_str("IF EXISTS ").unwrap();
        }

        JoinWrite {
            buf: sql,
            items: &drop.names,
            at_first: |_| Ok(()),
            r#do: |sql, name| {
                self.prepare_type_ref(name, sql);
                Ok(())
            },
            join: |sql| sql.write_str(", "),
            at_last: |_| Ok(()),
        }
        .exec()
        .unwrap();

        if let Some(option) = &drop.option {
            sql.write_str(" ").unwrap();
            self.prepare_drop_type_opt(option, sql);
        }
    }

    fn prepare_type_alter_statement(&self, alter: &TypeAlterStatement, sql: &mut impl SqlWriter) {
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
    pub(crate) fn prepare_enum_type_create_statement(
        &self,
        name: Option<&TypeRef>,
        values: &[DynIden],
        sql: &mut impl SqlWriter,
    ) {
        sql.write_str("CREATE TYPE ").unwrap();
        if let Some(name) = name {
            self.prepare_type_ref(name, sql);
        }
        sql.write_str(" AS ENUM (").unwrap();
        JoinWrite {
            buf: sql,
            items: values,
            at_first: |_| Ok(()),
            r#do: |sql, value| {
                self.prepare_value(value.to_string().into(), sql);
                Ok(())
            },
            join: |sql| sql.write_str(", "),
            at_last: |_| Ok(()),
        }
        .exec()
        .unwrap();
        sql.write_str(")").unwrap();
    }

    pub(crate) fn prepare_composite_type_create_statement(
        &self,
        name: Option<&TypeRef>,
        fields: &[CompositeFieldType],
        sql: &mut impl SqlWriter,
    ) {
        sql.write_str("CREATE TYPE ").unwrap();
        if let Some(name) = name {
            self.prepare_type_ref(name, sql);
        }
        sql.write_str(" AS (").unwrap();
        JoinWrite {
            buf: sql,
            items: fields,
            at_first: |_| Ok(()),
            r#do: |sql, field| {
                self.prepare_iden(&field.name, sql);
                sql.write_str(" ")?;
                self.prepare_column_type(&field.col_type, sql);
                Ok(())
            },
            join: |sql| sql.write_str(", "),
            at_last: |_| Ok(()),
        }
        .exec()
        .unwrap();
        sql.write_str(")").unwrap();
    }

    fn prepare_drop_type_opt(&self, opt: &TypeDropOpt, sql: &mut impl SqlWriter) {
        sql.write_str(match opt {
            TypeDropOpt::Cascade => "CASCADE",
            TypeDropOpt::Restrict => "RESTRICT",
        })
        .unwrap();
    }

    fn prepare_alter_type_opt(&self, opt: &TypeAlterOpt, sql: &mut impl SqlWriter) {
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
            TypeAlterOpt::AddAttribute(field) => {
                sql.write_str(" ADD ATTRIBUTE ").unwrap();
                self.prepare_iden(&field.name, sql);
                sql.write_str(" ").unwrap();
                self.prepare_column_type(&field.col_type, sql);
            }
            TypeAlterOpt::DropAttribute(name) => {
                sql.write_str(" DROP ATTRIBUTE ").unwrap();
                self.prepare_iden(name, sql);
            }
        }
    }
}
