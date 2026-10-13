use super::*;

impl ConstraintBuilder for SqliteQueryBuilder {
    fn prepare_constraint_create_statement_internal(
        &self,
        _create: &ConstraintCreateStatement,
        _sql: &mut impl SqlWriter,
        _mode: ConstraintMode,
    ) {
        panic!("Sqlite does not support modification of constraints to existing tables");
    }
}
