use super::*;

impl ConstraintBuilder for MysqlQueryBuilder {
    fn prepare_constraint_create_statement_internal(
        &self,
        create: &ConstraintCreateStatement,
        sql: &mut impl SqlWriter,
        mode: ConstraintMode,
    ) {
        let Some(constraint_type) = &create.constraint.constraint_type else {
            panic!("No constraint type found");
        };

        assert!(
            create.constraint.using_index.is_none(),
            "MySQL does not support USING INDEX in ADD CONSTRAINT"
        );
        assert!(
            !create.constraint.nulls_not_distinct,
            "MySQL does not support NULLS NOT DISTINCT in ADD CONSTRAINT"
        );
        assert!(
            create.constraint.include_columns.is_empty(),
            "MySQL does not support INCLUDE columns in ADD CONSTRAINT"
        );
        if matches!(constraint_type, ConstraintCreateStatementType::Check(_)) {
            assert!(
                create.constraint.index.name.is_none()
                    && create.constraint.index.columns.is_empty(),
                "MySQL does not support index options on CHECK constraints"
            );
        } else {
            assert!(
                !matches!(constraint_type, ConstraintCreateStatementType::PrimaryKey)
                    || create.constraint.index.name.is_none(),
                "MySQL does not support index names on PRIMARY KEY constraints"
            );
            if let Some(index_type) = &create.constraint.index_type {
                assert!(
                    matches!(index_type, IndexType::BTree | IndexType::Hash),
                    "MySQL supports only BTREE or HASH index types in ADD CONSTRAINT UNIQUE/PRIMARY KEY"
                );
            }
        }

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
                if let Some(constraint_name) = &create.constraint.name {
                    sql.write_str("CONSTRAINT ").unwrap();
                    sql.write_char(self.quote().left()).unwrap();
                    sql.write_str(constraint_name).unwrap();
                    sql.write_char(self.quote().right()).unwrap();
                    sql.write_str(" ").unwrap();
                }

                match value {
                    ConstraintCreateStatementType::PrimaryKey => {
                        sql.write_str("PRIMARY KEY ").unwrap()
                    }
                    ConstraintCreateStatementType::Unique => {
                        sql.write_str("UNIQUE KEY ").unwrap();
                    }
                    _ => unreachable!(),
                }

                if let Some(index_name) = &create.constraint.index.name {
                    sql.write_char(self.quote().left()).unwrap();
                    sql.write_str(index_name).unwrap();
                    sql.write_char(self.quote().right()).unwrap();
                    sql.write_str(" ").unwrap();
                }

                self.prepare_index_type(&create.constraint.index_type, sql);
                if matches!(create.constraint.index_type, Some(IndexType::FullText)) {
                    sql.write_str(" ").unwrap();
                }

                self.prepare_index_columns(&create.constraint.index.columns, sql);
            }
        }
    }
}
