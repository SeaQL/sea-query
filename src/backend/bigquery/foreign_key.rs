use super::*;

impl ForeignKeyBuilder for BigQueryQueryBuilder {
    fn prepare_table_ref_fk_stmt(&self, _table_ref: &TableRef, _sql: &mut dyn SqlWriter) {
        panic!("Not supported")
    }

    fn prepare_foreign_key_drop_statement_internal(
        &self,
        _drop: &ForeignKeyDropStatement,
        _sql: &mut dyn SqlWriter,
        _mode: Mode,
    ) {
        panic!("Not supported")
    }

    fn prepare_foreign_key_create_statement_internal(
        &self,
        _create: &ForeignKeyCreateStatement,
        _sql: &mut dyn SqlWriter,
        _mode: Mode,
    ) {
        panic!("Not supported")
    }
}
