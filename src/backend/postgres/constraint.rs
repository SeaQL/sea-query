use super::*;

impl ConstraintBuilder for PostgresQueryBuilder {
    fn prepare_constraint_create_statement_internal(
        &self,
        create: &ConstraintCreateStatement,
        sql: &mut impl SqlWriter,
        mode: ConstraintMode,
    ) {
        let Some(constraint_type) = &create.constraint.constraint_type else {
            panic!("No constraint type found");
        };

        if mode == ConstraintMode::Alter {
            sql.write_str("ALTER TABLE ").unwrap();
            if let Some(table) = &create.table {
                self.prepare_table_ref_table_stmt(table, sql);
                sql.write_str(" ").unwrap();
            }
        }

        sql.write_str("ADD ").unwrap();

        match constraint_type {
            ConstraintCreateStatementType::Check(check) => {
                self.prepare_check_constraint(check, sql)
            }
            value => {
                if let Some(name) = &create.constraint.name {
                    sql.write_str("CONSTRAINT ").unwrap();
                    sql.write_char(self.quote().left()).unwrap();
                    sql.write_str(name).unwrap();
                    sql.write_char(self.quote().right()).unwrap();
                    sql.write_str(" ").unwrap();
                }

                match value {
                    ConstraintCreateStatementType::PrimaryKey => {
                        sql.write_str("PRIMARY KEY ").unwrap()
                    }
                    ConstraintCreateStatementType::Unique => {
                        sql.write_str("UNIQUE ").unwrap();
                    }
                    _ => unreachable!(),
                }

                if let Some(using_index) = &create.constraint.using_index {
                    sql.write_str("USING INDEX ").unwrap();
                    self.prepare_iden(using_index, sql);
                    sql.write_str(" ").unwrap();
                }

                if create.constraint.nulls_not_distinct {
                    sql.write_str("NULLS NOT DISTINCT ").unwrap();
                }

                self.prepare_index_columns(&create.constraint.index.columns, sql);

                if !create.constraint.include_columns.is_empty() {
                    sql.write_str(" ").unwrap();
                    self.prepare_include_columns(&create.constraint.include_columns, sql);
                }

                // Used only with `EXCLUDE` constraint
                // self.prepare_filter(&create.r#where, sql);
            }
        };
    }
}
