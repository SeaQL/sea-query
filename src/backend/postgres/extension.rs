use super::*;
use crate::extension::postgres::*;

impl ExtensionBuilder for PostgresQueryBuilder {
    fn prepare_extension_create_statement(
        &self,
        create: &ExtensionCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        sql.write_str("CREATE EXTENSION ").unwrap();

        if create.if_not_exists {
            sql.write_str("IF NOT EXISTS ").unwrap();
        }

        sql.write_str(&create.name).unwrap();

        if let Some(schema) = create.schema.as_ref() {
            sql.write_str(" WITH SCHEMA ").unwrap();
            sql.write_str(schema).unwrap()
        }

        if let Some(version) = create.version.as_ref() {
            sql.write_str(" VERSION ").unwrap();
            sql.write_str(version).unwrap();
        }

        if create.cascade {
            sql.write_str(" CASCADE").unwrap();
        }
    }

    fn prepare_extension_drop_statement(
        &self,
        drop: &ExtensionDropStatement,
        sql: &mut dyn SqlWriter,
    ) {
        sql.write_str("DROP EXTENSION ").unwrap();

        if drop.if_exists {
            sql.write_str("IF EXISTS ").unwrap();
        }

        sql.write_str(&drop.name).unwrap();

        if drop.cascade {
            sql.write_str(" CASCADE").unwrap();
        }

        if drop.restrict {
            sql.write_str(" RESTRICT").unwrap();
        }
    }
}
